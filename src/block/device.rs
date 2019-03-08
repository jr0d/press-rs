extern crate serde;

use crate::gpt;
use crate::mbr;
use crate::udev;
use crate::sysfs;
use crate::{is_gpt, has_mbr};

pub type BlockDeviceResult = Result<BlockDevice, Box<std::error::Error>>;

#[derive(Debug, serde::Serialize)]
enum PartitionTable {
    GPT(gpt::GPTHeader),
    MBR(mbr::MBR)
}

#[derive(Debug, serde::Serialize)]
enum Partitions {
    GPT(gpt::GPTPartitionEntryArray),
    MBR(mbr::MBR)
}


#[derive(Debug, serde::Serialize)]
pub struct BlockDevice {
    geometry: sysfs::BlockDeviceGeometry,
    partition_table: Option<PartitionTable>,
    gpt_partition_array: Option<gpt::GPTPartitionEntryArray>
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