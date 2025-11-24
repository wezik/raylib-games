use crate::{components::{building, EntityId}, Game};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Test,
}

// I know in "ECS" systems "are not supposed to" handle state, but idc it's easy and not messy that way
#[derive(Default, Debug)]
pub struct BuildingSystem {
    pub build_in_progress: Option<EntityId>,
}

impl BuildingSystem {
    pub fn update(&mut self, game: &mut Game) {
        // handle initialization
        if game.input_state.build_pressed && self.build_in_progress.is_none() {
            let e_id = building::spawn_ghost(game, game.input_state.mouse_world);
            self.build_in_progress = Some(e_id);
        }

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
}
