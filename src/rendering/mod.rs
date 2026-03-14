use crate::time::{GameTime, Season};
use crate::world::{Map, Player};
use crate::inventory::Inventory;
use crossterm::{
    cursor,
    style::{Color, Print, ResetColor, SetForegroundColor},
    terminal::{self, ClearType},
    ExecutableCommand, QueueableCommand,
};
use std::io::{stdout, Write};

pub struct Renderer {
    use_emoji: bool,
}

impl Renderer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            use_emoji: true,
        })
    }

    pub fn init(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::Hide)?;
        Ok(())
    }

    pub fn render(&mut self, world: &crate::world::World, time: &crate::time::TimeManager, inventory: &Inventory, money: u32) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();

        // Clear screen
        stdout.execute(terminal::Clear(ClearType::All))?;

        // Render status bar
        self.render_status_bar(&mut stdout, &time.game_time, money)?;

        // Render map
        self.render_map(&mut stdout, world.map(), world.player())?;

        // Render inventory
        self.render_inventory(&mut stdout, inventory)?;

        // Render help bar
        self.render_help_bar(&mut stdout)?;

        stdout.flush()?;
        Ok(())
    }

    fn render_status_bar(&self, stdout: &mut std::io::Stdout, game_time: &GameTime, money: u32) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print(format!(
                "Day {} {:02}:{:02} | {} | Weather: {} | Money: 💰{}",
                game_time.day,
                game_time.hour,
                game_time.minute,
                self.season_to_emoji(game_time.season),
                game_time.weather.to_emoji(),
                money
            )))?
            .queue(ResetColor)?;

        Ok(())
    }

    fn render_map(&self, stdout: &mut std::io::Stdout, map: &Map, player: &Player) -> Result<(), Box<dyn std::error::Error>> {
        let start_y = 2;

        for y in 0..map.height() {
            stdout.queue(cursor::MoveTo(0, start_y + y as u16))?;

            for x in 0..map.width() {
                if x == player.x && y == player.y {
                    // Render player
                    stdout.queue(SetForegroundColor(Color::Yellow))?
                          .queue(Print("😐"))?
                          .queue(ResetColor)?;
                } else if let Some(tile) = map.get_tile(x, y) {
                    // Render tile
                    if self.use_emoji {
                        stdout.queue(Print(tile.to_emoji()))?;
                    } else {
                        stdout.queue(Print(tile.to_ascii()))?;
                    }
                }
            }
        }

        Ok(())
    }

    fn render_inventory(&self, stdout: &mut std::io::Stdout, inventory: &Inventory) -> Result<(), Box<dyn std::error::Error>> {
        let y = 25;

        stdout
            .queue(cursor::MoveTo(0, y))?
            .queue(SetForegroundColor(Color::Green))?
            .queue(Print("Inventory:"))?
            .queue(ResetColor)?;

        let mut x = 12;
        for (item_type, quantity) in inventory.iter() {
            let item_display = format!("{}x{} ", quantity, item_type.to_emoji());
            stdout
                .queue(cursor::MoveTo(x, y))?
                .queue(Print(&item_display))?;
            x += item_display.len() as u16;
        }

        Ok(())
    }

    fn render_help_bar(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let help_text = "WASD: Move | E: Use Tool | 1-9: Select Tool | Esc: Quit";
        let y = 27;

        stdout
            .queue(cursor::MoveTo(0, y))?
            .queue(SetForegroundColor(Color::DarkGrey))?
            .queue(Print(help_text))?
            .queue(ResetColor)?;

        Ok(())
    }

    fn season_to_emoji(&self, season: Season) -> &'static str {
        match season {
            Season::Spring => "🌸",
            Season::Summer => "☀️",
            Season::Autumn => "🍂",
            Season::Winter => "❄️",
        }
    }

    pub fn cleanup(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        stdout()
            .execute(terminal::Clear(ClearType::All))?
            .execute(cursor::Show)?;
        Ok(())
    }
}
