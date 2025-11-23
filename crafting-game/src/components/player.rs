use raylib::{color::Color, math::Vector2};

use crate::{
    Game,
    components::{Draw, EntityId}, systems::physics_system::CircleCollider2D,
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.player_controlled.push(entity_id);
    game.draw.insert(entity_id, player_draw());
    game.speed.insert(entity_id, 50.0);
    let circle_collider_2d = CircleCollider2D { is_static: true, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    entity_id
}

pub fn player_draw() -> Draw {
    Draw { color: Color::BLUE }
}
