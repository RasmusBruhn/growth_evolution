use winit::{
    event::{ElementState, KeyEvent},
    keyboard::{KeyCode, PhysicalKey},
};

use super::{constants::SQRT_3, types};

/// Describes a how the camera is moving
#[derive(Clone, Copy, Debug)]
pub struct HexCamera {
    /// All the settings
    settings: HexCameraSettings,
    /// The movement keys: d, e, w, a, z, x
    active_move: [bool; 6],
    /// The zoom keys: s, q
    active_zoom: [bool; 2],
    /// The rotation keys: r, c
    active_rotate: [bool; 2],
    /// True if any button is pressed and the camera needs to be updated
    active: bool,
    /// The current transform
    transform: types::Transform2D,
    /// The transform to make the aspect ratio correct
    transform_aspect: types::Transform2D,
    /// The transform to apply to the current transform every frame
    transform_update: types::Transform2D,
}

impl HexCamera {
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
        settings: &HexCameraSettings,
        transform: &types::Transform2D,
        size: &winit::dpi::PhysicalSize<u32>,
    ) -> Self {
        Self {
            active_move: [false; 6],
            active_zoom: [false; 2],
            active_rotate: [false; 2],
            active: false,
            settings: *settings,
            transform: *transform,
            transform_aspect: Self::size_to_aspect(size),
            transform_update: types::Transform2D::identity(),
        }
    }

    /// Retrieves a reference to the settings
    pub fn get_settings(&self) -> &HexCameraSettings {
        return &self.settings;
    }

    /// Constructs a mutator for the settings
    pub fn get_settings_mut(&mut self) -> HexCameraSettingsMut {
        return HexCameraSettingsMut { camera: self };
    }

    /// Attempts to use a key press from a key event, if the key press is used,
    /// it returns true, if it is ignored, it returns false
    ///
    /// # Parameters
    ///
    /// event: The key event to handle
    pub fn apply_key(&mut self, event: &KeyEvent) -> bool {
        let active = match event.state {
            ElementState::Pressed => true,
            ElementState::Released => false,
        };

        match event.physical_key {
            PhysicalKey::Unidentified(_) => return false,
            PhysicalKey::Code(code) => match code {
                KeyCode::KeyD => self.active_move[0] = active,
                KeyCode::KeyE => self.active_move[1] = active,
                KeyCode::KeyW => self.active_move[2] = active,
                KeyCode::KeyA => self.active_move[3] = active,
                KeyCode::KeyZ => self.active_move[4] = active,
                KeyCode::KeyX => self.active_move[5] = active,
                KeyCode::KeyS => self.active_zoom[0] = active,
                KeyCode::KeyQ => self.active_zoom[1] = active,
                KeyCode::KeyR => self.active_rotate[0] = active,
                KeyCode::KeyC => self.active_rotate[1] = active,
                _ => return false,
            },
        };

        // Reload the update transform
        self.reload_transform();

        return true;
    }

    /// Reset all of the input such that all of it is turned off
    pub fn reset_keys(&mut self) {
        self.active_move.iter_mut().for_each(|val| *val = false);
        self.active_zoom.iter_mut().for_each(|val| *val = false);
        self.active_rotate.iter_mut().for_each(|val| *val = false);
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
        let move_speed = self.settings.speed_move / self.settings.framerate;
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
        let zoom_val = 1.0 + self.settings.speed_zoom / self.settings.framerate;
        let key_zoom = [zoom_val, 1.0 / zoom_val];
        let zoom_dir = self
            .active_zoom
            .iter()
            .zip(key_zoom.iter())
            .filter_map(|(&active, zoom)| if active { Some(zoom) } else { None })
            .fold(1.0, |prev, next| prev * next);

        // Calculate the rotation velocity
        let rotate_val = self.settings.speed_rotate / self.settings.framerate;
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
        println!("{:?}", self.transform_update);
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

/// A mutator for the hex camera settings, the transform for the camera is
/// reloaded once the mutator is dropped
#[derive(Debug)]
pub struct HexCameraSettingsMut<'cam> {
    camera: &'cam mut HexCamera,
}

impl<'map> HexCameraSettingsMut<'map> {
    /// Retrieves a reference to the settings
    pub fn get(&self) -> &HexCameraSettings {
        return &self.camera.settings;
    }

    /// Retrieves a mutable reference to the settings
    pub fn get_mut(&mut self) -> &mut HexCameraSettings {
        return &mut self.camera.settings;
    }
}

impl<'map> Drop for HexCameraSettingsMut<'map> {
    fn drop(&mut self) {
        // Force update the camera transform
        self.camera.reload_transform();
    }
}

/// All settings for a camera
#[derive(Clone, Copy, Debug)]
pub struct HexCameraSettings {
    /// The speed of movement
    pub speed_move: f64,
    /// The speed of zooming
    pub speed_zoom: f64,
    /// The speed of rotation
    pub speed_rotate: f64,
    /// The framerate of the program, this is how many times a second the transform should be updated
    pub framerate: f64,
}

impl HexCameraSettings {
    /// Creates camera settings with default values
    pub fn default() -> Self {
        return Self {
            speed_move: 4.0,
            speed_zoom: 1.2,
            speed_rotate: 1.0,
            framerate: 60.0,
        };
    }

    /// Changes the movement speed and returns the updated object
    ///
    /// # Parameters
    ///
    /// speed: The new movement speed
    pub fn with_speed_move(mut self, speed: f64) -> Self {
        self.speed_move = speed;
        return self;
    }

    /// Changes the zoom speed and returns the updated object
    ///
    /// # Parameters
    ///
    /// speed: The new zoom speed
    pub fn with_speed_zoom(mut self, speed: f64) -> Self {
        self.speed_zoom = speed;
        return self;
    }

    /// Changes the rotation speed and returns the updated object
    ///
    /// # Parameters
    ///
    /// speed: The new rotation speed
    pub fn with_speed_rotate(mut self, speed: f64) -> Self {
        self.speed_rotate = speed;
        return self;
    }

    /// Changes the framerate and returns the updated object
    ///
    /// # Parameters
    ///
    /// framerate: The new framerate
    pub fn with_framerate(mut self, framerate: f64) -> Self {
        self.framerate = framerate;
        return self;
    }
}
