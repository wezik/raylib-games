use crate::{components::building, systems::input_system::BuildIntent, Game, SpawnEntity};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Test,
}

pub fn update(game: &mut Game) {
    match game.input_state.build_intent {
        None => {} // noop
        Some(intent) => match intent {
            BuildIntent::Initial(building_type, position) => {
                let entity_id = game.spawn(SpawnEntity::BuildingGhost(building_type, position));
                game.input_state.build_intent = BuildIntent::Ghost(entity_id, position).into();
            }
            BuildIntent::Ghost(entity_id, position) => {
                game.position.insert(entity_id, position);
            }
            BuildIntent::Confirmed(entity_id) => {
                building::ghost_into_building(game, entity_id);
                game.input_state.build_intent = None;
            }
            BuildIntent::Canceled(entity_id) => {
                building::despawn(game, entity_id);
                game.input_state.build_intent = None;
            }
        },
    }
}
