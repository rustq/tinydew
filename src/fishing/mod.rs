use rand::Rng;

#[derive(Debug, Clone, Copy)]
pub struct FishingResult {
    pub success: bool,
    pub fish: Option<FishType>,
    pub message: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FishType {
    RiverFish,
    BrightTrout,
    Salmon,
    Catfish,
    RareGoldenFish,
}

impl FishType {
    pub fn rarity(&self) -> Rarity {
        match self {
            FishType::RiverFish => Rarity::Common,
            FishType::BrightTrout => Rarity::Uncommon,
            FishType::Salmon => Rarity::Uncommon,
            FishType::Catfish => Rarity::Rare,
            FishType::RareGoldenFish => Rarity::Legendary,
        }
    }

    pub fn base_value(&self) -> u32 {
        match self {
            FishType::RiverFish => 10,
            FishType::BrightTrout => 25,
            FishType::Salmon => 30,
            FishType::Catfish => 50,
            FishType::RareGoldenFish => 100,
        }
    }

    pub fn to_emoji(&self) -> &'static str {
        match self {
            FishType::RiverFish => "🐟",
            FishType::BrightTrout => "🐠",
            FishType::Salmon => "🐟",
            FishType::Catfish => "🐱",
            FishType::RareGoldenFish => "✨",
        }
    }

    pub fn name(&self) -> &'static str {
        match self {
            FishType::RiverFish => "River Fish",
            FishType::BrightTrout => "Bright Trout",
            FishType::Salmon => "Salmon",
            FishType::Catfish => "Catfish",
            FishType::RareGoldenFish => "Rare Golden Fish",
        }
    }

    pub fn available_seasons(&self) -> &[crate::time::Season] {
        match self {
            FishType::RiverFish => &[
                crate::time::Season::Spring,
                crate::time::Season::Summer,
                crate::time::Season::Autumn,
            ],
            FishType::BrightTrout => &[
                crate::time::Season::Spring,
                crate::time::Season::Summer,
            ],
            FishType::Salmon => &[
                crate::time::Season::Autumn,
            ],
            FishType::Catfish => &[
                crate::time::Season::Summer,
                crate::time::Season::Autumn,
            ],
            FishType::RareGoldenFish => &[
                crate::time::Season::Spring,
                crate::time::Season::Summer,
                crate::time::Season::Autumn,
                crate::time::Season::Winter,
            ],
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rarity {
    Common,
    Uncommon,
    Rare,
    Legendary,
}

impl Rarity {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            Rarity::Common => "",
            Rarity::Uncommon => "⭐",
            Rarity::Rare => "⭐⭐",
            Rarity::Legendary => "⭐⭐⭐",
        }
    }
}

pub fn try_fish(season: crate::time::Season, weather: crate::time::Weather, time_phase: crate::time::TimePhase) -> FishingResult {
    let mut rng = rand::thread_rng();
    
    // Calculate catch chance based on conditions
    let base_chance = 0.6; // 60% base chance
    let season_modifier = season_catch_modifier(season);
    let weather_modifier = weather_catch_modifier(weather);
    let time_modifier = time_catch_modifier(time_phase);
    
    let total_chance = base_chance * season_modifier * weather_modifier * time_modifier;
    let catch_roll: f64 = rng.gen_range(0.0..1.0);
    
    if catch_roll > total_chance {
        return FishingResult {
            success: false,
            fish: None,
            message: "🎣 Nothing bit.".to_string(),
        };
    }
    
    // Select fish from catch table
    let fish = select_fish(weather);
    
    let rarity_emoji = fish.rarity().to_emoji();
    let message = format!("🎣 You caught a {} {}{}!", rarity_emoji, fish.name(), fish.to_emoji());
    
    FishingResult {
        success: true,
        fish: Some(fish),
        message,
    }
}

fn season_catch_modifier(season: crate::time::Season) -> f64 {
    match season {
        crate::time::Season::Spring => 1.0,
        crate::time::Season::Summer => 1.2,  // More active in summer
        crate::time::Season::Autumn => 1.0,
        crate::time::Season::Winter => 0.5,  // Less active in winter
    }
}

fn weather_catch_modifier(weather: crate::time::Weather) -> f64 {
    match weather {
        crate::time::Weather::Sunny => 1.0,
        crate::time::Weather::Cloudy => 1.1,
        crate::time::Weather::Rainy => 1.3,  // Better rates in rain
        crate::time::Weather::Stormy => 1.5,  // Even better in storms
        crate::time::Weather::Snowy => 0.3,  // Very poor in snow
    }
}

fn time_catch_modifier(time_phase: crate::time::TimePhase) -> f64 {
    match time_phase {
        crate::time::TimePhase::Morning => 1.3,  // Early morning is good
        crate::time::TimePhase::Day => 1.0,
        crate::time::TimePhase::Evening => 1.2,  // Late evening is good
        crate::time::TimePhase::Night => 0.8,  // Night is harder
    }
}

fn select_fish(weather: crate::time::Weather) -> FishType {
    let mut rng = rand::thread_rng();
    let n: u32 = rng.gen_range(0..100);
    
    // Base rarity distribution
    let (common_threshold, uncommon_threshold, rare_threshold) = match weather {
        crate::time::Weather::Stormy => (40, 75, 95),  // Better rare rates in storms
        crate::time::Weather::Rainy => (50, 80, 95),
        _ => (60, 85, 98),
    };
    
    if n < common_threshold {
        FishType::RiverFish
    } else if n < uncommon_threshold {
        if rng.gen_bool(0.5) {
            FishType::BrightTrout
        } else {
            FishType::Salmon
        }
    } else if n < rare_threshold {
        FishType::Catfish
    } else {
        FishType::RareGoldenFish
    }
}
