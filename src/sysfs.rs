// Methods for interactive with the Linux System Filesystem
pub static LINUX_SYSFS_BLOCK_DEVICE_PATH: &'static str = "/sys/block";

pub fn get_block_devices() -> Vec<std::fs::DirEntry> {
    let mut v: Vec<std::fs::DirEntry> = std::fs::read_dir(
        LINUX_SYSFS_BLOCK_DEVICE_PATH)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
    v.sort_by_key(|dir| dir.path());
    v
}
