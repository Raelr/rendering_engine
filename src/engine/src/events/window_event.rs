use std::collections::VecDeque;
use crate::platform::windows::windows_window;
use crate::platform::windows::windows_window::WindowsWindow;

pub struct WindowEvent<'a> {

    pub window : &'a mut windows_window::WindowsWindow,
    pub events : &'a mut VecDeque<Box<FnMut(&mut WindowsWindow)>>
}