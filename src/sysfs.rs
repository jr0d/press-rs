extern crate serde;

use std::io::{Read};
use std::fs::File;
use std::path::{Path, PathBuf};

// use std::error::Error;
// use std::fmt;

use serde::Serialize;

// Methods for interactive with the Linux System Filesystem
pub static LINUX_SYSFS_BLOCK_DEVICE_PATH: &'static str = "/sys/block";

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BlockDeviceGeometry {
    logical_block_size: u64,
    logical_blocks: u64,
    size: u64
}

// Generic attempts
// pub fn read_number<R: Read, T: std::str::FromStr>(reader: R) -> Result<T, Box<std::error::Error>> {
//     let mut buf = String::new();
//     R.read_to_string(&mut buf)?;
//     Ok(buf.trim().parse()?)
// }

// This doesn't work: 
//  the associated type `<T as std::str::FromStr>::Err` may not live long enough
// pub fn read_number<'a, T>(path: &'a PathBuf) -> Result<T, Box<std::error::Error>> 
//         where T: std::str::FromStr,
//               <T as std::str::FromStr>::Err: std::error::Error {
//     let mut buf = String::new();
//     let mut fp: File = File::open(path.to_str().unwrap())?;
//     fp.read_to_string(&mut buf)?;
//     // match buf.trim().parse::<T>() {
//     //     Ok(t) => Ok(t),
//     //     Err(_) => panic!("Error parsing file")
//     // }

//     Ok(buf.trim().parse()?)
// }

pub fn read_u64(path: &PathBuf) -> Result<u64, Box<std::error::Error>> {
    let mut buf = String::new();
    let mut fp: File = File::open(path.to_str().unwrap())?;
    fp.read_to_string(&mut buf)?;
    Ok(buf.trim().parse()?)
}

impl BlockDeviceGeometry {
    pub fn from_device(kernel_device_path: &str) -> Result<BlockDeviceGeometry,
        Box<std::error::Error>> {
        let size_path = Path::new(kernel_device_path).join("size");
        let lba_size_path = Path::new(kernel_device_path)
            .join("queue/logical_block_size");
        let logical_blocks = read_u64(&size_path)?;
        let logical_block_size = read_u64(&lba_size_path)?;
        Ok( BlockDeviceGeometry {
            logical_block_size: logical_block_size,
            logical_blocks: logical_blocks,
            size: logical_blocks * logical_block_size
        })
    }
}

pub fn get_block_devices() -> Vec<std::fs::DirEntry> {
    let mut v: Vec<std::fs::DirEntry> = std::fs::read_dir(
        LINUX_SYSFS_BLOCK_DEVICE_PATH)
            .unwrap()
            .filter_map(|e| e.ok())
            .collect();
    v.sort_by_key(|dir| dir.path());
    v
}
