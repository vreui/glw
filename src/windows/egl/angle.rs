//! ANGLE 相关

use glutin_egl_sys::egl::types::EGLenum;

// ANGLE: include/EGL/eglext_angle.h
// <https://github.com/google/angle/blob/main/include/EGL/eglext_angle.h>

// EGL 扩展: EGL_ANGLE_platform_angle
// #define EGL_PLATFORM_ANGLE_ANGLE          0x3202
pub const PLATFORM_ANGLE_ANGLE: EGLenum = 0x3202;

// {"d3d9", EGL_PLATFORM_ANGLE_TYPE_D3D9_ANGLE},
// {"d3d11", EGL_PLATFORM_ANGLE_TYPE_D3D11_ANGLE},
// {"gl", EGL_PLATFORM_ANGLE_TYPE_OPENGL_ANGLE},
// {"gles", EGL_PLATFORM_ANGLE_TYPE_OPENGLES_ANGLE},
// {"metal", EGL_PLATFORM_ANGLE_TYPE_METAL_ANGLE},
// {"null", EGL_PLATFORM_ANGLE_TYPE_NULL_ANGLE},
// {"swiftshader", EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE},
// {"vulkan", EGL_PLATFORM_ANGLE_TYPE_VULKAN_ANGLE},

// TODO
