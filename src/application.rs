use winit::{
    application::ApplicationHandler, dpi::PhysicalSize, event::WindowEvent, event_loop::EventLoop, window::Window
};

/// Runs the application
pub async fn run(main_loop: &mut MainLoop) {
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
    /// The size of the application window
    size: PhysicalSize<u32>,
    /// The currently opened window of the application
    window: Option<Window>,
}

impl MainLoop {
    /// Creates a new main loop with the supplied settings
    ///
    /// # Parameters
    ///
    /// name: The name of the application shown on the window
    ///
    /// size: The size of the window in pixels
    pub fn new(name: String, size: PhysicalSize<u32>) -> Self {
        return Self {
            name,
            size,
            window: None,
        };
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
        event_loop: &winit::event_loop::ActiveEventLoop,
        event: winit::event::WindowEvent,
    ) {
        // Find the correct event
        match event {
            WindowEvent::CloseRequested => self.main_window_close_request(event_loop),
            WindowEvent::RedrawRequested => self.main_window_redraw_requested(),
            WindowEvent::Resized(size) => self.main_window_resized(size),
            _ => (),
        }
    }

    /// Run when the main window is to be closed
    /// 
    /// # Parameters
    /// 
    /// event_loop: The event loop currently running
    fn main_window_close_request(&self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Stop the application
        event_loop.exit();
    }

    /// Run when the main window must be redrawn
    fn main_window_redraw_requested(&self) {
        println!("Redrawing main window");
    }

    /// Run when the size of the window has changed
    fn main_window_resized(&mut self, size: PhysicalSize<u32>) {
        self.size = size;
    }
}

impl ApplicationHandler for MainLoop {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        // Open a new window
        let window_attributes = Window::default_attributes()
            .with_title(&self.name)
            .with_inner_size(self.size);

        self.window = Some(
            event_loop
                .create_window(window_attributes)
                .expect("Unable to create window"),
        );
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
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
        if window_id == window.id() {
            self.main_window_event(event_loop, event);
        }
    }

    fn suspended(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Close the window
        self.window = None;
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        // Close the window
        self.window = None;
    }
}
