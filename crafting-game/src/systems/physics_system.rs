use raylib::{check_collision_circles};

use crate::Game;

#[derive(Debug, Clone)]
pub struct CircleCollider2D { 
    pub is_static: bool,
    pub radius: f32,
}

const FIXED_DELTA: f32 = 1.0 / 64.0;

// TODO: As physics always is, its a work in progress
pub fn update(game: &mut Game) {
    let circle_collider_entities = game.circle_collider_2d.keys().collect::<Vec<_>>();
    for entity_id in &circle_collider_entities {
        let Some(circle_collider) = game.circle_collider_2d.get(entity_id) else {
            continue;
        };

        let Some(position) = game.position.get(entity_id) else {
            continue;
        };
        let circle_collider = circle_collider.clone();
        let position = position.clone();

        for other_entity_id in &circle_collider_entities {
            if entity_id == other_entity_id {
                continue;
            }

            let Some(other_circle_collider) = game.circle_collider_2d.get(other_entity_id) else {
                continue;
            };

            let Some(other_position) = game.position.get(other_entity_id) else {
                continue;
            };
            let other_circle_collider = other_circle_collider.clone();
            let other_position = other_position.clone();

            if !check_collision_circles(position, circle_collider.radius, other_position, other_circle_collider.radius) {
                continue;
            }

            if other_circle_collider.is_static {
                let distance = position - other_position;
                let direction = distance.normalized();
                let new_position = other_position + direction * (circle_collider.radius + other_circle_collider.radius);
                game.position.insert(**entity_id, new_position);
            }
        }
    }
}
