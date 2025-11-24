use raylib::math::Vector2;

use crate::Game;

pub fn update(game: &mut Game) {
    let Some(player) = game.player_controlled.first() else { return };

    let Some(position) = game.position.get_mut(player) else { return };

    let Some(speed) = game.speed.get(player) else { return };

    let mut dx = 0.0;
    let mut dy = 0.0;
    if game.input_state.move_up_down {
        dy -= 1.0
    }
    if game.input_state.move_down_down {
        dy += 1.0
    }
    if game.input_state.move_left_down {
        dx -= 1.0
    }
    if game.input_state.move_right_down {
        dx += 1.0
    }
    let direction = Vector2::new(dx, dy).normalized();

    *position += direction * *speed * game.delta_time;
}
