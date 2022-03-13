pub use lg::*;
use std::time::Duration;

fn main() -> Result<()> {
	let mut ug = UltraGear::new();
	ug.open()?;

	ug.send(&Message::SetMode(Mode::Video))?;

	let mut colors = vec![Color::rgb(0, 0, 0,); 48];
	for i in 0..24 {
		colors[i] = Color::rgb(0xff, 0xff, 0x00);
	}
	for i in 24..48 {
		colors[i] = Color::rgb(0x00, 0x77, 0xff);
	}
	let mut msg = Message::SetVideoLEDs(colors);
	loop {
		match &mut msg {
			Message::SetVideoLEDs(colors) => colors.rotate_right(1),
			_ => {}
		}
		ug.send(&msg)?;
		std::thread::sleep(Duration::from_millis(32));
	}
}
