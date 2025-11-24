use raylib::{
    color::Color,
    math::{Rectangle, Vector2},
};

use crate::{
    components::{Draw, EntityId},
    systems::{
        interact_system::Interact,
        physics_system::{BodyType, CircleCollider2D},
        sprite_system::Sprite,
    },
    Game,
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.sprite.insert(entity_id, building_sprite());
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Static, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    game.interact.insert(entity_id, interact_test());
    entity_id
}

fn interact_test() -> Interact {
    Interact {
        activation_distance: 100.0,
        on_interact: Some(|_, _| println!("interacted with building")),
    }
}

pub fn building_sprite() -> Sprite {
    Sprite {
        texture_path: "resources/PostApoc_Workshop.png".to_string(),
        frame: Rectangle { x: 96.0, y: 192.0, width: 32.0, height: 32.0 },
        origin: Vector2 { x: 16.0, y: 16.0 },
        scale: Vector2 { x: 1.0, y: 1.0 },
        tint: Color::WHITE,
    }
}

pub fn spawn_ghost(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.sprite.insert(entity_id, building_ghost_sprite());
    entity_id
}

// can be used for both ghost and established building
pub fn despawn(game: &mut Game, entity_id: EntityId) {
    game.position.remove(&entity_id);
    game.draw.remove(&entity_id); // TODO: change to sprite removal
    game.circle_collider_2d.remove(&entity_id);
}

pub fn ghost_into_building(game: &mut Game, entity_id: EntityId) {
    game.sprite.insert(entity_id, building_sprite());
    let circle_collider_2d = CircleCollider2D { body_type: BodyType::Static, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    game.interact.insert(entity_id, interact_test());
}

pub fn building_ghost_sprite() -> Sprite {
    Sprite {
        texture_path: "resources/PostApoc_Workshop.png".to_string(),
        frame: Rectangle { x: 96.0, y: 192.0, width: 32.0, height: 32.0 },
        origin: Vector2 { x: 16.0, y: 16.0 },
        scale: Vector2 { x: 1.0, y: 1.0 },
        tint: Color::WHITE.alpha(0.3),
    }
}
