# ANGLE
ANGLE - Almost Native Graphics Layer Engine

项目链接: <https://github.com/google/angle>


## 简介

ANGLE 是跨平台的 OpenGL ES 兼容层, 向上提供 `OpenGL ES 3.0` 图形接口.
浏览器 Chrome 和 Firefox 都在使用 ANGLE.

+ 在 Windows 7+ 上底层使用 `Direct3D 11`, 可实现 `OpenGL ES 3.0`.

+ 如果底层使用 `vulkan`, 则可以实现 `OpenGL ES 3.1`.

由于 Windows 本身对 OpenGL (OpenGL ES) 的支持很不好,
所以在 Windows 平台上使用 ANGLE.


## 使用方式

1. 获取 `.dll` 文件: `libEGL.dll`, `libGLESv2.dll`, `d3dcompiler_47.dll`

   一个简单方便的方法, 是直接从 Chrome 浏览器安装目录复制这些文件:
   比如 `C:\Program Files\Google\Chrome\Application\105.0.5195.127`

2. 将这些 dll 文件放入可执行文件 (`.exe`) 相同的目录.


## ANGLE 后端选择

在 Windows 平台 ANGLE 默认使用 `Direct3D 11` 后端.

设置环境变量: `ANGLE_BACKEND`

注: 通过环境变量选择后端的功能由本库 (glw) 实现.

可用取值:

+ `ANGLE_BACKEND=null`: 用于测试, 不渲染任何东西

+ `ANGLE_BACKEND=d3d9`: Direct3D 9 (只能实现 OpenGL ES 2.0)

+ `ANGLE_BACKEND=d3d11`: Direct3D 11 (可以实现 OpenGL ES 3.0)

+ `ANGLE_BACKEND=d3d11on12`: Direct3D 11 on Direct3D 12

+ `ANGLE_BACKEND=gl`: OpenGL

+ `ANGLE_BACKEND=gles`: OpenGL ES

+ `ANGLE_BACKEND=swiftshader`: vulkan + SwiftShader (CPU 软件渲染, 可以实现 OpenGL ES 3.1)

+ `ANGLE_BACKEND=vulkan`: vulkan (可以实现 OpenGL ES 3.1)


----

TODO
