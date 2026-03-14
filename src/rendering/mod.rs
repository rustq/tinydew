use crate::time::{GameTime, Season};
use crate::world::{Map, Player};
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

    pub fn render(&mut self, world: &crate::world::World, time: &crate::time::TimeManager) -> Result<(), Box<dyn std::error::Error>> {
        let mut stdout = stdout();

        // Clear screen
        stdout.execute(terminal::Clear(ClearType::All))?;

        // Render status bar
        self.render_status_bar(&mut stdout, &time.game_time)?;

        // Render map
        self.render_map(&mut stdout, world.map(), world.player())?;

        // Render help bar
        self.render_help_bar(&mut stdout)?;

        stdout.flush()?;
        Ok(())
    }

    fn render_status_bar(&self, stdout: &mut std::io::Stdout, game_time: &GameTime) -> Result<(), Box<dyn std::error::Error>> {
        stdout
            .queue(cursor::MoveTo(0, 0))?
            .queue(SetForegroundColor(Color::Cyan))?
            .queue(Print(format!(
                "Day {} {:02}:{:02} | {} | Weather: ☀️ | Money: 💰0",
                game_time.day,
                game_time.hour,
                game_time.minute,
                self.season_to_emoji(game_time.season)
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

    fn render_help_bar(&self, stdout: &mut std::io::Stdout) -> Result<(), Box<dyn std::error::Error>> {
        let help_text = "WASD: Move | Esc: Quit";
        let y = 2 + 20; // map height + start_y

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
