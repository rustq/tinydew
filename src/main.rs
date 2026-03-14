pub mod engine;
pub mod rendering;
pub mod world;
pub mod input;
pub mod time;

use engine::Engine;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let mut engine = Engine::new()?;
    engine.run()?;
    Ok(())
}
