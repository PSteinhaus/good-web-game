#![allow(dead_code)]

use stdweb::web::event::MouseButton as WebMouseButton;

use cgmath::Point2;
use std::collections::HashSet;

use crate::event::KeyCode;

#[derive(Hash, Debug, Eq, PartialEq)]
pub enum MouseButton {
    Left,
    Middle,
    Right,
    Button4,
    Button5,
}

impl From<&WebMouseButton> for MouseButton {
    fn from(button: &WebMouseButton) -> MouseButton {
        match button {
            WebMouseButton::Left => MouseButton::Left,
            WebMouseButton::Right => MouseButton::Right,
            WebMouseButton::Wheel => MouseButton::Middle,
            WebMouseButton::Button4 => MouseButton::Button4,
            WebMouseButton::Button5 => MouseButton::Button5,
        }
    }
}

pub struct InputHandler {
    pub keys: HashSet<KeyCode>,
    pub frame_keys: HashSet<KeyCode>,
    pub mouse_position: Point2<f64>,
    pub mouse_keys: HashSet<MouseButton>,
    pub wheel: f32,
}

impl InputHandler {
    pub fn new() -> InputHandler {
        InputHandler {
            keys: HashSet::new(),
            frame_keys: HashSet::new(),
            mouse_position: Point2::new(0., 0.),
            mouse_keys: HashSet::new(),
            wheel: 0.,
        }
    }

    pub fn handle_mouse_move(&mut self, mouse_x: i32, mouse_y: i32) {
        let mouse = Point2::new(mouse_x as f64, mouse_y as f64);

        self.mouse_position = mouse;
    }

    pub fn handle_mouse_down(&mut self, button: WebMouseButton) {
        self.mouse_keys.insert(MouseButton::from(&button));
    }

    pub fn handle_mouse_up(&mut self, button: WebMouseButton) {
        self.mouse_keys.remove(&MouseButton::from(&button));
    }

    pub fn handle_key_down(&mut self, keycode: KeyCode) {
        self.keys.insert(keycode);
        self.frame_keys.insert(keycode);
    }

    pub fn handle_end_frame(&mut self) {
        self.frame_keys.clear();
        self.wheel = 0.;
    }

    pub fn handle_key_up(&mut self, keycode: KeyCode) {
        self.keys.remove(&keycode);
    }

    pub fn handle_mouse_wheel(&mut self, delta_y: f64) {
        self.wheel = delta_y as f32;
    }

    pub fn is_key_pressed(&self, keycode: KeyCode) -> bool {
        self.keys.contains(&keycode)
    }

    pub fn is_key_down(&self, keycode: KeyCode) -> bool {
        self.frame_keys.contains(&keycode)
    }

    pub fn is_mouse_key_down(&self, key: &MouseButton) -> bool {
        self.mouse_keys.contains(key)
    }
}
