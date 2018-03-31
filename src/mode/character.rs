//! Character display module

use displayrotation::DisplayRotation;
use command::Command;
use command::Page::{self, Page7};

use hal::blocking::delay::DelayMs;
use hal::digital::OutputPin;
use interface::DisplayInterface;
use properties::DisplayProperties;

use core::fmt;

/// Handling structure for character mode display
pub struct CharacterMode<DI> {
    properties: DisplayProperties<DI>,
}

impl<DI> CharacterMode<DI>
where
    DI: DisplayInterface,
{
    /// Create new CharacterMode instance
    pub fn new(properties: DisplayProperties<DI>) -> Self {
        CharacterMode { properties }
    }

    /// Clear the display buffer. You need to call `disp.flush()` for any effect on the screen
    pub fn clear(&mut self) {
        {
            let iface = self.properties.borrow_iface_mut();

            for _ in 0..8 * 16 {
                let _ = iface.send_data(&[0, 0, 0, 0, 0, 0, 0, 0]);
            }
        }

        // Reset position so we don't end up in some random place of our cleared screen
        let _ = self.set_position(0, 0);
    }

    /// Reset display
    pub fn reset<RST, DELAY>(&mut self, rst: &mut RST, delay: &mut DELAY)
    where
        RST: OutputPin,
        DELAY: DelayMs<u8>,
    {
        rst.set_high();
        delay.delay_ms(1);
        rst.set_low();
        delay.delay_ms(10);
        rst.set_high();
    }

    /// Write out data to display
    pub fn flush(&mut self) -> Result<(), ()> {
        Ok(())
    }

    /// Print characters on the display with the embedded 7x7 font
    pub fn print_chars(&mut self, bytes: &[u8]) -> Result<(), ()> {
        // A 7x7 font shamelessly borrowed from https://github.com/techninja/MarioChron/
        const FONT_7X7: [u8; 672] = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,// (space)
        0x00, 0x00, 0x5F, 0x00, 0x00, 0x00, 0x00,// !
        0x00, 0x07, 0x00, 0x07, 0x00, 0x00, 0x00,// "
        0x14, 0x7F, 0x14, 0x7F, 0x14, 0x00, 0x00,// #
        0x24, 0x2A, 0x7F, 0x2A, 0x12, 0x00, 0x00,// $
        0x23, 0x13, 0x08, 0x64, 0x62, 0x00, 0x00,// %
        0x36, 0x49, 0x55, 0x22, 0x50, 0x00, 0x00,// &
        0x00, 0x05, 0x03, 0x00, 0x00, 0x00, 0x00,// '
        0x00, 0x1C, 0x22, 0x41, 0x00, 0x00, 0x00,// (
        0x00, 0x41, 0x22, 0x1C, 0x00, 0x00, 0x00,// )
        0x08, 0x2A, 0x1C, 0x2A, 0x08, 0x00, 0x00,// *
        0x08, 0x08, 0x3E, 0x08, 0x08, 0x00, 0x00,// +
        0x00, 0x50, 0x30, 0x00, 0x00, 0x00, 0x00,// ,
        0x00, 0x18, 0x18, 0x18, 0x18, 0x18, 0x00,// -
        0x00, 0x60, 0x60, 0x00, 0x00, 0x00, 0x00,// .
        0x20, 0x10, 0x08, 0x04, 0x02, 0x00, 0x00,// /
        0x1C, 0x3E, 0x61, 0x41, 0x43, 0x3E, 0x1C,// 0
        0x40, 0x42, 0x7F, 0x7F, 0x40, 0x40, 0x00,// 1
        0x62, 0x73, 0x79, 0x59, 0x5D, 0x4F, 0x46,// 2
        0x20, 0x61, 0x49, 0x4D, 0x4F, 0x7B, 0x31,// 3
        0x18, 0x1C, 0x16, 0x13, 0x7F, 0x7F, 0x10,// 4
        0x27, 0x67, 0x45, 0x45, 0x45, 0x7D, 0x38,// 5
        0x3C, 0x7E, 0x4B, 0x49, 0x49, 0x79, 0x30,// 6
        0x03, 0x03, 0x71, 0x79, 0x0D, 0x07, 0x03,// 7
        0x36, 0x7F, 0x49, 0x49, 0x49, 0x7F, 0x36,// 8
        0x06, 0x4F, 0x49, 0x49, 0x69, 0x3F, 0x1E,// 9
        0x00, 0x36, 0x36, 0x00, 0x00, 0x00, 0x00,// :
        0x00, 0x56, 0x36, 0x00, 0x00, 0x00, 0x00,// ;
        0x00, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00,// <
        0x14, 0x14, 0x14, 0x14, 0x14, 0x00, 0x00,// =
        0x41, 0x22, 0x14, 0x08, 0x00, 0x00, 0x00,// >
        0x02, 0x01, 0x51, 0x09, 0x06, 0x00, 0x00,// ?
        0x32, 0x49, 0x79, 0x41, 0x3E, 0x00, 0x00,// @
        0x7E, 0x11, 0x11, 0x11, 0x7E, 0x00, 0x00,// A
        0x7F, 0x49, 0x49, 0x49, 0x36, 0x00, 0x00,// B
        0x3E, 0x41, 0x41, 0x41, 0x22, 0x00, 0x00,// C
        0x7F, 0x7F, 0x41, 0x41, 0x63, 0x3E, 0x1C,// D
        0x7F, 0x49, 0x49, 0x49, 0x41, 0x00, 0x00,// E
        0x7F, 0x09, 0x09, 0x01, 0x01, 0x00, 0x00,// F
        0x3E, 0x41, 0x41, 0x51, 0x32, 0x00, 0x00,// G
        0x7F, 0x08, 0x08, 0x08, 0x7F, 0x00, 0x00,// H
        0x00, 0x41, 0x7F, 0x41, 0x00, 0x00, 0x00,// I
        0x20, 0x40, 0x41, 0x3F, 0x01, 0x00, 0x00,// J
        0x7F, 0x08, 0x14, 0x22, 0x41, 0x00, 0x00,// K
        0x7F, 0x7F, 0x40, 0x40, 0x40, 0x40, 0x00,// L
        0x7F, 0x02, 0x04, 0x02, 0x7F, 0x00, 0x00,// M
        0x7F, 0x04, 0x08, 0x10, 0x7F, 0x00, 0x00,// N
        0x3E, 0x7F, 0x41, 0x41, 0x41, 0x7F, 0x3E,// O
        0x7F, 0x09, 0x09, 0x09, 0x06, 0x00, 0x00,// P
        0x3E, 0x41, 0x51, 0x21, 0x5E, 0x00, 0x00,// Q
        0x7F, 0x7F, 0x11, 0x31, 0x79, 0x6F, 0x4E,// R
        0x46, 0x49, 0x49, 0x49, 0x31, 0x00, 0x00,// S
        0x01, 0x01, 0x7F, 0x01, 0x01, 0x00, 0x00,// T
        0x3F, 0x40, 0x40, 0x40, 0x3F, 0x00, 0x00,// U
        0x1F, 0x20, 0x40, 0x20, 0x1F, 0x00, 0x00,// V
        0x7F, 0x7F, 0x38, 0x1C, 0x38, 0x7F, 0x7F,// W
        0x63, 0x14, 0x08, 0x14, 0x63, 0x00, 0x00,// X
        0x03, 0x04, 0x78, 0x04, 0x03, 0x00, 0x00,// Y
        0x61, 0x51, 0x49, 0x45, 0x43, 0x00, 0x00,// Z
        0x00, 0x00, 0x7F, 0x41, 0x41, 0x00, 0x00,// [
        0x02, 0x04, 0x08, 0x10, 0x20, 0x00, 0x00,// "\"
        0x41, 0x41, 0x7F, 0x00, 0x00, 0x00, 0x00,// ]
        0x04, 0x02, 0x01, 0x02, 0x04, 0x00, 0x00,// ^
        0x40, 0x40, 0x40, 0x40, 0x40, 0x00, 0x00,// _
        0x00, 0x01, 0x02, 0x04, 0x00, 0x00, 0x00,// `
        0x20, 0x54, 0x54, 0x54, 0x78, 0x00, 0x00,// a
        0x7F, 0x48, 0x44, 0x44, 0x38, 0x00, 0x00,// b
        0x38, 0x44, 0x44, 0x44, 0x20, 0x00, 0x00,// c
        0x38, 0x44, 0x44, 0x48, 0x7F, 0x00, 0x00,// d
        0x38, 0x54, 0x54, 0x54, 0x18, 0x00, 0x00,// e
        0x08, 0x7E, 0x09, 0x01, 0x02, 0x00, 0x00,// f
        0x08, 0x14, 0x54, 0x54, 0x3C, 0x00, 0x00,// g
        0x7F, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00,// h
        0x00, 0x44, 0x7D, 0x40, 0x00, 0x00, 0x00,// i
        0x20, 0x40, 0x44, 0x3D, 0x00, 0x00, 0x00,// j
        0x00, 0x7F, 0x10, 0x28, 0x44, 0x00, 0x00,// k
        0x00, 0x41, 0x7F, 0x40, 0x00, 0x00, 0x00,// l
        0x7C, 0x04, 0x18, 0x04, 0x78, 0x00, 0x00,// m
        0x7C, 0x08, 0x04, 0x04, 0x78, 0x00, 0x00,// n
        0x38, 0x44, 0x44, 0x44, 0x38, 0x00, 0x00,// o
        0x7C, 0x14, 0x14, 0x14, 0x08, 0x00, 0x00,// p
        0x08, 0x14, 0x14, 0x18, 0x7C, 0x00, 0x00,// q
        0x7C, 0x08, 0x04, 0x04, 0x08, 0x00, 0x00,// r
        0x48, 0x54, 0x54, 0x54, 0x20, 0x00, 0x00,// s
        0x04, 0x3F, 0x44, 0x40, 0x20, 0x00, 0x00,// t
        0x3C, 0x40, 0x40, 0x20, 0x7C, 0x00, 0x00,// u
        0x1C, 0x20, 0x40, 0x20, 0x1C, 0x00, 0x00,// v
        0x3C, 0x40, 0x30, 0x40, 0x3C, 0x00, 0x00,// w
        0x00, 0x44, 0x28, 0x10, 0x28, 0x44, 0x00,// x
        0x0C, 0x50, 0x50, 0x50, 0x3C, 0x00, 0x00,// y
        0x44, 0x64, 0x54, 0x4C, 0x44, 0x00, 0x00,// z
        0x00, 0x08, 0x36, 0x41, 0x00, 0x00, 0x00,// {
        0x00, 0x00, 0x7F, 0x00, 0x00, 0x00, 0x00,// |
        0x00, 0x41, 0x36, 0x08, 0x00, 0x00, 0x00,// }
        0x08, 0x08, 0x2A, 0x1C, 0x08, 0x00, 0x00,// ->
        0x08, 0x1C, 0x2A, 0x08, 0x08, 0x00, 0x00 // <-
    ];
        let iface = self.properties.borrow_iface_mut();

        for c in bytes {
            // Create an array with our byte data instruction and a blank column at the end
            let mut data: [u8; 8] = [0, 0, 0, 0, 0, 0, 0, 0];

            /* Calculate our index into the character table above */
            let index = (*c as usize - 0x20) * 7;

            /* Populate the middle of the array with the data from the character array at the right
             * index */
            data[0..7].copy_from_slice(&FONT_7X7[index..index + 7]);

            /* Write it out to the I2C bus */
            iface.send_data(&data)?
        }

        Ok(())
    }

    /// Display is set up in column mode, i.e. a byte walks down a column of 8 pixels from
    /// column 0 on the left, to column _n_ on the right
    pub fn init(&mut self) -> Result<(), DI::Error> {
        self.properties.init_column_mode()?;
        Ok(())
    }

    /// Get display dimensions, taking into account the current rotation of the display
    pub fn get_dimensions(&self) -> (u8, u8) {
        self.properties.get_dimensions()
    }

    /// Set the display rotation
    pub fn set_rotation(&mut self, rot: DisplayRotation) -> Result<(), ()> {
        self.properties.set_rotation(rot)
    }

    /// Position cursor at specified x, y character coordinate (multiple of 8)
    pub fn set_position(&mut self, x: u8, y: u8) -> Result<(), ()> {
        let iface = self.properties.borrow_iface_mut();

        // FIXME: Should be width
        Command::ColumnAddress(x * 8, 0x7f).send(iface)?;
        Command::PageAddress(Page::from(y), Page7).send(iface)
    }
}

impl<DI> fmt::Write for CharacterMode<DI>
where
    DI: DisplayInterface,
{
    fn write_str(&mut self, s: &str) -> Result<(), fmt::Error> {
        self.print_chars(s.as_bytes()).map_err(|_| fmt::Error)
    }
}

#[cfg(test)]
mod tests {
    // TODO lol
}
