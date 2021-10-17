use super::*;

// Size of a Block in bytes
pub const BLOCK_SIZE: usize = 0x4_00_00;

// Number of Blocks an AllocationTable can refer to
pub const ALLOC_TABLE_NUM_REFERRABLE_BLOCKS: usize = BLOCK_SIZE * 8;

// Number of Blocks an AllocationTable can refer to plus one for the AllocationTable
pub const SEGMENT_LEN: usize = ALLOC_TABLE_NUM_REFERRABLE_BLOCKS + 1;

// The first heap block
pub const FIRST_HEAP_BLOCK: BlockIndex = 1;