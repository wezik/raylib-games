use raylib::{camera::Camera2D, prelude::RaylibDraw};
use std::collections::HashMap;

use raylib::math::Vector2;

use crate::{
    components::{building, monster, player, Draw, EntityId},
    systems::{
        building_system::{self, BuildingSystem, BuildingType}, camera_system, drawing_system, input_system::InputState, interact_system::{self, Interact}, monster_follow_system::{self, MoveTowards}, physics_system::{self, CircleCollider2D}, player_movement_system::{self}, spawn_system, sprite_system::{self, SpriteCache}
    },
};

mod components;
mod systems;
// mod ui;

#[derive(Default, Debug)]
struct Game {
    // temp states
    next_entity_id: usize,
    pub delta_time: f32,
    pub fixed_delta_time: f32,
    pub accumulated_fixed_delta_time: f32,
    pub input_state: InputState,
    // components
    pub player_controlled: Vec<EntityId>,
    pub position: HashMap<EntityId, Vector2>,
    pub draw: HashMap<EntityId, Draw>,
    pub speed: HashMap<EntityId, f32>,
    pub move_towards: HashMap<EntityId, MoveTowards>,
    pub sprite: SpriteCache,
    pub interact: HashMap<EntityId, Interact>,
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
            SpawnEntity::BuildingGhost(_building_type, position) => {
                building::spawn_ghost(self, position)
            } // SpawnEntity::Building(building_type, position) => building::spawn(self, position),
        }
    }
}

pub enum SpawnEntity {
    Player(Vector2),
    Monster(Vector2),
    // Building(BuildingType, Vector2),
    BuildingGhost(BuildingType, Vector2),
}

fn main() {
    // window setup
    let width = 800.0;
    let height = 600.0;

    // camera
    let mut camera = Camera2D {
        target: Vector2::default(),
        offset: Vector2 { x: width / 2.0, y: height / 2.0 },
        rotation: 0.0,
        zoom: 1.0,
    };

    // game setup
    let mut game = Game::default();
    game.spawn(SpawnEntity::Player(Vector2::default()));
    game.fixed_delta_time = 1.0 / 64.0;
    let (mut rl, thread) =
        raylib::init()
        .size(width as i32, height as i32)
        .title("Crafting Game")
        .build();
    rl.set_exit_key(None);

    // systems
    let mut building_system = BuildingSystem::default();

    while !rl.window_should_close() {
        // game update
        game.delta_time = rl.get_frame_time();
        game.accumulated_fixed_delta_time += game.delta_time;
        game.input_state.update(&rl, &camera);

        // systems update
        player_movement_system::update(&mut game);
        camera_system::update(&mut game, &mut camera);
        spawn_system::update(&mut game);
        monster_follow_system::update(&mut game);
        building_system.update(&mut game);
        sprite_system::update(&mut game, &mut rl, &thread);
        // interact_system::update(&mut game);

        // fixed systems update
        while game.accumulated_fixed_delta_time > game.fixed_delta_time {
            game.accumulated_fixed_delta_time -= game.fixed_delta_time;
            physics_system::update(&mut game);
        }

        // drawing systems update
        let mut d = rl.begin_drawing(&thread);
        drawing_system::draw(&mut d, &game, &camera);
        d.draw_fps(10, 10);
    }
}
