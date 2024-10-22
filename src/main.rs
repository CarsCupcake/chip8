pub mod input;
pub mod screen;

use core::convert::*;
use input::{await_key, to_key};
use minifb::*;
use phf::*;
use screen::{SCREEN, WINDOW};
use std::collections::VecDeque;
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

struct ScreenArray<T> {
    data: [T; 2048],
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

impl<T: fmt::Debug> fmt::Debug for ScreenArray<T> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        self.data[..].fmt(formatter)
    }
}

pub static mut PPROGRAM_STACK: VecDeque<u16> = VecDeque::new();
pub static mut TIMER: u16 = 0;
pub static mut RUNNING: bool = false;
pub static SOUND_TIMER: u16 = 0;
pub const PROGRAM_TIME_60HZ: time::Duration = time::Duration::from_nanos(16666666);
pub const PROGRAM_TIME_7000_INSTR_PM: time::Duration = time::Duration::from_nanos(1428571);
pub const PROGRAM_INSTRUCTIONS: Map<u8, fn(u16) -> u16> = phf_map! {
    0u8 => clear_screen,
    2u8 => subroutine,
    1u8 => jump,
    3u8 => equals,
    4u8 => not_equals,
    5u8 => equal_registers,
    6u8 => set,
    7u8 => add,
    8u8 => math_instruction,
    9u8 => not_equal_registers,
    10u8 => i_register_interaction,
    11u8 => jump_with_offset,
    12u8 => random,
    13u8 => draw,
    14u8 => key_input,
    15u8 => f_instructions,
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
    } else {
        write_memory(0, 128);
        write_memory(512, 0xA);
        write_memory(514, 0x3);
        write_memory(516, 0x7);
        write_memory(517, 0x01);
        write_memory(518, 0xD0);
        write_memory(519, 0x11);
        write_memory(520, 0x40);
        write_memory(521, 63);
        write_memory(522, 0x12);
        write_memory(523, 0x08);
    }
    let _ = run(args.len() == 2).join();
}

fn run(with_load_memory: bool) -> JoinHandle<()> {
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
        //write_register(0, 16);
        //write_memory(512, 0xa0);
        //write_memory(513, 0x5A);
        //write_memory(514, 0xd0);
        //write_memory(515, 0x05);
    }
    unsafe {
        if WINDOW.is_none() {
            let _screen_thread = thread::spawn(|| {
                screen::main();

                loop {
                    thread::sleep(PROGRAM_TIME_60HZ);
                    screen::update_screen();
                }
            });
        }
    }
    let program_thread = thread::spawn(|| {
        let mut pointer = 512u16;
        loop {
            unsafe {
                if pointer as usize >= MEMORY.len() {
                    RUNNING = !RUNNING;
                }
                thread::sleep(PROGRAM_TIME_7000_INSTR_PM);
                //Main Program Loop
                if !RUNNING {
                    break;
                }
                let pointer_nibbles = AsNibbles([MEMORY[pointer as usize]]);
                let op_id = pointer_nibbles.get(0).expect("Illegal Pointer").into();
                let nibble = AsNibbles([read_nn(pointer)]);
                println!(
                    "PC: {} Operation: {} {}:{}:{}",
                    pointer,
                    op_id,
                    read_x(pointer),
                    read_y(pointer),
                    read_n(pointer)
                );
                let opt_intruction = PROGRAM_INSTRUCTIONS.get(&op_id);
                if let Some(instruction) = opt_intruction {
                    pointer = instruction(pointer);
                }
                pointer += 2;
            }
        }
    });
    let _sound_thread = thread::spawn(|| loop {
        thread::sleep(PROGRAM_TIME_60HZ);
        unsafe {
            TIMER = TIMER.saturating_sub(1);
            if !RUNNING {
                break;
            }
        }
    });
    program_thread
}

fn f_instructions(pointer: u16) -> u16 {
    unsafe {
        let op_type = read_nn(pointer);
        match op_type {
            0x1E => {
                REGISTER_I += read_register(read_x(pointer).into()) as u16;
            }
            0x0A => {
                let key = input::to_key(AsNibbles([read_x(pointer).into()]).get(1).expect(""));
                await_key(key);
            }
            0x29 => {
                let key: u16 = AsNibbles([read_x(pointer).into()]).get(1).expect("").into();
                REGISTER_I = 0x50 + (5 * key);
            }
            0x55 => {
                let x: usize = read_x(pointer).into();
                let mut i = 0;
                while i <= x {
                    write_memory(REGISTER_I as usize + i, read_register(i));
                    i += 1;
                }
            }
            0x65 => {
                let x: usize = read_x(pointer).into();
                let mut i = 0;
                while i <= x {
                    write_register(i, read_memory(REGISTER_I as usize + i));
                    i += 1;
                }
            }
            0x33 => {
                let vx = read_register(read_x(pointer).into());
                let hundreds = vx / 100;
                let tens = vx / 10 - hundreds * 10;
                let ones = vx - hundreds * 100 - tens * 10;
                write_memory(REGISTER_I as usize, hundreds);
                write_memory(REGISTER_I as usize + 1, tens);
                write_memory(REGISTER_I as usize + 2, ones);
            }
            _ => {}
        }
        pointer
    }
}

fn key_input(pointer: u16) -> u16 {
    unsafe {
        let op_type = read_nn(pointer);
        match op_type {
            0x9E => {
                let reg = read_register(read_x(pointer).into());
                let key = input::to_key(AsNibbles([reg]).get(1).expect(""));
                if screen::WINDOW
                    .as_ref()
                    .expect("")
                    .is_key_pressed(key, KeyRepeat::No)
                {
                    return pointer + 2;
                }
                pointer
            }
            0xA1 => {
                let reg = read_register(read_x(pointer).into());
                let key = input::to_key(AsNibbles([reg]).get(1).expect(""));
                if !screen::WINDOW
                    .as_ref()
                    .expect("")
                    .is_key_pressed(key, KeyRepeat::No)
                {
                    return pointer + 2;
                }
                pointer
            }
            _ => panic!("Illegal key operation"),
        }
    }
}

fn random(pointer: u16) -> u16 {
    let upper: u8 = read_nn(pointer);
    let random: u8 = rand::random();
    write_register(read_x(pointer).into(), (random & upper) as u8);
    pointer
}

fn jump_with_offset(pointer: u16) -> u16 {
    let jum_offstet = read_nnn(pointer);
    (jum_offstet + read_register(0) as u16) - 2
}

fn subroutine(pointer: u16) -> u16 {
    unsafe {
        PPROGRAM_STACK.push_front(pointer);
        read_nnn(pointer) - 2
    }
}

fn draw(pointer: u16) -> u16 {
    unsafe {
        let vx = read_register(read_x(pointer).into());
        let vy = read_register(read_y(pointer).into());
        write_register(15, 0);
        let ox = vx & 63;
        let mut y = vy & 31;
        let n: usize = read_n(pointer).into();
        let mut i = 0usize;
        while i < n && y < 32 {
            let regp: usize = REGISTER_I.into();
            let sprite_data = read_memory(regp + i);
            let mut j = 0;
            let mut x = ox;
            while j < 8 && x < 64 {
                let on = sprite_data >> (7 - j) & 1;
                if screen::set_pixel(x, y, on == 1) && on == 0 {
                    write_register(15, 1);
                }
                x += 1;
                j += 1;
            }
            y += 1;
            i += 1;
        }
    }
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
            let opt_type = read_nn(pointer);
            match opt_type {
                0xE0 => {
                    screen::SCREEN = [0u32; 2048];
                    screen::update_screen();
                }
                0xEE => {
                    return PPROGRAM_STACK.pop_front().expect("Illegal subroutine call");
                }
                _ => {
                    panic!("Illegal operation");
                }
            }
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

fn whipe_memory() {
    unsafe {
        MEMORY = [0; 4096];
    }
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
            let _ = run(false).join();
            assert_eq!(REGISTERS[0], 1);
            assert_eq!(REGISTERS[15], 1);
            REGISTERS[0] = 2;
            REGISTERS[1] = 1;
            MEMORY[512] = 0b1000_0000_u8;
            MEMORY[513] = 0b0001_0111_u8;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run(false).join();
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
            let _ = run(false).join();
            assert_eq!(REGISTERS[0], 2);
            assert_eq!(REGISTERS[15], 0);

            REGISTERS[1] = 0xFF;
            MEMORY[512] = 0b1000_0000_u8;
            MEMORY[513] = 0b0001_1110_u8;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run(false).join();
            assert_eq!(REGISTERS[0], 0b1111_1110_u8);
            assert_eq!(REGISTERS[15], 1);

            REGISTERS[1] = 1;
            MEMORY[512] = 0x80;
            MEMORY[513] = 0x16;
            MEMORY[514] = 0b0001_1111_u8;
            MEMORY[515] = 0b1110_0000_u8;
            let _ = run(false).join();
            assert_eq!(REGISTERS[0], 0);
            assert_eq!(REGISTERS[15], 1);
        }
        subroutine_test();
    }

    fn subroutine_test() {
        write_memory(512, 0x22); // ROUT 600
        write_memory(513, 0x58);
        write_memory(514, 0x60); // SET 0 2
        write_memory(515, 0x02);
        write_memory(516, 0x1F); // JUMP 4094
        write_memory(517, 0xFC);
        write_memory(600, 0x61); // SET 1 1
        write_memory(601, 0x01);
        write_memory(602, 0x00); // LASTROUT
        write_memory(603, 0xEE);
        let _ = run(false).join();
        assert_eq!(read_register(0), 2);
        assert_eq!(read_register(1), 1);
        shr();
    }

    fn shr() {
        println!("SHR test");
        whipe_memory();
        write_register(0, 2);
        write_memory(512, 0x80);
        write_memory(513, 0x06);
        write_memory(516, 0x1F); // JUMP 4094
        write_memory(517, 0xFC);

        let _ = run(false).join();
        assert_eq!(read_register(0), 1);
        assert_eq!(read_register(15), 0);

        write_register(0, 3);
        write_memory(512, 0x80);
        write_memory(513, 0x06);
        let _ = run(false).join();
        assert_eq!(read_register(0), 1);
        assert_eq!(read_register(15), 1);

        write_register(0, 1);
        write_memory(512, 0x80);
        write_memory(513, 0x06);
        let _ = run(false).join();
        assert_eq!(read_register(0), 0);
        assert_eq!(read_register(15), 1);
        shl();
    }

    fn shl() {
        println!("SHL test");
        whipe_memory();
        write_register(0, 0b0100_0000_u8);
        write_memory(512, 0x80);
        write_memory(513, 0x0E);
        write_memory(516, 0x1F); // JUMP 4094
        write_memory(517, 0xFC);
        let _ = run(false).join();
        assert_eq!(read_register(0), 0b1000_0000_u8);
        assert_eq!(read_register(15), 0);

        write_register(0, 0b1100_0000_u8);
        write_memory(512, 0x80);
        write_memory(513, 0x0E);
        let _ = run(false).join();
        assert_eq!(read_register(0), 0b1000_0000_u8);
        assert_eq!(read_register(15), 1);

        write_register(0, 0b1000_0000_u8);
        write_memory(512, 0x80);
        write_memory(513, 0x0E);
        let _ = run(false).join();
        assert_eq!(read_register(0), 0);
        assert_eq!(read_register(15), 1);
        math_or();
    }

    fn math_or() {
        write_register(0, 1);
        write_register(1, 2);
        write_memory(512, 0x80);
        write_memory(513, 0x11);
        let _ = run(false).join();
        assert_eq!(read_register(0), 3);
        math_and();
    }

    fn math_and() {
        write_register(0, 3);
        write_register(1, 2);
        write_memory(512, 0x80);
        write_memory(513, 0x12);
        let _ = run(false).join();
        assert_eq!(read_register(0), 2);
        math_xor();
    }

    fn math_xor() {
        write_register(0, 3);
        write_register(1, 2);
        write_memory(512, 0x80);
        write_memory(513, 0x13);
        let _ = run(false).join();
        assert_eq!(read_register(0), 1);
    }
}
