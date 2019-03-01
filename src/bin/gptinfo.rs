extern crate press;
extern crate serde;
extern crate uuid;

use std::io::prelude::*;
use std::fs::File;
use std::process::exit;

use serde_json::{json, to_string_pretty};
use press::gpt::{GPTHeader, GPTPartitionEntryArray};
use press::is_gpt;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    let mut buffer = vec![0 as u8; 1024];
    let mut fp = File::open(&args[1])?;
    fp.read(&mut buffer)?;

    if ! is_gpt(&buffer) {
        eprintln!("{} does not contain a valid GPT header", args[1]);
        exit(1);
    }

    let gpt_header = GPTHeader::new(&buffer[512..]);
    let gpt_partitions = GPTPartitionEntryArray::from_reader(
        &mut fp, &gpt_header).unwrap();

    println!("{}", to_string_pretty(&json!(
        {
            "gptHeader": gpt_header.json_value(),
            "gptPartitions": gpt_partitions.json_value()
        })).unwrap());

    Ok(())

}