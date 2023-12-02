include!(concat!(env!("OUT_DIR"), "/conformance/mod.rs"));

use self::conformance::conformance_response::Result as ResultCase;
use crate::case::Case;
use anyhow::{anyhow, Context, Ok};
use protobuf::Message;
use protobuf_json_mapping::parse_from_str;
use test_messages_proto2::TestAllTypesProto2;
use test_messages_proto3::TestAllTypesProto3;

pub enum TestMessage {
    Proto2(TestAllTypesProto2),
    Proto3(TestAllTypesProto3),
}

impl TestMessage {
    fn to_text(&self) -> String {
        match self {
            Self::Proto2(proto2) => protobuf::text_format::print_to_string_pretty(proto2),
            Self::Proto3(proto3) => protobuf::text_format::print_to_string_pretty(proto3),
        }
    }
}

fn new_test_message_from_protobuf_payload(
    message_type: &str,
    payload: &[u8],
) -> anyhow::Result<TestMessage> {
    match message_type {
        "protobuf_test_messages.proto3.TestAllTypesProto3" => Ok(TestMessage::Proto3(
            TestAllTypesProto3::parse_from_bytes(payload)?,
        )),
        "protobuf_test_messages.proto2.TestAllTypesProto2" => Ok(TestMessage::Proto2(
            TestAllTypesProto2::parse_from_bytes(payload)?,
        )),
        _ => Err(anyhow!("unknown message type {}", message_type)),
    }
}

fn new_test_message_from_json_payload(
    message_type: &str,
    payload: &str,
) -> anyhow::Result<TestMessage> {
    match message_type {
        "protobuf_test_messages.proto3.TestAllTypesProto3" => Ok(TestMessage::Proto3(
            parse_from_str::<TestAllTypesProto3>(&payload)?,
        )),
        "protobuf_test_messages.proto2.TestAllTypesProto2" => Ok(TestMessage::Proto2(
            parse_from_str::<TestAllTypesProto2>(&payload)?,
        )),
        _ => Err(anyhow!("unknown message type {}", message_type)),
    }
}

fn diff(left: &str, right: &str) -> (bool, String) {
    let mut differs = false;
    let mut diff = String::new();
    for d in diff::lines(left, right) {
        match d {
            diff::Result::Both(l, _) => {
                diff.push_str(&format!("{}\n", l));
                differs = false;
            }
            diff::Result::Left(l) => {
                diff.push_str(&format!("- {}\n", l));
                differs = true;
            }
            diff::Result::Right(r) => {
                diff.push_str(&format!("+ {}\n", r));
                differs = true;
            }
        }
    }

    (differs, diff)
}

pub struct CaseResult {
    pub diagnostics: Vec<String>,
    pub passed: bool,
    pub skipped: bool,
}

pub fn case(case: &Case, res: &Vec<u8>) -> anyhow::Result<CaseResult> {
    let response = conformance::ConformanceResponse::parse_from_bytes(res)
        .with_context(|| "failed to parse response")?;
    let request = conformance::ConformanceRequest::parse_from_bytes(&case.payload)
        .with_context(|| "failed to parse request")?;

    let mut diagnostics = vec![];

    let mut skipped = false;
    let mut error = false;
    let mut message: Option<TestMessage> = None;


    let result = response.result.ok_or(anyhow!("response was not set."))?;
    match result {
        ResultCase::ProtobufPayload(buf) => {
            message = Some(new_test_message_from_protobuf_payload(&request.message_type, buf.as_slice())
                .with_context(|| "failed to parse the message from protobuf payload")?)
        }
        ResultCase::JsonPayload(payload) => {
            message = Some(new_test_message_from_json_payload(&request.message_type, &payload)
            .with_context(|| "failed to parse the message from json payload")?)
        }
        ResultCase::TextPayload(_) => todo!("text is not supported yet."),

        ResultCase::JspbPayload(_) => todo!("jspb is not supported."),

        ResultCase::ParseError(r) => {
            diagnostics.push(format!("parse error: {}", r));
            error = true;
        },
        ResultCase::RuntimeError(r) => {
            diagnostics.push(format!("runtime error: {}", r));
            error = true;
        },
        ResultCase::SerializeError(r) => {
            diagnostics.push(format!("serialize error: {}", r));
            error = true;
        },
        ResultCase::TimeoutError(r) => {
            diagnostics.push(format!("timeout error: {}", r));
            error = true;
        },
        ResultCase::Skipped(r) => {
            diagnostics.push(format!("skipped: {}", r));
            skipped = true;
        }
    };

    if error || skipped {
        return Ok(CaseResult {
            diagnostics,
            passed: !error,
            skipped: skipped,
        });
    }

    let equivalent_message =
        new_test_message_from_protobuf_payload(&request.message_type, &case.equivalent)
            .with_context(|| "failed to parse the message")?;

    let (differs, difference) = diff(&message.unwrap().to_text(), &equivalent_message.to_text());
    diagnostics.push(difference);

    Ok(CaseResult {
        diagnostics,
        passed: !differs,
        skipped: false,
    })
}
