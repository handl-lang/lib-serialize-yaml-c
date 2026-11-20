//! `serialize-yaml-c` — native YAML serializer backend for `lib-serialize`.
//!
//! `lib-serialize` is the `.hnl` contract: `serialize.serialize(v, fmt)` /
//! `serialize.deserialize(s, fmt)` dispatch on the format name or on the
//! `serialize`/`deserialize` arrow fields of a serializer record produced
//! by `serialize.yaml()`. This crate holds the native impls (extracted
//! from `plugin-lib-stdlib`'s `ops::yaml`), registered as
//! `yaml_c.serialize` / `yaml_c.deserialize` ops via an [`OpRegistry`].

use std::sync::Arc;

use handl_lib_serialize::{Decoder, Encoder, SerializeError, Serializer};
use handl_ops::{Op, OpContext, OpError, OpRegistry};
use serde_json::Value;
use serde_yaml::Value as YamlValue;

/// The handl source declaring the `yaml_c` `op` interface — the single
/// source of truth for the yaml_c API surface.
pub const YAML_C_HNL: &str = include_str!("index.hnl");

/// The op names declared in `index.hnl` and bound by
/// [`yaml_c_op_registry`].
pub const YAML_C_OPS: &[&str] = &["yaml_c.serialize", "yaml_c.deserialize"];

// ---------------------------------------------------------------------------
// Yaml codec (backward compat for the lib-serialize SerializerRegistry)
// ---------------------------------------------------------------------------

/// Combined YAML codec.
#[derive(Debug, Clone, Copy, Default)]
pub struct Yaml;

/// YAML encoder.
#[derive(Debug, Clone, Copy, Default)]
pub struct YamlEncoder;

/// YAML decoder.
#[derive(Debug, Clone, Copy, Default)]
pub struct YamlDecoder;

impl Encoder<YamlValue> for Yaml {
    type Output = String;

    fn encode(&self, value: &YamlValue) -> Result<Self::Output, SerializeError> {
        serde_yaml::to_string(value).map_err(|e| SerializeError::new(e.to_string()))
    }
}

impl Decoder<str, YamlValue> for Yaml {
    fn decode(&self, input: &str) -> Result<YamlValue, SerializeError> {
        serde_yaml::from_str(input).map_err(|e| SerializeError::new(e.to_string()))
    }
}

impl Serializer for Yaml {
    fn serialize(&self, value: &Value) -> Result<String, SerializeError> {
        serde_yaml::to_string(value).map_err(|e| SerializeError::new(e.to_string()))
    }

    fn deserialize(&self, input: &str) -> Result<Value, SerializeError> {
        serde_yaml::from_str(input).map_err(|e| SerializeError::new(e.to_string()))
    }
}

impl Encoder<YamlValue> for YamlEncoder {
    type Output = String;

    fn encode(&self, value: &YamlValue) -> Result<Self::Output, SerializeError> {
        Yaml.encode(value)
    }
}

impl Decoder<str, YamlValue> for YamlDecoder {
    fn decode(&self, input: &str) -> Result<YamlValue, SerializeError> {
        Yaml.decode(input)
    }
}

// ---------------------------------------------------------------------------
// Function impls (the handls — Rust bodies for the `op` declarations)
// ---------------------------------------------------------------------------

/// The Rust handl for `op "yaml_c.serialize" : Value -> String`.
///
/// Serializes a value to a YAML string. This is the eval-time `Function`
/// impl registered under the op name `"yaml_c.serialize"` in the
/// [`OpRegistry`].
#[derive(Debug, Clone, Copy)]
pub struct YamlCSerialize;

impl Op for YamlCSerialize {
    fn call(&self, args: &[Value], _ctx: &mut dyn OpContext) -> Result<Value, OpError> {
        if args.len() != 1 {
            return Err(OpError::TypeMismatch(format!(
                "expected {} args, got {}",
                1,
                args.len()
            )));
        }
        let s = serde_yaml::to_string(&args[0])
            .map_err(|e| OpError::Other(format!("yaml_c.serialize error: {e}")))?;
        Ok(Value::String(s))
    }
}

/// The Rust handl for `op "yaml_c.deserialize" : String -> Value`.
///
/// Parses a YAML string into a value. This is the eval-time `Function`
/// impl registered under the op name `"yaml_c.deserialize"` in the
/// [`OpRegistry`].
#[derive(Debug, Clone, Copy)]
pub struct YamlCDeserialize;

impl Op for YamlCDeserialize {
    fn call(&self, args: &[Value], _ctx: &mut dyn OpContext) -> Result<Value, OpError> {
        if args.len() != 1 {
            return Err(OpError::TypeMismatch(format!(
                "expected {} args, got {}",
                1,
                args.len()
            )));
        }
        let s = args[0]
            .as_str()
            .ok_or_else(|| OpError::Other("yaml_c.deserialize expects a string".to_string()))?;
        let value: Value = serde_yaml::from_str(s)
            .map_err(|e| OpError::Other(format!("yaml_c.deserialize error: {e}")))?;
        Ok(value)
    }
}

// ---------------------------------------------------------------------------
// Op Registry
// ---------------------------------------------------------------------------

/// Build the [`OpRegistry`] for `yaml_c`, registering the Rust handls under
/// their op names.
///
/// This registry maps:
/// - `"yaml_c.serialize"` → [`YamlCSerialize`]
/// - `"yaml_c.deserialize"` → [`YamlCDeserialize`]
pub fn yaml_c_op_registry() -> OpRegistry {
    let mut registry = OpRegistry::new();
    registry.register("yaml_c.serialize", Arc::new(YamlCSerialize));
    registry.register("yaml_c.deserialize", Arc::new(YamlCDeserialize));
    registry
}
