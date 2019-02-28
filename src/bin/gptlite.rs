// Attempts to parse the GUID partition table of the provided disk
// On sucess, this program will print the Unique GPT disk GUID and 
// the unique GUID for all partitions

// Only supports 512 byte LBAs (sectors)

extern crate byteorder;

use std::process::exit;
use std::io::prelude::*;
use std::fs::File;
use std::io::SeekFrom;

use byteorder::{LittleEndian, ByteOrder, BigEndian};

static GPT_SIGNATURE: u64 = 0x5452415020494645;
static MBR_SIGNATURE: u16 = 0xaa55;

fn usage(name: &String) {
    println!("usage: {} <DISK>", name);
}

fn has_mbr(buffer: &[u8]) -> bool {
    // MBR signature (16 bit) is located at offset 510
    LittleEndian::read_u16(&buffer[510..512]) == MBR_SIGNATURE
}

fn is_mbr_protective(buffer: &[u8]) -> bool {
    buffer[450] == 0xee
}

fn is_gpt(buffer: &[u8]) -> bool {
    LittleEndian::read_u64(&buffer[512..520]) == GPT_SIGNATURE
}

fn get_disk_guid(buffer: &[u8]) -> Vec<u8> {
    buffer[568..584].to_vec()
}

fn get_partition_table_guids(fp: &mut File, header_buffer: &[u8]) -> Result<Vec<Vec<u8>>, Box<std::error::Error>> {
    let entry_lba = LittleEndian::read_u64(&header_buffer[584..592]) as usize;
    let entry_bytes = entry_lba * 512;
    let num_partitions = LittleEndian::read_u32(&header_buffer[592..596]) as usize;
    let entry_size = LittleEndian::read_u32(&header_buffer[596..600]) as usize;
    let mut guids: Vec<Vec<u8>> = Vec::with_capacity(num_partitions as usize);

    let mut entry_table_buffer = vec![0 as u8; entry_size * num_partitions];
    fp.seek(SeekFrom::Start(entry_bytes as u64))?;
    fp.read(&mut entry_table_buffer)?;

    for n in 0..num_partitions {
        let mut offset: usize = n * entry_size;
        // 16 cmp or 16 bitwise operations... 
        // Choosing 16 bitwise operation for readability
        let partition_type_guid = LittleEndian::read_u128(
            &entry_table_buffer[offset..offset+16]);
        // If the part-type guid is `0` then skip the entry
        if partition_type_guid != 0 {
            // Location of partion Unique GUID
            offset += 16;
            guids.push(entry_table_buffer[offset..offset+16].to_vec());
        }
    }
    Ok(guids)
}

// Convertes a mixed endian GUID to big endian
fn convert_mixed_endian(guid: &mut Vec<u8>) {
    // join these into one Vec and splice?
    let mut temp_vec = LittleEndian::read_u32(&guid[..4])
        .to_be_bytes().to_vec();
    temp_vec.extend(LittleEndian::read_u16(&guid[4..6])
        .to_be_bytes().iter().cloned());
    temp_vec.extend(LittleEndian::read_u16(&guid[6..8])
        .to_be_bytes().iter().cloned());
    guid.splice(..8, temp_vec.iter().cloned());
}

fn format_guid(guid: &Vec<u8>) -> String {
    format!("{:08x}-{:04x}-{:04x}-{:04x}-{:012x}",
        BigEndian::read_u32(&guid[..4]),
        BigEndian::read_u16(&guid[4..6]),
        BigEndian::read_u16(&guid[6..8]),
        BigEndian::read_u16(&guid[8..10]),
        BigEndian::read_u48(&guid[10..]),
    )
}

// fn sysfs_get_partition_names(device_name: &String) 
//     -> Result<Vec<String>, Box<std::error::Error>>{
//     Ok()
// }


fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();

    if args.len() != 2 {
        usage(&args[0]);
        exit(1);
    }

    let mut buffer = vec![0 as u8; 1024];
    let mut fp = File::open(&args[1])?;
    let n = fp.read(&mut buffer)?;

    if n < buffer.len() {
        panic!("Did not read enough bytes");
    }

    if ! is_gpt(&buffer) {
        eprintln!("Target does not contain a valid GPT.");
        exit(1);
    }

    if has_mbr(&buffer) {
        if ! is_mbr_protective(&buffer) {
            eprintln!("Real MBR present, this should not happen");
            exit(1);
        }
    }

    let mut disk_guid = get_disk_guid(&buffer);
    convert_mixed_endian(&mut disk_guid);
    println!("{}\t:\t{}", &args[1], format_guid(&disk_guid));

    let mut pt_guids = get_partition_table_guids(&mut fp, &buffer).unwrap();

    for (i, guid) in pt_guids.iter_mut().enumerate() {
        convert_mixed_endian(guid);
        println!("{}{}\t:\t{}", &args[1], i+1, format_guid(guid));
    }
    Ok(())
}
