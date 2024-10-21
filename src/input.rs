use minifb::{Key, KeyRepeat};
use std::*;
use u4::*;

use crate::{screen, PROGRAM_TIME_60HZ};

pub fn await_key(key: Key) {
    unsafe {
        let window_ref = screen::WINDOW.as_ref().expect("");
        while !window_ref.is_key_pressed(key, KeyRepeat::No) {
            thread::sleep(PROGRAM_TIME_60HZ);
        }
    }
}
pub fn to_key(nibble: U4) -> Key {
    match nibble {
        U4::Dec00 => Key::Key1,
        U4::Dec01 => Key::Key2,
        U4::Dec02 => Key::Key3,
        U4::Dec03 => Key::Key4,
        U4::Dec04 => Key::Q,
        U4::Dec05 => Key::W,
        U4::Dec06 => Key::E,
        U4::Dec07 => Key::R,
        U4::Dec08 => Key::A,
        U4::Dec09 => Key::S,
        U4::Dec10 => Key::D,
        U4::Dec11 => Key::F,
        U4::Dec12 => Key::Y,
        U4::Dec13 => Key::X,
        U4::Dec14 => Key::C,
        U4::Dec15 => Key::V,
    }
}
