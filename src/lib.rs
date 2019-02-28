extern crate byteorder;

pub mod mbr;
pub mod gpt;

use byteorder::{LittleEndian, ByteOrder, BigEndian};

static GPT_SIGNATURE: u64 = 0x5452415020494645;
static MBR_SIGNATURE: u16 = 0xaa55;

pub fn usage(name: &String) {
    println!("usage: {} <DISK>", name);
}

pub fn has_mbr(buffer: &[u8]) -> bool {
    // MBR signature (16 bit) is located at offset 510
    LittleEndian::read_u16(&buffer[510..512]) == MBR_SIGNATURE
}

pub fn is_mbr_protective(buffer: &[u8]) -> bool {
    buffer[450] == 0xee
}

pub fn is_gpt(buffer: &[u8]) -> bool {
    LittleEndian::read_u64(&buffer[512..520]) == GPT_SIGNATURE
}

pub fn get_disk_guid(buffer: &[u8]) -> Vec<u8> {
    buffer[568..584].to_vec()
}

// Convertes a mixed endian GUID to big endian
pub fn convert_mixed_endian(guid: &mut Vec<u8>) {
    // join these into one Vec and splice?
    let mut temp_vec = LittleEndian::read_u32(&guid[..4])
        .to_be_bytes().to_vec();
    temp_vec.extend(LittleEndian::read_u16(&guid[4..6])
        .to_be_bytes().iter().cloned());
    temp_vec.extend(LittleEndian::read_u16(&guid[6..8])
        .to_be_bytes().iter().cloned());
    guid.splice(..8, temp_vec.iter().cloned());
}

pub fn format_guid(guid: &Vec<u8>) -> String {
    format!("{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        BigEndian::read_u32(&guid[..4]),
        BigEndian::read_u16(&guid[4..6]),
        BigEndian::read_u16(&guid[6..8]),
        BigEndian::read_u16(&guid[8..10]),
        BigEndian::read_u48(&guid[10..]),
    )
}
