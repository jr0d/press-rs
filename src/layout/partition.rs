use crate::block::BlockDevice;
use super::fs::FileSystem;

// Supported partition tables
#[derive(PartialEq)]
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

impl PartitionTable {
    pub fn new(format: TableFormat) -> Self {
        Self {
            table_type: format,
            .. Default::default()
        }
    }
}

impl Default for PartitionTable {
    fn default() -> Self {
        Self {
            table_type: TableFormat::GPT,
            target: None,
            partition_start: 2u64.pow(20),
            alignment: 2u64.pow(20),
            partitions: Vec::new()
        }
    }
}


pub struct Partition {
    // The name of the partition
    pub name: String,
    pub file_system: Option<FileSystem>,
    pub size: u64
}

#[cfg(test)]
mod tests {
    use crate::layout::partition::*;
    use crate::sysfs::BlockDeviceGeometry;
    use crate::block::BlockDevice;

    #[test]
    fn test_partial() {
        let table = PartitionTable {
            partition_start: 512,
            .. Default::default()
        };

        assert!(table.table_type == TableFormat::GPT);
        assert!(table.partition_start == 512);
        assert!(table.target.is_none());
    }
    #[test]
    fn test_block_device() {

        let geom = BlockDeviceGeometry {
            logical_block_size: 512,
            logical_blocks: 1953525168,
            size: 1000204886016
        };

        let bd = BlockDevice {
            geometry: geom,
            partition_table: None,
            gpt_partition_array: None
        };

        let mut pt = PartitionTable::default();

        pt.target = Some(bd);

        assert_eq!(pt.target.unwrap().geometry.logical_block_size, 512);
    }
}