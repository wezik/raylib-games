use raylib::{camera::Camera2D, color::Color, math::Vector2, prelude::{RaylibDraw, RaylibMode2DExt}};

use crate::core::EpicGamer;

mod core;
mod systems;

fn main() {
    let width = 800.0;
    let height = 450.0;
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Another Crafting Game")
        .build();

    let mut debug_mode = true;
    let mut camera = Camera2D {
        offset: Vector2 { x: width / 2.0, y: height / 2.0 },
        target: Vector2 { x: 0.0, y: 0.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut player = EpicGamer {
        position: Vector2::default(),
    };

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::WHITE);

        d.draw_mode2D(camera, |mut d, camera| {
            d.draw_circle_v(player.position, 2.0, Color::RED);
        });
    }
}
