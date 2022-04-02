use arrayvec::{ArrayString, ArrayVec};

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "value_type", content = "value")]
pub enum Value {
    None,
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    U8(u8),
    U16(u16),
    U32(u32),
    U64(u64),
    F32(f32),
    F64(f64),
    SmallBytes(ArrayVec<u8, { Value::CAPACITY_BYTES }>),
    SmallString(ArrayString<{ Value::CAPACITY_STRING }>),
    LargeBytes(Vec<u8>),
    LargeString(String),
}

impl Value {
    pub const CAPACITY_BYTES: usize = 64;
    pub const CAPACITY_STRING: usize = Self::CAPACITY_BYTES;
}
