extern crate byteorder;
extern crate uuid;

pub mod mbr;
pub mod gpt;
pub mod sysfs;
pub mod udev;
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
