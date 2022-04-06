#[macro_use]
extern crate serde;

pub mod account;
pub mod function;
pub mod node;
pub mod result;
pub mod source;
pub mod value;

pub mod codec {
    pub fn encode<T>(val: &T) -> Result<Vec<u8>, rmp_serde::encode::Error>
    where
        T: serde::Serialize + ?Sized,
    {
        rmp_serde::encode::to_vec(val).map_err(Into::into)
    }

    pub fn decode<T>(rd: &[u8]) -> Result<T, rmp_serde::decode::Error>
    where
        T: serde::de::DeserializeOwned + ?Sized,
    {
        rmp_serde::decode::from_read(rd).map_err(Into::into)
    }
}
