use core::arch::asm;

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        unsafe {
            asm!("hlt");
        }
    }
}
