use std::fmt::{self, Formatter};

use super::conformance::{self, conformance_response::Result};
use super::differ;
use super::message::TestMessage;
use super::validator::Validator;
use crate::test_case::{AssertBy, TestCase};
use anyhow::{anyhow, Context, Ok};
use protobuf::{text_format, Message};

impl Result {
    pub fn is_err(&self) -> bool {
        match &self {
            Self::ParseError(_)
            | Self::RuntimeError(_)
            | Self::TimeoutError(_)
            | Self::SerializeError(_) => true,
            _ => false,
        }
    }
    pub fn is_parse_error(&self) -> bool {
        match &self {
            Self::ParseError(_) => true,
            _ => false,
        }
    }
    pub fn is_serialize_error(&self) -> bool {
        match &self {
            Self::SerializeError(_) => true,
            _ => false,
        }
    }
    pub fn is_skipped(&self) -> bool {
        if let Self::Skipped(_) = &self {
            return true;
        }
        false
    }
}

impl ToString for Result {
    fn to_string(&self) -> String {
        match self {
            Result::ParseError(r) => format!("parse error: {}", r),
            Result::RuntimeError(r) => format!("runtime error: {}", r),
            Result::SerializeError(r) => format!("serialize error: {}", r),
            Result::TimeoutError(r) => format!("timeout error: {}", r),
            Result::Skipped(r) => format!("skipped: {}", r),
            Result::JsonPayload(r) => {
                format!("json payload: [redacted payload] {} bytes", r.len())
            }
            Result::ProtobufPayload(r) => {
                format!("protobuf payload: [redacted payload] {} bytes", r.len())
            }
            Result::JspbPayload(r) => {
                format!("jspb payload: [redacted payload] {} bytes", r.len())
            }
            Result::TextPayload(r) => {
                format!("text payload: [redacted payload] {} bytes", r.len())
            }
        }
    }
}

#[derive(Default)]
pub struct Outcome {
    pub case: Case,
    pub diagnostics: Vec<String>,
}

#[derive(Default, PartialEq, Eq)]
pub enum Case {
    Skipped,
    #[default]
    Failed,
    Passed,
}

impl From<bool> for Case {
    fn from(passed: bool) -> Self {
        if passed {
            Self::Passed
        } else {
            Self::Failed
        }
    }
}

impl Outcome {
    fn set_case(mut self, case: Case) -> Self {
        self.case = case;
        self
    }
    fn add_diagnostic(&mut self, diagnostic: String) {
        self.diagnostics.push(diagnostic);
    }

    pub fn passed(&self) -> bool {
        self.case == Case::Passed
    }
    pub fn failed(&self) -> bool {
        self.case == Case::Failed
    }
    pub fn skipped(&self) -> bool {
        self.case == Case::Skipped
    }
}

impl fmt::Display for Outcome {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.diagnostics.join("\n"))
    }
}

pub fn assert(case: &TestCase, res: &Vec<u8>) -> Outcome {
    assert_inner(case, res).unwrap_or_else(|e| {
        let mut outcome = Outcome::default().set_case(Case::Failed);
        outcome.add_diagnostic(e.to_string());
        outcome
    })
}

pub fn assert_inner(case: &TestCase, res: &Vec<u8>) -> anyhow::Result<Outcome> {
    let response = conformance::ConformanceResponse::parse_from_bytes(res)
        .context("failed to parse response")?;
    let request = conformance::ConformanceRequest::parse_from_bytes(&case.payload)
        .context("failed to parse request")?;

    let mut outcome = Outcome::default();

    outcome.add_diagnostic(text_format::print_to_string_pretty(&request).replace("\\n", "\n"));
    outcome.add_diagnostic(text_format::print_to_string_pretty(&response).replace("\\n", "\n"));

    let result = response.result.ok_or(anyhow!("response was not set."))?;

    if result.is_skipped() {
        return Ok(outcome.set_case(Case::Skipped));
    }

    if let AssertBy::Equivalence(equivalent, _same_wire) = &case.assert_by {
        if result.is_err() {
            return Ok(outcome.set_case(Case::Failed));
        }

        let message = match result {
            Result::ProtobufPayload(ref buf) => Some(
                TestMessage::from_protobuf_payload(&request.message_type, &buf.as_slice())
                    .with_context(|| {
                        format!(
                            "{}\nfailed to parse the message from protobuf payload",
                            outcome
                        )
                    })?,
            ),
            Result::JsonPayload(ref payload) => Some(
                TestMessage::from_json_payload(&request.message_type, &payload).with_context(
                    || format!("{}\nfailed to parse the message from json payload", outcome),
                )?,
            ),
            Result::TextPayload(_) => todo!("text is not supported yet."),
            Result::JspbPayload(_) => todo!("jspb is not supported."),
            _ => None,
        };

        let equivalent_message =
            TestMessage::from_protobuf_payload(&request.message_type, equivalent)?;

        let (differs, difference) =
            differ::diff(&message.unwrap().to_text(), &equivalent_message.to_text());
        outcome.add_diagnostic(difference);

        let passed = !differs;

        return Ok(outcome.set_case(passed.into()));
    } else if AssertBy::ParseError == case.assert_by {
        if !result.is_parse_error() {
            outcome.add_diagnostic("should have failed to parse but didn't.".to_string());
        }
        let passed = result.is_parse_error();
        return Ok(outcome.set_case(passed.into()));
    } else if AssertBy::SerializationError == case.assert_by {
        if !result.is_serialize_error() {
            outcome.add_diagnostic("should have failed to serialize but didn't.".to_string());
        }
        let passed = result.is_serialize_error();
        return Ok(outcome.set_case(passed.into()));
    } else if AssertBy::Validator == case.assert_by {
        let raw_json = if let Result::JsonPayload(json) = result {
            json
        } else {
            String::new()
        };

        if raw_json.is_empty() {
            outcome.add_diagnostic(format!("expected json payload"));
            return Ok(outcome.set_case(Case::Failed));
        }

        let validator: Validator = case.into();
        let value = serde_json::from_str::<serde_json::Value>(raw_json.as_str())
            .with_context(|| format!("{}\nfailed to parse the response json", outcome))?;
        let validation_result = validator(value);
        let passed = !validation_result.is_err();
        if !passed {
            outcome.add_diagnostic(format!(
                "json validation failed: {}",
                validation_result.err().unwrap()
            ));
        }
        return Ok(outcome.set_case(passed.into()));
    } else {
        unreachable!("invalid assertion case")
    }
}
