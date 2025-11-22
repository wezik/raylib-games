use raylib::math::Vector2;

#[derive(Default)]
pub struct EpicGamer {
    pub position: Vector2,
    pub speed: f32,
}

impl EpicGamer {
    pub fn perform(&mut self, action: PlayerAction, dt: f32) {
        match action {
            PlayerAction::Move(direction) => self.perform_move(direction, dt),
        };
    }

    fn perform_move(&mut self, direction: Vector2, dt: f32) {
        self.position += direction * self.speed * dt;
    }
}

pub enum PlayerAction {
    Move(Vector2), // Direction
}
