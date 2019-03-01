extern crate byteorder;
extern crate serde;
extern crate uuid;

use byteorder::{LittleEndian, ByteOrder};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

use super::uuid_from_le_bytes;

#[derive(Debug, Default, Serialize)]
pub struct GPTHeader {
    pub signature: u64,
    pub revision: u32,
    pub header_size: u32,
    pub header_crc32: u32,
    pub reserved: u32,
    pub current_lba: u64,
    pub backup_lba: u64,
    pub first_usable_lba: u64,
    pub last_uasable_lba: u64,
    pub guid: Uuid,
    pub partition_entry_lba: u64,
    pub number_of_partions: u32,
    pub size_of_partition: u32,
    pub partition_entry_crc32: u32
}

// TODO: Add validation methods
impl GPTHeader {
    // slice containing a gpt header starting at index 0
    pub fn new(data: &[u8]) -> GPTHeader {
        if ! (data.len() >= 92) {
            panic!("Provided GPT header is too small")
        }
        GPTHeader {
            signature: LittleEndian::read_u64(&data[..8]),
            revision: LittleEndian::read_u32(&data[8..12]),
            header_size: LittleEndian::read_u32(&data[12..16]),
            header_crc32: LittleEndian::read_u32(&data[16..20]),
            reserved: LittleEndian::read_u32(&data[20..24]),
            current_lba: LittleEndian::read_u64(&data[24..32]),
            backup_lba: LittleEndian::read_u64(&data[32..40]),
            first_usable_lba: LittleEndian::read_u64(&data[40..48]),
            last_uasable_lba: LittleEndian::read_u64(&data[48..56]),
            guid: uuid_from_le_bytes(&data[56..72]),
            partition_entry_lba: LittleEndian::read_u64(&data[72..80]),
            number_of_partions: LittleEndian::read_u32(&data[80..84]),
            size_of_partition: LittleEndian::read_u32(&data[84..88]),
            partition_entry_crc32: LittleEndian::read_u32(&data[88..92])
        }
    }

    pub fn json_value(&self) -> serde_json::value::Value {
        json!({
            "signature": std::str::from_utf8(
                &self.signature.to_le_bytes().to_vec()).unwrap(),
            "revision": format!("0x{:04X}", self.revision),
            "headerSizeBytes": self.header_size,
            "headerCRC32": format!("0x{:04X}", self.header_crc32),
            "currentLBA": self.current_lba,
            "backupLBA": self.backup_lba,
            "firstUsableLBA": self.first_usable_lba,
            "lastUsableLBA": self.last_uasable_lba,
            "partitionTableGUID": self.guid,
            "partitionEntryLBA": self.partition_entry_lba,
            "numberOfPartitions": self.number_of_partions,
            "sizeOfPartition": self.size_of_partition,
            "partitionEntryCRC32": format!("0x{:04X}", self.partition_entry_crc32)
        })
    }

    pub fn json(&self) -> String {
        serde_json::to_string_pretty(&self.json_value()).unwrap()
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GPTPartitionEntry {
    pub partition_type_guid: Uuid,
    pub unique_partition_guid: Uuid,
    pub starting_lba: u64,
    pub ending_lba: u64,
    pub attributes: u64,
    pub partition_name: String,
}

impl GPTPartitionEntry {
    pub fn new(data: &[u8]) -> GPTPartitionEntry {
        if ! (data.len() >= 128) {
            panic!("Slice is not large enough to contain a valid GPT partition");
        }

        // Convert byte array to u16 for partition name string
        let mut utf16: Vec<u16> = Vec::with_capacity(36);
        for n in (0..72).step_by(2) {
            let offset = n + 56;
            let value =  LittleEndian::read_u16(&data[offset..offset+2]);
            if value == 0 {
                break;
            }
            utf16.push(value);
        }

        GPTPartitionEntry {
            partition_type_guid: uuid_from_le_bytes(&data[..16]),
            unique_partition_guid: uuid_from_le_bytes(&data[16..32]),
            starting_lba: LittleEndian::read_u64(&data[32..40]),
            ending_lba: LittleEndian::read_u64(&data[40..48]),
            attributes: LittleEndian::read_u64(&data[48..56]),
            partition_name: String::from_utf16(&utf16).unwrap()
        }
    }

    pub fn json_value(&self) -> serde_json::value::Value {
        json!({
            "partitionTypeGuid": self.partition_type_guid,
            "uniquePartitionGuid": self.unique_partition_guid,
            "startingLBA": self.starting_lba,
            "endingLBA": self.ending_lba,
            "attibutes": format!("0x{:08X}", self.attributes),
            "partitionName": self.partition_name 
        })
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GPTPartitionEntryArray {
    pub partitions: Vec<GPTPartitionEntry>
}

impl GPTPartitionEntryArray {
    pub fn from_reader<R: std::io::Seek + std::io::Read>(reader: &mut R, header: &GPTHeader) 
            -> Result<GPTPartitionEntryArray, Box<std::error::Error>> {
        let entry_bytes = header.partition_entry_lba * 512;

        let mut entry_table_buffer = vec![0 as u8;
            (header.size_of_partition * header.number_of_partions) as usize];

        let mut entries: Vec<GPTPartitionEntry> = Vec::with_capacity(
            header.number_of_partions as usize);

        reader.seek(std::io::SeekFrom::Start(entry_bytes))?;
        reader.read(&mut entry_table_buffer)?;

        for n in 0..header.number_of_partions {
            let offset = (n * header.size_of_partition) as usize;
            entries.push(
                GPTPartitionEntry::new(
                    &entry_table_buffer[offset..offset+(header.size_of_partition as usize)]))
        }
        Ok(GPTPartitionEntryArray {
            partitions: entries
        })
    }

    pub fn json_value(&self) -> serde_json::value::Value {
        let mut temp_vec: Vec<&GPTPartitionEntry> = Vec::with_capacity(
            self.partitions.len());
        for partition in self.partitions.iter() {
            if partition.partition_type_guid == Uuid::nil() {
                continue;
            } else {
                temp_vec.push(partition);
            }
        }
        json!({
            "gptPartitionEntries": temp_vec,
            "usablePartitions": self.partitions.len(),
            "activePartitions": temp_vec.len()
        })
    }
}