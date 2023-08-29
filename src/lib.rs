use std::process::ExitStatus;

#[derive(Debug, Default)]
pub struct Kernel {
    pub path: String,
    pub uefi: bool,
}

#[derive(Debug)]
pub struct QemuConfig {
    pub memory: String,
    pub devices: String,
}

impl Default for QemuConfig {
    fn default() -> Self {
        Self {
            memory: "4G".into(),
            devices: "".into(),
        }
    }
}

pub fn launch_qemu(kernel: Kernel, qemu: QemuConfig) -> std::io::Result<ExitStatus> {
    let uefi_path = kernel.path;
    let mut cmd = std::process::Command::new("qemu-system-x86_64");

    if kernel.uefi {
        cmd.arg("-bios").arg(ovmf_prebuilt::ovmf_pure_efi());
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    } else {
        cmd.arg("-drive")
            .arg(format!("format=raw,file={uefi_path}"));
    }

    cmd.arg("-enable-kvm");
    cmd.arg("-m").arg(qemu.memory);

    if !qemu.devices.is_empty() {
        cmd.arg("-device").arg(qemu.devices);
    }

    let mut child = cmd.spawn().unwrap();
    child.wait()
}
