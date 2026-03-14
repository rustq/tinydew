#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Season {
    Spring,
    Summer,
    Autumn,
    Winter,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TimePhase {
    Morning,
    Day,
    Evening,
    Night,
}

#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
    pub season: Season,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            day: 1,
            hour: 6,
            minute: 0,
            season: Season::Spring,
        }
    }

    pub fn get_phase(&self) -> TimePhase {
        match self.hour {
            5..=8 => TimePhase::Morning,
            9..=16 => TimePhase::Day,
            17..=20 => TimePhase::Evening,
            _ => TimePhase::Night,
        }
    }

    pub fn advance_minute(&mut self) {
        self.minute += 1;
        if self.minute >= 60 {
            self.minute = 0;
            self.hour += 1;
            if self.hour >= 24 {
                self.hour = 0;
                self.day += 1;
                // Season changes every 28 days
                if (self.day - 1) % 28 == 0 && self.day > 1 {
                    self.season = match self.season {
                        Season::Spring => Season::Summer,
                        Season::Summer => Season::Autumn,
                        Season::Autumn => Season::Winter,
                        Season::Winter => Season::Spring,
                    };
                }
            }
        }
    }
}

pub struct TimeManager {
    pub game_time: GameTime,
    tick_count: u64,
}

impl TimeManager {
    pub fn new() -> Self {
        Self {
            game_time: GameTime::new(),
            tick_count: 0,
        }
    }

    pub fn tick(&mut self) {
        self.tick_count += 1;
        // Advance 1 game minute per tick (2.5s real time)
        self.game_time.advance_minute();
    }
}
