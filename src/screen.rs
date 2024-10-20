#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(unused_imports)]
#![allow(clippy::single_match)]
#![allow(clippy::zero_ptr)]

use beryllium::{
    events::Event,
    init::InitFlags,
    video::{CreateWinArgs, GlContextFlags, GlProfile, GlSwapInterval, GlWindow},
    *,
};
use core::{
    convert::{TryFrom, TryInto},
    mem::{size_of, size_of_val},
    ptr::null,
};
use ogl33::*;
use std::*;
use ultraviolet::*;

pub static mut SCREEN: [[bool; 32]; 64] = [[false; 32]; 64];
pub static mut WINDOW: Option<GlWindow> = None;

pub fn main() {
    let sdl = Sdl::init(init::InitFlags::EVERYTHING);
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_context_major_version(3).unwrap();
    sdl.set_gl_profile(video::GlProfile::Core).unwrap();
    #[cfg(target_os = "macos")]
    {
        sdl.set_gl_context_flags(video::GlContextFlags::FORWARD_COMPATIBLE)
            .unwrap();
    }
    let win_args = video::CreateWinArgs {
        title: "chip8",
        width: 64 * 4,
        height: 32 * 4,
        allow_high_dpi: true,
        borderless: true,
        resizable: false,
    };
    unsafe {
        WINDOW = Some(
            sdl.create_gl_window(win_args)
                .expect("couldn't make a window and context"),
        );
        'main_loop: loop {
            // handle events this frame
            while let Some(event) = sdl.poll_events() {
                match event {
                    (events::Event::Quit, _) => break 'main_loop,
                    _ => (),
                }
            }
            // now the events are clear

            // here's where we could change the world state and draw.
        }
    }
}

pub fn update_screen() {}
