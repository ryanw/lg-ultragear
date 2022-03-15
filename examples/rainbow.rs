use lg::*;
use std::time::Duration;

fn main() -> Result<()> {
	let mut ug = UltraGear::new();
	ug.open()?;

	ug.send(&Message::SetMode(Mode::Video))?;

	let mut colors = vec![Color::rgb(0, 0, 0,); 48];

	let count = 10;
	for i in 0..count {
		let h = i as f32 / count as f32;
		colors[i] = Color::hsl(h, 1.0, 0.5);
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
