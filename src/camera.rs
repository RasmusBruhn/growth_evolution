use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{constants::SQRT_3, types};

/// Describes a how the camera is moving
pub struct Camera {
    /// The movement keys: d, e, w, a, z, x
    active_move: [bool; 6],
    /// The zoom keys: s, q
    active_zoom: [bool; 2],
    /// The rotation keys: r, c
    active_rotate: [bool; 2],
    /// True if any button is pressed and the camera needs to be updated
    active: bool,
    /// The speed of movement
    speed_move: f64,
    /// The speed of zooming
    speed_zoom: f64,
    /// The speed of rotation
    speed_rotate: f64,
    /// The framerate of the program, this is how many times a second the transform should be updated
    framerate: f64,
    /// The current transform
    transform: types::Transform2D,
    /// The transform to make the aspect ratio correct
    transform_aspect: types::Transform2D,
    /// The transform to apply to the current transform every frame
    transform_update: types::Transform2D,
}

impl Camera {
    /// Creates a new camera
    ///
    /// # Parameters
    ///
    /// framerate: The expected framerate of the program, this is how many times a second the transform should be updated
    ///
    /// transform: The initial transform to use
    ///
    /// size: The current size of the window
    pub fn new(
        speed_move: f64,
        speed_zoom: f64,
        speed_rotate: f64,
        framerate: f64,
        transform: &types::Transform2D,
        size: &winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        Self {
            active_move: [false; 6],
            active_zoom: [false; 2],
            active_rotate: [false; 2],
            active: false,
            speed_move,
            speed_zoom,
            speed_rotate,
            framerate,
            transform: *transform,
            transform_aspect: Self::size_to_aspect(size),
            transform_update: types::Transform2D::identity(),
        }
    }

    /// Set one of the movement keys, id 0-5 for d, e, w, a, z, x
    ///
    /// # Parameters
    ///
    /// id: The id of the key to set
    ///
    /// active: True if it is pressed down
    pub fn set_key_move(&mut self, id: usize, active: bool) {
        self.active_move[id] = active;
        self.reload_transform();
    }

    /// Set one of the zoom keys, id 0-1 for s, q
    ///
    /// # Parameters
    ///
    /// id: The id of the key to set
    ///
    /// active: True if it is pressed down
    pub fn set_key_zoom(&mut self, id: usize, active: bool) {
        self.active_zoom[id] = active;
        self.reload_transform();
    }

    /// Set one of the rotate keys, id 0-1 for r, c
    ///
    /// # Parameters
    ///
    /// id: The id of the key to set
    ///
    /// active: True if it is pressed down
    pub fn set_key_rotate(&mut self, id: usize, active: bool) {
        self.active_rotate[id] = active;
        self.reload_transform();
    }

    /// Sets all the keys
    ///
    /// # Parameters
    ///
    /// active_move: Tells if each move key is active or nor
    ///
    /// active_zoom: Tells if each zoom key is active or not
    ///
    /// active_rotate: Tells if each rotate key is active or not
    pub fn set_keys(
        &mut self,
        active_move: &[bool; 6],
        active_zoom: &[bool; 2],
        active_rotate: &[bool; 2],
    ) {
        self.active_move = *active_move;
        self.active_zoom = *active_zoom;
        self.active_rotate = *active_rotate;
        self.reload_transform();
    }

    /// Attempts to use a key press from a key event, if the key press is used,
    /// it returns true, if it is ignored, it returns false
    ///
    /// # Parameters
    ///
    /// event: The key event to handle
    pub fn apply_key(&mut self, event: KeyEvent) -> bool {
        let active = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        return match event.physical_key {
            PhysicalKey::Unidentified(_) => false,
            PhysicalKey::Code(code) => match code {
                KeyCode::KeyD => {
                    self.set_key_move(0, active);
                    true
                }
                KeyCode::KeyE => {
                    self.set_key_move(1, active);
                    true
                }
                KeyCode::KeyW => {
                    self.set_key_move(2, active);
                    true
                }
                KeyCode::KeyA => {
                    self.set_key_move(3, active);
                    true
                }
                KeyCode::KeyZ => {
                    self.set_key_move(4, active);
                    true
                }
                KeyCode::KeyX => {
                    self.set_key_move(5, active);
                    true
                }
                KeyCode::KeyS => {
                    self.set_key_zoom(0, active);
                    true
                }
                KeyCode::KeyQ => {
                    self.set_key_zoom(1, active);
                    true
                }
                KeyCode::KeyR => {
                    self.set_key_rotate(0, active);
                    true
                }
                KeyCode::KeyC => {
                    self.set_key_rotate(1, active);
                    true
                }
                _ => false,
            },
        };
    }

    /// Reset all of the input such that all of it is turned off
    pub fn reset_updates(&mut self) {
        self.active_move.iter_mut().for_each(|val| *val = false);
        self.active_zoom.iter_mut().for_each(|val| *val = false);
        self.active_rotate.iter_mut().for_each(|val| *val = false);
        self.reload_transform();
    }

    /// Sets the movement speed
    ///
    /// # Parameters
    ///
    /// speed: The new movement speed
    pub fn set_speed_move(&mut self, speed: f64) {
        self.speed_move = speed;
        self.reload_transform();
    }

    /// Sets the zoom speed
    ///
    /// # Parameters
    ///
    /// speed: The new zoom speed
    pub fn set_speed_zoom(&mut self, speed: f64) {
        self.speed_zoom = speed;
        self.reload_transform();
    }

    /// Sets the rotation speed
    ///
    /// # Parameters
    ///
    /// speed: The new rotation speed
    pub fn set_speed_rotate(&mut self, speed: f64) {
        self.speed_rotate = speed;
        self.reload_transform();
    }

    /// Sets all speeds
    ///
    /// # Parameters
    ///
    /// speed_move: The new movement speed
    ///
    /// speed_zoom: The new zoom speed
    ///
    /// speed_rotate: The new rotation speed
    pub fn set_speeds(&mut self, speed_move: f64, speed_zoom: f64, speed_rotate: f64) {
        self.speed_move = speed_move;
        self.speed_zoom = speed_zoom;
        self.speed_rotate = speed_rotate;
        self.reload_transform();
    }

    /// Sets the framerate for if it changes
    ///
    /// # Parameters
    ///
    /// framerate: The new framerate, this is how many times a second the transform should be updated
    pub fn set_framerate(&mut self, framerate: f64) {
        self.framerate = framerate;
        self.reload_transform();
    }

    /// Recalculates the aspect transform after resizing
    ///
    /// # Parameters
    ///
    /// size: THe new size of the window
    pub fn resize(&mut self, size: &winit::dpi::PhysicalSize<u32>) {
        self.transform_aspect = Self::size_to_aspect(size);
    }

    /// Retrieves the transform
    pub fn get_transform(&self) -> types::Transform2D {
        &self.transform_aspect * self.transform
    }

    /// Sets a new transform
    ///
    /// # Parameters
    ///
    /// transform: The new transform to set
    pub fn set_transform(&mut self, transform: &types::Transform2D) {
        self.transform = *transform;
    }

    /// Update the transform using the current input, should be run once per frame
    ///
    /// Returns true if the transform has updated
    pub fn update_transform(&mut self) -> bool {
        if !self.active {
            return false;
        }

        self.transform = self.transform_update * self.transform;

        return true;
    }

    /// Reload the transform_update for when the input has changed
    fn reload_transform(&mut self) {
        // Check if it is active
        self.active = self.active_move.iter().any(|&x| x)
            || self.active_zoom.iter().any(|&x| x)
            || self.active_rotate.iter().any(|&x| x);

        if !self.active {
            return;
        }

        // Calculate the movement velocity
        let move_speed = self.speed_move / self.framerate;
        let mut move_dir = self
            .active_move
            .iter()
            .zip(KEY_DIRECTION_HEX.iter())
            .filter_map(|(&active, dir)| if active { Some(dir) } else { None })
            .fold(types::Point::new(0.0, 0.0), |prev, next| prev + next);
        if move_dir.x != 0.0 || move_dir.y != 0.0 {
            move_dir = move_dir * move_speed / move_dir.norm();
        }

        // Calculate the zoom velocity
        let zoom_val = 1.0 + self.speed_zoom / self.framerate;
        let key_zoom = [zoom_val, 1.0 / zoom_val];
        let zoom_dir = self
            .active_zoom
            .iter()
            .zip(key_zoom.iter())
            .filter_map(|(&active, zoom)| if active { Some(zoom) } else { None })
            .fold(1.0, |prev, next| prev * next);

        // Calculate the rotation velocity
        let rotate_val = self.speed_rotate / self.framerate;
        let key_rotate = [-rotate_val, rotate_val];
        let rotate_dir = self
            .active_rotate
            .iter()
            .zip(key_rotate.iter())
            .filter_map(|(&active, rotate)| if active { Some(rotate) } else { None })
            .fold(0.0, |prev, next| prev + next);

        // Combine all of the transforms
        let transform_move = types::Transform2D::translate(&move_dir);
        let transform_zoom = types::Transform2D::scale(&types::Point::new(zoom_dir, zoom_dir));
        let transform_rotate = types::Transform2D::rotation(rotate_dir);

        self.transform_update = transform_rotate * transform_zoom * transform_move;
    }

    /// Converts a size to an aspect transform
    ///
    /// # Parameters
    ///
    /// size: The size of the window
    fn size_to_aspect(size: &winit::dpi::PhysicalSize<u32>) -> types::Transform2D {
        types::Transform2D::scale(&types::Point::new(
            (size.height as f64) / (size.width as f64),
            1.0,
        ))
    }
}

const KEY_DIRECTION_HEX: [types::Point; 6] = [
    types::Point { x: 1.0, y: 0.0 },
    types::Point {
        x: 0.5,
        y: 0.5 * SQRT_3,
    },
    types::Point {
        x: -0.5,
        y: 0.5 * SQRT_3,
    },
    types::Point { x: -1.0, y: 0.0 },
    types::Point {
        x: -0.5,
        y: -0.5 * SQRT_3,
    },
    types::Point {
        x: 0.5,
        y: -0.5 * SQRT_3,
    },
];
