extern crate byteorder;
extern crate uuid;

pub mod mbr;
pub mod gpt;
pub mod sysfs;
pub mod block;

use byteorder::{LittleEndian, ByteOrder};
use uuid::Uuid;

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

// Safely converts a &[u8] slice to a u16 vector in LE byte order
pub fn convert_u8a_to_u16a_le(u8_array: &[u8]) -> Vec<u16> {
    let mut u16a: Vec<u16> = Vec::with_capacity(u8_array.len() / 2 + 1);
    let remainder: usize = u8_array.len() % 2;
    let mod_len = u8_array.len() - remainder;
    for n in (0..mod_len).step_by(2) {
        let u16_val = ((u8_array[n+1] as u16) << 8) | u8_array[n] as u16;
        u16a.push(u16_val);
    }
    
    if remainder == 1 {
        u16a.push(*u8_array.last().unwrap() as u16);
    }
    u16a
}

pub fn uuid_from_le_bytes(bytes: &[u8]) -> Uuid {
    if bytes.len() != 16 {
        panic!("GUIDs must be 16 bytes long")
    }
    Uuid::from_fields(
        LittleEndian::read_u32(&bytes[..4]),
        LittleEndian::read_u16(&bytes[4..6]),
        LittleEndian::read_u16(&bytes[6..8]),
        &bytes[8..]).unwrap()
}
