//! ANGLE 相关

use glutin_egl_sys::egl::types::EGLenum;

// ANGLE: include/EGL/eglext_angle.h
// <https://github.com/google/angle/blob/main/include/EGL/eglext_angle.h>

// EGL 扩展: EGL_ANGLE_platform_angle
// #define EGL_PLATFORM_ANGLE_ANGLE          0x3202
pub const PLATFORM_ANGLE_ANGLE: EGLenum = 0x3202;

// #define EGL_PLATFORM_ANGLE_TYPE_ANGLE     0x3203
pub const PLATFORM_ANGLE_TYPE_ANGLE: EGLenum = 0x3203;
// #define EGL_PLATFORM_ANGLE_TYPE_DEFAULT_ANGLE 0x3206
pub const PLATFORM_ANGLE_TYPE_DEFAULT_ANGLE: EGLenum = 0x3206;

// #define EGL_PLATFORM_ANGLE_DEVICE_TYPE_ANGLE 0x3209
pub const PLATFORM_ANGLE_DEVICE_TYPE_ANGLE: EGLenum = 0x3209;
// #define EGL_PLATFORM_ANGLE_DEVICE_TYPE_HARDWARE_ANGLE 0x320A
pub const PLATFORM_ANGLE_DEVICE_TYPE_HARDWARE_ANGLE: EGLenum = 0x320a;
// #define EGL_PLATFORM_ANGLE_DEVICE_TYPE_NULL_ANGLE 0x345E
pub const PLATFORM_ANGLE_DEVICE_TYPE_NULL_ANGLE: EGLenum = 0x345e;
// #define EGL_PLATFORM_ANGLE_NATIVE_PLATFORM_TYPE_ANGLE 0x348F

// EGL_ANGLE_platform_angle_d3d
// #define EGL_PLATFORM_ANGLE_TYPE_D3D9_ANGLE 0x3207
pub const PLATFORM_ANGLE_TYPE_D3D9_ANGLE: EGLenum = 0x3207;
// #define EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE 0x3208
pub const PLATFORM_ANGLE_TYPE_D3D11_ANGLE: EGLenum = 0x3208;

// EGL_ANGLE_platform_angle_d3d11on12
// #define EGL_PLATFORM_ANGLE_D3D11ON12_ANGLE 0x3488
pub const PLATFORM_ANGLE_D3D11ON12_ANGLE: EGLenum = 0x3488;

// EGL_ANGLE_platform_angle_opengl
// #define EGL_PLATFORM_ANGLE_TYPE_OPENGL_ANGLE 0x320D
pub const PLATFORM_ANGLE_TYPE_OPENGL_ANGLE: EGLenum = 0x320d;
// #define EGL_PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE 0x320E
pub const PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE: EGLenum = 0x320e;

// EGL_ANGLE_platform_angle_null
// #define EGL_PLATFORM_ANGLE_TYPE_NULL_ANGLE 0x33AE
pub const PLATFORM_ANGLE_TYPE_NULL_ANGLE: EGLenum = 0x33ae;

// EGL_ANGLE_platform_angle_vulkan
// #define EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE 0x3450
pub const PLATFORM_ANGLE_TYPE_VULKAN_ANGLE: EGLenum = 0x3450;

// EGL_ANGLE_platform_angle_device_type_swiftshader
// #define EGL_PLATFORM_ANGLE_DEVICE_TYPE_SWIFTSHADER_ANGLE 0x3487
pub const PLATFORM_ANGLE_DEVICE_TYPE_SWIFTSHADER_ANGLE: EGLenum = 0x3487;
// EGL_ANGLE_platform_angle_device_type_egl_angle
// #define EGL_PLATFORM_ANGLE_DEVICE_TYPE_EGL_ANGLE 0x348E
pub const PLATFORM_ANGLE_DEVICE_TYPE_EGL_ANGLE: EGLenum = 0x348e;

// EGL_ANGLE_context_virtualization
// #define EGL_CONTEXT_VIRTUALIZATION_GROUP_ANGLE 0x3481

// ANGLE 后端类型
#[derive(Debug, Clone, PartialEq)]
pub enum Angle后端 {
    默认,

    Null,
    D3d9,
    D3d11,
    D3d11on12,
    Gl,
    Gles,
    Swiftshader,
    Vulkan,

    未知(String),
}

impl Angle后端 {
    // EGL_PLATFORM_ANGLE_TYPE_ANGLE
    pub fn 平台类型(&self) -> Result<Option<EGLenum>, String> {
        match self {
            Angle后端::默认 => Ok(None),
            Angle后端::Null => Ok(Some(PLATFORM_ANGLE_TYPE_NULL_ANGLE)),
            Angle后端::D3d9 => Ok(Some(PLATFORM_ANGLE_TYPE_D3D9_ANGLE)),
            Angle后端::D3d11 => Ok(Some(PLATFORM_ANGLE_TYPE_D3D11_ANGLE)),
            Angle后端::D3d11on12 => Ok(Some(PLATFORM_ANGLE_TYPE_D3D11_ANGLE)),
            Angle后端::Gl => Ok(Some(PLATFORM_ANGLE_TYPE_OPENGL_ANGLE)),
            Angle后端::Gles => Ok(Some(PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE)),
            Angle后端::Swiftshader => Ok(Some(PLATFORM_ANGLE_TYPE_VULKAN_ANGLE)),
            Angle后端::Vulkan => Ok(Some(PLATFORM_ANGLE_TYPE_VULKAN_ANGLE)),
            Angle后端::未知(名称) => Err(format!("未知 ANGLE 后端 {}", 名称)),
        }
    }

    // EGL_PLATFORM_ANGLE_DEVICE_TYPE_ANGLE
    pub fn 设备类型(&self) -> Option<EGLenum> {
        match self {
            Angle后端::Swiftshader => Some(PLATFORM_ANGLE_DEVICE_TYPE_SWIFTSHADER_ANGLE),
            _ => None,
        }
    }
}

impl From<&str> for Angle后端 {
    fn from(名称: &str) -> Self {
        match 名称 {
            "" => Angle后端::默认,
            "null" => Angle后端::Null,
            "d3d9" => Angle后端::D3d9,
            "d3d11" => Angle后端::D3d11,
            "d3d11on12" => Angle后端::D3d11on12,
            "gl" => Angle后端::Gl,
            "gles" => Angle后端::Gles,
            "swiftshader" => Angle后端::Swiftshader,
            "vulkan" => Angle后端::Vulkan,
            _ => Angle后端::未知(名称.to_string()),
        }
    }
}
