extern crate press;

use std::io::prelude::*;
use std::fs::File;
use std::process::exit;
use press::mbr::MBR;

fn main() -> Result<(), Box<std::error::Error>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("No arg..");
        exit(1);
    }

    let mut buffer = vec![0 as u8; 1024];
    let mut fp = File::open(&args[1])?;
    fp.read(&mut buffer)?;

    let mbr = MBR::new(&buffer);

    println!("{}", mbr.json());
    Ok(())
}