/// Stores binary prefix sizes, 
pub mod binary_prefixes {
    type ByteUnit = usize;

    /// 1 Byte
    pub const ONE_B: ByteUnit = 1;
    /// 1 KB
    pub const KB: ByteUnit = 1024 * ONE_B;
    /// 1 MB
    pub const MB: ByteUnit = 1024 * KB;
    /// 1 GB
    pub const GB: ByteUnit = 1024 * MB;
    /// 1 TB
    pub const TB: ByteUnit = 1024 * GB;
}