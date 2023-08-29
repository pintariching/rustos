#![no_std]

use bootloader_api::BootInfo;

use crate::common::init_logger;

pub mod common;

pub fn init(boot_info: &'static mut BootInfo, message: &str) {
    if let Some(framebuffer) = boot_info.framebuffer.as_mut() {
        let info = framebuffer.info().clone();
        let buffer = framebuffer.buffer_mut();

        init_logger(buffer, info);
    }

    log::info!("{message}");
}
