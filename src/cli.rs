use crate::entity::Direction;
use crate::map::Region;
use crate::piano::PianoNote;
use crate::state::GameState;
use crate::{farming, fishing, movement, sleep};

pub fn dispatch_action(state: &mut GameState, action: &str, args: &[String]) -> String {
    match action {
        "move" => {
            let dir = args
                .first()
                .and_then(|s| Direction::from_str(s))
                .unwrap_or(state.player.direction);
            let msg = movement::move_player(state, dir);
            state.message = msg.clone();
            msg
        }
        "water" => {
            let dir = args.first().and_then(|s| Direction::from_str(s));
            let msg = farming::water(state, dir);
            state.message = msg.clone();
            msg
        }
        "clear" => {
            let dir = args.first().and_then(|s| Direction::from_str(s));
            let msg = farming::clear(state, dir);
            state.message = msg.clone();
            msg
        }
        "plant" => {
            let dir = args.first().and_then(|s| Direction::from_str(s));
            let msg = farming::plant(state, dir);
            state.message = msg.clone();
            msg
        }
        "harvest" => {
            let dir = args.first().and_then(|s| Direction::from_str(s));
            let msg = farming::harvest(state, dir);
            state.message = msg.clone();
            msg
        }
        "buy" => {
            let item = args.first().map(|s| s.as_str()).unwrap_or("");
            let qty = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            let msg = match state.inventory.buy(item, qty) {
                Ok(m) => m,
                Err(m) => m,
            };
            state.message = msg.clone();
            msg
        }
        "sell" => {
            let item = args.first().map(|s| s.as_str()).unwrap_or("");
            let qty = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
            let msg = match state.inventory.sell(item, qty) {
                Ok(m) => m,
                Err(m) => m,
            };
            state.message = msg.clone();
            msg
        }
        "fish" => {
            let dir = args.first().and_then(|s| Direction::from_str(s));
            let msg = fishing::fish(state, dir);
            state.message = msg.clone();
            msg
        }
        "sleep" => {
            let msg = sleep::sleep(state);
            msg
        }
        "play" => {
            let note_name = args.first().map(|s| s.as_str()).unwrap_or("");
            // Must be at Farm (4,3) to play piano
            if state.player.location != Region::Farm
                || state.player.x != 4
                || state.player.y != 3
            {
                let msg = "Not near the piano.".to_string();
                state.message = msg.clone();
                return msg;
            }
            match PianoNote::from_name(note_name) {
                Some(note) => {
                    let msg = format!("🎵 {}", note.display_name());
                    state.message = msg.clone();
                    msg
                }
                None => {
                    let msg = format!("Unknown note: {}", note_name);
                    state.message = msg.clone();
                    msg
                }
            }
        }
        _ => {
            let msg = format!("Unknown action: {}", action);
            state.message = msg.clone();
            msg
        }
    }
}
