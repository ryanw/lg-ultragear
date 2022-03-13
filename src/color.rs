#[repr(packed)]
#[derive(Copy, Clone)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
}

impl Color {
	pub fn rgb(r: u8, g: u8, b: u8) -> Self {
		Color { r, g, b }
	}

	pub fn hsl(h: f32, s: f32, l: f32) -> Self {
		if s == 0.0 {
			let g = (l * 255.0) as u8;
			return Color::rgb(g, g, g);
		}

		let q = if l < 0.5 {
			l * (1.0 + s)
		} else {
			l + s - l * s
		};
		let p = 2.0 * l - q;

		let r = (hue_to_rgb(p, q, h + 1.0 / 3.0) * 255.0) as u8;
		let g = (hue_to_rgb(p, q, h) * 255.0) as u8;
		let b = (hue_to_rgb(p, q, h - 1.0 / 3.0) * 255.0) as u8;

		Color::rgb(r, g, b)
	}

	pub fn checksum(&self) -> u8 {
		self.r ^ self.g ^ self.b ^ 0xc5
	}
}

fn hue_to_rgb(p: f32, q: f32, mut t: f32) -> f32 {
	if t < 0.0 {
		t += 1.0;
	}
	if t > 1.0 {
		t -= 1.0;
	}

	if t < 1.0 / 6.0 {
		return p + (q - p) * 6.0 * t;
	}

	if t < 1.0 / 2.0 {
		return q;
	}

	if t < 2.0 / 3.0 {
		return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
	}

	return p;
}
