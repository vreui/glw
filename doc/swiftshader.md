# SwiftShader

项目链接: <https://github.com/google/swiftshader>


## 简介

SwiftShader 使用 CPU 软件渲染, 向上提供 `vulkan 1.3` 图形接口.

+ 预编译版本 (Windows) 从这里直接下载:
  <https://github.com/pal1000/swiftshader-dist-win>

由于 VirtualBox 虚拟机运行 Windows 7 只支持 `Direct3D 9` (不支持 `Direct3D 11`),
ANGLE 在 `Direct3D 9` 之下只能实现 `OpenGL ES 2.0` (不能实现 `OpenGL ES 3.0`).
此时可以考虑使用 SwiftShader (CPU 软件渲染).


## 使用方式

1. 将 `vulkan-1.dll` 文件放入可执行文件 (`.exe`) 相同的目录.

2. 设置环境变量: `ANGLE_BACKEND=swiftshader`


----

TODO
