use raylib::{check_collision_circles, math::Vector2};

use crate::{components::EntityId, Game};

#[derive(Debug, Clone)]
pub struct CircleCollider2D {
    pub body_type: BodyType,
    pub radius: f32,
}

#[derive(Debug, Clone)]
pub enum BodyType {
    Static,
    Dynamic,
    Kinematic,
}

struct Contact {
    a: EntityId,
    b: EntityId,
    penetration: f32,
    normal: Vector2,
}

pub fn update(game: &mut Game) {
    let circle_entities = game.circle_collider_2d.keys().collect::<Vec<_>>();
    let mut contacts = Vec::new();

    // detect collisions
    for i in 0..circle_entities.len() {
        let a = circle_entities[i];
        let Some(col_a) = game.circle_collider_2d.get(a) else { continue };
        let Some(pos_a) = game.position.get(a) else { continue };

        for j in i + 1..circle_entities.len() {
            let b = circle_entities[j];

            let Some(col_b) = game.circle_collider_2d.get(b) else { continue };
            let Some(pos_b) = game.position.get(b) else { continue };

            if check_collision_circles(pos_a, col_a.radius, pos_b, col_b.radius) {
                let delta = *pos_a - *pos_b;
                let dist = delta.length();
                if dist == 0.0 {
                    continue;
                }
                let min_dist = col_a.radius + col_b.radius;
                let penetration = min_dist - dist;
                let normal = delta / dist;
                let contact = Contact { a: *a, b: *b, penetration, normal };
                contacts.push(contact);
            }
        }
    }

    // resolve collissions
    for contact in &contacts {
        let a = contact.a;
        let col_a = game
            .circle_collider_2d
            .get(&a)
            .expect("expected resolution to contain only valid entities");
        let pos_a =
            *game.position.get(&a).expect("expected resolution to contain only valid entities");

        let b = contact.b;
        let col_b = game
            .circle_collider_2d
            .get(&b)
            .expect("expected resolution to contain only valid entities");
        let pos_b =
            *game.position.get(&b).expect("expected resolution to contain only valid entities");

        let push_out_a = |eid: EntityId, pos: Vector2, game: &mut Game| {
            let new_pos = pos + contact.normal * contact.penetration;
            game.position.insert(eid, new_pos);
        };

        let push_out_b = |eid: EntityId, pos: Vector2, game: &mut Game| {
            let new_pos = pos - contact.normal * contact.penetration;
            game.position.insert(eid, new_pos);
        };

        let split = |a: EntityId, a_pos: Vector2, b: EntityId, b_pos: Vector2, game: &mut Game| {
            let half = contact.penetration * 0.5;
            game.position.insert(a, a_pos + contact.normal * half);
            game.position.insert(b, b_pos - contact.normal * half);
        };

        match col_a.body_type {
            BodyType::Static => match col_b.body_type {
                BodyType::Kinematic | BodyType::Dynamic => push_out_b(b, pos_b, game),
                BodyType::Static => { /* noop */ }
            },
            BodyType::Kinematic => match col_b.body_type {
                BodyType::Static => push_out_a(a, pos_a, game),
                BodyType::Kinematic => { /* noop */ }
                BodyType::Dynamic => push_out_b(b, pos_b, game),
            },
            BodyType::Dynamic => match col_b.body_type {
                BodyType::Kinematic | BodyType::Static => push_out_a(a, pos_a, game),
                BodyType::Dynamic => split(a, pos_a, b, pos_b, game),
            },
        }
    }
}
