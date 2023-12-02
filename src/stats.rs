use core::fmt;
use std::fmt::Formatter;

use serde::Serialize;

#[derive(Serialize)]
pub struct Stats {
    pub total: u32,
    pub passed: u32,
    pub skipped: u32,
    pub failed: u32,
    pub percentile: f32,
}

impl Stats {
    pub fn new(total: u32) -> Self {
        Self {
            total,
            passed: 0,
            skipped: 0,
            failed: 0,
            percentile: 0.0,
        }
    }
    pub fn failed(&self) -> bool {
        return self.failed > 0;
    }
    pub fn calculate(&mut self) {
        let total = self.total - self.skipped;
        self.percentile = (100f32 / total as f32) * (self.passed as f32);
    }
    pub fn json(&self) -> anyhow::Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }
}

impl fmt::Display for Stats {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let formatted = indoc::formatdoc! {"
        {} passed
        {} skipped
        {} failed \
        ",
            self.passed,
            self.skipped,
            self.failed,

        };
        write!(f, "{}", formatted)
    }
}
