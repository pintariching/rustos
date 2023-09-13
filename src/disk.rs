use std::fs::{self, OpenOptions};
use std::io::{self, Seek, Write};
use std::path::Path;
use std::process::Command;

const BLOCK_SIZE: u64 = 512;
const MB: u64 = 1024 * 1024;

pub fn create_disk_image(bootloader_elf_path: &Path, output_bin_path: &Path) -> anyhow::Result<()> {
    let llvm_tools = llvm_tools::LlvmTools::new().unwrap();
    let objcopy = llvm_tools.tool(&llvm_tools::exe("llvm-objcopy")).unwrap();

    let mut cmd = Command::new(objcopy);
    cmd.arg("-I").arg("elf64-x86-64");
    cmd.arg("-O").arg("binary");
    cmd.arg("--binary-architecture=i386:x86-64");
    cmd.arg(bootloader_elf_path);
    cmd.arg(output_bin_path);

    let output = cmd.output()?;

    if !output.status.success() {
        return Err(anyhow::anyhow!("objcopy failed"));
    }

    let mut disk_image = OpenOptions::new().write(true).open(&output_bin_path)?;

    let file_size = disk_image.metadata()?.len();

    assert_eq!(file_size, BLOCK_SIZE);

    let fat_size = 0; // = kernel size
    let fat_size_padded_and_rounded = ((fat_size + 1024 * 64 - 1) / MB + 1) * MB;
    let _fat_file_path = {
        let fat_path = output_bin_path.with_extension("fat");
        let fat_file = fs::OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&fat_path)?;

        fat_file.set_len(fat_size_padded_and_rounded)?;

        // create new FAT partition
        let format_options = fatfs::FormatVolumeOptions::new().volume_label(*b"BOOT       ");
        fatfs::format_volume(&fat_file, format_options)?;

        // copy kernel to FAT partition

        fat_path
    };

    disk_image.seek(io::SeekFrom::Start(446))?;
    disk_image.write_all(&[0x80, 0, 0, 0, 0x04, 0, 0, 0])?;

    let start_sector = 1u32.to_le_bytes();
    let size_sectors = ((fat_size_padded_and_rounded / 512) as u32).to_le_bytes();
    disk_image.write_all(&start_sector)?;
    disk_image.write_all(&size_sectors)?;

    disk_image.seek(io::SeekFrom::Start(512))?;

    pad_to_nearest_block_size(output_bin_path)?;
    Ok(())
}

fn pad_to_nearest_block_size(output_bin_path: &Path) -> anyhow::Result<()> {
    let file = OpenOptions::new().write(true).open(output_bin_path)?;

    let file_size = file.metadata()?.len();

    let remainder = file_size % BLOCK_SIZE;
    let padding = if remainder > 0 {
        BLOCK_SIZE - remainder
    } else {
        0
    };

    file.set_len(file_size + padding)?;

    Ok(())
}
