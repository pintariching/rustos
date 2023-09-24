#![no_std]
#![no_main]

mod disk_address_packet;
mod fail;
mod mbr;
mod print;
mod setup;

use core::arch::global_asm;
use disk_address_packet::DiskAddressPacket;
use mbr::PartitionTableEntry;
use print::print_char;
use setup::setup;

global_asm!(include_str!("boot-start.s"), options(raw));

extern "C" {
    static _partition_table: u8;
    static _start_stage_2: u8;
}

#[no_mangle]
pub extern "C" fn start() {
    let disk_number = setup();

    let second_stage_start = {
        let ptr: *const u8 = { unsafe { &_start_stage_2 } };
        ptr as *const ()
    };

    let partition_table_ptr = unsafe { &_partition_table };
    let partition_table = PartitionTableEntry::from_raw_pointer(partition_table_ptr);

    let mut start_lba = 1;
    let mut number_of_sectors = 1;
    let mut target_addr = second_stage_start as u32;

    loop {
        let sectors = u32::min(number_of_sectors, 32) as u16;
        let dap = DiskAddressPacket::from_lba(
            start_lba,
            sectors,
            (target_addr & 0b11110000) as u16,
            (target_addr >> 4).try_into().unwrap(),
        );

        dap.load(disk_number);

        start_lba += sectors as u64;
        number_of_sectors -= sectors as u32;
        target_addr += sectors as u32 * 512;

        if number_of_sectors == 0 {
            break;
        }
    }

    stage_2(disk_number, partition_table_ptr)
}

#[no_mangle]
#[link_section = ".stage2"]
pub extern "C" fn stage_2(_disk_number: u8, _partition_table_ptr: *const u8) {
    print_char(b'2');
    print_char(b'3');
    print_char(b'4');
    print_char(b'5');
    print_char(b'2');
    print_char(b'3');
}
