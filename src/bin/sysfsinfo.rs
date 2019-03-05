extern crate press;

fn main() {
    for d in press::sysfs::get_block_devices() {
        println!("{}", d.path().to_str().unwrap());
    }

    let devices = press::block::get_disks();

    for d in devices {
        println!("{}" , d.properties().get("DEVNAME")
            .unwrap_or(&"NONAME".to_string()));
    }
}