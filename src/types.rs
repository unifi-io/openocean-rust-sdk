use serde::{Deserialize, Deserializer, Serialize};
use std::fmt;



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct U128(pub u128);

impl<'de> Deserialize<'de> for U128 {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = U128;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("u128 or string")
            }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> { Ok(U128(v as u128)) }
            fn visit_u128<E>(self, v: u128) -> Result<Self::Value, E> { Ok(U128(v)) }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E>
            where E: serde::de::Error {
                if v < 0 { return Err(E::custom("negative not allowed")); }
                Ok(U128(v as u128))
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E>
            where E: serde::de::Error {
                if v < 0.0 { return Err(E::custom("negative not allowed")); }
                Ok(U128(v as u128))
            }
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                s.parse::<u128>().map(U128).map_err(E::custom)
            }
        }
        de.deserialize_any(V)
    }
}






#[derive(Debug, Clone, Copy, PartialEq)]
pub struct F64(pub f64);

impl<'de> Deserialize<'de> for F64 {
    fn deserialize<D>(de: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        struct V;
        impl<'de> serde::de::Visitor<'de> for V {
            type Value = F64;
            fn expecting(&self, f: &mut fmt::Formatter) -> fmt::Result {
                f.write_str("f64 or string")
            }
            fn visit_f64<E>(self, v: f64) -> Result<Self::Value, E> { Ok(F64(v)) }
            fn visit_u64<E>(self, v: u64) -> Result<Self::Value, E> { Ok(F64(v as f64)) }
            fn visit_i64<E>(self, v: i64) -> Result<Self::Value, E> { Ok(F64(v as f64)) }
            fn visit_str<E>(self, s: &str) -> Result<Self::Value, E>
            where E: serde::de::Error {
                s.parse::<f64>().map(F64).map_err(E::custom)
            }
        }
        de.deserialize_any(V)
    }
}