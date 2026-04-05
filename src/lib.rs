pub mod action;
pub mod db;
pub mod map;
pub mod state;
pub mod types;
pub mod ui;

#[cfg(test)]
mod tests {
    use crate::state::GameState;
    use crate::ui;

    #[test]
    fn initial_farm_ui() {
        let state = GameState::new();
        let output = ui::format_status(&state);
        println!("{}", output);

        // Verify header
        assert!(output.contains("tinydew day 1"));
        assert!(output.contains("06:00"));

        // Verify map elements
        assert!(output.contains("\u{1f9d1}")); // 🧑 player
        assert!(output.contains("\u{1f3e0}")); // 🏠 house
        assert!(output.contains("\u{1f333}")); // 🌳 boundary

        // Verify inventory
        assert!(output.contains("\u{1fad9} x1")); // 🫙 x1 seed

        // Verify money
        assert!(output.contains("Money: \u{1f4b0} $100"));

        // Verify greeting
        assert!(output.contains("Good morning!"));
    }
}
