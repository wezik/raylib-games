use raylib::camera::Camera2D;

use crate::Game;

pub fn update(game: &mut Game, camera: &mut Camera2D) {
    let Some(player) = game.player_controlled.first() else {
        return;
    };

    let Some(position) = game.position.get(player) else {
        return;
    };

    camera.target = *position;
}
