use std::time::Duration;

use crate::{payloads, Message, Mode, Payload, HEADER, TERMINATOR};

const DEFAULT_VID: u16 = 0x043e; // LG
const DEFAULT_PID: u16 = 0x9a8a; // UltraGear 38GN950-B
const INTERFACE: u8 = 0x01;
const ENDPOINT: u8 = 0x02;

pub struct UltraGear {
	vid: u16,
	pid: u16,
	handle: Option<rusb::DeviceHandle<rusb::GlobalContext>>,
}

#[derive(Debug)]
pub enum UltraGearError {
	Success,
	Io,
	InvalidParam,
	Access,
	NoDevice,
	NotFound,
	Busy,
	Timeout,
	Overflow,
	Pipe,
	Interrupted,
	NoMem,
	NotSupported,
	BadDescriptor,
	UltraGearDeviceNotFound,
	UltraGearDeviceNotOpen,
	Unknown,
}

pub type Result<T> = std::result::Result<T, UltraGearError>;

impl UltraGear {
	pub fn new() -> Self {
		Self {
			vid: DEFAULT_VID,
			pid: DEFAULT_PID,
			handle: None,
		}
	}

	pub fn open(&mut self) -> Result<()> {
		log::debug!("Fetching list of USB devices");
		let devices = rusb::devices()?;

		let device = devices.iter().find(|d| {
			let device_desc = d.device_descriptor().unwrap();
			device_desc.vendor_id() == self.vid && device_desc.product_id() == self.pid
		});

		if let Some(device) = device {
			let mut handle = device.open()?;
			let _ = handle.detach_kernel_driver(INTERFACE);
			handle.claim_interface(INTERFACE)?;
			self.handle = Some(handle);
		} else {
			return Err(UltraGearError::UltraGearDeviceNotFound);
		}

		Ok(())
	}

	pub fn close(&mut self) {
		self.handle = None;
	}

	pub fn send(&self, message: &Message) -> Result<()> {
		match message {
			Message::SetMode(mode) => {
				self.send_payload(payloads::SetMode::new(*mode))?;
				if *mode == Mode::Video {
					self.send_payload(payloads::SetModeChunkCount::new(*mode, 3))?;
				}
			}
			Message::SetStaticColor(index, color) => {
				self.send_payload(payloads::SetStaticColor::new(*index, *color))?
			}
			Message::SetAudioLEDs(colors) => {
				self.send_payload(payloads::SetAudioLEDs::new(colors))?;
			}
			Message::SetVideoLEDs(colors) => {
				self.send_payload(payloads::SetVideoLEDs::new(colors))?;
			}
		}

		Ok(())
	}

	pub fn send_payload(&self, payload: impl Payload) -> Result<()> {
		self.send_bytes(payload.as_slice())?;

		Ok(())
	}

	fn send_bytes(&self, bytes: &[u8]) -> Result<()> {
		let mut buffer = Vec::with_capacity(bytes.len() + 4);
		buffer.push(HEADER[0]);
		buffer.push(HEADER[1]);
		buffer.extend(bytes);
		buffer.push(TERMINATOR[0]);
		buffer.push(TERMINATOR[1]);

		// Each chunk sent to the device must be no more than 64 bytes
		for chunk in buffer.chunks(64) {
			self.send_chunk(chunk)?;
		}

		Ok(())
	}

	fn send_chunk(&self, bytes: &[u8]) -> Result<()> {
		if let Some(handle) = self.handle.as_ref() {
			handle.write_interrupt(ENDPOINT, bytes, Duration::from_millis(100))?;
		} else {
			return Err(UltraGearError::UltraGearDeviceNotOpen);
		}
		Ok(())
	}
}

impl From<rusb::Error> for UltraGearError {
	fn from(error: rusb::Error) -> Self {
		match error {
			rusb::Error::Io => Self::Io,
			rusb::Error::InvalidParam => Self::InvalidParam,
			rusb::Error::Access => Self::Access,
			rusb::Error::NoDevice => Self::NoDevice,
			rusb::Error::NotFound => Self::NotFound,
			rusb::Error::Busy => Self::Busy,
			rusb::Error::Timeout => Self::Timeout,
			rusb::Error::Overflow => Self::Overflow,
			rusb::Error::Pipe => Self::Pipe,
			rusb::Error::Interrupted => Self::Interrupted,
			rusb::Error::NoMem => Self::NoMem,
			rusb::Error::NotSupported => Self::NotSupported,
			rusb::Error::BadDescriptor => Self::BadDescriptor,
			rusb::Error::Other => Self::Unknown,
		}
	}
}
