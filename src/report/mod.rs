use auto_enums::auto_enum;
use std::{
    fs::File,
    io::{stdout, Write},
};

#[auto_enum(Write)]
pub fn get_output(report_to: &String) -> impl Write {
    if report_to == "-" {
        stdout()
    } else {
        File::create(report_to).expect("failed to open the report file")
    }
}

pub trait Report {
    fn plan(&mut self, start: u32, finish: u32);
    fn ok<S>(&mut self, number: u32, name: S)
    where
        S: Into<String>;
    fn not_ok<S>(&mut self, number: u32, name: S)
    where
        S: Into<String>;
    fn diagnostic<S>(&mut self, message: S)
    where
        S: Into<String>;
}

pub mod tap;
