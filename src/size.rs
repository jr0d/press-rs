extern crate serde;

use std::fmt;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess}

use crate::errors::*;


pub struct Size {
    _bytes: u64
}

impl Size {
    fn bytes(&self) -> u64 {
        self.bytes()
    }
}

impl From<&str> for Size {
    fn from(item: &str) -> Result<Self> {
        let u64
    }
}


struct SizeVisitor;

impl<'de> Visitor for SizeVisitor {
    type value = Size;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        write!(formatter, "A u64 value or a string value")
    }
}


impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SizeVisitor);
    }
}


// fn split_suffix(value: &str) {
    
// }

// fn main() {
//     let i = "123.23".parse::<f64>();
//     println!("{:?}", i);
    
//     let s1 = "2032.27";
//     let s2 = "2048 MiB";
    
//     let split_whitespace: Vec<&str> = s2.split_whitespace().collect();
    
//     println!("{:?}", split_whitespace);
//     // let bytes = match assert!(s1.contains(".")) {
//     //     true => {
                    
//     //     }
//     // }
    
// }