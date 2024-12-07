use std::{env, f64::consts::PI};

use winit::dpi::PhysicalSize;

pub mod application;
pub mod constants;
pub mod graphics;
pub mod map;
pub mod render;
pub mod types;

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

    // Setup the map
    let sources = map::SourceMap {
        nutrients: vec![map::Source::Gaussian(types::Gaussian::new(
            2.0 * PI * 1.0 * 1.0,
            types::Point::new(0.0, 0.0),
            types::Matrix::new(&[[1.0, 0.0], [0.0, 1.0]]),
        ))],
        energy: vec![map::Source::Gaussian(types::Gaussian::new(
            2.0 * PI * 2.0 * 1.0,
            types::Point::new(3.0, 0.0),
            types::Matrix::new(&[[1.0, 0.0], [0.0, 2.0]]),
        ))],
        water: vec![map::Source::Gaussian(types::Gaussian::new(
            2.0 * PI * 1.5 * 0.5,
            types::Point::new(0.0, 19.0),
            types::Matrix::new(&[[1.0, 0.5], [0.5, 1.0]]),
        ))],
    };
    let map_data = Box::new(map::MapCyclic::new());
    let map = map::Map::new(map_data, sources);

    // Setup the main loop
    let mut main_loop = application::MainLoop::new(name, size, graphics_settings, map);

    // Run the application
    application::run(&mut main_loop);
}
