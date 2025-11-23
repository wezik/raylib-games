use raylib::{color::Color, math::Vector2};

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
    game.draw.insert(entity_id, player_draw());
    game.speed.insert(entity_id, 50.0);
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Kinematic, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    let sprite = Sprite {
        texture_path: "resources/WitchFlying.png".to_string(),
        frame_size: Vector2 { x: 48.0, y: 64.0 },
    };
    game.sprite.insert(entity_id, sprite);
    entity_id
}

pub fn player_draw() -> Draw {
    Draw { color: Color::BLUE }
}
