use num_bigint::BigUint;
use serde::de;
use serde::{
    de::{DeserializeSeed, Error, Visitor},
    Deserialize,
};
use std::fs::File;
use std::path::Path;

#[derive(Debug)]
struct Input {
    pub p: Box<BigUint>,
    pub q: Box<BigUint>,
}

#[derive(Deserialize)]
#[serde(field_identifier, rename_all = "lowercase")]
enum Fields {
    P,
    Q,
}

pub struct FromStringVisitor;

impl<'de> DeserializeSeed<'de> for FromStringVisitor {
    type Value = String;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: de::Deserializer<'de>,
    {
        deserializer.deserialize_str(self)
    }
}

impl<'de> Visitor<'de> for FromStringVisitor {
    type Value = String;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("expects a string")
    }

    fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        self.visit_string(s.to_owned())
    }

    fn visit_string<E>(self, s: String) -> Result<Self::Value, E>
    where
        E: de::Error,
    {
        Ok(s)
    }
}

impl<'de> Deserialize<'de> for Input {
    fn deserialize<D>(deserializer: D) -> Result<Self, <D as serde::Deserializer<'de>>::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct InputVisitor;

        impl<'de> Visitor<'de> for InputVisitor {
            type Value = Input;

            fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
                formatter.write_str("expected either string or int from json")
            }

            fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
            where
                A: serde::de::MapAccess<'de>,
            {
                let mut p = None;
                let mut q = None;
                while let Some(key) = map.next_key()? {
                    match key {
                        Fields::P => {
                            if p.is_some() {
                                return Err(de::Error::duplicate_field("p"));
                            }
                            let v: String = map.next_value_seed(FromStringVisitor)?;
                            p = Some(BigUint::parse_bytes(v.as_bytes(), 10).unwrap())
                        }
                        Fields::Q => {
                            if q.is_some() {
                                return Err(de::Error::duplicate_field("q"));
                            }
                            let v: String = map.next_value_seed(FromStringVisitor)?;
                            q = Some(BigUint::parse_bytes(v.as_bytes(), 10).unwrap())
                        }
                    }
                }
                let p = p.ok_or_else(|| de::Error::missing_field("p"))?;
                let q = q.ok_or_else(|| de::Error::missing_field("q"))?;

                Ok(Input {
                    p: Box::new(p),
                    q: Box::new(q),
                })
            }
        }

        const FIELDS: &'static [&'static str] = &["p", "q"];
        deserializer.deserialize_struct("Input", FIELDS, InputVisitor)
    }
}

fn main() {
    let json_file_path = Path::new("./wyuen8_input.json");
    let file = File::open(json_file_path).expect("file not found");
    let input: Input = serde_json::from_reader(file).expect("error while reading");
    println!("input: {:?}", input);
}
