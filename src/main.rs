extern crate rustos;

use rustos::{launch_qemu, Kernel, QemuConfig};

fn main() {
    let kernel_path = env!("KERNEL_PATH");
    println!("Kernel elf at: {kernel_path}");

    // read env variables that were set in build script
    let uefi_path = env!("UEFI_PATH");

    let kernel = Kernel {
        path: uefi_path.to_string(),
        uefi: true,
    };

    let qemu = QemuConfig::default();

    launch_qemu(kernel, qemu).unwrap();
}
