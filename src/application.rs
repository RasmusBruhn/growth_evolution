use crate::{camera, graphics, map, render};
use std::{
    sync::Arc,
    time::{Duration, Instant},
};
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceId, KeyEvent, StartCause, WindowEvent},
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::Window,
};

/// Runs the application
pub fn run(main_loop: &mut MainLoop) {
    // Setup logging
    env_logger::init();

    // Create the event loop
    let event_loop = match EventLoop::new() {
        Ok(value) => value,
        Err(error) => {
            eprintln!("Unable to create event loop: {:?}", error);
            return;
        }
    };

    if let Err(error) = event_loop.run_app(main_loop) {
        eprintln!("An error occured in the main loop: {:?}", error);
        return;
    }
}

/// Controls the main game loop of the application
pub struct MainLoop {
    /// The name of the application
    name: String,
    /// The framerate of the application
    framerate: f64,
    /// The size of the application window
    size: PhysicalSize<u32>,
    /// The settings for rendering
    graphics_settings: graphics::Settings,
    /// The currently opened window of the application
    window: Option<RenderedWindow>,
    /// The map to display
    map: map::Map,
    /// The camera for controlling what is displayed
    camera: camera::HexCamera,
}

impl MainLoop {
    /// Creates a new main loop with the supplied settings
    ///
    /// # Parameters
    ///
    /// name: The name of the application shown on the window
    ///
    /// size: The size of the window in pixels
    pub fn new(
        name: String,
        framerate: f64,
        size: PhysicalSize<u32>,
        graphics_settings: graphics::Settings,
        map: map::Map,
        camera: camera::HexCamera,
    ) -> Self {
        return Self {
            name,
            framerate,
            size,
            graphics_settings,
            window: None,
            map,
            camera,
        };
    }

    /// Handles the initialization of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    fn game_loop_init(&mut self, event_loop: &ActiveEventLoop) {
        // Set resume time for the first game loop iteration
        let wait_time = (1e6 / self.framerate).floor() as u64;
        event_loop.set_control_flow(ControlFlow::WaitUntil(
            Instant::now() + Duration::from_micros(wait_time),
        ));

        // Set the size of the camera
        self.camera.resize(&self.size);
    }

    /// Handles the iteration of the game loop
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop to handle
    ///
    /// requested_resume: The time requested to resume
    fn game_loop_iteration(&mut self, event_loop: &ActiveEventLoop, requested_resume: Instant) {
        // Update the time, make sure we do not get a backlog by skipping if we should wait until before now
        let mut new_time =
            requested_resume + Duration::from_micros((1e6 / self.framerate).floor() as u64);
        let now_time = Instant::now();
        if new_time < now_time {
            new_time = now_time;
        }
        event_loop.set_control_flow(ControlFlow::WaitUntil(new_time));

        // Get the window and id
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot process game loop because window is not initialized");
                return;
            }
        };

        // Update the camera
        if self.camera.update_transform() {
            window.get_window().request_redraw();
        }
    }

    /// Handles a window event for the main window
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    ///
    /// event: The event to be handled
    fn main_window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        event: winit::event::WindowEvent,
    ) {
        // Find the correct event
        match event {
            WindowEvent::CloseRequested => self.main_window_close_request(event_loop),
            WindowEvent::RedrawRequested => self.main_window_redraw_requested(),
            WindowEvent::Resized(size) => self.main_window_resized(size),
            WindowEvent::KeyboardInput {
                device_id,
                event,
                is_synthetic,
            } => self.main_window_keyboard_input(device_id, event, is_synthetic),
            _ => (),
        }
    }

    /// Run when the main window is to be closed
    ///
    /// # Parameters
    ///
    /// event_loop: The event loop currently running
    fn main_window_close_request(&self, event_loop: &ActiveEventLoop) {
        // Stop the application
        event_loop.exit();
    }

    /// Run when the main window must be redrawn
    fn main_window_redraw_requested(&self) {
        let window = self.window.as_ref().expect("Should not happen");

        // Get the current view
        let output_texture = match window
            .get_render_state()
            .get_surface()
            .get_current_texture()
        {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Unable to get texture: {:?}", error);
                return;
            }
        };
        let view = output_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        // Draw the map
        window.graphics_state.render(
            window.get_render_state(),
            &view,
            &self.camera.get_transform(),
        );

        // Show to screen
        output_texture.present();
    }

    /// Run when the size of the window has changed
    ///
    /// # Parameters
    ///
    /// size: The new size of the window
    fn main_window_resized(&mut self, size: PhysicalSize<u32>) {
        // Set the new size
        self.size = size;

        // Update the window
        self.window
            .as_mut()
            .expect("Should not happen")
            .get_render_state_mut()
            .resize(size);

        // Update the camera
        self.camera.resize(&size);
    }

    /// Handles any keyboard input like camera movement
    ///
    /// # Parameters
    ///
    /// device_id: The id of the device giving the input
    ///
    /// event: The event to handle
    ///
    /// is_synthetic: True if the event was created by winit
    fn main_window_keyboard_input(
        &mut self,
        _device_id: DeviceId,
        event: KeyEvent,
        _is_synthetic: bool,
    ) {
        // Handle camera events, stop if input was captured
        if self.camera.apply_key(&event) {
            return;
        }
    }
}

impl ApplicationHandler for MainLoop {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        // Open a new window
        let window_attributes = Window::default_attributes()
            .with_title(&self.name)
            .with_inner_size(self.size);

        let window = match event_loop.create_window(window_attributes) {
            Ok(value) => value,
            Err(error) => {
                eprintln!("Unable to create window: {:?}", error);
                event_loop.exit();
                return;
            }
        };

        // Add a render state
        self.window = match pollster::block_on(RenderedWindow::new(window, self.graphics_settings))
        {
            Ok(value) => Some(value),
            Err(error) => {
                eprintln!("Unable to add render state: {:?}", error);
                event_loop.exit();
                return;
            }
        }
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        // Get the window and id
        let window = match &self.window {
            Some(window) => window,
            None => {
                eprintln!("Cannot process events because window is not initialized");
                return;
            }
        };

        // Find the correct window and handle event correspondingly
        if window_id == window.get_window().id() {
            self.main_window_event(event_loop, event);
        }
    }

    fn new_events(&mut self, event_loop: &ActiveEventLoop, cause: StartCause) {
        // Figure out why it is waiting
        match cause {
            StartCause::Init => self.game_loop_init(event_loop),
            StartCause::ResumeTimeReached {
                requested_resume, ..
            } => self.game_loop_iteration(event_loop, requested_resume),
            _ => (),
        }
    }

    fn suspended(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = None;
    }

    fn exiting(&mut self, _event_loop: &ActiveEventLoop) {
        // Close the window
        self.window = None;
    }
}

/// A window with an assosciated render state
pub struct RenderedWindow {
    /// The window, it must be in an Arc because it is shared with the render state
    window: Arc<Window>,
    /// The render state to render onto the window
    render_state: render::RenderState,
    /// The graphics state used for rendering
    graphics_state: graphics::State,
}

impl RenderedWindow {
    /// Constructs a new rendered window
    ///
    /// # Parameters
    ///
    /// window: The window to add a render state to
    pub async fn new(
        window: Window,
        graphics_settings: graphics::Settings,
    ) -> Result<Self, render::NewRenderStateError> {
        let window = Arc::new(window);
        let render_state = render::RenderState::new(&window).await?;
        let graphics_state = graphics::State::new(&render_state, graphics_settings);

        return Ok(Self {
            window,
            render_state,
            graphics_state,
        });
    }

    /// Retrieves a reference to the render state
    pub fn get_render_state(&self) -> &render::RenderState {
        return &self.render_state;
    }

    /// Retrieves a mutable reference to the render state
    pub fn get_render_state_mut(&mut self) -> &mut render::RenderState {
        return &mut self.render_state;
    }

    /// Retrieves a reference to the window
    pub fn get_window(&self) -> &Window {
        return &self.window;
    }
}
