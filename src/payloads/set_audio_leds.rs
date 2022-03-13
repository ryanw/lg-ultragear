use crate::{Color, Payload};

const SET_AUDIO_LEDS: u8 = 0xc2;

#[allow(dead_code)]
#[repr(packed)]
pub struct SetAudioLEDs {
	message: u8,
	unknown1: u16,
	unknown2: u8,
	colors: [Color; 48],
	unknown3: u8,
}

impl SetAudioLEDs {
	pub fn new(colors_vec: &[Color]) -> Self {
		let mut colors = [Color::rgb(0x00, 0x00, 0x00); 48];
		for (i, color) in colors_vec.iter().enumerate() {
			if i >= colors.len() {
				break;
			}

			colors[i] = color.clone();
		}

		Self {
			message: SET_AUDIO_LEDS,
			unknown1: 0x9102,
			unknown2: 0x00,
			colors,
			unknown3: 0x41, // A
		}
	}
}

impl Payload for SetAudioLEDs {}
