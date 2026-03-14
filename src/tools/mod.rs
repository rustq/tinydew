#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tool {
    Hoe,
    WateringCan,
    Axe,
    Pickaxe,
    FishingRod,
}

impl Tool {
    pub fn to_emoji(&self) -> &'static str {
        match self {
            Tool::Hoe => "⛏️",
            Tool::WateringCan => "💧",
            Tool::Axe => "🪓",
            Tool::Pickaxe => "⛏️",
            Tool::FishingRod => "🎣",
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ToolSlot {
    pub tool: Option<Tool>,
}

impl ToolSlot {
    pub fn new() -> Self {
        Self { tool: None }
    }

    pub fn set_tool(&mut self, tool: Tool) {
        self.tool = Some(tool);
    }

    pub fn clear(&mut self) {
        self.tool = None;
    }

    pub fn get_tool(&self) -> Option<Tool> {
        self.tool
    }
}
