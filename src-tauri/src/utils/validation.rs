//! 占位校验模块

pub fn is_non_empty(value: &str) -> bool {
    !value.trim().is_empty()
}
