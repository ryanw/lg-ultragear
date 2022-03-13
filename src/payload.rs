use std::{mem, slice};
pub const HEADER: [u8; 2] = [0x53, 0x43]; // SC
pub const TERMINATOR: [u8; 2] = [0x45, 0x44]; // ED

pub trait Payload {
	fn as_slice(&self) -> &[u8] {
		let ptr = self as *const Self as *const u8;
		let size = mem::size_of_val(self);
		let bytes = unsafe { slice::from_raw_parts(ptr, size) };
		bytes
	}

	fn as_bytes(&self) -> Vec<u8> {
		self.as_slice().iter().map(|n| *n).collect()
	}
}
