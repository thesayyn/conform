use crate::test_case::TestCase;
use anyhow::anyhow;
pub type Validator = fn(serde_json::Value) -> anyhow::Result<()>;

fn assert_map_contains_key(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> anyhow::Result<bool> {
    if !map.contains_key(key) {
        return Err(anyhow!("json value doesn't contain the key `{}`", key));
    }
    Ok(true)
}

fn assert_map_not_contains_key(
    map: &serde_json::Map<String, serde_json::Value>,
    key: &str,
) -> anyhow::Result<bool> {
    if map.contains_key(key) {
        return Err(anyhow!("json value should contain the key `{}`", key));
    }
    Ok(true)
}

macro_rules! assert_object_and_has_keys {
    ( $( $f:literal ),* ) => {
        |v| {
            let obj = v.as_object().ok_or(anyhow!("value is not an object."))?;
            $(
                assert_map_contains_key(obj, $f)?;
            )*
            Ok(())
        }
    };
}

macro_rules! assert_object_and_key_value_and_type {
    ( $f:literal, $cast:ident, $eq:expr ) => {
        |v: serde_json::Value| {
            let obj = v.as_object().ok_or(anyhow!("value is not an object."))?;
            let v = obj
                .get($f)
                .ok_or(anyhow!("json value should contain the key `{}`", $f))?
                .$cast()
                .ok_or(anyhow!("{} is not a {}.", $f, stringify!($cast)))?;

            if v != $eq {
                return Err(anyhow!("{} was not equal to {}", $f, $eq));
            }
            Ok(())
        }
    };
}

macro_rules! assert_object_and_is_null {
    ( $f:literal) => {
        |v: serde_json::Value| {
            let obj = v.as_object().ok_or(anyhow!("value is not an object."))?;
            obj.get($f)
                .ok_or(anyhow!("json value should contain the key `{}`", $f))?
                .as_null()
                .ok_or(anyhow!("{} is not null", $f))?;
            Ok(())
        }
    };
}

impl From<&TestCase> for Validator {
    fn from(case: &TestCase) -> Self {
        match case.name.as_str() {
            "Required.Proto3.JsonInput.FieldNameInLowerCamelCase.Validator" => {
                assert_object_and_has_keys!("fieldname1", "fieldName2", "FieldName3", "fieldName4")
            }

            "Required.Proto3.JsonInput.FieldNameWithNumbers.Validator" => {
                assert_object_and_has_keys!("field0name5", "field0Name6")
            }

            "Required.Proto3.JsonInput.FieldNameWithMixedCases.Validator" => {
                assert_object_and_has_keys!(
                    "fieldName7",
                    "FieldName8",
                    "fieldName9",
                    "FieldName10",
                    "FIELDNAME11",
                    "FIELDName12"
                )
            }

            "Recommended.Proto3.JsonInput.FieldNameWithDoubleUnderscores.Validator" => {
                assert_object_and_has_keys!(
                    "FieldName13",
                    "FieldName14",
                    "fieldName15",
                    "fieldName16",
                    "fieldName17",
                    "FieldName18"
                )
            }

            "Required.Proto3.JsonInput.SkipsDefaultPrimitive.Validator" => |v| {
                let obj = v.as_object().ok_or(anyhow!("value is not an object."))?;
                assert_map_not_contains_key(obj, "FieldName13")?;
                Ok(())
            },
            "Recommended.Proto3.JsonInput.Int64FieldBeString.Validator" => {
                assert_object_and_key_value_and_type!("optionalInt64", as_str, "1")
            }
            "Recommended.Proto3.JsonInput.Uint64FieldBeString.Validator" => {
                assert_object_and_key_value_and_type!("optionalUint64", as_str, "1")
            }
            "Required.Proto3.JsonInput.EnumFieldUnknownValue.Validator" => {
                assert_object_and_key_value_and_type!("optionalNestedEnum", as_i64, 123)
            }
            "Recommended.Proto3.JsonInput.DurationHasZeroFractionalDigit.Validator" => {
                assert_object_and_key_value_and_type!("optionalDuration", as_str, "1s")
            }
            "Recommended.Proto3.JsonInput.DurationHas3FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!("optionalDuration", as_str, "1.010s")
            }
            "Recommended.Proto3.JsonInput.DurationHas6FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!("optionalDuration", as_str, "1.000010s")
            }
            "Recommended.Proto3.JsonInput.DurationHas9FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!("optionalDuration", as_str, "1.000000010s")
            }
            "Recommended.Proto3.JsonInput.TimestampZeroNormalized.Validator" => {
                assert_object_and_key_value_and_type!(
                    "optionalTimestamp",
                    as_str,
                    "1970-01-01T00:00:00Z"
                )
            }
            "Recommended.Proto3.JsonInput.TimestampHasZeroFractionalDigit.Validator" => {
                assert_object_and_key_value_and_type!(
                    "optionalTimestamp",
                    as_str,
                    "1970-01-01T00:00:00Z"
                )
            }
            "Recommended.Proto3.JsonInput.TimestampHas3FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!(
                    "optionalTimestamp",
                    as_str,
                    "1970-01-01T00:00:00.010Z"
                )
            }
            "Recommended.Proto3.JsonInput.TimestampHas6FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!(
                    "optionalTimestamp",
                    as_str,
                    "1970-01-01T00:00:00.000010Z"
                )
            }
            "Recommended.Proto3.JsonInput.TimestampHas9FractionalDigits.Validator" => {
                assert_object_and_key_value_and_type!(
                    "optionalTimestamp",
                    as_str,
                    "1970-01-01T00:00:00.000000010Z"
                )
            }
            "Recommended.Proto3.JsonInput.NullValueInOtherOneofOldFormat.Validator" => {
                assert_object_and_key_value_and_type!("oneofNullValue", as_str, "NULL_VALUE")
            }
            "Recommended.Proto3.JsonInput.NullValueInOtherOneofNewFormat.Validator" => {
                assert_object_and_is_null!("oneofNullValue")
            }
            "Recommended.Proto3.JsonInput.NullValueInNormalMessage.Validator" => |v| {
                let obj = v.as_object().ok_or(anyhow!("value is not an object."))?;
                if obj.keys().len() > 0 {
                    return Err(anyhow!("value should be an empty object."));
                }
                Ok(())
            },
            "Required.Proto2.JsonInput.StoresDefaultPrimitive.Validator" => {
                assert_object_and_key_value_and_type!("FieldName13", as_i64, 0)
            }
            _ => |_| Err(anyhow!("unimplemented validator")),
        }
    }
}
