extern crate press;

use std::process::exit;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    println!("{:?}", press::sysfs::BlockDeviceGeometry::from_device(&args[1]));
}