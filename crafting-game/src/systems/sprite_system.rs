use std::collections::HashMap;

use raylib::{
    math::Vector2, prelude::RaylibDrawHandle, texture::Texture2D, RaylibHandle, RaylibThread,
};

use crate::{components::EntityId, Game};

#[derive(Default, Debug)]
pub struct SpriteCache {
    queue: Vec<(EntityId, Sprite)>,
    pub entities: HashMap<EntityId, Sprite>,
    pub loaded: HashMap<String, LoadedSprite>,
}

impl SpriteCache {
    pub fn insert(&mut self, entity_id: EntityId, sprite: Sprite) {
        self.queue.push((entity_id, sprite));
    }

    pub fn get(&self, entity_id: &EntityId) -> Option<&LoadedSprite> {
        self.entities.get(&entity_id).and_then(|sprite| {
            self.loaded.get(sprite.texture_path.as_str()).map(|loaded_sprite| loaded_sprite)
        })
    }
}

#[derive(Debug)]
pub struct Sprite {
    pub texture_path: String,
    pub frame_size: Vector2,
}

#[derive(Debug)]
pub struct LoadedSprite {
    pub texture: Texture2D,
}

const MAX_LOADS_PER_FRAME: usize = 10;

// We want to load sprites before drawing starts so it has to stay separated
pub fn update(game: &mut Game, rl: &mut RaylibHandle, thread: &RaylibThread) {
    let mut attempted_loads = 0;
    while game.sprite.queue.len() > 0 && attempted_loads < MAX_LOADS_PER_FRAME {
        let Some((entity_id, sprite)) = game.sprite.queue.pop() else {
            panic!("This should never happen ???");
        };

        attempted_loads += 1;
        let texture = match rl.load_texture(thread, sprite.texture_path.as_str()) {
            Ok(texture) => texture,
            Err(err) => {
                println!("Failed to load texture: {}", err);
                continue;
            }
        };

        let loaded_sprite = LoadedSprite { texture };
        game.sprite.loaded.insert(sprite.texture_path.clone(), loaded_sprite);
        game.sprite.entities.insert(entity_id, sprite);
    }
}
