extern crate serde;

use serde::de::{self, Visitor, Deserializer, Deserialize};
use std::fmt;
use std::error::Error;
use std::str::FromStr;

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


pub fn parse_bytes(value: &str) -> SizeParseResult<u64> {
    let split_value: Vec<&str> = value.splitn(2, ' ')
        .map(|s| s.trim()).collect();

    let (val, multiplier) = match split_value.len() < 2 {
        // Support for representations without spaces, 10MiB for example
        true => {
            let mut split_index: usize = 0;
            for (i, c) in split_value[0].char_indices() {
                // find the first index that is not a digit
                if !(c.is_digit(10)) {
                    split_index = i;
                    break
                }
            }
            if split_index > 0 {
                // Found something, try to parse the suffix
                let (start, end) = split_value[0].split_at(split_index);
                (start.to_owned(), get_multiplier(end)?)
            } else { 
                (split_value[0].to_owned(), 1) 
            }
        },
        false => {
            (split_value[0].to_owned(), get_multiplier(split_value[1])?)
        }
    };
    // Finally parse the value as a u64
    let val: u64 = match val.parse() {
        Ok(v) => v,
        Err(_) => return Err(
            SizeParseError::new(
                format!("{} is not valid", split_value[0]).as_str()))
    };
    Ok(val * multiplier)
}

pub fn get_multiplier(suffix: &str) -> SizeParseResult<u64> {
    let v = match suffix.to_ascii_lowercase().as_str() {
        "b" => 1,
        "k" => 1000,
        "kb" => 1000,
        "kib" => 1 << 10,
        "m" => 1_000_000,
        "mb" => 1_000_000,
        "mib" => 1 << 20,
        "g" => 1_000_000_000,
        "gb" => 1_000_000_000,
        "gib" => 1 << 30,
        "t" => 1_000_000_000_000,
        "tb" => 1_000_000_000_000,
        "tib" => 1 << 40,
        "p" => 1_000_000_000_000_000,
        "pb" => 1_000_000_000_000_000,
        "pib" => 1 << 50,
        _ => return Err(
            SizeParseError::new(
                format!("{} is not a supported suffix", suffix).as_str()))
    };
    Ok(v)
}

#[derive(Debug)]
pub struct Size {
    _bytes: u64
}

impl Size {
    pub fn new(bytes: u64) -> Self {
        Self {
            _bytes: bytes
        }
    }

    pub fn bytes(&self) -> u64 {
        self._bytes
    }

    pub fn as_symbol(&self, symbol: &str) -> SizeParseResult<String> {
        let divisor = get_multiplier(symbol)?;
        Ok(format!("{} {}", self._bytes / divisor, symbol))
    }

    pub fn as_sectors(&self, lba_size: u64) -> u64 {
        self._bytes / lba_size
    }
}

impl FromStr for Size {
    type Err = SizeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Size {
            _bytes: parse_bytes(s)?
        })
    }
}

// Deserializer
struct SizeVistor;

impl<'de> Visitor<'de> for SizeVistor {
    type Value = Size;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter
            .write_str("A string which can be parsed to a press::Size structure")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: serde::de::Error {
            Ok(Size::from_str(s).unwrap())
    }

    fn visit_u64<E>(self, value: u64) -> Result<Self::Value, E>
    where
        E: serde::de::Error {
            Ok(Size::new(value))
    }
}

impl<'de> Deserialize<'de> for Size {
    fn deserialize<D>(deserializer: D) -> Result<Size, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_any(SizeVistor)
    }
}

// Tests
#[test]
fn test_get_multiplier() {
    let val = get_multiplier("XXX");
    assert!(val.is_err());

    let s = Size::from_str("100 GiB").unwrap();
    assert!(s.bytes() == 100 * 2u64.pow(30));

    let s = "1024 MiB".parse::<Size>().unwrap();

    assert!(s.bytes() == 2u64.pow(30));

    let s = "1024MiB".parse::<Size>().unwrap();
    assert!(s.bytes() == 2u64.pow(30));
}

#[test]
fn test_de() {
    #[derive(Debug, serde::Deserialize)]
    struct Sizes {
        s1: Size,
        s2: Size,
        s3: Size
    };

    let json = r#"
        {
            "s1": "100 MiB",
            "s2": 2048,
            "s3": 1048576
        }
    "#;
    
    let sizes: Sizes = serde_json::from_str(json).unwrap();

    assert_eq!(sizes.s1.bytes(), 100 * 1 << 20);
    assert_eq!(sizes.s2.as_symbol("KiB").unwrap(), "2 KiB");
    assert_eq!(sizes.s3.as_sectors(512), 2048);
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_as_symbol() {
        let s1: Size = "1 MiB".parse().unwrap();
        assert_eq!(s1.as_symbol("KiB").unwrap(), "1024 KiB");
        assert_eq!("1 MiB".parse::<Size>()
            .unwrap()
            .as_sectors(512),
        2048);
    }
}
