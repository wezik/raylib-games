use raylib::{color::Color, math::Vector2};

use crate::{
    Game,
    components::{Draw, EntityId},
    systems::{monster_follow_player_system::MoveTowards, physics_system::CircleCollider2D},
};

pub fn spawn(game: &mut Game, position: Vector2) -> EntityId {
    let entity_id = game.next_id();
    game.position.insert(entity_id, position);
    game.draw.insert(entity_id, monster_draw());
    game.speed.insert(entity_id, 20.0);
    let move_towards = MoveTowards { range: 100.0 };
    game.move_towards.insert(entity_id, move_towards);
    let circle_collider_2d = CircleCollider2D { is_static: false, radius: 10.0 };
    game.circle_collider_2d.insert(entity_id, circle_collider_2d);
    entity_id
}

pub fn monster_draw() -> Draw {
    Draw { color: Color::RED }
}
