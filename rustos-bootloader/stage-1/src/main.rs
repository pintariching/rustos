#![no_std]
#![no_main]

mod disk_address_packet;
mod fail;
mod print;

use core::arch::{asm, global_asm};
use core::ptr;

use fail::print_char;

global_asm!(include_str!("boot.s"), options(raw));

#[no_mangle]
pub extern "C" fn start(disk_number: u16) {
    // print_char(b'A');
    // print_char(disk_number as u8);
    // //read_disk(disk_number);
    // print_char(b'B');
}

fn zero_registers() {
    unsafe {
        asm! {
            "xor ax, ax",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax"
        }
    }
}

fn read_disk(disk_number: u16) {
    //let mut read_count = 0u8;
    unsafe {
        asm! {
            "push 'z'",
            "mov ah, 2", // Read with CHS
            "mov al, 1", // Read 1 sectors
            "mov ch, 0", // Cylinder number
            "mov cl, 2", // Sector number
            "mov dh, 0", // Head number

            "mov bx, 0",
            "mov es, bx",
            "mov bx, 0x7e00", // Load address

            "int 0x13", // Read disk
            "jc fail",
            "add esp, 4", // Pop 'z' from the stack without storing it

            in("dx") disk_number,
            //out("al") read_count,
        }
    }

    // print_char(char::from_digit(read_count as u32, 10).unwrap() as u8);
}

fn print_at_address(addr: *const u16) {
    let val = unsafe { ptr::read(addr) };
    unsafe {
        asm! {
            "mov ah, 0x0e",
            "int 0x10",
            in("ax") val
        }
    }
}
