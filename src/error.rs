#[derive(Debug)]
pub enum Error {
	AllocationOutOfBoundsForTable,
	ReachedEOF,
	IO(std::io::Error),
	Bincode(bincode::Error),
}

impl From<std::io::Error> for Error {
	fn from(e: std::io::Error) -> Self {
		Self::IO(e)
	}
}

impl From<bincode::Error> for Error {
	fn from(e: bincode::Error) -> Self {
		Self::Bincode(e)
	}
}