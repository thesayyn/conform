use crate::ffi;
use base64::{engine::general_purpose::STANDARD, Engine};
use core::fmt;
use std::{fmt::Formatter, pin::Pin};

#[derive(PartialEq, Eq)]
pub enum Level {
    Required,
    Recommended,
}

impl From<i32> for Level {
    fn from(value: i32) -> Self {
        match value {
            0 => Self::Required,
            1 => Self::Recommended,
            _ => panic!("unknown level {}", value),
        }
    }
}

impl ToString for Level {
    fn to_string(&self) -> String {
        match self {
            Self::Recommended => String::from("Recommended"),
            Self::Required => String::from("Required"),
        }
    }
}

#[derive(PartialEq, Eq)]
pub enum AssertBy {
    Equivalence(Vec<u8>, bool),
    ParseError,
    SerializationError,
    Validator,
}

pub struct TestCase {
    pub name: String,
    pub level: Level,
    pub syntax: String,
    pub payload: Vec<u8>,
    pub assert_by: AssertBy,
}

impl TestCase {
    pub fn is_recommended(&self) -> bool {
        self.level == Level::Recommended
    }

    pub fn is_required(&self) -> bool {
        self.level == Level::Required
    }
}

impl From<&mut Pin<&mut ffi::Case>> for TestCase {
    fn from(value: &mut Pin<&mut ffi::Case>) -> Self {
        let assert_by = match value.as_mut().get_assert_by().to_string().as_str() {
            "equivalence" => AssertBy::Equivalence(
                value.as_mut().get_equivalent().as_bytes().to_vec(),
                value.as_mut().get_require_same_wire_format(),
            ),
            "f_parse" => AssertBy::ParseError,
            "f_serialize" => AssertBy::SerializationError,
            "json_validator" => AssertBy::Validator,
            v => panic!("unknown assertion type {}", &v)
         };
        Self {
            name: value.as_mut().get_name().to_string(),
            level: value.as_mut().get_level().0.into(),
            syntax: value.as_mut().get_syntax().to_string(),
            payload: value.as_mut().get_payload().as_bytes().to_vec(),
            assert_by,
        }
    }
}

impl fmt::Display for TestCase {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let formatted = indoc::formatdoc! {"
            syntax: {}
            level: {}
            payload: {} \
        ",
                self.syntax,
                self.level.to_string(),
                STANDARD.encode(&self.payload),
        };

        write!(f, "{}", formatted)
    }
}
