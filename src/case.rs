use crate::ffi;
use base64::{engine::general_purpose::STANDARD, Engine};
use core::fmt;
use std::{
    fmt::Formatter,
    pin::Pin,
};

pub struct Case {
    pub name: String,
    pub level: String,
    pub syntax: String,
    pub payload: Vec<u8>,
    pub equivalent: Vec<u8>,
    pub require_same_write_format: bool,
}

impl Case {
    pub fn is_recommended(&self) -> bool { 
        self.level == "Required"
    }

    pub fn is_required(&self) -> bool {
       self.level == "Recommended"
    }
}

impl From<&mut Pin<&mut ffi::Case>> for Case {
    fn from(value: &mut Pin<&mut ffi::Case>) -> Self {
        Self {
            name: value.as_mut().get_name().to_string(),
            level: value.as_mut().get_level().to_string(),
            syntax: value.as_mut().get_syntax().to_string(),
            equivalent: value.as_mut().get_equivalent().as_bytes().to_vec(),
            payload: value.as_mut().get_payload().as_bytes().to_vec(),
            require_same_write_format: value.as_mut().get_require_same_wire_format()
        }
    }
}

impl fmt::Display for Case {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let formatted = indoc::formatdoc! {"
            syntax: {}
            level: {}
            payload: {} \
",
                self.syntax,
                self.level,
                STANDARD.encode(&self.payload),
        };

        write!(f, "{}", formatted)
    }
}
