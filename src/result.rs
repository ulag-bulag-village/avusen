#[repr(C)]
#[derive(Default)]
pub struct BytesResult {
    pub ok: u32,
    pub buf: u32,
    pub len: u32,
}
