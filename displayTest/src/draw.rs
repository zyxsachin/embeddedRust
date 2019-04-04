#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![warn(clippy::all)]

mod clicker;

pub struct Drawer {
    clicker: clicker::Clicker,
}

impl Drawer {
    pub fn new(clicker: clicker::Clicker,) -> Self {
        Drawer {
            Self.clicker = clicker;
        }
    }
}