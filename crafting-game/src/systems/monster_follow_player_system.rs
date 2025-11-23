use raylib::RaylibHandle;

use crate::Game;

#[derive(Debug)]
pub struct MoveTowards {
    pub range: f32,
}

pub fn update(game: &mut Game, rl: &RaylibHandle) {
    let Some(player) = game.player_controlled.first() else {
        return;
    };

    let target = {
        let Some(position) = game.position.get(player) else {
            return;
        };
        *position
    };

    for (entity_id, move_towards) in &game.move_towards {
        let Some(position) = game.position.get_mut(entity_id) else {
            continue;
        };

        let Some(speed) = game.speed.get(entity_id) else {
            continue;
        };

        let distance = target - *position;
        if distance.length() > move_towards.range {
            continue;
        }

        let direction = distance.normalized();
        *position += direction * *speed * rl.get_frame_time();
    }
}
