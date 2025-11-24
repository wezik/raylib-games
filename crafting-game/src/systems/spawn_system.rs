use crate::{Game, SpawnEntity};

// TODO: Debug system for spawning entities
pub fn update(game: &mut Game) {
    if !game.input_state.dash_pressed {
        return;
    }
    let pos = game.input_state.mouse_world;
    game.spawn(SpawnEntity::Monster(pos));
}
