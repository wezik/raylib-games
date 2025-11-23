use raylib::{
    camera::Camera2D,
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2DExt},
};

use crate::Game;

pub fn draw(d: &mut RaylibDrawHandle, game: &Game, camera: &Camera2D) {
    d.clear_background(Color::WHITE);

    d.draw_mode2D(*camera, |mut d, _| {
        for (entity_id, draw) in &game.draw {
            let Some(position) = game.position.get(entity_id) else {
                continue;
            };

            d.draw_circle_v(position, 10.0, draw.color);
        }

        for (entity_id, sprite) in &game.sprite.entities {
            let Some(texture) = game.sprite.loaded.get(&sprite.texture_path) else { continue };
            let Some(position) = game.position.get(entity_id) else { continue };

            let source_rec = Rectangle {
                x: 0.0,
                y: 0.0,
                width: sprite.frame_size.x,
                height: sprite.frame_size.y,
            };

            let dest_rec = Rectangle {
                x: position.x,
                y: position.y,
                width: sprite.frame_size.x,
                height: sprite.frame_size.y,
            };

            d.draw_texture_pro(
                &texture.texture,
                source_rec,
                dest_rec,
                Vector2 { x: sprite.frame_size.x / 2.0, y: sprite.frame_size.y / 2.0 },
                0.0,
                Color::WHITE,
            );
        }
    });
}
