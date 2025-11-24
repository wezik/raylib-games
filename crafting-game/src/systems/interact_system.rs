// use crate::Game;
//
#[derive(Default, Debug)]
pub struct Interact {
    pub on_interact: Option<fn(&mut crate::Game, crate::components::EntityId)>,
    pub activation_distance: f32,
}
//
// pub fn update(game: &mut Game) {
//     let Some(player) = game.player_controlled.first() else { return };
//     let Some(p_pos) = game.position.get(player) else { return };
//
//     let mut candidates = Vec::new();
//     for (&e_id, interact) in game.interact.iter() {
//         let Some(e_pos) = game.position.get(&e_id) else { continue };
//         let distance = e_pos.distance_to(*p_pos);
//         if distance <= interact.activation_distance {
//             candidates.push((e_id, interact, *e_pos, distance));
//         }
//     }
//
//     candidates.sort_by(|(_, _, _, a_dist), (_, _, _, b_dist)| {
//         a_dist.partial_cmp(b_dist).unwrap_or(std::cmp::Ordering::Equal)
//     });
//
//     if let Some((e_id, interact, pos, _)) = candidates.first() {
//         game.input_state.interact_gui_popup = Some(*pos);
//         if game.input_state.interact_intent {
//             if let Some(interact_callback) = interact.on_interact {
//                 interact_callback(game, *e_id);
//             }
//         }
//     } else {
//         game.input_state.interact_gui_popup = None;
//     }
//
//     game.input_state.interact_intent = false;
// }
