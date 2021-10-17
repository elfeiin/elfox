use super::*;

pub type BlockIndex = u128;
pub type DataLength = u128;
pub type OffsetIntoBlock = u32;
pub type Result<T> = std::result::Result<T, Error>;
pub type DataBuf = [u8; BLOCK_SIZE - size_of::<u128>()];
pub type RawBuf = [u8; BLOCK_SIZE];
