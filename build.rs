use std::path::{Path, PathBuf};
use std::process::Command;

const BOOTLOADER_VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let out_dir = PathBuf::from(std::env::var("OUT_DIR").unwrap());

    bios_main(&out_dir);

    let bios_path = out_dir.join("bin").join("bootloader-stage-1.bin");
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}

fn bios_main(out_dir: &Path) {
    let boot_sector_path = build_bios_boot_sector(&out_dir);

    println!(
        "cargo:rustc-env=BIOS_BOOT_SECTOR_PATH={}",
        boot_sector_path.display()
    );
}

fn build_bios_boot_sector(out_dir: &Path) -> PathBuf {
    let cargo = std::env::var("CARGO").unwrap_or_else(|_| "cargo".into());

    let mut cmd = Command::new(cargo);
    cmd.arg("install").arg("bootloader-stage-1");

    let local_path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("rustos-bootloader")
        .join("bios")
        .join("stage-1");

    if local_path.exists() {
        cmd.arg("--path").arg(&local_path);
        println!("cargo:rerun-if-changed={}", local_path.display());
    } else {
        cmd.arg("--version").arg(BOOTLOADER_VERSION);
    }

    cmd.arg("--locked");
    cmd.arg("--target").arg("i386-code16-boot-sector.json");
    cmd.arg("--profile").arg("stage-1");
    cmd.arg("-Zbuild-std=core")
        .arg("-Zbuild-std-features=compiler-builtins-mem");
    cmd.arg("--root").arg(out_dir);
    cmd.env_remove("RUSTFLAGS");
    cmd.env_remove("CARGO_ENCODED_RUSTFLAGS");
    cmd.env_remove("RUSTC_WORKSPACE_WRAPPER");

    let status = cmd
        .status()
        .expect("failed to run cargo install for bios bootsector");
    let elf_path = if status.success() {
        let path = out_dir.join("bin").join("bootloader-stage-1");
        assert!(
            path.exists(),
            "bios boot sector executable does not exist after building"
        );
        path
    } else {
        panic!("failed to build bios boot sector");
    };

    convert_elf_to_bin(elf_path)
}

fn convert_elf_to_bin(elf_path: PathBuf) -> PathBuf {
    let flat_binary_path = elf_path.with_extension("bin");

    let llvm_tools = llvm_tools::LlvmTools::new().expect("failed to get llvm tools");
    let objcopy = llvm_tools
        .tool(&llvm_tools::exe("llvm-objcopy"))
        .expect("LlvmObjcopyNotFound");

    // convert first stage to binary
    let mut cmd = Command::new(objcopy);
    cmd.arg("-I").arg("elf64-x86-64");
    cmd.arg("-O").arg("binary");
    cmd.arg("--binary-architecture=i386:x86-64");
    cmd.arg(&elf_path);
    cmd.arg(&flat_binary_path);
    let output = cmd
        .output()
        .expect("failed to execute llvm-objcopy command");
    if !output.status.success() {
        panic!(
            "objcopy failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
    }
    flat_binary_path
}
