extern crate attiny85_hal as hal;
extern crate embedded_hal;

use attiny85_hal::pac::TC0;

pub struct Timer {
    timer: TC0,
}

impl Timer {
    pub fn new(timer: TC0) -> Self {
        Timer { timer }
    }
}