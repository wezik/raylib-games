use crate::{Game, SpawnEntity, components::building, systems::input_system::BuildPlaceIntent};

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildingType {
    Test,
}

pub fn update(game: &mut Game) {
    if let Some(intent) = game.input_state.build_intent {
        // transform build intent into a build_place_intent shadow
        let entity_id = game.spawn(SpawnEntity::BuildingShadow(
            intent.building_type,
            intent.position,
        ));
        game.input_state.build_intent = None;
        let follow_up = BuildPlaceIntent {
            entity_id,
            position: intent.position,
            confirmed: false,
        };
        game.input_state.build_place_intent = Some(follow_up);
    };

    if let Some(intent) = game.input_state.build_place_intent {
        game.position.insert(intent.entity_id, intent.position);
        if intent.confirmed {
            building::shadow_into_building(game, intent.entity_id);
            game.input_state.build_place_intent = None;
        }
    }
}
