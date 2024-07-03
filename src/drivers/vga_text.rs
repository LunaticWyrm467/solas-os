//===================================================================================================================================================================================//
//
//  /$$    /$$  /$$$$$$   /$$$$$$        /$$$$$$$$                    /$$           /$$      /$$                 /$$          
// | $$   | $$ /$$__  $$ /$$__  $$      |__  $$__/                   | $$          | $$$    /$$$                | $$          
// | $$   | $$| $$  \__/| $$  \ $$         | $$  /$$$$$$  /$$   /$$ /$$$$$$        | $$$$  /$$$$  /$$$$$$   /$$$$$$$  /$$$$$$ 
// |  $$ / $$/| $$ /$$$$| $$$$$$$$         | $$ /$$__  $$|  $$ /$$/|_  $$_/        | $$ $$/$$ $$ /$$__  $$ /$$__  $$ /$$__  $$
//  \  $$ $$/ | $$|_  $$| $$__  $$         | $$| $$$$$$$$ \  $$$$/   | $$          | $$  $$$| $$| $$  \ $$| $$  | $$| $$$$$$$$
//   \  $$$/  | $$  \ $$| $$  | $$         | $$| $$_____/  >$$  $$   | $$ /$$      | $$\  $ | $$| $$  | $$| $$  | $$| $$_____/
//    \  $/   |  $$$$$$/| $$  | $$         | $$|  $$$$$$$ /$$/\  $$  |  $$$$/      | $$ \/  | $$|  $$$$$$/|  $$$$$$$|  $$$$$$$
//     \_/     \______/ |__/  |__/         |__/ \_______/|__/  \__/   \___/        |__/     |__/ \______/  \_______/ \_______/
//
//===================================================================================================================================================================================//

//?
//? Created by LunaticWyrm467
//?

//!
//! A driver used to render text via the VGA text mode.
//!

use core::fmt::{ self, Write };

use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;


/*
 * Constant & Static
 *      Declarations
 */


/// The VGA buffer's height.
const BUFFER_HEIGHT: usize = 25;

/// The VGA buffer's width.
const BUFFER_WIDTH: usize = 80;

/// The pointer to the VGA buffer which encompasses it safetly.
const VGA_BUFFER: *mut VGABuffer = 0xb8000 as *mut VGABuffer;

lazy_static! {
    
    /// A global static reference to the VGA text mode drivers.
    pub static ref WRITER: Mutex<VGADriver> = Mutex::new(VGADriver::new(VGAColourDesc::new(VGAColourFull::White, VGAColour::Black, false)));
}


/*
 * VGA Colour
 *      Enums
 */


/// Encapsulates the VGA character raw colours.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VGAColour {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
}

impl VGAColour {
    
    /// Converts the enum into a 4-bit colour channel.
    pub fn to_bits(&self) -> u8 {
        match self {
            Self::Black      => 0x0,
            Self::Blue       => 0x1,
            Self::Green      => 0x2,
            Self::Cyan       => 0x3,
            Self::Red        => 0x4,
            Self::Magenta    => 0x5,
            Self::Brown      => 0x6,
            Self::LightGrey  => 0x7,
        }
    }
}

/// Encapsulates the VGA character full colour range.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VGAColourFull {
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Magenta,
    Brown,
    LightGrey,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yellow,
    White
}

impl VGAColourFull {
    
    /// Converts the VGAColourFull type into its raw colour and whether then colour is the light
    /// variant of the raw colour or not.
    pub fn to_raw(&self) -> (VGAColour, bool) {
        match self {

            // Normal
            Self::Black      => (VGAColour::Black,     false),
            Self::Blue       => (VGAColour::Blue,      false),
            Self::Green      => (VGAColour::Green,     false),
            Self::Cyan       => (VGAColour::Cyan,      false),
            Self::Red        => (VGAColour::Red,       false),
            Self::Magenta    => (VGAColour::Magenta,   false),
            Self::Brown      => (VGAColour::Brown,     false),
            Self::LightGrey  => (VGAColour::LightGrey, false),

            // Light
            Self::DarkGray    => (VGAColour::Black,     true),
            Self::LightBlue   => (VGAColour::Blue,      true),
            Self::LightGreen  => (VGAColour::Green,     true),
            Self::LightCyan   => (VGAColour::Cyan,      true),
            Self::LightRed    => (VGAColour::Red,       true),
            Self::Pink        => (VGAColour::Magenta,   true),
            Self::Yellow      => (VGAColour::Brown,     true),
            Self::White       => (VGAColour::LightGrey, true)
        }
    }
}

/// Encapsulates a full 8-bit VGA colour parameter for both the foreground and the background, as well as
/// the blink parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct VGAColourDesc(u8);

impl VGAColourDesc {
    
    /// Creates this colour descriptor from a VGA colour parameter via the following parameters:
    /// - Foreground Colour,
    /// - Background Colour,
    /// - The blink status on the character.
    pub fn new(foreground: VGAColourFull, background: VGAColour, blink: bool) -> Self {
        
        /// Appends a 4th bit for a 3-bit value.
        fn comb4(bit3: u8, last_bit: bool) -> u8 {
            let last_bit_mask: u8 = 1 << 3; // Create a mask with the fourth bit set (0b00001000)
            if last_bit {
                bit3 | last_bit_mask
            } else {
                bit3 & !last_bit_mask
            }
        }
        
        let (foreground, light): (VGAColour, bool) = foreground.to_raw();

        let foreground_4b: u8 = comb4(foreground.to_bits(), light);
        let background_4b: u8 = comb4(background.to_bits(), blink);
        
        VGAColourDesc((background_4b as u8) << 4 | (foreground_4b as u8))
    }
}


/*
 * VGA Text
 *      Buffer
 */


/// Describes a character on the screen.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct VGAChar {
    char: u8,
    desc: VGAColourDesc
}

/// Encapsulates the VGA text buffer region in memory and allows safe access to it.
/// # Note
/// This internally uses the Volatile wrapper to ensure that the rust compiler doesn't optimize the
/// writes to this encapsulated memory region, which could result in corruption.
/// Writing to this memory region has side-effects, but the rust optimizer doesn't know this.
#[derive(Debug)]
#[repr(transparent)]
struct VGABuffer {
    chars: [[Volatile<VGAChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

/// A full VGA driver that encapsulates the VGA text buffer region in memory and allows for the
/// safe utilization of said display feature for printing text.
pub struct VGADriver {
    column_position: usize,
    colour_desc:     VGAColourDesc,
    buffer:          &'static mut VGABuffer
}

impl VGADriver {

    /// Creates a new VGA driver to access the VGA buffer.
    pub fn new(desc: VGAColourDesc) -> Self {
        VGADriver {
            column_position: 0,
            colour_desc:     desc,
            buffer:          unsafe { &mut *VGA_BUFFER }
        }
    }
    
    /// Writes a single byte onto the VGA buffer.
    pub fn write_byte(&mut self, byte: u8) -> () {
        match byte {
            b'\n' => self.new_line(),
            _     => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row:  usize         = BUFFER_HEIGHT - 1;
                let col:  usize         = self.column_position;
                let desc: VGAColourDesc = self.colour_desc;

                self.buffer.chars[row][col].write(VGAChar {
                    char: byte,
                    desc
                });
                self.column_position += 1;
            }
        }
    }

    /// Clears the whole buffer.
    pub fn clear(&mut self) -> () {
        for row in 0..BUFFER_HEIGHT {
            self.clear_row(row);
        }
    }

    /// Adds a new line to the buffer, scrolling the text up by one.
    fn new_line(&mut self) -> () {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                    let char: VGAChar = self.buffer.chars[row][col].read();
                    self.buffer.chars[row - 1][col].write(char);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    /// Clears a row on the buffer.
    fn clear_row(&mut self, row: usize) -> () {
        let blank: VGAChar = VGAChar {
            char: b' ',
            desc: self.colour_desc
        };

        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for VGADriver {

    /// Writes a whole string onto the VGA buffer.
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),   // Printable and supported VGA character.
                _                   => self.write_byte(0xfe)    // Unsupported character outside of the VGA range.
            }
        }
        Ok(())
    }
}


/*
 * Print Macro
 *      Support
 */


#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::drivers::vga_text::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    ()            => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

/// A global print function.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) -> () {
    WRITER.lock().write_fmt(args).unwrap();
}


/*
 * VGA Driver
 *      Tests
 */


#[test_case]
fn test_println_simple() -> () {
    println!("Hello, World!");
}

#[test_case]
fn test_println_many() -> () {
    for _ in 0..200 {
        println!("Hello, World");
    }
}

#[test_case]
fn test_println_output() -> () {
    let s: &str = "Hello I fit on a single line.";
    println!("{s}");
    for (i, c) in s.chars().enumerate() {
        let vga_char: VGAChar = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(vga_char.char), c);
    }
}
