use std::io::Write;

use crate::report::Report;

/// Global constant for the "ok"
const OK_SYMBOL: &str = "ok";
/// Global constant for the "not ok"
const NOT_OK_SYMBOL: &str = "not ok";

pub struct Tap<T: Write> {
    stream: T,
}

impl<T> Tap<T>
where
    T: Write,
{
    pub fn new(stream: T) -> Self {
        Self { stream }
    }
}

impl<T> Report for Tap<T>
where
    T: Write,
{
    fn plan(&mut self, start: u32, finish: u32) {
        writeln!(self.stream, "TAP version 14").unwrap();
        writeln!(self.stream, "{}..{}", start, finish).unwrap();
    }

    fn ok<S>(&mut self, number: u32, name: S)
    where
        S: Into<String>,
    {
        writeln!(self.stream, "{} {} - {}", OK_SYMBOL, number, name.into()).unwrap();
    }

    fn not_ok<S>(&mut self, number: u32, name: S)
    where
        S: Into<String>,
    {
        writeln!(
            self.stream,
            "{} {} - {}",
            NOT_OK_SYMBOL,
            number,
            name.into()
        )
        .unwrap();
    }

    fn diagnostic<S>(&mut self, message: S)
    where
        S: Into<String>,
    {
        let mut m: String = message.into();
        if m.contains("\n") {
            m = m.replace("\n", "\n# ")
        }

        writeln!(self.stream, "# {}", m).unwrap();
    }
}
