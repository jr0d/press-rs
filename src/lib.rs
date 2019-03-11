//! # Press
//! Press is an operating system image creator and installer with customer partitioning,
//! LVM, and software RAID support. Press is written in Rust and compatible with
//! most operating systems. 

extern crate byteorder;
extern crate uuid;

pub mod mbr;
pub mod gpt;
pub mod sysfs;
pub mod udev;
pub mod block;