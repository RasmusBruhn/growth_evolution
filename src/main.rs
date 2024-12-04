use std::env;

use winit::dpi::PhysicalSize;

mod application;
mod render;

fn main() {
    env::set_var("RUST_BACKTRACE", "1");

    // Get crate data
    let crate_name = env!("CARGO_PKG_NAME");
    let crate_version = env!("CARGO_PKG_VERSION");

    // Setup the main loop
    let name = format!("{crate_name} v{crate_version}");
    let size = PhysicalSize::new(500, 500);
    let mut main_loop = application::MainLoop::new(name, size);

    // Run the application
    pollster::block_on(application::run(&mut main_loop));
}
