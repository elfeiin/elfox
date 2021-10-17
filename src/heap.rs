use super::*;

pub enum Either<T, K> {
	A(T),
	B(K),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeIndex {
	block_offset: BlockIndex,     // The offset in blocks of this data
	byte_offset: OffsetIntoBlock, // The offset into the block that contains this data
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DataIndex {
	block_offset: BlockIndex,
	length: DataLength, // Necessarily larger range than a block's size.
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HeapNode<T: Ord + Eq> {
	key: T,
	value: DataIndex, // The index of the data this node refers to
	l: NodeIndex, // The index of the left node
	r: NodeIndex, // The index of the right node
}

impl<T: Ord + Eq> HeapNode<T> {
	pub fn test(&self, key: T) -> Either<DataIndex, NodeIndex> {
		{
			use std::cmp::Ordering::*;
			match self
				.key
				.cmp(&key)
			{
				Less => Either::B(self.r.clone()),
				Equal => Either::A(self.value.clone()),
				Greater => Either::B(self.l.clone()),
			}
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Heaps {
	next: u128,
	length: DataLength,
	#[serde(with = "BigArray")]
	buf: [u8; BLOCK_SIZE - (size_of::<u128>() + size_of::<DataLength>())],
}
