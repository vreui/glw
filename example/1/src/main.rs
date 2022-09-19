extern crate gleam;
extern crate glw;

use gleam::gl;
use glw::{窗口, 窗口创建参数};

fn main() {
    // 创建一个简单的测试窗口
    println!("创建窗口");

    let mut 窗 = 窗口::new(窗口创建参数 {
        标题: "测试1",
        大小: (1280, 750),
        //背景色: (0.7, 0.7, 0.0, 0.8),
        背景色: (1.0, 1.0, 0.0, 0.8),
        gl: true,
    });

    let gl = 窗.取gl().unwrap();
    // DEBUG
    println!("GL version {}", gl.get_string(gl::VERSION));
    println!("GL vendor {}", gl.get_string(gl::VENDOR));
    println!("GL renderer {}", gl.get_string(gl::RENDERER));

    println!("进入主循环");
    窗.主循环();

    println!("退出");
    窗.清理();
}
