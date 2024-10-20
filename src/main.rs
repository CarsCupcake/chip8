use core::convert::*;
use phf::*;
use std::vec::Vec;
use std::*;
use std::{thread, time};
use u4::*;

pub const PPROGRAM_STACK: Vec<u16> = Vec::new();
pub static mut TIMER: u16 = 0;
pub static mut RUNNING: bool = true;
pub static SOUND_TIMER: u16 = 0;
pub const PROGRAM_TIME: time::Duration = time::Duration::from_millis(16);
pub const PROGRAM_INSTRUCTIONS: Map<u8, fn(u16) -> (u16)> = phf_map! {
    1u8 => jump
};
pub static mut MEMORY: [u8; 4096] = [0; 4096];
pub static mut REGISTERS: [u8; 16] = [0; 16];

fn one() {}

fn main() {
    unsafe {
        MEMORY[100] = 1;
        MEMORY[101] = 2
    }
    let program_thread = thread::spawn(|| {
        let mut pointer = 0u16;
        loop {
            unsafe {
                if pointer as usize >= MEMORY.len() {
                    RUNNING = !RUNNING;
                }
                thread::sleep(PROGRAM_TIME);
                //Main Program Loop
                if !RUNNING {
                    break;
                }
                println!("{}", pointer);
                let opt_intruction = PROGRAM_INSTRUCTIONS.get(&MEMORY[pointer as usize]);
                if let Some(instruction) = opt_intruction {
                    pointer = instruction(pointer);
                }
                pointer += 2;
            }
        }
    });
    let _sound_thread = thread::spawn(|| loop {
        thread::sleep(PROGRAM_TIME);
        unsafe {
            if TIMER > 0 {
                TIMER -= 1;
            }
            if !RUNNING {
                break;
            }
        }
    });
    program_thread.join();
}

fn fetch() {}

fn decode() {}

fn execute() {}

fn clear_screen(pointer: u16) -> u16 {
    return pointer;
}

fn jump(pointer: u16) -> u16 {
    return read_nnn(pointer) - 2;
}

fn read_n(pointer: u16) -> U4 {
    unsafe {
        let nibble = AsNibbles([MEMORY[(pointer + 1) as usize]]);
        return nibble.get(1).expect("Not existing");
    }
}

fn read_nn(pointer: u16) -> u8 {
    unsafe {
        return MEMORY[(pointer + 1) as usize];
    }
}

fn read_nnn(pointer: u16) -> u16 {
    unsafe {
        return (((AsNibbles([MEMORY[pointer as usize]])
            .get(0)
            .expect("Not Existing") as u16)
            << 8)
            + MEMORY[(pointer + 1) as usize] as u16) as u16;
    }
}
