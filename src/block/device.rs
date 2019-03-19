extern crate serde;

use crate::gpt::*;
use crate::mbr::*;
use crate::udev;
use crate::sysfs;

pub type BlockDeviceResult = Result<BlockDevice, Box<std::error::Error>>;

#[derive(Debug, serde::Serialize)]
pub enum PartitionTable {
    GPT(GPTHeader),
    MBR(MBR)
}

#[derive(Debug, serde::Serialize)]
pub struct BlockDevice {
    pub geometry: sysfs::BlockDeviceGeometry,
    pub partition_table: Option<PartitionTable>,
    pub gpt_partition_array: Option<GPTPartitionEntryArray>
}


impl BlockDevice {
    pub fn assemble(device: &str) -> BlockDeviceResult {
        let udev_info = udev::get_block_devices_with_property(
            "DEVNAME", device)?;
        // Add find one method to udev::
        let udev_info = udev_info.first().unwrap();
        let sysfs_geom = sysfs::BlockDeviceGeometry::from_device(
            &sysfs::kernel_path_to_sys(
                // Add accessor for properties
                udev_info.properties().get("DEVPATH").unwrap()
            ))?;
        Ok(BlockDevice {
            geometry: sysfs_geom,
            partition_table: None,
            gpt_partition_array: None
        })
    }
}