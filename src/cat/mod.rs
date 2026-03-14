use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct Cat {
    pub x: usize,
    pub y: usize,
    pub mood: CatMood,
    pub last_petted_day: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CatMood {
    Happy,
    Sleepy,
    Hungry,
    Playful,
    Wet,
    Cold,
}

impl Cat {
    pub fn new(x: usize, y: usize) -> Self {
        Self {
            x,
            y,
            mood: CatMood::Happy,
            last_petted_day: None,
        }
    }

    pub fn update_mood(&mut self, weather: crate::time::Weather, time_phase: crate::time::TimePhase) {
        let mut rng = rand::thread_rng();
        
        self.mood = match weather {
            crate::time::Weather::Rainy => CatMood::Wet,
            crate::time::Weather::Snowy => CatMood::Cold,
            _ => match time_phase {
                crate::time::TimePhase::Night => CatMood::Sleepy,
                _ => if rng.gen_bool(0.3) {
                    CatMood::Playful
                } else {
                    CatMood::Happy
                }
            }
        };
    }

    pub fn can_pet(&self, current_day: u32) -> bool {
        if let Some(last_day) = self.last_petted_day {
            last_day != current_day
        } else {
            true
        }
    }

    pub fn pet(&mut self, current_day: u32) -> bool {
        if self.can_pet(current_day) {
            self.last_petted_day = Some(current_day);
            true
        } else {
            false
        }
    }

    pub fn get_reaction(&self, weather: crate::time::Weather, time_phase: crate::time::TimePhase) -> &'static str {
        match (weather, time_phase, self.mood) {
            (crate::time::Weather::Rainy, _, CatMood::Wet) => {
                "The cat shakes off water and looks at you. 🐱💧"
            }
            (crate::time::Weather::Snowy, _, CatMood::Cold) => {
                "The cat curls up tight to stay warm. 🐱❄️"
            }
            (_, crate::time::TimePhase::Night, CatMood::Sleepy) => {
                "The cat is sleeping... shh! 😴"
            }
            (_, _, CatMood::Playful) => {
                "The cat bats at your hand playfully! 🐾"
            }
            (_, _, CatMood::Happy) => {
                "The cat purrs contentedly. 😺"
            }
            _ => {
                "The cat looks at you curiously. 🐱"
            }
        }
    }

    pub fn to_emoji(&self) -> &'static str {
        match self.mood {
            CatMood::Happy => "😺",
            CatMood::Sleepy => "😴",
            CatMood::Hungry => "😿",
            CatMood::Playful => "😸",
            CatMood::Wet => "🐱💧",
            CatMood::Cold => "🐱❄️",
        }
    }

    pub fn move_randomly(&mut self, width: usize, height: usize, player_x: usize, player_y: usize) {
        let mut rng = rand::thread_rng();
        
        let towards_player = rng.gen_bool(0.4);
        
        let (dx, dy) = if towards_player {
            let dx = if player_x > self.x { 1 } else if player_x < self.x { -1 } else { 0 };
            let dy = if player_y > self.y { 1 } else if player_y < self.y { -1 } else { 0 };
            (dx, dy)
        } else {
            let moves = [(0, 1), (0, -1), (1, 0), (-1, 0), (0, 0)];
            moves[rng.gen_range(0..moves.len())]
        };
        
        let new_x = (self.x as i32 + dx).clamp(0, (width - 1) as i32) as usize;
        let new_y = (self.y as i32 + dy).clamp(0, (height - 1) as i32) as usize;
        
        self.x = new_x;
        self.y = new_y;
    }
}
