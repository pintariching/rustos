#![no_std]
#![no_main]

use core::panic::PanicInfo;

use bootloader_api::BootInfo;
use kernel::common::init_logger;
use log::info;

bootloader_api::entry_point!(kernel_main);

fn kernel_main(boot_info: &'static mut BootInfo) -> ! {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info().clone();
        let buffer = framebuffer.buffer_mut();

        init_logger(buffer, info);
    }

    info!("Test log");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
