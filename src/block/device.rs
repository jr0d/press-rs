extern crate serde;

// lol.. figure out how to do this correctly
use super::super::gpt;
use super::super::mbr;
use super::super::udev;
use super::super::sysfs;

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
    #[serde(skip_serializing)]
    udev_info: udev::UdevBlockDeviceInfo,
    geometry: sysfs::BlockDeviceGeometry,
    partition_table: Option<PartitionTable>,
    gpt_partition_array: Option<gpt::GPTPartitionEntryArray>
}


// impl BlockDevice {
//     pub fn assemble(device: &str) -> BlockDeviceResult {

//     }
// }