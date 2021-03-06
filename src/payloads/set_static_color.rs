use crate::{Color, Payload};

const SET_STATIC_COLOR: u8 = 0xd2;

#[allow(dead_code)]
#[repr(packed)]
pub struct SetStaticColor {
	message: u8,
	index_check: u16,
	index: u8,
	color: Color,
	color_checksum: u8,
}

impl SetStaticColor {
	pub fn new(index: u8, color: Color) -> Self {
		Self {
			message: SET_STATIC_COLOR,
			index_check: index as u16 ^ 0x0403,
			index,
			color_checksum: color.checksum(),
			color,
		}
	}
}

impl Payload for SetStaticColor {}
