use crate::festival;
use crate::grow;
use crate::map::Region;
use crate::spawn;
use crate::state::GameState;
use crate::time::WorldTime;
use crate::weather;

pub fn sleep(state: &mut GameState) -> String {
    // Advance to next day
    state.day += 1;
    state.time = WorldTime::new(6, 0);

    // Wake up at Farm (3,3)
    state.player.x = 3;
    state.player.y = 3;
    state.player.location = Region::Farm;

    // Day-start processing
    state.weather = weather::roll_weather(state.day);

    // Crop growth
    grow::grow_crops(state);

    // River bubble reset
    grow::reset_river_bubbles(state);

    // Nightly spawns
    spawn::nightly_spawns(state);

    // Festival check
    festival::check_festival(state);

    if !festival::is_festival_day(state.day) {
        state.message = format!(
            "Good morning! Day {} {} {}",
            state.day,
            state.weather.icon(false),
            state.time.format()
        );
    }

    state.message.clone()
}
