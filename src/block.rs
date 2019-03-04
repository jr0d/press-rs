extern crate libudev;

pub fn get_block_devices() {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = libudev::Enumerator::new(&context).unwrap();

    enumerator.match_subsystem("block").unwrap();
    enumerator.match_property("DEVTYPE", "disk").unwrap();

    for device in enumerator.scan_devices().unwrap() {
        println!("found device: {:?}", device.devnode());
        // for prop in device.properties() {
        //     println!("{:?} = {:?}", prop.name(), prop.value());
        // }
        println!("{:?}", device.property_value("DEVNAME").unwrap());
    }
}