use raylib::{color::Color, math::Vector2};

use crate::{
    Game,
    components::{Draw, EntityId}, systems::physics_system::CircleCollider2D,
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.draw.insert(entity_id, building_draw());
    let circle_collider_2d = CircleCollider2D { is_static: true, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    entity_id
}

pub fn building_draw() -> Draw {
    Draw {
        color: Color::GREEN,
    }
}

pub fn spawn_shadow(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.draw.insert(entity_id, building_shadow_draw());
    entity_id
}

pub fn shadow_into_building(game: &mut Game, entity_id: EntityId) {
    game.draw.insert(entity_id, building_draw());
    let circle_collider_2d = CircleCollider2D { is_static: true, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
}

pub fn building_shadow_draw() -> Draw {
    Draw {
        color: Color::GREEN.alpha(0.3),
    }
}
