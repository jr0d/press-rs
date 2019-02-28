extern crate byteorder;
extern crate serde;

use byteorder::{LittleEndian, ByteOrder};
use serde::Serialize;
use serde_json::json;

use super::{convert_mixed_endian, format_guid};

#[derive(Debug, Default, Serialize)]
pub struct GPTHeader {
    signature: u64,
    revision: u32,
    header_size: u32,
    header_crc32: u32,
    reserved: u32,
    current_lba: u64,
    backup_lba: u64,
    first_usable_lba: u64,
    last_uasable_lba: u64,
    guid: Vec<u8>,
    partition_entry_lba: u64,
    number_of_partions: u32,
    size_of_partition: u32,
    partition_entry_crc32: u32
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
            guid: data[56..72].to_vec(),
            partition_entry_lba: LittleEndian::read_u64(&data[72..80]),
            number_of_partions: LittleEndian::read_u32(&data[80..84]),
            size_of_partition: LittleEndian::read_u32(&data[84..88]),
            partition_entry_crc32: LittleEndian::read_u32(&data[88..92])
        }
    }

    pub fn convert_and_format_guid(&self) -> String {
        let mut guid = self.guid.clone();
        convert_mixed_endian(&mut guid);
        format_guid(&guid)
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
            "partitionTableGUID": self.convert_and_format_guid(),
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
    partition_type_guid: Vec<u8>,
    unique_partition_guid: Vec<u8>,
    starting_lba: u64,
    ending_lba: u64,
    attributes: u64,
    partition_name: String,
}

impl GPTPartitionEntry {
    pub fn new(data: &[u8]) -> GPTPartitionEntry {
        if ! (data.len() >= 128) {
            panic!("Slice is not large enough to contain a valid GPT partition");
        }

        GPTPartitionEntry {
            partition_type_guid: data[..16].to_vec(),
            unique_partition_guid: data[16..32].to_vec(),
            starting_lba: LittleEndian::read_u64(&data[32..40]),
            ending_lba: LittleEndian::read_u64(&data[40..48]),
            attributes: LittleEndian::read_u64(&data[48..56]),
            partition_name: String::from_utf8(data[56..72].to_vec()).unwrap()

        }
    }
}

#[derive(Debug, Default, Serialize)]
pub struct GPTPartitionEntryArray {
    partitions: Vec<GPTPartitionEntry>
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
}