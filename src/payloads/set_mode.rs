use crate::Payload;

const SET_MODE: u8 = 0xc7;

#[repr(u8)]
#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
	Static1 = 1,
	Static2,
	Static3,
	Static4,
	Peaceful,
	Dynamic,
	Audio,
	Video,
}

#[allow(dead_code)]
#[repr(packed)]
pub struct SetMode {
	message: u8,
	unknown1: u16,
	unknown2: u8,
	index: Mode,
	index_check: u8,
}

impl SetMode {
	pub fn new(mode: Mode) -> Self {
		Self {
			message: SET_MODE,
			unknown1: 0x0202,
			unknown2: 0x00,
			index: mode,
			index_check: mode as u8 ^ 0xd7,
		}
	}
}

impl Payload for SetMode {}
