use crate::input::InputManager;
use crate::rendering::Renderer;
use crate::time::TimeManager;
use crate::world::World;
use crossterm::event::{Event, KeyCode};
use std::error::Error;
use std::time::Duration;

pub struct Engine {
    input_manager: InputManager,
    renderer: Renderer,
    time_manager: TimeManager,
    world: World,
    running: bool,
}

impl Engine {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        let input_manager = InputManager::new()?;
        let renderer = Renderer::new()?;
        let time_manager = TimeManager::new();
        let world = World::new()?;

        Ok(Self {
            input_manager,
            renderer,
            time_manager,
            world,
            running: true,
        })
    }

    pub fn run(&mut self) -> Result<(), Box<dyn Error>> {
        self.renderer.init()?;

        while self.running {
            // Process input
            if let Some(event) = self.input_manager.poll()? {
                self.handle_event(event)?;
            }

            // Update time
            self.time_manager.tick();

            // Render
            self.renderer.render(&self.world, &self.time_manager)?;

            // Sleep to maintain ~2.5s per game minute
            std::thread::sleep(Duration::from_millis(2500));
        }

        self.renderer.cleanup()?;
        Ok(())
    }

    fn handle_event(&mut self, event: Event) -> Result<(), Box<dyn Error>> {
        match event {
            Event::Key(key_event) => {
                match key_event.code {
                    KeyCode::Esc => self.running = false,
                    KeyCode::Char('w') | KeyCode::Char('W') | KeyCode::Up => {
                        self.world.move_player(0, -1)?;
                    }
                    KeyCode::Char('s') | KeyCode::Char('S') | KeyCode::Down => {
                        self.world.move_player(0, 1)?;
                    }
                    KeyCode::Char('a') | KeyCode::Char('A') | KeyCode::Left => {
                        self.world.move_player(-1, 0)?;
                    }
                    KeyCode::Char('d') | KeyCode::Char('D') | KeyCode::Right => {
                        self.world.move_player(1, 0)?;
                    }
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(())
    }
}
