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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Weather {
    Sunny,
    Cloudy,
    Rainy,
    Stormy,
    Snowy,
}

impl Weather {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            Weather::Sunny => "☀️",
            Weather::Cloudy => "☁️",
            Weather::Rainy => "🌧",
            Weather::Stormy => "⛈",
            Weather::Snowy => "❄️",
        }
    }

    pub fn is_rainy(&self) -> bool {
        matches!(self, Weather::Rainy | Weather::Stormy)
    }

    pub fn is_snowy(&self) -> bool {
        matches!(self, Weather::Snowy)
    }

    pub fn affects_farming(&self) -> bool {
        !matches!(self, Weather::Snowy)
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GameTime {
    pub day: u32,
    pub hour: u8,
    pub minute: u8,
    pub season: Season,
    pub weather: Weather,
}

impl GameTime {
    pub fn new() -> Self {
        Self {
            day: 1,
            hour: 6,
            minute: 0,
            season: Season::Spring,
            weather: Weather::Sunny,
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
                // Roll new weather at start of each day
                self.weather = Self::roll_weather(self.season);
            }
        }
    }

    fn roll_weather(season: Season) -> Weather {
        use rand::Rng;
        let mut rng = rand::thread_rng();
        let n: u32 = rng.gen_range(0..100);

        match season {
            Season::Spring => {
                if n < 50 { Weather::Sunny }
                else if n < 80 { Weather::Cloudy }
                else { Weather::Rainy }
            }
            Season::Summer => {
                if n < 60 { Weather::Sunny }
                else if n < 90 { Weather::Cloudy }
                else { Weather::Rainy }
            }
            Season::Autumn => {
                if n < 30 { Weather::Sunny }
                else if n < 70 { Weather::Cloudy }
                else { Weather::Rainy }
            }
            Season::Winter => {
                if n < 40 { Weather::Sunny }
                else if n < 80 { Weather::Cloudy }
                else { Weather::Snowy }
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
