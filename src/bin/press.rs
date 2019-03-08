// Simply dumps JSON udev information of a block device

extern crate press;
extern crate serde;

use std::process::exit;
use serde_json::{json, to_string_pretty};

use press::block::device::*;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    let d = BlockDevice::assemble(&args[1]).expect("Ahhh!!!");
    println!("{:?}", d);
    Ok(())
}