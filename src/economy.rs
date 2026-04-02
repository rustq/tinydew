use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    pub seeds: u32,
    pub carrots: u32,
    pub strawberries: u32,
    pub cauliflowers: u32,
    pub flowers: u32,
    pub mushrooms: u32,
    pub fish: u32,
    pub rare_fish: u32,
    pub money: i32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self {
            seeds: 5,
            carrots: 0,
            strawberries: 0,
            cauliflowers: 0,
            flowers: 0,
            mushrooms: 0,
            fish: 0,
            rare_fish: 0,
            money: 100,
        }
    }
}

const SEED_PRICE: i32 = 10;

impl Inventory {
    pub fn buy(&mut self, item: &str, qty: u32) -> Result<String, String> {
        match item {
            "seed" => {
                let cost = SEED_PRICE * qty as i32;
                if self.money < cost {
                    Err("Not enough money.".to_string())
                } else {
                    self.money -= cost;
                    self.seeds += qty;
                    Ok(format!("Bought {} seed(s) for ${}.", qty, cost))
                }
            }
            _ => Err(format!("Cannot buy '{}'.", item)),
        }
    }

    pub fn sell(&mut self, item: &str, qty: u32) -> Result<String, String> {
        let (count, value, name) = match item {
            "🍓" | "strawberry" => (&mut self.strawberries, 30, "strawberry"),
            "🥕" | "carrot" => (&mut self.carrots, 20, "carrot"),
            "🥦" | "cauliflower" => (&mut self.cauliflowers, 35, "cauliflower"),
            "🌺" | "flower" => (&mut self.flowers, 15, "flower"),
            "🍄" | "mushroom" => (&mut self.mushrooms, 25, "mushroom"),
            "🐟" | "fish" => (&mut self.fish, 15, "fish"),
            "🐠" | "rare" => (&mut self.rare_fish, 50, "rare fish"),
            _ => return Err(format!("Cannot sell '{}'.", item)),
        };
        if *count < qty {
            Err(format!("Not enough {} to sell.", name))
        } else {
            *count -= qty;
            let income = value * qty as i32;
            self.money += income;
            Ok(format!("Sold {} {}(s) for ${}.", qty, name, income))
        }
    }

    pub fn format_items(&self) -> Vec<String> {
        let mut lines = Vec::new();
        if self.seeds > 0 {
            lines.push(format!("🫙 x{}", self.seeds));
        }
        if self.carrots > 0 {
            lines.push(format!("🥕 x{}", self.carrots));
        }
        if self.strawberries > 0 {
            lines.push(format!("🍓 x{}", self.strawberries));
        }
        if self.cauliflowers > 0 {
            lines.push(format!("🥦 x{}", self.cauliflowers));
        }
        if self.flowers > 0 {
            lines.push(format!("🌺 x{}", self.flowers));
        }
        if self.mushrooms > 0 {
            lines.push(format!("🍄 x{}", self.mushrooms));
        }
        if self.fish > 0 {
            lines.push(format!("🐟 x{}", self.fish));
        }
        if self.rare_fish > 0 {
            lines.push(format!("🐠 x{}", self.rare_fish));
        }
        lines
    }
}
