use raylib::{
    camera::Camera2D,
    color::Color,
    math::{Rectangle, Vector2},
    prelude::{RaylibDraw, RaylibDrawHandle, RaylibMode2DExt},
    text::RaylibFont,
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

            let source_rec = sprite.frame;

            let dest_rec = Rectangle {
                x: position.x,
                y: position.y,
                width: sprite.frame.width,
                height: sprite.frame.height,
            };

            d.draw_texture_pro(
                &texture.texture,
                source_rec,
                dest_rec,
                sprite.origin,
                0.0,
                sprite.tint,
            );
        }

        // if let Some(interact_popup) = game.input_state.interact_gui_popup {
        //     let font = d.get_font_default();
        //     let text_size = font.measure_text("E to interact", 20.0, 1.0);
        //     d.draw_text_ex(
        //         d.get_font_default(),
        //         "E to interact",
        //         Vector2 {
        //             x: interact_popup.x - text_size.x / 2.0,
        //             y: interact_popup.y - text_size.y * 2.0,
        //         },
        //         20.0,
        //         1.0,
        //         Color::DARKRED,
        //     );
        // }
    });
}
