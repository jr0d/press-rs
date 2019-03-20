extern crate serde;

use std::fmt;
use std::error::Error;
use std::str::FromStr;
use serde::de::{self, Deserialize, Deserializer, Visitor, SeqAccess, MapAccess};

// Error Boiler plate
#[derive(Debug)]
pub struct SizeParseError {
    details: String
}

impl SizeParseError {
    fn new(msg: &str) -> Self {
        Self {
            details: msg.to_string()
        }
    }
}

impl fmt::Display for SizeParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for SizeParseError {
    fn description(&self) -> &str {
        &self.details
    }
}

type SizeParseResult<T> = Result<T, SizeParseError>;
// End Error boiler plate


fn parse_bytes(value: &str) -> SizeParseResult<u64> {
    let split_value: Vec<&str> = value.splitn(2, ' ')
        .map(|s| s.trim()).collect();

    let value: u64 = match split_value[0].parse() {
        Ok(v) => v,
        Err(_) => return Err(
            SizeParseError::new(format!("{} is not valid", split_value[0]).as_str()))
    };

    if split_value.len() < 1 {
        Ok(value)
    } else {
        Ok(value * get_multiplier(split_value[1])?)
    }
}

fn get_multiplier(suffix: &str) -> SizeParseResult<u64> {
    let v = match suffix.to_ascii_lowercase().as_str() {
        "b" => 1,
        "k" => 1000,
        "kb" => 1000,
        "kib" => 1024,
        "m" => 10u64.pow(6),
        "mb" => 10u64.pow(6),
        "mib" => 2u64.pow(20),
        "g" => 10u64.pow(9),
        "gb" => 10u64.pow(9),
        "gib" => 2u64.pow(30),
        "t" => 10u64.pow(12),
        "tb" => 10u64.pow(12),
        "tib" => 2u64.pow(40),
        "p" => 10u64.pow(15),
        "pb" => 10u64.pow(15),
        "pib" => 2u64.pow(50),
        _ => return Err(
            SizeParseError::new(format!("{} is not a supported suffix", suffix).as_str()))
    };
    Ok(v)
}

#[derive(Debug)]
pub struct Size {
    _bytes: u64
}

impl Size {
    pub fn bytes(&self) -> u64 {
        self._bytes
    }
}

impl FromStr for Size {
    type Err = SizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(
            Size {
                _bytes: parse_bytes(s)?
            }
        )
    }
}

#[test]
fn test_get_multiplier() {
    let val = get_multiplier("XXX");
    assert!(val.is_err());
    println!("{:?}", val);

    let s = Size::from_str("100 GiB").unwrap();
    println!("{:?}", &s);
    assert!(s.bytes() == 100 * 2u64.pow(30));
}


// struct SizeVisitor;

// impl<'de> Visitor for SizeVisitor {
//     type value = Size;

//     fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
//         write!(formatter, "A u64 value or a string value")
//     }
// }


// impl<'de> Deserialize<'de> for Size {
//     fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//     where
//         D: Deserializer<'de>,
//     {
//         deserializer.deserialize_any(SizeVisitor);
//     }
// }


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