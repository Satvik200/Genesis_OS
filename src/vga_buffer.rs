use core::fmt::{Write, Result, Arguments};
use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]

pub enum Colour {
	Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]


struct ColourCode(u8);
impl ColourCode {
	fn new(foreground: Colour, background: Colour) -> ColourCode {
		ColourCode((background as u8) << 4 | (foreground as u8))
	}
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]


struct ScreenCharacter {
	ascii_char: u8,
	colour_code: ColourCode,
}

const BUFFER_WIDTH: usize = 80;
const BUFFER_HEIGHT: usize = 25;

#[repr(transparent)]

struct Buffer {
	chars: [[Volatile<ScreenCharacter>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct ScreenWriter {
	column_pos: usize,
	colour_code: ColourCode,
	buffer: &'static mut Buffer,	
}


lazy_static! {
	pub static ref SCREENWRITER: Mutex<ScreenWriter> = Mutex::new(ScreenWriter {
		column_pos:0,
		colour_code:ColourCode::new(Colour::Yellow, Colour::Black),
		buffer: unsafe {&mut *(0xb8000 as *mut Buffer)},
	});
}
 

impl ScreenWriter {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column_pos >= BUFFER_WIDTH {
					self.new_line();
				}

				let row = BUFFER_HEIGHT - 1;
				let col = self.column_pos;

				let colour_code = self.colour_code;
				self.buffer.chars[row][col].write(ScreenCharacter {
					ascii_char: byte, 
					colour_code,
				});
				self.column_pos += 1;
			}
		}
	}

	pub fn write_string(&mut self, s: &str) {
		for byte in s.bytes() {
			match byte {
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
	
	pub fn new_line(&mut self) {
		for row in 1..BUFFER_HEIGHT {
			for col in 0..BUFFER_WIDTH {
				let character = self.buffer.chars[row][col].read();
				self.buffer.chars[row - 1][col].write(character);
			}
		}
		self.clear_row(BUFFER_HEIGHT - 1);
		self.column_pos = 0;
	}

	pub fn clear_row(&mut self, row: usize) {
		let blank = ScreenCharacter {
			ascii_char: b' ',
			colour_code: self.colour_code,
		};
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}

}

impl Write for ScreenWriter {
	fn write_str(&mut self, s: &str) -> Result {
		self.write_string(s);
		Ok(())
	}
}


// pub fn print_on_Display() {
// 	let mut screenWriter = ScreenWriter {
// 		column_pos: 0,
// 		colour_code: ColourCode::new(Colour::Yellow, Colour::Black),
// 		buffer: unsafe {
// 			&mut *(0xb8000 as *mut Buffer)
// 		},
// 	};

// 	screenWriter.write_byte(b'H');
// 	screenWriter.write_string("ello I am Jarvis XD ");
// 	write!(screenWriter, " Created with love Shreesh, Samarth & Satvik").unwrap();
// }



#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::vga_buffer::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: Arguments) {
	use core::fmt::Write;
	use x86_64::instructions::interrupts;

	interrupts::without_interrupts(|| {
		SCREENWRITER
			.lock()
			.write_fmt(args)
			.unwrap();
	})
}

#[test_case]
fn test_println_simple() {
	println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
	for _ in 0..200 {
		println!("test_println_many output");
	}
}

#[test_case]
fn test_println_output() {
	// use core::fmt::Write;
	// use x86_64::instructions::interrupts;

	// let s = "Some test string that fits in a single line";
	// interrupts::without_interrupts(|| {
	// 	let mut writer = SCREENWRITER.lock();
	// 	writeln!(writer, "\n{}", s).expect("writeln failed");
	// 	for (i, c) in s.chars().enumerate() {
	// 		let screen_char = SCREENWRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
	// 		assert_eq!(char::from(screen_char.ascii_char), c);
	// 	}
	// })
	// 
	// let s = "Some test string that fits in a single line";
	// println!("{}", s);
	// for (i, c) in s.chars().enumerate() {
	// 	let screen_char = SCREENWRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
	// 	assert_eq!(char::from(screen_char.ascii_char), c);
	// }
	// 
	use core::fmt::Write;
    use x86_64::instructions::interrupts;
	let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = SCREENWRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_char), c);
        }
    });
	
}

