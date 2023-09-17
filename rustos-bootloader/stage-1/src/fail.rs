use core::arch::asm;

pub fn print_char(char: u8) {
    let ax = char as u16 | 0x0e00;
    unsafe {
        asm! {
            "int 0x10",
            in("ax") ax,
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
