//! 工具模块
//! 提供通用工具函数

use uuid::Uuid;

/// 生成追踪ID
pub fn new_trace_id() -> String {
    Uuid::new_v4().to_string()
}

/// 将错误信息标准化为用户可读文本
pub fn normalize_error_message(prefix: &str, detail: &str) -> String {
    format!("{}: {}", prefix, detail)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trace_id_should_not_be_empty() {
        assert!(!new_trace_id().is_empty());
    }

    #[test]
    fn normalize_message_should_include_prefix_and_detail() {
        let msg = normalize_error_message("Auth", "invalid token");
        assert!(msg.contains("Auth"));
        assert!(msg.contains("invalid token"));
    }
}
