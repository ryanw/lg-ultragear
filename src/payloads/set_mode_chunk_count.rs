use crate::{Mode, Payload};

const SET_MODE_CHUNK_COUNT: u8 = 0xca;

#[allow(dead_code)]
#[repr(packed)]
pub struct SetModeChunkCount {
	message: u8,
	unknown1: u16,
	count: u8, // ?? Not sure
	index: Mode,
	index_check: u8,
}

impl SetModeChunkCount {
	pub fn new(mode: Mode, count: u8) -> Self {
		Self {
			message: SET_MODE_CHUNK_COUNT,
			unknown1: 0x0202,
			count,
			index: mode,
			index_check: mode as u8 ^ 0xd9,
		}
	}
}

impl Payload for SetModeChunkCount {}
