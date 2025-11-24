use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
};

use crate::{
    components::{Draw, EntityId},
    systems::{
        physics_system::{BodyType, CircleCollider2D},
        sprite_system::Sprite,
    },
    Game,
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.player_controlled.push(entity_id);
    // game.draw.insert(entity_id, player_draw());
    game.speed.insert(entity_id, 250.0);
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Kinematic, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    let sprite = Sprite {
        texture_path: "resources/WitchFlying.png".to_string(),
        frame: Rectangle { x: 0.0, y: 0.0, width: 48.0, height: 64.0 },
        origin: Vector2 { x: 24.0, y: 48.0 },
        scale: Vector2 { x: 1.0, y: 1.0 },
        tint: Color::WHITE,
    };
    game.sprite.insert(entity_id, sprite);
    entity_id
}

pub fn player_draw() -> Draw {
    Draw { color: Color::BLUE }
}
