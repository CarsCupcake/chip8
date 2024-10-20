pub mod screen;

use core::convert::*;
use phf::*;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::vec::Vec;
use std::*;
use std::{thread, thread::JoinHandle, time};
use u4::*;

struct Array<T> {
    data: [T; 4096],
}

struct VecFormat<T> {
    data: Vec<T>,
}
impl<T: fmt::Debug> fmt::Debug for VecFormat<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

impl<T: fmt::Debug> fmt::Debug for Array<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

pub const PPROGRAM_STACK: Vec<u16> = Vec::new();
pub static mut TIMER: u16 = 0;
pub static mut RUNNING: bool = false;
pub static SOUND_TIMER: u16 = 0;
pub const PROGRAM_TIME: time::Duration = time::Duration::from_millis(16);
pub const PROGRAM_INSTRUCTIONS: Map<u8, fn(u16) -> u16> = phf_map! {
    0u8 => clear_screen,
    1u8 => jump,
    3u8 => equals,
    4u8 => not_equals,
    5u8 => equal_registers,
    6u8 => set,
    7u8 => add,
    8u8 => math_instruction,
    9u8 => not_equal_registers,
    10u8 => i_register_interaction,
    13u8 => draw
};
pub static mut MEMORY: [u8; 0x1000] = [0; 0x1000];
pub static mut REGISTERS: [u8; 16] = [0; 16];
pub static mut REGISTER_I: u16 = 0;
pub fn read_register(index: usize) -> u8 {
    unsafe { REGISTERS[index] }
}
pub fn read_memory(index: usize) -> u8 {
    unsafe { MEMORY[index] }
}
pub fn write_register(index: usize, value: u8) {
    unsafe {
        REGISTERS[index] = value;
    }
}
pub fn write_memory(index: usize, value: u8) {
    unsafe {
        MEMORY[index] = value;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 2 {
        let filename = args[1].clone();
        for (i, byte) in File::open(&filename).expect("err").bytes().enumerate() {
            write_memory(i + 0x200, byte.unwrap());
        }
        /*while i - 512 < buffer.len() {
        write_memory(i, buffer[i - 512]);
        i += 1;
        }*/
        unsafe {
            println!("{:?}", Array { data: MEMORY });
        }
    }
    let _ = run().join();
}

fn run() -> JoinHandle<()> {
    unsafe {
        assert!(!RUNNING);
        RUNNING = true;
        //Initiating Font in memory
        let font: [u8; 80] = [
            0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
            0x20, 0x60, 0x20, 0x20, 0x70, // 1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
            0x90, 0x90, 0xF0, 0x10, 0x10, // 4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
            0xF0, 0x10, 0x20, 0x40, 0x40, // 7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
            0xF0, 0x90, 0xF0, 0x90, 0x90, // A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
            0xF0, 0x80, 0x80, 0x80, 0xF0, // C
            0xE0, 0x90, 0x90, 0x90, 0xE0, // D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
            0xF0, 0x80, 0xF0, 0x80, 0x80, // F
        ];
        let mut p = 0x050usize;
        for i in font {
            write_memory(p, i);
            p += 1;
        }
        //REGISTER_I = 0x050;
        //MEMORY[512] = 0xd0;
        //MEMORY[513] = 0x14;
        //MEMORY[514] = 0x1F;
        //MEMORY[515] = 0xB0;
    }
    let _screen_thread = thread::spawn(|| {
        screen::main();
    });
    let program_thread = thread::spawn(|| {
        let mut pointer = 512u16;
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
                //println!("PC: {} Operation: {}", pointer, op_id);
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

fn draw(pointer: u16) -> u16 {
    unsafe {
        let vx = read_register(read_x(pointer).into());
        let vy = read_register(read_y(pointer).into());
        write_register(15, 0);
        let mut x = vx & 63;
        let mut y = vy & 31;
        let n: usize = read_n(pointer).into();
        let mut i = 0usize;
        while i < n || i < 63 {
            let regp: usize = REGISTER_I.into();
            let sprite_data = read_memory(regp + i);
            let mut j = 0;
            while j < 8 {
                let on = sprite_data >> j & 1;
                if screen::set_pixel(x, y, on == 1) && on == 0 {
                    write_register(15, 1);
                }
                y += 1;
                j += 1;
            }
            x += 1;
            i += 1;
        }
    }
    screen::update_screen();
    pointer
}

fn i_register_interaction(pointer: u16) -> u16 {
    unsafe {
        REGISTER_I = read_nnn(pointer);
    }
    pointer
}

fn set(pointer: u16) -> u16 {
    write_register(read_x(pointer).into(), read_nn(pointer));
    pointer
}

fn add(pointer: u16) -> u16 {
    let x: usize = read_x(pointer).into();
    write_register(x, read_register(x) + read_nn(pointer));
    pointer
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
    if read_y(pointer) == u4!(0xE) {
        //Clear screen
        unsafe {
            screen::SCREEN = [0u32; 2048];
            screen::update_screen();
        }
    }
    pointer
}

fn equals(pointer: u16) -> u16 {
    let x: usize = read_x(pointer).into();
    let nn = read_nn(pointer);
    if read_register(x) == nn {
        pointer + 2
    } else {
        pointer
    }
}

fn not_equals(pointer: u16) -> u16 {
    let x: usize = read_x(pointer).into();
    let nn = read_nn(pointer);
    if read_register(x) != nn {
        pointer + 2
    } else {
        pointer
    }
}

fn equal_registers(pointer: u16) -> u16 {
    if read_register(read_x(pointer).into()) == read_register(read_y(pointer).into()) {
        pointer + 2
    } else {
        pointer
    }
}

fn not_equal_registers(pointer: u16) -> u16 {
    if read_register(read_x(pointer).into()) != read_register(read_y(pointer).into()) {
        pointer + 2
    } else {
        pointer
    }
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
            MEMORY[512] = 0b1000_0000_u8;
            MEMORY[513] = 0b0001_0101_u8;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 1);
            assert_eq!(REGISTERS[15], 1);
            REGISTERS[0] = 2;
            REGISTERS[1] = 1;
            MEMORY[512] = 0b1000_0000_u8;
            MEMORY[513] = 0b0001_0111_u8;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 255);
            assert_eq!(REGISTERS[15], 0);
        }
        bitshift();
    }

    fn bitshift() {
        unsafe {
            REGISTERS[1] = 1;
            MEMORY[512] = 0b1000_0000_u8;
            MEMORY[513] = 0b0001_1110_u8;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run().join();
            assert_eq!(REGISTERS[0], 2);
            assert_eq!(REGISTERS[15], 0);
        }
    }
}
