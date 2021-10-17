use super::*;



pub enum Status {
	Free = 0,
	Used = 1,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[repr(C)]
struct Entry {
	data: u8,
}

impl Entry {
	pub fn get_status(self, index: usize) -> Status {
		if self.data >> index & 0x01 == 1 {
			Status::Used
		} else {
			Status::Free
		}
	}

	pub fn set_status(&mut self, index: usize, status: Status) {
		match status {
			Status::Free => {
				self.data &= 0xFF ^ (0x01 << index);
			}
			Status::Used => {
				self.data |= 0x01 << index;
			}
		}
	}
}

/// Describes which blocks are free for a fixed range.
#[derive(Debug, Serialize, Deserialize)]
pub struct AllocationTable {
	#[serde(with = "BigArray")]
	entries: [Entry; BLOCK_SIZE],
}

impl AllocationTable {
	pub fn new() -> Self {
		Self {
			entries: [Entry { data: 0 }; BLOCK_SIZE]
		}
	}
	pub fn get_status(&self, index: usize) -> Result<Status> {
		if index > ALLOC_TABLE_NUM_REFERRABLE_BLOCKS {
			Err(Error::AllocationOutOfBoundsForTable)
		} else {
			Ok(self.entries[index/8].get_status(index % 8))
		}
	}
	pub fn set_status(&mut self, index: usize, status: Status) -> Result<()> {
		if index > ALLOC_TABLE_NUM_REFERRABLE_BLOCKS {
			Err(Error::AllocationOutOfBoundsForTable)
		} else {
			self.entries[index/8].set_status(index % 8, status);
			Ok(())
		}
	}
	
	pub fn find_first_free_block(&self) -> Option<usize> {
		for (i, entry) in self.entries.iter().enumerate() {
			for j in 0..8 {
				if let Ok(Status::Free) = self.get_status(i * 8 + j) {
					return Some(i * 8 + j);
				}
			}
		}
		None
	}
}

#[test]
fn allocation_table_size() {
	assert_eq![BLOCK_SIZE, std::mem::size_of::<AllocationTable>()];
}
