pub mod cli;
pub mod economy;
pub mod entity;
pub mod festival;
pub mod fishing;
pub mod map;
pub mod season;
pub mod spawn;
pub mod state;
pub mod time;
pub mod types;
pub mod ui;
pub mod weather;

#[cfg(test)]
mod tests {
    use crate::state::GameState;
    use crate::types::{Direction, Region};
    use crate::ui;

    #[test]
    fn initial_farm_ui() {
        let state = GameState::new();
        let output = ui::render_status(&state);
        assert!(output.contains("tinydew day 1"));
        assert!(output.contains("🌿"));
        assert!(output.contains("🧑"));
        assert!(output.contains("Money: 💰 $100"));
    }

    #[test]
    fn test_movement() {
        let mut state = GameState::new();
        let result = state.try_move(Direction::Right);
        assert!(state.player.x != 6 || result.contains("EastPath"));
    }

    #[test]
    fn test_planting() {
        let mut state = GameState::new();
        state.try_move(Direction::Up);
        state.try_clear(Direction::Up);
        let result = state.try_plant(Direction::Up);
        assert!(result.contains("Planted"));
        assert_eq!(state.inventory.seeds, 4);
    }

    #[test]
    fn test_watering() {
        let mut state = GameState::new();
        state.try_move(Direction::Up);
        state.try_clear(Direction::Up);
        state.try_plant(Direction::Up);
        let result = state.try_water(Direction::Up);
        assert!(result.contains("Watered"));
    }

    #[test]
    fn test_buy_seed() {
        let mut state = GameState::new();
        let result = state.buy_seed();
        assert!(result.contains("Bought"));
        assert_eq!(state.money, 90);
        assert_eq!(state.inventory.seeds, 6);
    }

    #[test]
    fn test_sell_item() {
        let mut state = GameState::new();
        state.inventory.add_produce("🍓", 1);
        let result = state.sell_item("🍓");
        assert!(result.contains("Sold"));
        assert_eq!(state.money, 115);
    }

    #[test]
    fn test_sleep() {
        let mut state = GameState::new();
        state.time.advance(60);
        let result = state.sleep();
        assert!(result.contains("Day 2"));
        assert_eq!(state.player.region, crate::types::Region::Farm);
    }

    #[test]
    fn test_region_transition() {
        let mut state = GameState::new();
        state.player.x = 6;
        state.player.y = 5;
        state.try_move(Direction::Right);
        assert_eq!(state.player.region, Region::EastPath);
    }

    #[test]
    fn test_time_advance() {
        let mut state = GameState::new();
        state.advance_time(30);
        assert_eq!(state.time.minutes, 390);
        state.advance_time(1050);
        assert_eq!(state.time.day, 2);
    }

    #[test]
    fn test_day_transition() {
        let mut state = GameState::new();
        state.process_day_transition();
        assert_eq!(state.time.day, 2);
        assert_eq!(state.time.minutes, 360);
    }
}
