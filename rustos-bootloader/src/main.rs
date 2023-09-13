#![no_std]
#![no_main]

mod fail;

use core::arch::global_asm;

global_asm!(include_str!("boot.s"), options(raw));

#[no_mangle]
pub extern "C" fn boot(_disk_number: u16) {}
