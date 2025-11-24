use crate::Game;

#[derive(Debug)]
pub struct MoveTowards {
    pub range: f32,
}

pub fn update(game: &mut Game) {
    let Some(player) = game.player_controlled.first() else {
        return;
    };

    let Some(target) = game.position.get(player).cloned() else {
        return;
    };

    for (e_id, move_towards) in game.move_towards.iter() {
        let Some(pos) = game.position.get_mut(e_id) else {
            continue;
        };

        let Some(speed) = game.speed.get(e_id) else {
            continue;
        };

        let delta = target - *pos;
        let dist = delta.length();
        if dist > move_towards.range {
            continue;
        }
        let direction = delta.normalized();
        *pos += direction * *speed * game.delta_time;
    }
}
