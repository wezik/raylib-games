use raylib::RaylibHandle;

use crate::Game;

pub fn update(game: &mut Game, rl: &RaylibHandle) {
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
    *position += direction * *speed * rl.get_frame_time();
}
