#![no_std]

pub use bcm2711_lpa as pac;

pub mod gpio;

mod critical_section;
mod entry;
