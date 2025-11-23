use raylib::{color::Color, math::Vector2};

use crate::{
    components::{Draw, EntityId},
    systems::physics_system::{BodyType, CircleCollider2D},
    Game,
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.draw.insert(entity_id, building_draw());
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Static, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    entity_id
}

pub fn building_draw() -> Draw {
    Draw { color: Color::GREEN }
}

pub fn spawn_ghost(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.draw.insert(entity_id, building_ghost_draw());
    entity_id
}

// can be used for both ghost and established building
pub fn despawn(game: &mut Game, entity_id: EntityId) {
    game.position.remove(&entity_id);
    game.draw.remove(&entity_id);
    game.circle_collider_2d.remove(&entity_id);
}

pub fn ghost_into_building(game: &mut Game, entity_id: EntityId) {
    game.draw.insert(entity_id, building_draw());
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Static, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
}

pub fn building_ghost_draw() -> Draw {
    Draw { color: Color::GREEN.alpha(0.3) }
}
