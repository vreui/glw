//! 全局常量和类型定义

#[cfg(feature = "egl")]
use crate::api::Gl要求;

// 使用 GL 的版本
#[cfg(feature = "egl")]
pub const GL版本: Gl要求 = Gl要求::Gles { gles版本: (3, 0) };

// TODO
