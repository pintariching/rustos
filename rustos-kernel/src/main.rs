#![no_std]
#![no_main]

use bootloader_api::BootInfo;
use core::panic::PanicInfo;

use rustos_kernel::init;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info, "Init complete");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
