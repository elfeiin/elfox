use super::*;

pub struct RawBlock {
	location: u128,
	data: RawBuf,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct AllocTable {
	location: u128,
	table: AllocationTable,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct DataBlock {
	location: u128,
	data: Data,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct HeapBlock {
	location: u128,
	heap_block: Heaps,
}

pub struct Elfox<T> {
	path: PathBuf,
	the_index_type: std::marker::PhantomData<T>,
}

// https://discord.com/channels/273534239310479360/274215136414400513/895516425031127141

impl<'a, T> Elfox<T> {
	pub fn new(path: PathBuf) -> Self {
		Self {
			path,
			the_index_type: std::marker::PhantomData,
		}
	}

	pub fn init(&self) -> Result<()> {
		let mut file = if let Ok(file) = File::open(&self.path) {
			file
		} else {
			File::create(&self.path)?
		};

		let to_write = bincode::serialize(&AllocationTable::new())?;

		#[cfg(test)]
		println!["{}", to_write.len()];

		file.write_all(&to_write)?;

		Ok(())
	}

	pub fn file_len(&self) -> Result<u128> {
		let file = File::open(&self.path)?;
		Ok(file
			.metadata()?
			.len() as u128)
	}

	pub fn file_len_in_blocks(&self) -> Result<u128> {
		Ok(self.file_len()? / BLOCK_SIZE as u128)
	}

	pub fn num_alloc_tables(&self) -> Result<u128> {
		Ok(self.file_len_in_blocks()? / SEGMENT_LEN as u128)
	}

	fn block_exists(&self, n: BlockIndex) -> Result<bool> {
		Ok(self.file_len_in_blocks()? >= n)
	}
	
	// n is offset into file in blocks
	fn load_raw_block(&self, n: BlockIndex) -> Result<RawBlock> {
		let mut file = File::open(&self.path)?;
		file.seek(SeekFrom::Start((n * BLOCK_SIZE as u128) as u64))?;
		let mut buf = [0u8; BLOCK_SIZE];
		let rehd = file.read(&mut buf)?;
		if rehd < BLOCK_SIZE {
			return Err(Error::ReachedEOF);
		}
		Ok(RawBlock {
			location: n,
			data: buf,
		})
	}

	fn load_data(&self, n: BlockIndex) -> Result<DataBlock> {
		let buf = self.load_raw_block(n)?;
		Ok(DataBlock {
			location: buf.location,
			data: bincode::deserialize(&buf.data)?,
		})
	}

	fn load_allocation_table(&self, n: BlockIndex) -> Result<AllocTable> {
		let buf = self.load_raw_block(n * SEGMENT_LEN as u128)?;
		Ok(AllocTable {
			location: buf.location,
			table: bincode::deserialize(&buf.data)?,
		})
	}

	fn load_heap_block(&self, n: BlockIndex) -> Result<HeapBlock> {
		let buf = self.load_raw_block(n * SEGMENT_LEN as u128)?;
		Ok(HeapBlock {
			location: buf.location,
			heap_block: bincode::deserialize(&buf.data)?,
		})
	}

	fn find_first_free_block(&self) -> Result<Option<BlockIndex>> {
		for i in 0..self.num_alloc_tables()? {
			let current_alloc_table = self.load_allocation_table(i)?;
			if let Some(x) = current_alloc_table
				.table
				.find_first_free_block()
			{
				return Ok(Some(x as u128 + (i * SEGMENT_LEN as u128)));
			}
		}
		Ok(None)
	}

	fn create_block(&self) -> Result<()> {
		let mut file = File::open(&self.path)?;
		let buf = [0u8; BLOCK_SIZE];
		file.write_all(&buf)?;
		Ok(())
	}

	pub fn get_data_address(&self, i: T) -> Result<Option<BlockIndex>> {
		let mut curr_heap_block = self.load_heap_block(FIRST_HEAP_BLOCK);

		loop {
			todo![];
		}

		Ok(None)
	}
}

#[test]
fn make_a_file() {
	let mut boop = Elfox::<u128>::new("./the_first_file".into());

	boop.init();

	println![
		"{:X}",
		boop.file_len()
			.unwrap()
	];
}
