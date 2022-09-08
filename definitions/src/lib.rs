#![no_std]
#![feature(lang_items)]

extern crate avr_std_stub;

#[cfg(target_arch = "avr")]
pub mod atmega128rfa1;