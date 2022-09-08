#![no_std]
#![no_main]

extern crate avr_std_stub;
extern crate definitions;

use definitions::atmega128rfa1::{DDRE, PORTE};
use core::ptr::{read_volatile, write_volatile};

#[no_mangle]
pub extern fn main() {
    unsafe {
        // Set the upper four physical pins on PORT B to inputs, the lower four to outputs.
        // The AVR interprets '1' in the data direction register as 'output', '0' input
        // for the corresponding pin.
        write_volatile(DDRE, read_volatile(DDRE) | 0b11111111);

        // Write the output pins.
        write_volatile(PORTE, 0b01001010);
    }
}