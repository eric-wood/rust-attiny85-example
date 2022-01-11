use crate::Timer;

static DEBOUNCE_TIME_MS: u8 = 7;
// Note that this is scaled by 10 so as not to overflow!
static HOLD_TIME_TEN_MS: u8 = 70;

pub struct SwitchTimer {
    debounce_timer: Timer,
    hold_timer: Timer,
}

impl SwitchTimer {
    pub fn new() -> Self {
        let debounce_timer = Timer::new(DEBOUNCE_TIME_MS, 0);
        let hold_timer = Timer::new(HOLD_TIME_TEN_MS, 10);

        SwitchTimer {
            debounce_timer,
            hold_timer,
        }
    }

    pub fn debounce_reset(&mut self) {
        self.debounce_timer.reset();
    }

    pub fn hold_reset(&mut self) {
        self.debounce_timer.reset();
    }

    pub fn debounce_threshold_reached(&mut self) -> bool {
        self.debounce_timer.threshold_reached
    }

    pub fn hold_threshold_reached(&mut self) -> bool {
        self.debounce_timer.threshold_reached
    }

    pub fn tick(&mut self) {
        self.debounce_timer.tick();
        self.hold_timer.tick();
    }
}
