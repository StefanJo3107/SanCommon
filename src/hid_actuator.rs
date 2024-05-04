use std::time::Duration;
use crate::keycodes::KeyCode;

pub trait HidActuator {
    fn get_cursor_position(&self) -> (u16, u16);
    fn move_cursor(&mut self, x: i16, y: i16);
    fn mouse_down(&mut self, button: u8);
    fn mouse_up(&mut self);
    fn scroll_mouse_wheel(&mut self, x: i16, y: i16);
    fn key_down(&mut self, key: &Vec<u8>);
    fn clear_keys(&mut self);
    fn sleep(&mut self, duration_ms: usize);
}