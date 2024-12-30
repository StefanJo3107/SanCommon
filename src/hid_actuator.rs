pub trait HidActuator {
    fn move_cursor(&mut self, x: i8, y: i8);
    fn mouse_down(&mut self, button: u8);
    fn mouse_up(&mut self);
    fn scroll_mouse_wheel(&mut self, x: i8, y: i8);
    fn key_down(&mut self, key: &Vec<u8>);
    fn clear_keys(&mut self);
    fn sleep(&mut self, duration_ms: usize);
}