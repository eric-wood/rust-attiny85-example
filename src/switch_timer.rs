use crate::Timer;

static DEBOUNCE_TIME_MS: u8 = 7;
// Note that this is scaled by 10 so as not to overflow!
static HOLD_TIME_TEN_MS: u8 = 70;

pub struct SwitchTimer {
    pub debounce: Timer,
    pub hold: Timer,
}

impl SwitchTimer {
    pub fn new() -> Self {
        let debounce = Timer::new(DEBOUNCE_TIME_MS, 0);
        let hold = Timer::new(HOLD_TIME_TEN_MS, 10);

        SwitchTimer {
            debounce,
            hold,
        }
    }

    pub fn tick(&mut self) {
        self.debounce.tick();
        self.hold.tick();
    }
}
