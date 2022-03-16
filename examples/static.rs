use lg::*;
use std::time::Duration;

fn main() -> Result<()> {
	let mut ug = UltraGear::new();
	ug.open()?;
	ug.send(&Message::SetStaticColor(1, Color::rgb(0xff, 0x00, 0x00)))?;
	std::thread::sleep(Duration::from_millis(400));

	ug.send(&Message::SetStaticColor(2, Color::rgb(0x00, 0xff, 0x00)))?;
	std::thread::sleep(Duration::from_millis(400));

	ug.send(&Message::SetStaticColor(3, Color::rgb(0x00, 0x00, 0xff)))?;
	std::thread::sleep(Duration::from_millis(400));

	ug.send(&Message::SetStaticColor(4, Color::rgb(0xff, 0x00, 0xff)))?;
	std::thread::sleep(Duration::from_millis(400));

	Ok(())
}
