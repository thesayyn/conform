use clap::Parser;
use conform::case::Case;
use conform::report::{get_output, tap::Tap, Report};
use conform::runner::Runner;
use conform::stats::Stats;
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
}

fn main() {
    let cli = Conform::parse();

    let mut binding = ffi::extract_suite();
    let cases = binding.as_mut().unwrap();
    let cases_len = cases.len() as u32;

    let mut runner = Runner::new(&cli.program);
    let mut tap = Tap::new(get_output(&cli.output));
    let mut stats = Stats::new(cases_len);

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
        let case = Case::from(&mut raw);

        let case_response = runner.send_case(&case);

        if case_response.is_err() {
            tap.not_ok(num, &case.name);
            tap.diagnostic(format!("{}", case_response.err().unwrap()));
            stats.skipped += 1;
            break;
        }

        let assertion =
            assert::case(&case, &case_response.unwrap()).unwrap_or_else(|e| assert::CaseResult {
                diagnostics: vec![e.to_string()],
                passed: false,
                skipped: false,
            });

        if !assertion.passed {
            tap.not_ok(num, &case.name);
            tap.diagnostic(assertion.diagnostics.join("\n"));
            tap.diagnostic(format!("{}", case));

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
