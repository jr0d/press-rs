extern crate serde;
extern crate libudev;

use std::collections::HashMap;
use serde::Serialize;

// There is realy no reason to have all of this data stored in memory
// I'll likely create a final struct with all of the data the application
// needs and selectively query these sources
#[derive(Debug, Serialize)]
pub struct BlockDevice {
    name: String,
    udev_properties: HashMap<String, String>
}

impl BlockDevice {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn properties(&self) -> &HashMap<String, String> {
        &self.udev_properties
    }
}

pub fn get_block_device(device: libudev::Device) -> BlockDevice {
    let mut hm = HashMap::new();
    for property in device.properties() {
        hm.insert(
            property.name()
                .to_str()
                .unwrap()
                .to_owned(),
            property.value()
                .to_str()
                .unwrap()
                .to_owned()
        );
    }
    BlockDevice {
                name: device.devnode()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                udev_properties: hm
            }
}

fn _get_block_devices(enumerator: &mut libudev::Enumerator) -> Vec<BlockDevice> {
    let mut devices = Vec::new();
    for d in enumerator.scan_devices().unwrap() {
        devices.push(get_block_device(d))
    }
    devices
}

fn _get_enumerator(context: &libudev::Context) -> libudev::Enumerator {
    let mut enumerator = libudev::Enumerator::new(&context).unwrap();
    enumerator.match_subsystem("block").unwrap();
    enumerator
}

pub fn get_block_devices_with_property(name: &str, value: &str) -> Vec<BlockDevice> {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = _get_enumerator(&context);
    enumerator.match_property(name, value).unwrap();
    _get_block_devices(&mut enumerator)
}

pub fn get_block_devices() -> Vec<BlockDevice> {
    let context = libudev::Context::new().unwrap();
    let mut enumerator = _get_enumerator(&context);
    _get_block_devices(&mut enumerator)
}

pub fn get_disks() -> Vec<BlockDevice> {
    get_block_devices_with_property("DEVTYPE", "disk")
}

pub fn get_partitions() -> Vec<BlockDevice> {
    get_block_devices_with_property("DEVTYPE", "partition")
}
