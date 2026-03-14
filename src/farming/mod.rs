#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CropType {
    Strawberry,
    Corn,
    Tomato,
    Pumpkin,
    Carrot,
    Eggplant,
    Blueberry,
}

impl CropType {
    /// Returns the seasons in which this crop can be planted
    pub fn available_seasons(&self) -> &[crate::time::Season] {
        use crate::time::Season;
        match self {
            CropType::Strawberry => &[Season::Spring, Season::Summer],
            CropType::Corn => &[Season::Summer],
            CropType::Tomato => &[Season::Summer, Season::Autumn],
            CropType::Pumpkin => &[Season::Autumn],
            CropType::Carrot => &[Season::Spring, Season::Autumn],
            CropType::Eggplant => &[Season::Summer],
            CropType::Blueberry => &[Season::Spring, Season::Summer],
        }
    }

    /// Checks if this crop can be planted in the given season
    pub fn can_plant_in_season(&self, season: crate::time::Season) -> bool {
        self.available_seasons().contains(&season)
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CropStage {
    Seed,
    Sprout,
    Growing,
    Mature,
}

#[derive(Debug, Clone, Copy)]
pub struct Crop {
    pub crop_type: CropType,
    pub stage: CropStage,
    pub days_grown: u32,
    pub watered_today: bool,
}

impl Crop {
    pub fn new(crop_type: CropType) -> Self {
        Self {
            crop_type,
            stage: CropStage::Seed,
            days_grown: 0,
            watered_today: false,
        }
    }

    pub fn growth_duration(&self) -> u32 {
        match self.crop_type {
            CropType::Strawberry => 3,
            CropType::Corn => 4,
            CropType::Tomato => 3,
            CropType::Pumpkin => 5,
            CropType::Carrot => 2,
            CropType::Eggplant => 4,
            CropType::Blueberry => 3,
        }
    }

    pub fn advance_day(&mut self) {
        if self.watered_today {
            self.days_grown += 1;
            let duration = self.growth_duration();
            
            self.stage = match self.days_grown {
                0..=1 => CropStage::Seed,
                2 => CropStage::Sprout,
                3..=4 => CropStage::Growing,
                _ if self.days_grown >= duration => CropStage::Mature,
                _ => self.stage,
            };
        }
        self.watered_today = false;
    }

    pub fn is_mature(&self) -> bool {
        self.stage == CropStage::Mature
    }

    pub fn to_emoji(&self) -> &'static str {
        match self.stage {
            CropStage::Seed => "🌱",
            CropStage::Sprout => "🌱",
            CropStage::Growing => "🌱",
            CropStage::Mature => match self.crop_type {
                CropType::Strawberry => "🍓",
                CropType::Corn => "🌽",
                CropType::Tomato => "🍅",
                CropType::Pumpkin => "🎃",
                CropType::Carrot => "🥕",
                CropType::Eggplant => "🍆",
                CropType::Blueberry => "🫐",
            },
        }
    }
}

