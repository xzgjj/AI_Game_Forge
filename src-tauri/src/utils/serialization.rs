//! 占位序列化模块

use serde::{Deserialize, Serialize};

pub fn to_json<T: Serialize>(value: &T) -> Result<String, serde_json::Error> {
    serde_json::to_string(value)
}

pub fn from_json<T: for<'de> Deserialize<'de>>(value: &str) -> Result<T, serde_json::Error> {
    serde_json::from_str(value)
}
