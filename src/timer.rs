extern crate attiny85_hal as hal;
extern crate embedded_hal;

pub struct Timer {
    ticks: u8,
    stopped: bool,
}

impl Timer {
    pub fn new() -> Self {
        Timer {
            ticks: 0,
            stopped: true,
        }
    }

    pub fn start(&mut self) {
        self.stopped = false;
    }

    pub fn stop(&mut self) {
        self.stopped = true;
    }

    pub fn reset(&mut self) {
        self.ticks = 0;
    }

    pub fn elapsed_time_ms(&self) -> u8 {
        self.ticks * 16
    }

    pub fn tick(&mut self) {
        if self.stopped {
            return;
        }

        self.ticks += 1;
    }
}
