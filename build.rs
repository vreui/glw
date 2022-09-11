use cfg_aliases::cfg_aliases;

fn main() {
    // from glutin
    cfg_aliases! {
        android: { target_os = "android" },
        wasm: { target_arch = "wasm32" },
        free_unix: { all(unix, not(apple), not(android)) },

        // Native displays
        //wayland_platform: { all(feature = "wayland", free_unix, not(wasm)) },
        wayland_platform: { all(free_unix, not(wasm)) },

        egl_backend: { all(feature = "egl", any(windows, unix), not(apple), not(wasm)) },
        wgl_backend: { all(feature = "wgl", windows, not(wasm)) },
    }
}
