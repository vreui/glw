# Windows 平台


## Windows 7 安装 rust

+ 时间: `2022-09-18`

+ 操作系统版本: Windows 7 sp1 x64 旗舰版 (重新安装)

+ rustup 和 rustc 版本:

  ```
  > rustup --version
  rustup 1.25.1 (bb60b1e89 2022-07-12)

  > rustc --version
  rustc 1.63.0 (4b91a6ea7 2022-08-08)
  ```

Windows 7 是一个比较老的系统 (2009 年发布, 13 年前), 且已经停止官方支持 (2020 年停止支持, 2 年前).

但是 rust [最低仍然支持](https://doc.rust-lang.org/beta/rustc/platform-support.html) Windows 7 (`x86_64-pc-windows-msvc`).
更低版本的 Windows 很难再使用 rust,
Windows 7 差不多是最低支持的极限了.

国内 Windows 7 使用仍然较多, 且硬件配置较低的老机器不方便升级.
所以支持 Windows 7 具有重要意义.

### 安装过程

Windows 7 安装 rust 会遇到一堆坑, 对于重新安装的 Windows 7 sp1 系统来说 (没有安装补丁/更新).

1. **下载 `rustup-init.exe`**

   从这个页面 <https://www.rust-lang.org/learn/get-started> 点击下载 `rustup-init.exe` (64 位).

   这一步很顺利, 下载之后运行.
   选项 1 自动安装 VS 社区版工具.

2. **证书错误**

   第一个遇到的问题是, 下载 VS 网站的证书不被系统信任 (报错).

   复制下载链接, 在浏览器 (Chrome 105) 打开, 仍然报相同的错误.
   点击地址栏左边图标, 查看并手动下载证书.

   Win+R 运行, 输入 `certmgr.msc` 启动系统证书管理器.
   左侧选择 `受信任的根证书颁发机构` / `证书`, 右键菜单 `所有任务` / `导入`.
   安装刚才下载的根证书 (`DigiCert Global Root G2.crt`).

   然后重新运行 `rustup-init.exe`, 此时顺利下载了 VS 安装程序.

3. **报错: `无法定位程序输入点 SetDefaultDllDirectories 于动态链接库 KERNEL32.dll`**

   解决方法参考文章: <https://superuser.com/questions/1492060/procedure-entry-point-setdefaultdlldirectories-could-not-be-located-kernel32-dll>

   需要安装补丁 `KB4457144`, 从这里下载: <https://www.catalog.update.microsoft.com/Search.aspx?q=KB4457144>

   下载后运行安装 `windows6.1-kb4457144-x64_5ca467d42deadc2b2f4010c4a26b4a6903790dd5.msu`, 重启系统.

   然后第 3 次运行 `rustup-init.exe`.

4. **缺少 `.net 4.6` 运行环境**

   经测试, Windows 7 最高可以安装 `.net 4.7` (`.net 4.8 不支持`).

   从这里下载 `.net 4.7`: <https://dotnet.microsoft.com/en-us/download/dotnet-framework/net472>

   下载后运行安装 `ndp472-kb4054530-x86-x64-allos-enu.exe`.

5. **安装成功**

   至此, 终于安装上了 VS 社区版工具.

   然后 rust 也很快装好了.  ;-)

----


TODO
