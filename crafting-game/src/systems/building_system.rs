use crate::{
    components::{building, EntityId}, systems::event_bus::Event, Game
};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Test,
    Wood,
    Stone,
    Boo,
}

impl BuildingType {
    pub fn all() -> Vec<BuildingType> {
        vec![BuildingType::Test, BuildingType::Wood, BuildingType::Stone, BuildingType::Boo]
    }
}

// I know in "ECS" systems "are not supposed to" handle state, but idc it's easy and not messy that way
#[derive(Default, Debug)]
pub struct BuildingSystem {
    pub build_in_progress: Option<EntityId>,
}

impl BuildingSystem {
    pub fn update(&mut self, game: &mut Game) {
        self.consume_events(game);

        let Some(e_id) = self.build_in_progress else { return };

        // handle cancellation
        if game.input_state.cancel_pressed {
            building::despawn(game, e_id);
            self.build_in_progress = None;
        }

        // handle mouse follow
        game.position.insert(e_id, game.input_state.mouse_world);

        // handle confirmation
        if game.input_state.confirm_pressed || game.input_state.mouse_lbm_pressed {
            building::ghost_into_building(game, e_id);
            self.build_in_progress = None;
        }
    }

    fn consume_events(&mut self, game: &mut Game) {
        if let Some(event) = game.event_bus.fetch(|e| matches!(e, Event::BuildingSelected(_) | Event::BuildingMenuOpened)) {
            match event {
                Event::BuildingSelected(_building_type) => {
                    let e_id = building::spawn_ghost(game, game.input_state.mouse_world);
                    self.build_in_progress = Some(e_id);
                },
                Event::BuildingMenuOpened => {
                    if let Some(e_id) = self.build_in_progress {
                        building::despawn(game, e_id);
                        self.build_in_progress = None;
                    }
                },

            }
        }
    }
}
