use allocation_table::AllocationTable;
use serde::{Deserialize, Serialize};
use serde_big_array::BigArray;
use std::fs::File;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::mem::size_of;
use std::path::{Path, PathBuf};
use std::collections::HashMap;

mod error;
use error::*;
mod constants;
use constants::*;
mod types;
use types::*;
mod allocation_table;
mod heap;
use heap::*;
mod elfox;
pub use elfox::*;

// Blocks: linked list
// The heap table stored inside the blocks that describes where data in blocks starts

// #[derive(Debug, Copy, Clone, Serialize, Deserialize)]
// #[repr(C)]
// struct RawBlock<T> {
// 	location: u128,
// 	data: T,
// }

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
struct Data {
	next: u128,
	#[serde(with = "BigArray")]
	data: [u8; BLOCK_SIZE - size_of::<u128>()],
}