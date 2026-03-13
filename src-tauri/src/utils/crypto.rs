//! 占位加密模块

pub fn hash_placeholder(input: &str) -> String {
    format!("hash:{}", input)
}
