use lg::*;

fn main() -> Result<()> {
	let mut ug = UltraGear::new();
	ug.open()?;

	let num = std::env::args()
		.nth(1)
		.map(|n| n.parse::<u8>().unwrap_or(0))
		.unwrap_or(0);

	let color = std::env::args()
		.nth(2)
		.map(|ref n| u32::from_str_radix(n, 16).unwrap_or(0))
		.unwrap_or(0);
	let pure = std::env::args().nth(3) == Some("pure".into());

	if pure {
		let r: f32 = ((color & 0x00ff0000) >> 16) as f32 / 255.0;
		let g: f32 = ((color & 0x0000ff00) >> 8) as f32 / 255.0;
		let b: f32 = ((color & 0x000000ff) >> 0) as f32 / 255.0;

		let (h, _s, _l) = rgb_to_hsl(r, g, b);

		ug.send(&Message::SetStaticColor(num, Color::hsl(h, 1.0, 0.5)))?;
	} else {
		let r: u8 = ((color & 0x00ff0000) >> 16) as u8;
		let g: u8 = ((color & 0x0000ff00) >> 8) as u8;
		let b: u8 = ((color & 0x000000ff) >> 0) as u8;
		ug.send(&Message::SetStaticColor(num, Color::rgb(r, g, b)))?;
	}
	Ok(())
}

fn rgb_to_hsl(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
	let max = r.max(g).max(b);
	let min = r.min(g).min(b);

	let mut h = 0.0;
	let mut s = 0.0;
	let l = (max + min) / 2.0;

	if max != min {
		let d = max - min;
		s = if l > 0.5 {
			d / (2.0 - max - min)
		} else {
			d / (max + min)
		};

		h = {
			if r == max {
				(g - b) / d + (if g < b { 6.0 } else { 0.0 })
			} else if g == max {
				(b - r) / d + 2.0
			} else {
				(r - g) / d + 4.0
			}
		};
		h = h / 6.0;
	}

	return (h, s, l);
}
