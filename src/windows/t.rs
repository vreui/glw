//! 全局常量和类型定义

#[cfg(feature = "egl")]
use crate::api::Gl要求;

// 使用 GL 的版本
// 使用 ANGLE: OpenGL ES 3.0 (Direct3D 11, Windows 7)
#[cfg(feature = "egl")]
pub const GL版本: Gl要求 = Gl要求::Gles { gles版本: (3, 0) };

// 控制 ANGLE 后端类型的环境变量
#[cfg(feature = "egl")]
pub const ANGLE后端环境变量: &'static str = "ANGLE_BACKEND";
