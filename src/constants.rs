pub type DataUnit = u64;

/// Stores binary prefix sizes, 
pub mod binary_prefixes {
    use super::*;

    /// 1 Byte
    pub const ONE_B: DataUnit = 1;
    /// 1 KB
    pub const KB: DataUnit = 1024 * ONE_B;
    /// 1 MB
    pub const MB: DataUnit = 1024 * KB;
    /// 1 GB
    pub const GB: DataUnit = 1024 * MB;
    /// 1 TB
    pub const TB: DataUnit = 1024 * GB;
}