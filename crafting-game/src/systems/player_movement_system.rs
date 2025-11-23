use crate::Game;

pub fn update(game: &mut Game) {
    let Some(player) = game.player_controlled.first() else {
        return;
    };

    let Some(position) = game.position.get_mut(player) else {
        return;
    };

    let Some(speed) = game.speed.get(player) else {
        return;
    };

    let direction = game.input_state.move_intent.normalized();
    *position += direction * *speed * game.delta_time;
}
