extern crate serde;
extern crate libudev;

use std::collections::HashMap;
use serde::Serialize;

// There is realy no reason to have all of this data stored in memory
// I'll likely create a final struct with all of the data the application
// needs and selectively query these sources
#[derive(Debug, Serialize)]
pub struct UdevBlockDeviceInfo {
    name: String,
    udev_properties: HashMap<String, String>
}

impl UdevBlockDeviceInfo {
    pub fn name(&self) -> &str {
        self.name.as_str()
    }

    pub fn properties(&self) -> &HashMap<String, String> {
        &self.udev_properties
    }
}

pub fn get_block_device(device: libudev::Device) -> UdevBlockDeviceInfo {
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
    UdevBlockDeviceInfo {
                name: device.devnode()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                udev_properties: hm
            }
}

fn _get_block_devices(enumerator: &mut libudev::Enumerator) -> Result<Vec<UdevBlockDeviceInfo>, Box<std::error::Error>> {
    let mut devices = Vec::new();
    for d in enumerator.scan_devices()? {
        devices.push(get_block_device(d))
    }
    Ok(devices)
}

fn _get_enumerator(context: &libudev::Context) -> Result<libudev::Enumerator, Box<std::error::Error>> {
    let mut enumerator = libudev::Enumerator::new(&context)?;
    enumerator.match_subsystem("block")?;
    Ok(enumerator)
}

pub fn get_block_devices_with_property(name: &str, value: &str) -> Result<Vec<UdevBlockDeviceInfo>, Box<std::error::Error>> {
    let context = libudev::Context::new()?;
    let mut enumerator = _get_enumerator(&context)?;
    enumerator.match_property(name, value)?;
    Ok(_get_block_devices(&mut enumerator)?)
}

pub fn get_block_devices() -> Result<Vec<UdevBlockDeviceInfo>, Box<std::error::Error>> {
    let context = libudev::Context::new()?;
    let mut enumerator = _get_enumerator(&context)?;
    Ok(_get_block_devices(&mut enumerator)?)
}

pub fn get_disks() -> Result<Vec<UdevBlockDeviceInfo>, Box<std::error::Error>> {
    Ok(get_block_devices_with_property("DEVTYPE", "disk")?)
}

pub fn get_partitions() -> Result<Vec<UdevBlockDeviceInfo>, Box<std::error::Error>> {
    Ok(get_block_devices_with_property("DEVTYPE", "partition")?)
}
