#![no_std]
#![no_main]

mod fail;

use core::arch::{asm, global_asm};

global_asm!(include_str!("boot.s"), options(raw));

#[no_mangle]
pub extern "C" fn boot(disk_number: u16) {
    read_disk(disk_number);
}

fn read_disk(disk_number: u16) {
    unsafe {
        asm! {
            "mov ah, 2", // Read sector #2 (starts from 1)
            "mov al, 1", // Read 1 sector
            "mov ch, 0", // Cylinder number
            "mov cl, 2", // Sector number
            "mov dh, 0", // Head number
            "mov bx, 0x7e00", // Load address
            "mov dx, {0:x}",
            in(reg) disk_number,
        }

        asm! {
            "int 0x13" // Read disk
        }
    }
}
