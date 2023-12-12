use super::{test_messages_proto2::TestAllTypesProto2, test_messages_proto3::TestAllTypesProto3};
use anyhow::{anyhow, Ok};
use protobuf::Message;
use protobuf_json_mapping::parse_from_str;

pub enum TestMessage {
    Proto2(TestAllTypesProto2),
    Proto3(TestAllTypesProto3),
}

impl TestMessage {
    pub fn to_text(&self) -> String {
        match self {
            Self::Proto2(proto2) => protobuf::text_format::print_to_string_pretty(proto2),
            Self::Proto3(proto3) => protobuf::text_format::print_to_string_pretty(proto3),
        }
    }
    pub fn from_protobuf_payload(message_type: &str, payload: &[u8]) -> anyhow::Result<Self> {
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
    pub fn from_json_payload(message_type: &str, payload: &str) -> anyhow::Result<TestMessage> {
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
}
