use raylib::{
    camera::Camera2D,
    ffi::{KeyboardKey, MouseButton},
    math::Vector2,
    RaylibHandle,
};

use crate::{components::EntityId, systems::building_system::BuildingType};

#[derive(Default, Debug)]
pub struct InputState {
    pub move_intent: Vector2,
    pub spawn_intent: Option<Vector2>,
    pub close_intent: bool,

    pub build_intent: Option<BuildIntent>,
    pub build_place_intent: Option<BuildPlaceIntent>,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BuildIntent {
    pub building_type: BuildingType,
    pub position: Vector2,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub struct BuildPlaceIntent {
    pub entity_id: EntityId,
    pub position: Vector2,
    pub confirmed: bool,
}

impl InputState {
    pub fn update(&mut self, rl: &RaylibHandle, camera: &Camera2D) {
        let mouse_pos = rl.get_mouse_position();
        let mouse_pos_in_world = rl.get_screen_to_world2D(mouse_pos, *camera);

        self.move_intent = Vector2::default();
        if rl.is_key_down(KeyboardKey::KEY_W) {
            self.move_intent.y -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_S) {
            self.move_intent.y += 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_A) {
            self.move_intent.x -= 1.0;
        }
        if rl.is_key_down(KeyboardKey::KEY_D) {
            self.move_intent.x += 1.0;
        }

        if rl.is_key_pressed(KeyboardKey::KEY_E) && self.spawn_intent.is_none() {
            self.spawn_intent = Some(mouse_pos_in_world);
        }

        if rl.is_key_pressed(KeyboardKey::KEY_B) {
            if self.build_place_intent.is_none() {
                let intent =
                    BuildIntent { building_type: BuildingType::Test, position: mouse_pos_in_world };
                self.build_intent = Some(intent);
            }
        }

        if let Some(intent) = &mut self.build_place_intent {
            intent.position = mouse_pos_in_world;
        }

        if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            if let Some(intent) = &mut self.build_place_intent {
                intent.confirmed = true;
            }
        }

        if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            if self.build_place_intent.is_some() {
                // TODO: This doesnt delete the shadow for the building since its an already
                // existing entity
                self.build_place_intent = None;
            } else {
                self.close_intent = true;
            }
        }

        if rl.window_should_close() {
            self.close_intent = true;
        }
    }
}
