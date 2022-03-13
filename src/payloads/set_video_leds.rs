use crate::{Color, Payload};

const SET_VIDEO_LEDS: u8 = 0xc1;

#[allow(dead_code)]
#[repr(packed)]
pub struct SetVideoLEDs {
	message: u8,
	unknown1: u16,
	unknown2: u8,
	colors: [Color; 48],
	unknown3: u8,
}

impl SetVideoLEDs {
	pub fn new(colors_vec: &[Color]) -> Self {
		let mut colors = [Color::rgb(0x00, 0x00, 0x00); 48];
		for (i, color) in colors_vec.iter().enumerate() {
			if i >= colors.len() {
				break;
			}

			colors[i] = color.clone();
		}

		Self {
			message: SET_VIDEO_LEDS,
			unknown1: 0x9102,
			unknown2: 0x00,
			colors,
			unknown3: 0x42, // B
		}
	}
}

impl Payload for SetVideoLEDs {}
