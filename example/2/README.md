# 测试2

平台: Android

简单窗口测试程序.

使用 <https://github.com/rust-windowing/android-ndk-rs>


## 编译运行

安装所需工具 (`cargo-apk`):

```sh
cargo install cargo-apk
```

编译:

```sh
cargo apk build
```

运行:

```sh
cargo apk run
```

查看 Android logcat 输出:

```sh
adb logcat RustStdoutStderr:D '*:s'
```
