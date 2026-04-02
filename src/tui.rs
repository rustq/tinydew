#[cfg(feature = "interactive")]
pub mod interactive {
    use crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
        execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use std::io::{self, Write};

    use crate::entity::Direction;
    use crate::state::GameState;
    use crate::ui::render_status;
    use crate::{farming, movement};

    pub fn run() -> io::Result<()> {
        let mut state = GameState::load();

        terminal::enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen)?;

        let result = game_loop(&mut state, &mut stdout);

        terminal::disable_raw_mode()?;
        execute!(stdout, LeaveAlternateScreen)?;

        state.save();
        result
    }

    fn game_loop(state: &mut GameState, stdout: &mut io::Stdout) -> io::Result<()> {
        loop {
            // Clear and render
            execute!(
                stdout,
                crossterm::cursor::MoveTo(0, 0),
                crossterm::terminal::Clear(crossterm::terminal::ClearType::All)
            )?;

            let status = render_status(state);
            write!(stdout, "{}", status)?;

            // Controls line
            let controls = if state.guest.location == state.player.location {
                "\nmove: ↑↓←→ | clear: [C] | plant: [P] | water: [W] | harvest: [H] | trade: [T]"
            } else {
                "\nmove: ↑↓←→ | clear: [C] | plant: [P] | water: [W] | harvest: [H] | trade: [T]"
            };
            write!(stdout, "{}", controls)?;
            stdout.flush()?;

            // Wait for input
            if let Event::Key(KeyEvent {
                code, modifiers, ..
            }) = event::read()?
            {
                // Exit on Esc or Ctrl+C
                if code == KeyCode::Esc
                    || (code == KeyCode::Char('c') && modifiers.contains(KeyModifiers::CONTROL))
                {
                    break;
                }

                match code {
                    KeyCode::Up => {
                        state.message = movement::move_player(state, Direction::Up);
                    }
                    KeyCode::Down => {
                        state.message = movement::move_player(state, Direction::Down);
                    }
                    KeyCode::Left => {
                        state.message = movement::move_player(state, Direction::Left);
                    }
                    KeyCode::Right => {
                        state.message = movement::move_player(state, Direction::Right);
                    }
                    KeyCode::Char(' ') => {
                        // Greet / interact
                        state.message = "Hello there! 👋".to_string();
                    }
                    KeyCode::Char('w') | KeyCode::Char('W') => {
                        state.message = farming::water(state, None);
                    }
                    KeyCode::Char('p') | KeyCode::Char('P') => {
                        state.message = farming::plant(state, None);
                    }
                    KeyCode::Char('h') | KeyCode::Char('H') => {
                        state.message = farming::harvest(state, None);
                    }
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
