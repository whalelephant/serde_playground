use num_bigint::BigUint;
use serde::de;
use serde::{
    de::{DeserializeSeed, Visitor},
    Deserialize,
};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::Path;
use std::str::FromStr;

#[derive(Debug)]
struct DeBigUintType(BigUint);

#[derive(Debug, Deserialize)]
struct Input {
    pub p: DeBigUintType,
    pub q: DeBigUintType,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Fields {
    P,
    Q,
}

impl<'de> Deserialize<'de> for DeBigUintType {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let raw_value: &serde_json::value::RawValue =
            <&serde_json::value::RawValue>::deserialize(deserializer).map_err(|e| {
                println! {"helloo erro"};
                e
            })?;
        let raw_value = raw_value.get();

        Ok(DeBigUintType(
            BigUint::from_str(raw_value).map_err(de::Error::custom)?,
        ))
    }
}

fn main() {
    let json_file_path = Path::new("./input.json");
    let file = File::open(json_file_path).expect("file not found");
    let mut r = BufReader::new(file);
    let mut input = Vec::new();
    r.read_to_end(&mut input).unwrap();
    let re: Input = serde_json::from_slice(&input).unwrap();
    println!("input: {:?}", re);
}
