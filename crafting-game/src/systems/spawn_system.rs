use crate::{Game, SpawnEntity};

pub fn update(game: &mut Game) {
    let Some(position) = game.input_state.spawn_intent else {
        return;
    };

    game.spawn(SpawnEntity::Monster(position));
    game.input_state.spawn_intent = None;
}
