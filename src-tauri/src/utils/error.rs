//! 占位错误模块

use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("unknown error")]
    Unknown,
}
