use raylib::{
    camera::Camera2D,
    color::Color,
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2DExt},
};

use crate::Game;

pub fn update(d: &mut RaylibDrawHandle, game: &Game, camera: &Camera2D) {
    d.clear_background(Color::WHITE);

    d.draw_mode2D(*camera, |mut d, _| {
        for (entity_id, draw) in &game.draw {
            let Some(position) = game.position.get(entity_id) else {
                continue;
            };

            d.draw_circle_v(position, 10.0, draw.color);
        }
    });
}
