extern crate press;

fn main() {
    for d in press::sysfs::get_block_devices() {
        println!("{}", d.path().to_str().unwrap());
    }

    press::block::get_block_devices();



}