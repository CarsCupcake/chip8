use core::convert::*;
use phf::*;
use std::vec::Vec;
use std::*;
use std::{thread, thread::JoinHandle, time};
use u4::*;

pub const PPROGRAM_STACK: Vec<u16> = Vec::new();
pub static mut TIMER: u16 = 0;
pub static mut RUNNING: bool = false;
pub static SOUND_TIMER: u16 = 0;
pub const PROGRAM_TIME: time::Duration = time::Duration::from_millis(16);
pub const PROGRAM_INSTRUCTIONS: Map<u8, fn(u16) -> u16> = phf_map! {
    1u8 => jump,
    8u8 => math_instruction
};
pub static mut MEMORY: [u8; 4096] = [0; 4096];
pub static mut REGISTERS: [u8; 16] = [0; 16];

fn main() {
    unsafe {
        MEMORY[100] = 0b0001_0000_u8;
        MEMORY[101] = 2
    }

    let _ = run().join();
}

fn run() -> JoinHandle<()> {
    unsafe {
        assert_eq!(RUNNING, false);
        RUNNING = true;
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
                let op_id = AsNibbles([MEMORY[pointer as usize]])
                    .get(0)
                    .expect("Illegal Pointer")
                    .into();
                println!("PC: {} Operation: {}", pointer, op_id);
                let opt_intruction = PROGRAM_INSTRUCTIONS.get(&op_id);
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
            TIMER = TIMER.saturating_sub(1);
            if !RUNNING {
                break;
            }
        }
    });
    program_thread
}

fn math_instruction(pointer: u16) -> u16 {
    let operation_type: u8 = read_n(pointer).into();
    let vx = read_x(pointer);
    let vy = read_y(pointer);
    let x: usize = vx.into();
    let y: usize = vy.into();

    unsafe {
        match operation_type {
            0 => {
                REGISTERS[x] = REGISTERS[y];
            }
            1 => {
                REGISTERS[x] |= REGISTERS[y];
            }
            2 => {
                REGISTERS[x] &= REGISTERS[y];
            }
            3 => {
                REGISTERS[x] ^= REGISTERS[y];
            }
            4 => {
                REGISTERS[15] = if REGISTERS[x].checked_add(REGISTERS[y]).is_none() {
                    1
                } else {
                    0
                };
                REGISTERS[x] += REGISTERS[y];
            }
            5 => {
                REGISTERS[15] = if None == REGISTERS[x].checked_sub(REGISTERS[y]) {
                    0
                } else {
                    1
                };
                REGISTERS[x] -= REGISTERS[y];
            }
            7 => {
                REGISTERS[15] = if None == REGISTERS[y].checked_sub(REGISTERS[x]) {
                    0
                } else {
                    1
                };
                REGISTERS[x] = REGISTERS[y] - REGISTERS[x];
            }
            6 => {
                let reg_y = REGISTERS[y];
                let shifted = reg_y >> 1;
                let remainder = reg_y - (shifted << 1);
                REGISTERS[x] = shifted;
                REGISTERS[15] = remainder;
            }
            14 => {
                let reg_y = REGISTERS[y];
                let shifted = reg_y << 1;
                let remainder = reg_y - (shifted >> 1);
                REGISTERS[x] = shifted;
                REGISTERS[15] = remainder >> 7;
            }

            _ => {
                panic!("Illigal Math Operation")
            }
        };
    }

    pointer
}

fn clear_screen(pointer: u16) -> u16 {
    pointer
}

fn jump(pointer: u16) -> u16 {
    read_nnn(pointer) - 2
}

fn read_n(pointer: u16) -> U4 {
    unsafe {
        let nibble = AsNibbles([MEMORY[(pointer + 1) as usize]]);
        nibble.get(1).expect("Not existing")
    }
}

/**
 * Reads the 3th nibble
 */
fn read_y(pointer: u16) -> U4 {
    unsafe {
        let nibble = AsNibbles([MEMORY[(pointer + 1) as usize]]);
        nibble.get(0).expect("Not existing")
    }
}
/**
 * Reads the 2rd nibble
 */
fn read_x(pointer: u16) -> U4 {
    unsafe {
        let nibble = AsNibbles([MEMORY[(pointer) as usize]]);
        nibble.get(1).expect("Not existing")
    }
}

fn read_nn(pointer: u16) -> u8 {
    unsafe { MEMORY[(pointer + 1) as usize] }
}

fn read_nnn(pointer: u16) -> u16 {
    let first: u16 = read_n(pointer - 1).into();
    unsafe { ((first << 8) + MEMORY[(pointer + 1) as usize] as u16) as u16 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nibble_reading() {
        unsafe {
            MEMORY[0] = 0b0000_0001_u8;
            MEMORY[1] = 0b0010_0011_u8;
        }
        assert_eq!(read_n(0), u4!(3));
        assert_eq!(read_x(0), u4!(1));
        assert_eq!(read_y(0), u4!(2));
        assert_eq!(read_nn(0), 0b0010_0011_u8);
        assert_eq!(read_nnn(0), 0b0000_0001_0010_0011_u16);
    }

    #[test]
    fn main_test() {
        unsafe {
            REGISTERS[0] = 2;
            REGISTERS[1] = 1;
            MEMORY[0] = 0b1000_0000_u8;
            MEMORY[1] = 0b0001_0101_u8;
            MEMORY[2] = 0b0001_1111_u8;
            MEMORY[3] = 0b1010_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 1);
            assert_eq!(REGISTERS[15], 1);
            REGISTERS[0] = 2;
            REGISTERS[1] = 1;
            MEMORY[0] = 0b1000_0000_u8;
            MEMORY[1] = 0b0001_0111_u8;
            MEMORY[2] = 0b0001_1111_u8;
            MEMORY[3] = 0b1010_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 255);
            assert_eq!(REGISTERS[15], 0);
        }
        bitshift();
    }

    fn bitshift() {
        unsafe {
            REGISTERS[1] = 1;
            MEMORY[0] = 0b1000_0000_u8;
            MEMORY[1] = 0b0001_1110_u8;
            MEMORY[2] = 0b0001_1111_u8;
            MEMORY[3] = 0b1010_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 2);
            assert_eq!(REGISTERS[15], 0);
        }
    }
}
