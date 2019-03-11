extern crate byteorder;
extern crate serde;

use byteorder::{LittleEndian, ByteOrder};
use serde::Serialize;
use serde_json::json;

static MBR_SIGNATURE: u16 = 0xaa55;
pub static PROTECTIVE_MBR_OSTYPE: u8 = 0xee;


pub fn has_mbr(buffer: &[u8]) -> bool {
    // MBR signature (16 bit) is located at offset 510
    LittleEndian::read_u16(&buffer[510..512]) == MBR_SIGNATURE
}

pub fn is_mbr_protective(buffer: &[u8]) -> bool {
    buffer[450] == 0xee
}

#[derive(Debug, Default, Serialize)]
pub struct MBRPartition {
    pub boot_indicator: u8,
    pub starting_chs: u32,
    pub os_type: u8,
    pub ending_chs: u32,
    pub starting_lba: u32,
    pub size_in_lba: u32
}

impl MBRPartition {
    pub fn new(data: &[u8]) -> MBRPartition {
        if data.len() != 16 {
            panic!("MBR partition strucutres must be 16 bytes.");
        }
        MBRPartition {
            boot_indicator: data[0],
            starting_chs: LittleEndian::read_u24(&data[1..4]),
            os_type: data[4],
            ending_chs: LittleEndian::read_u24(&data[5..8]),
            starting_lba: LittleEndian::read_u32(&data[8..12]),
            size_in_lba: LittleEndian::read_u32(&data[12..16])
        }
    }

    fn _json(&self) -> serde_json::Value {
        json!({
          "bootIndicator": format!("0x{:02X}", self.boot_indicator),
          "osType": format!("0x{:02X}", self.os_type),
          "startingByte": self.starting_lba as u64 * 512,
          "sizeBytes": self.size_in_lba as u64 * 512
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MBR {
    pub disk_signature: u32,
    pub unknown: u16,
    pub partition_records: [MBRPartition; 4],
    pub sig: u16
}

impl MBR {
    pub fn new(data: &[u8]) -> MBR {
        if ! (data.len() >= 512) {
            panic!("Data containing an MBR must be at least 512 bytes.")
        }
        let mut records: [MBRPartition; 4] = Default::default();
        for n in 0..4 {
            let offset = 446 + n * 16;
            records[n] = MBRPartition::new(&data[offset..offset+16]);
        }
        MBR {
            disk_signature: LittleEndian::read_u32(&data[424..428]),
            unknown: LittleEndian::read_u16(&data[428..430]),
            partition_records: records,
            sig: LittleEndian::read_u16(&data[510..512])
        }
    }

    fn _json(&self) -> serde_json::Value {
        let mut records: Vec<serde_json::Value> = Vec::with_capacity(4);
        for r in self.partition_records.iter() {
            if r.size_in_lba == 0 {
                // Don't display empty partitions
                continue
            }
            records.push(r._json())
        }

        json!({
            "mbrInfo": json!({
                "diskSignature": format!("0x{:08X}", self.disk_signature),
                "signature": format!("0x{:04X}", self.sig),
                "partitionRecords": records
            })
        })
    }

    pub fn json(&self) -> String {
        // Ok(serde_json::to_string_pretty(&self)?)
        serde_json::to_string_pretty(&self._json()).unwrap()
    }

    pub fn is_protective(&self) -> bool {
        if ! (self.partition_records.len() > 0) {
            false
        } else {
            self.partition_records[0].os_type == PROTECTIVE_MBR_OSTYPE
        }
        
    }
}