use lg::*;
use std::time::Duration;

fn main() -> Result<()> {
	let mut ug = UltraGear::new();
	ug.open()?;

	ug.send(&Message::SetMode(Mode::Static1))?;
	std::thread::sleep(Duration::from_millis(1000));

	ug.send(&Message::SetMode(Mode::Static2))?;
	std::thread::sleep(Duration::from_millis(1000));

	ug.send(&Message::SetMode(Mode::Static3))?;
	std::thread::sleep(Duration::from_millis(1000));

	ug.send(&Message::SetMode(Mode::Static4))?;
	std::thread::sleep(Duration::from_millis(1000));

	ug.send(&Message::SetMode(Mode::Peaceful))?;
	std::thread::sleep(Duration::from_millis(1000));

	ug.send(&Message::SetMode(Mode::Dynamic))?;
	std::thread::sleep(Duration::from_millis(1000));

	Ok(())
}
