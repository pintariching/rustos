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
