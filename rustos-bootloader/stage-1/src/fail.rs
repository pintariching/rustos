use core::arch::asm;

use crate::print::print_char;

#[panic_handler]
pub fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt()
    }
}

#[cold]
#[inline(never)]
#[no_mangle]
pub extern "C" fn fail(code: u8) -> ! {
    print_char(b'!');
    print_char(code);
    print_char(b'!');
    loop {
        hlt()
    }
}

fn hlt() {
    unsafe {
        asm!("hlt");
    }
}
