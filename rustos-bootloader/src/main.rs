#![no_std]
#![no_main]

mod fail;

use core::arch::{asm, global_asm};
use core::ptr;

use fail::print_char;

global_asm!(include_str!("boot.s"), options(raw));

#[no_mangle]
pub extern "C" fn boot(disk_number: u16) {
    print_char(b'A');
    print_char(disk_number as u8);
    read_disk(0x80);
    print_char(b'B');

    let mut addr = 0x7d00;
    loop {
        print_at_address(addr);
        addr += 1;

        if addr >= 0x8000 {
            break;
        }
    }

    print_char(b'C');
}

fn read_disk(disk_number: u16) {
    //let mut read_count = 0u8;
    unsafe {
        asm! {
            "push 'z'",
            "mov ah, 2", // Read sector #2 (starts from 1)
            "mov al, 1", // Read 1 sectors
            "mov ch, 0", // Cylinder number
            "mov cl, 2", // Sector number
            "mov dh, 0", // Head number

            "mov bx, 0",
            "mov es, bx",
            "mov bx, 0x7d00", // Load address

            "int 0x13", // Read disk
            "jc fail",
            "add esp, 4", // Pop 'z' from the stack without storing it

            in("dx") disk_number,
            //out("al") read_count,
        }
    }

    // print_char(char::from_digit(read_count as u32, 10).unwrap() as u8);
}

#[no_mangle]
pub extern "C" fn print_at_address(addr: u16) {
    let val = unsafe { ptr::read(addr as *const u16) } | 0x0e00;
    unsafe {
        asm! {
            "int 0x10",
            in("ax") val
        }
    }
}
