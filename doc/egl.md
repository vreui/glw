# EGL

[OpenGL](https://www.khronos.org/opengl/) /
[OpenGL ES](https://www.khronos.org/opengles/)
只是定义了图形 API 本身,
与操作系统 (窗口系统) 的集成, 需要额外支持.

[EGL](https://www.khronos.org/egl)
是一种被广泛支持的将 GL 与操作系统集成的方式.
GNU/Linux 平台, Android 平台, Windows 平台 (ANGLE)
都支持 EGL.


## EGL 初始化过程

本库使用 [`glutin_egl_sys`](https://github.com/rust-windowing/glutin) 绑定.

1. **加载 EGL 库**

   使用类似 [`dlopen()`](https://man7.org/linux/man-pages/man3/dlopen.3.html) 的方法加载平台的动态链接库.

   在 GNU/Linux 和 Android 平台加载 `libEGL.so`,
   在 Windows 平台加载 `libEGL.dll`.

2. **创建 `EGLDisplay`**

   `EGLDisplay` 是 EGL 和具体平台 (操作系统) 的连接通道, 后续操作都要用到.
   使用如下函数进行创建:

   ```rust
   egl.GetPlatformDisplay(平台, 显示指针, 属性)
   ```

   `平台` 对应具体的平台.
   比如 `egl::PLATFORM_WAYLAND_KHR` 对应 wayland (GNU/Linux),
   `egl::PLATFORM_ANDROID_KHR` 对应 Android,
   `PLATFORM_ANGLE_ANGLE` 对应 Windows (ANGLE).

   `显示指针` 不同的平台有不同的具体要求.
   wayland 使用 `WlDisplay` 的指针 (也就是和 wayland 服务器的连接),
   Android 使用 `egl::DEFAULT_DISPLAY`,
   Windows (ANGLE) 使用 `GetDC(HWND)` 返回的窗口的绘图句柄.

   `属性` 是一个整数列表 (`Vec<EGLAttrib>`), 用于配置一些选项,
   默认可以使用 `egl::NONE`.
   使用 ANGLE 时可以配置一些具体选项, 比如 ANGLE 的后端类型 (Direct3D 11, vulkan, swiftshader).

3. **初始化 EGL**

   使用如下函数:

   ```rust
   egl.Initialize(EGLDisplay, 主版本号, 次版本号)
   ```

   `EGLDisplay` 就是上一步创建的.
   如果初始化成功, **返回** EGL 库的主/次版本号 (目前一般是 1.5 版本).

   如果失败, 可以调用 `egl.GetError()` 获取错误代码.

4. **查找并选择可用配置 (`EGLConfig`)**

   使用如下函数:

   ```rust
   egl.ChooseConfig(EGLDisplay, 属性, 缓冲区指针, 缓冲区字节数, 返回的结果字节数)
   ```

   `属性` 也是一个整数列表 (`Vec<EGLint>`),
   用于配置缓冲区颜色位数, 表面类型, API 类型 (OpenGL / OpenGL ES 3) 等.

5. **创建 `EGLContext`**

   也就是 EGL 上下文.
   使用如下函数:

   ```rust
   egl.CreateContext(EGLDisplay, EGLConfig, egl::NO_CONTEXT, 属性)
   ```

   `属性` 是一个整数列表 (`Vec<EGLint>`),
   用于配置 API 类型 (OpenGL / OpenGL ES 3),
   API 的主/次版本号 (比如 OpenGL 4.6, OpenGL ES 3.1) 等.
   对于 OpenGL 还可以配置使用核心模式 (core profile) 或兼容模式 (compatibility profile).

6. **创建 `EGLSurface`**

   也就是 EGL 表面.
   表面有 3 种类型, 窗口表面 (WindowSurface),
   P缓冲区表面 (PbufferSurface), PixmapSurface.
   其中窗口表面是用于直接在屏幕上显示的.
   此处只讨论窗口表面.

   使用如下函数:

   ```rust
   egl.CreatePlatformWindowSurface(EGLDisplay, EGLConfig, 窗口指针, 属性)
   ```

   `窗口指针` 不同的平台有不同的要求.
   wayland 使用 `WlEglSurface` 的指针 (`libwayland-egl`),
   Android 使用 `ANativeWindow` 的指针,
   Windows 使用 `HWND` (窗口句柄).

   `属性` 是一个整数列表 (`Vec<EGLAttrib>`),
   用于配置双缓冲, 颜色空间等.

7. **设为当前 (MakeCurrent)**

   使用如下函数:

   ```rust
   egl.MakeCurrent(EGLDisplay, EGLSurface, EGLSurface, EGLContext)
   ```

   也就是把前面创建的表面/上下文等进行绑定, 成为 "当前 GL 环境".

   在 OpenGL (ES) 中, 一个线程只能有一个 "当前环境".
   如果再次调用 `MakeCurrent()` 则之前的失效, 被替换成新设置的.

至此, EGL 初始化完毕, 可以开始使用 GL 函数进行绘制了.


## 常用 EGL 函数

除了上面介绍的初始化 EGL 所使用的 EGL 函数,
还有一些比较有用的 EGL 函数.

+ **获取 EGL 扩展**

  使用如下函数:

  ```rust
  egl.QueryString(EGLDisplay, egl::EXTENSIONS)
  ```

  其中 `EGLDisplay` 可以使用 `egl::NO_DISPLAY`,
  也就是在创建 `EGLDisplay` 之前查询 EGL 库支持的功能 (EGL extensions).

+ **交换缓冲区**

  使用如下函数:

  ```rust
  egl.SwapBuffers(EGLDisplay, EGLSurface)
  ```

  对于窗口表面, 一般使用双缓冲区进行绘制.
  前缓冲区用于显示, 后缓冲区用于绘制.
  绘制结束后, 调用此函数交换前/后缓冲区, 从而显示新的内容.


## gleam

[gleam](https://github.com/servo/gleam) 是 [WebRender](https://github.com/servo/webrender)
使用的 OpenGL / OpenGL ES 绑定.

本库内置了 gleam 的初始化功能.
初始化 gleam 很简单, 只是调用 `egl.GetProcAddress(名称)`
加载 GL 函数的指针 (内存地址) 而已.

gleam 使用示例:

+ 获取 GL 信息:

  ```rust
  println!("GL version {}", gl.get_string(gl::VERSION));
  println!("GL vendor {}", gl.get_string(gl::VENDOR));
  println!("GL renderer {}", gl.get_string(gl::RENDERER));
  ```

+ 使用颜色填充整个缓冲区:

  ```rust
  gl.clear_color(1.0, 1.0, 0.0, 0.8);  // RGBA
  gl.clear(gl::COLOR_BUFFER_BIT);
  ```


TODO
