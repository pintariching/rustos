use core::arch::asm;

#[no_mangle]
pub extern "C" fn print_char(char: u8) {
    unsafe {
        asm! {
            // "push bx",
            // "mov bx, 0",
            "mov ah, 0x0e",
            "int 0x10",
            // "pop bx",
            in("al") char,
        }
    }
}

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
    loop {
        hlt()
    }
}

fn hlt() {
    unsafe {
        asm!("hlt");
    }
}
