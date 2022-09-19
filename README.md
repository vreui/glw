# glw
<https://github.com/vreui/glw>

> glw = glutin + winit

[![CI](https://github.com/vreui/glw/actions/workflows/ci.yml/badge.svg)](https://github.com/vreui/glw/actions)

Less feature, less BUG.

glw 的主要功能是创建窗口以及 OpenGL / OpenGL ES 环境.

本库主要为了支持 威惹界面 (vreui).
winit 功能太繁杂, 且 BUG 太多.
glutin 和 winit 功能耦合较高, 分成两个库太麻烦.

本库不对各个平台的功能做太多的跨平台抽象和封装.
大部分平台的功能直接原样暴露.


## 编译开发

+ 代码格式化:

  ```
  cargo fmt
  ```

+ 编译:

  ```
  cargo build
  ```

+ 测试:

  ```
  cargo test
  ```

+ 代码文档:

  ```
  cargo doc
  ```


## 平台支持

glw (计划) 支持下列平台 (协议):

+ `wayland` (GNU/Linux)

  为了本库的轻量, 在此平台不会依赖 gtk/qt 等库.

+ `android` 9+

+ `windows` 7+

  由于此平台不是开源平台, 本库仅对其进行基本支持, 且优先级较低.

  在此平台使用 ANGLE 作为 OpenGL ES / EGL 兼容层.

+ `web` (wasm)

  此平台目前不能使用 WebRender (vreui), 因此优先级较低.

某果平台太过封闭, 且不适用于穷人, 因此不考虑支持.


## 感谢

+ glutin
  <https://github.com/rust-windowing/glutin>

+ winit
  <https://github.com/rust-windowing/winit>

+ wayland-rs
  <https://github.com/smithay/wayland-rs>

+ ANGLE - Almost Native Graphics Layer Engine
  <https://github.com/google/angle>


## LICENSE

`Apache License 2.0`
