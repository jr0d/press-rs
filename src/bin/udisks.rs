// Simply dumps JSON udev information of a block device

extern crate press;
extern crate serde;

use std::process::exit;
use serde_json::{json, to_string_pretty};

use press::block::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    let target = &args[1];

    let block_devices = get_block_devices_with_property("DEVTYPE", "disk");
    println!("{}", to_string_pretty(&json!(block_devices)).unwrap());
    Ok(())
}