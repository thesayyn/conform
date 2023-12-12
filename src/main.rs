use clap::Parser;
use conform::report::{get_output, tap::Tap, Report};
use conform::runner::Runner;
use conform::stats::Stats;
use conform::test_case::TestCase;
use conform::{assert, ffi};
use std::borrow::BorrowMut;
use std::fs;

#[derive(Parser)]
#[command(author = "thesayyn", version = "0.0.0", about, long_about = None)]
struct Conform {
    #[arg(short, long, help = "conformance runner program")]
    program: String,
    #[arg(short, long, help = "where to write the report", default_value_t = String::from("-"))]
    output: String,

    #[arg(
        long,
        help = "stop the test runner at first error",
        default_value_t = false
    )]
    exit_early: bool,

    #[arg(long, help = "generate stats in json format")]
    json_stats: Option<String>,

    #[arg(long, help = "enforce recommended test", default_value_t = false)]
    enforce_recommended: bool,

    #[arg(
        long,
        help = "where to write the stderr from runner. possible values are either `ignore` or a file path.",
        default_value_t = String::from("ignore")
    )]
    runner_stderr: String,

    #[arg(long, help = "environment variables for the runner.", value_parser)]
    runner_env: Vec<String>,
}

fn parse_env(v: &String) -> (String, String) {
    let mut kv = v.split("=").into_iter();
    (
        kv.next().expect("failed to parse --runner-env").to_string(),
        kv.next().expect("").to_string(),
    )
}

fn main() {
    let cli = Conform::parse();

    let mut binding = ffi::extract_suite();
    let cases = binding.as_mut().unwrap();
    let cases_len = cases.len() as u32;

    let mut runner = Runner::new(&cli.program);
    let mut tap = Tap::new(get_output(&cli.output));
    let mut stats = Stats::new(cases_len);

    runner.set_env_all(cli.runner_env.iter().map(parse_env).collect());
    runner
        .set_stderr(cli.runner_stderr)
        .expect("failed to set stderr for the runner");

    tap.plan(0, cases_len);
    tap.diagnostic("conform - a better conformance test runner");

    let spawn = runner.spawn();
    if spawn.is_err() {
        tap.diagnostic(format!("{}", spawn.err().unwrap()));
    } else {
        tap.diagnostic(format!("{} is running now", cli.program));
    }

    for (pos, mut raw) in cases.iter_mut().borrow_mut().enumerate() {
        let num = pos as u32;
        let case = TestCase::from(&mut raw);

        let case_response = runner.send_case(&case);

        if case_response.is_err() {
            tap.not_ok(num, &case.name);
            tap.diagnostic(format!("{}", case_response.err().unwrap()));
            stats.failed += 1;
            break;
        }

        let assertion = assert::case::assert(&case, &case_response.unwrap());

        if !assertion.passed() {
            tap.not_ok(num, &case.name);
            tap.diagnostic(format!("{}", case));
            tap.diagnostic(format!("{}", assertion));

            stats.failed += 1;
            if cli.exit_early {
                break;
            }

            if case.is_recommended() && !cli.enforce_recommended {
                stats.failed -= 1;
            }
        } else {
            stats.passed += 1;
            tap.ok(num, &case.name)
        }
    }

    runner.kill().expect("failed to kill the runner");

    stats.calculate();

    if stats.failed() {
        tap.diagnostic(format!("test suite has failed ({}%)", stats.percentile));
    } else {
        tap.diagnostic(format!("test suite has succeded ({}%)", stats.percentile));
    };

    tap.diagnostic(format!("{}", stats));

    if let Some(path) = cli.json_stats {
        let json = stats.json().expect("failed to serialize stats into json");
        fs::write(path, json).expect("failed to write json stats");
    }
}
