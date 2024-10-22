use crate::RUNNING;
use minifb::*;
use std::*;

pub static mut SCREEN: [u32; 2048] = [0u32; 2048];
pub static mut WINDOW: Option<Window> = None;

pub fn main() {
    unsafe {
        let mut w = Window::new(
            "chip8",
            64,
            32,
            WindowOptions {
                borderless: true,
                title: true,
                resize: false,
                scale: Scale::X16,
                scale_mode: ScaleMode::Stretch,
                topmost: false,
                transparency: false,
                none: false,
            },
        )
        .unwrap();
        w.update();
        WINDOW = Some(w);
    }
}

pub fn update_screen() {
    unsafe {
        if let Some(w) = WINDOW.as_mut() {
            if w.update_with_buffer(&SCREEN, 64, 32).is_err() {
                RUNNING = false;
                panic!("Screen drawer faild!")
            }
        }
    }
}

pub fn set_pixel(x: u8, y: u8, on: bool) -> bool {
    unsafe {
        let prev = SCREEN[y as usize * 64 + x as usize];
        SCREEN[y as usize * 64 + x as usize] = if on { 0xFFFFFF } else { 0 };
        prev == 0xFFFFFF
    }
}
