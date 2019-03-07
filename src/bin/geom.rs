extern crate press;
extern crate serde;

use std::process::exit;
use serde_json::json;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    println!("{}",
        &serde_json::to_string_pretty(
            &json!(
                press::sysfs::BlockDeviceGeometry::from_device(&args[1])
                    .unwrap()
                )).unwrap());
}