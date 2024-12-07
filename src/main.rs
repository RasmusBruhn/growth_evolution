use std::env;

use winit::dpi::PhysicalSize;

pub mod application;
pub mod graphics;
pub mod render;
pub mod types;
pub mod map;
pub mod constants;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Get crate data
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    // Set basic settings
    let name = format!("{crate_name} v{crate_version}");
    let size = PhysicalSize::new(500, 500);

    // Set graphics settings
    let color_background = wgpu::Color {
        r: 1.0,
        g: 1.0,
        b: 1.0,
        a: 1.0,
    };
    let color_edge = wgpu::Color {
        r: 0.1,
        g: 0.1,
        b: 0.1,
        a: 1.0,
    };
    let graphics_settings = graphics::Settings {
        color_background,
        color_edge,
    };

    // Setup the main loop
    let mut main_loop = application::MainLoop::new(name, size, graphics_settings);

    // Run the application
    application::run(&mut main_loop);
}
