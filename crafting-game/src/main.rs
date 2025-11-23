use raylib::{camera::Camera2D, prelude::RaylibDraw};
use std::collections::HashMap;

use raylib::math::Vector2;

use crate::{
    components::{Draw, EntityId, building, monster, player},
    systems::{
        building_system::{self, BuildingType}, camera_system, drawing_system, input_system::InputState, monster_follow_player_system::{self, MoveTowards}, physics_system::{self, CircleCollider2D}, player_movement_system::{self}, spawn_system
    },
};

mod components;
mod systems;

#[derive(Default, Debug)]
struct Game {
    pub next_entity_id: usize,
    pub input_state: InputState,
    // components
    pub player_controlled: Vec<EntityId>,
    pub position: HashMap<EntityId, Vector2>,
    pub draw: HashMap<EntityId, Draw>,
    pub speed: HashMap<EntityId, f32>,
    pub move_towards: HashMap<EntityId, MoveTowards>,
    // physics
    pub circle_collider_2d: HashMap<EntityId, CircleCollider2D>,
}

impl Game {
    pub fn next_id(&mut self) -> EntityId {
        let entity_id = EntityId(self.next_entity_id);
        self.next_entity_id += 1;
        entity_id
    }

    pub fn spawn(&mut self, entity: SpawnEntity) -> EntityId {
        match entity {
            SpawnEntity::Player(position) => player::spawn(self, position),
            SpawnEntity::Monster(position) => monster::spawn(self, position),
            SpawnEntity::BuildingShadow(_building_type, position) => {
                building::spawn_shadow(self, position)
            }
            // SpawnEntity::Building(building_type, position) => building::spawn(self, position),
        }
    }
}

pub enum SpawnEntity {
    Player(Vector2),
    Monster(Vector2),
    // Building(BuildingType, Vector2),
    BuildingShadow(BuildingType, Vector2),
}

fn main() {
    let width = 800.0;
    let height = 600.0;
    let (mut rl, thread) = raylib::init()
        .size(width as i32, height as i32)
        .title("Crafting Game")
        .build();

    rl.set_exit_key(None);

    let mut camera = Camera2D {
        target: Vector2::default(),
        offset: Vector2 {
            x: width / 2.0,
            y: height / 2.0,
        },
        rotation: 0.0,
        zoom: 1.0,
    };

    let mut game = Game::default();
    game.spawn(SpawnEntity::Player(Vector2::default()));

    while !game.input_state.close_intent {
        game.input_state.update(&rl, &camera);

        // delta update
        player_movement_system::update(&mut game, &rl);
        camera_system::update(&mut game, &mut camera);
        spawn_system::update(&mut game);
        monster_follow_player_system::update(&mut game, &rl);
        building_system::update(&mut game);
        // TOOD: MOVE TO FIXED UPDATE
        physics_system::update(&mut game);

        // draw
        let mut d = rl.begin_drawing(&thread);
        drawing_system::update(&mut d, &game, &camera);
        d.draw_fps(10, 10);
    }
}
