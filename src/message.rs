use crate::{Color, Mode};

pub enum Message {
	SetMode(Mode),
	SetStaticColor(u8, Color),
	SetAudioLEDs(Vec<Color>),
	SetVideoLEDs(Vec<Color>),
}
