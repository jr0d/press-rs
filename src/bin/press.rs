#[macro_use]
extern crate log;
extern crate env_logger;
extern crate press;
extern crate serde;

use std::process::exit;
use serde_json::{json, to_string_pretty};

use press::block::device::*;
use press::udev::get_block_devices_with_property;
use press::sysfs::{BlockDeviceGeometry, kernel_path_to_sys};
use press::gpt::{GPTHeader, gpt_header_as_bytes};

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    env_logger::init();

    debug!("Assembling {}", &args[1]);
    let d = BlockDevice::assemble(&args[1]).expect("Ahhh!!!");
    println!("{:?}", d);

    let block_devices = get_block_devices_with_property("DEVNAME", &args[1])?;
    if ! block_devices.len() == 1 {
        panic!("BLLBLBLBLB");
    }
    let geometry = BlockDeviceGeometry::from_device(
        &kernel_path_to_sys(&block_devices[0].properties()["DEVPATH"]))?;
    let gpt_header = GPTHeader::new(&geometry);
    println!("{:?}", gpt_header);

    let gpt_bytes = gpt_header_as_bytes(&gpt_header);

    for b in gpt_bytes.iter() {
        print!("{:02x}", b);
    }
    print!("\n");
    //println!("{:?}", &gpt_header_as_bytes(&gpt_header));
    Ok(())
}
