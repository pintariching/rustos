#![no_std]
#![no_main]

mod disk_address_packet;
mod fail;
mod print;

use core::arch::{asm, global_asm};
use core::ptr;

use print::print_char;

global_asm!(include_str!("boot.s"), options(raw));

#[no_mangle]
pub extern "C" fn start() {
    print_char(b'A');

    zero_registers();
    print_char(b'B');

    let disk_number = read_disk_number();
    print_char(b'C');

    setup_stack();
    print_char(b'D');

    clear_direction_flag();
    print_char(b'E');

    enable_a20_line();
    print_char(b'F');

    check_int13_extensions_available(disk_number);
}

fn zero_registers() {
    unsafe {
        asm! {
            "xor ax, ax", // Set AX register to 0
            "mov ds, ax", // Set Data Segment register to 0
            "mov es, ax", // Set Extra Segment register to 0
            "mov ss, ax" // Set Stack Segment register to 0
        }
    }
}

fn read_disk_number() -> u8 {
    let mut disk_number: u8;

    unsafe {
        asm! {
            "", // Empty instruction
            out("dl") disk_number
        }
    }

    disk_number
}

fn setup_stack() {
    unsafe {
        asm! {
            "mov ax, 0x8000",
            "mov ss, ax", // Set Stack Segment to 0x8000
            "add ax, 512",
            "mov sp, ax" // Set Stack Pointer to 0x8000 + 512
        }
    }
}

fn clear_direction_flag() {
    unsafe {
        asm! {
            "cld"
        }
    }
}

fn enable_a20_line() {
    unsafe {
        asm! {
            "in al, 0x92", // Read from IO port 0x92 into register al
            "test al, 2", // If al is 2, set flag zf to 1
            "jnz 2f", // If zf == 0, jump forward to label 2
            "or al, 2", // Set al to 2
            "and al, 0xfe",
            "out 0x92, al", // Output al to port 0x92
            "2:",
        }
    }
}

fn check_int13_extensions_available(disk_number: u8) {
    let mut supported_interfaces: u16;
    unsafe {
        asm! {
            "push 'Y'", // Push error code
            "mov ah, 0x41", // INT 13h AH=41h: Check Extensions Present
            "mov bx, 0x55aa",
            "int 0x13",
            "jc fail",
            "pop ax",
            in("dl") disk_number, // Set drive number
            out("cx") supported_interfaces

        }
    }

    // Device Access using the packet structure
    if (supported_interfaces & 1) == 1 {
        print_char(b'G');
    }

    // Drive Locking and Ejecting
    if (supported_interfaces & 2) == 2 {
        print_char(b'H');
    }

    // Enhanced Disk Drive Support (EDD)
    if (supported_interfaces & 4) == 4 {
        print_char(b'I');
    }
}

// fn read_disk(disk_number: u16) {
//     //let mut read_count = 0u8;
//     unsafe {
//         asm! {
//             "push 'z'",
//             "mov ah, 2", // Read with CHS
//             "mov al, 1", // Read 1 sectors
//             "mov ch, 0", // Cylinder number
//             "mov cl, 2", // Sector number
//             "mov dh, 0", // Head number

//             "mov bx, 0",
//             "mov es, bx",
//             "mov bx, 0x7e00", // Load address

//             "int 0x13", // Read disk
//             "jc fail",
//             "add esp, 4", // Pop 'z' from the stack without storing it

//             in("dx") disk_number,
//             //out("al") read_count,
//         }
//     }

//     // print_char(char::from_digit(read_count as u32, 10).unwrap() as u8);
// }

// fn print_at_address(addr: *const u16) {
//     let val = unsafe { ptr::read(addr) };
//     unsafe {
//         asm! {
//             "mov ah, 0x0e",
//             "int 0x10",
//             in("ax") val
//         }
//     }
// }
