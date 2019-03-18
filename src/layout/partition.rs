use crate::block::BlockDevice;
use super::fs::FileSystem;

// Supported partition tables
pub enum TableFormat {
    GPT,
    MBR
}

/// Logical representation of a partition table
pub struct PartitionTable {
    /// Which partition table
    pub table_type: TableFormat,
    /// A target block device structure.
    /// This field is populated once a physical device is selected
    pub target: Option<BlockDevice>,
    /// The partition table offset
    pub partition_start: u64, // Default 2048s
    /// Partition alignment value
    pub alignment: u64, // Default 2048s or LBA size??
    /// A list of created partition objects
    pub partitions: Vec<Partition>
}


pub struct Partition {
    // The name of the partition
    pub name: String,
    pub file_system: Option<FileSystem>,
    pub size: u64
}