extern crate serde;

use serde::Deserialize;

use crate::size::Size;
use super::fs::FileSystem;

// Supported partition tables
#[derive(Debug, PartialEq, Deserialize)]
pub enum TableFormat {
    GPT,
    MBR
}

impl Default for TableFormat {
    fn default() -> Self {
        TableFormat::GPT
    }
}

/// Logical representation of a partition table
#[derive(Debug, Deserialize)]
pub struct PartitionTable {
    /// Which partition table
    #[serde(default)]
    pub table_type: TableFormat,

    /// A target block device structure.
    /// This field is populated once a physical device is selected
    pub target: Option<String>,
    
    /// The partition table offset
    #[serde(default = "default_pt_size")]
    pub partition_start: Size, // Default 2048s
    
    /// Partition alignment value
    #[serde(default = "default_pt_size")]
    pub alignment: Size, // Default 2048s or LBA size??
    
    /// A list of created partition objects
    #[serde(default)]
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
            partition_start: default_pt_size(),
            alignment: default_pt_size(),
            partitions: Vec::new()
        }
    }
}
fn default_pt_size() -> Size {
    "1MiB".parse::<Size>().unwrap()
}

#[derive(Debug, Deserialize)]
pub struct Partition {
    // The name of the partition
    pub name: String,
    pub file_system: Option<FileSystem>,
    pub size: u64
}

#[cfg(test)]
mod tests {
    use crate::layout::partition::*;

    #[test]
    fn test_partial() {
        let table = PartitionTable {
            partition_start: "1MiB".parse().unwrap(),
            .. Default::default()
        };

        assert!(table.table_type == TableFormat::GPT);
        assert!(table.partition_start.bytes() == 
                "1MiB".parse::<Size>().unwrap().bytes());
        assert!(table.target.is_none());
    }
}