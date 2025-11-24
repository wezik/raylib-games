use raylib::{
    camera::Camera2D,
    ffi::{KeyboardKey, MouseButton},
    math::Vector2,
    RaylibHandle,
};

use crate::{components::EntityId, systems::building_system::BuildingType};

#[derive(Default, Debug)]
pub struct InputState {
    pub mouse_screen: Vector2,
    pub mouse_world: Vector2,
    pub mouse_lbm_pressed: bool,
    pub mouse_rmb_pressed: bool,
    // pub mouse_delta: Vector2,
    pub build_pressed: bool,
    pub cancel_pressed: bool,
    pub confirm_pressed: bool,
    pub dash_pressed: bool,
    pub interact_pressed: bool,

    pub move_up_pressed: bool,
    pub move_down_pressed: bool,
    pub move_left_pressed: bool,
    pub move_right_pressed: bool,

    // funny naming but it's still reasonable, don't care, might change later
    pub move_up_down: bool,
    pub move_down_down: bool,
    pub move_left_down: bool,
    pub move_right_down: bool,
}

#[derive(PartialEq, Clone, Copy, Debug)]
pub enum BuildIntent {
    Initial(BuildingType, Vector2),
    Ghost(EntityId, Vector2),
    Confirmed(EntityId),
    Canceled(EntityId),
}

impl InputState {
    pub fn update(&mut self, rl: &RaylibHandle, camera: &Camera2D) {
        self.mouse_screen = rl.get_mouse_position();
        self.mouse_world = rl.get_screen_to_world2D(self.mouse_screen, *camera);
        self.mouse_lbm_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        self.mouse_rmb_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_RIGHT);

        self.build_pressed = rl.is_key_pressed(KeyboardKey::KEY_B);
        self.cancel_pressed = rl.is_key_pressed(KeyboardKey::KEY_ESCAPE);
        self.dash_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        self.interact_pressed = rl.is_key_pressed(KeyboardKey::KEY_E);

        self.move_up_pressed = rl.is_key_pressed(KeyboardKey::KEY_W);
        self.move_down_pressed = rl.is_key_pressed(KeyboardKey::KEY_S);
        self.move_left_pressed = rl.is_key_pressed(KeyboardKey::KEY_A);
        self.move_right_pressed = rl.is_key_pressed(KeyboardKey::KEY_D);

        self.move_up_down = rl.is_key_down(KeyboardKey::KEY_W);
        self.move_down_down = rl.is_key_down(KeyboardKey::KEY_S);
        self.move_left_down = rl.is_key_down(KeyboardKey::KEY_A);
        self.move_right_down = rl.is_key_down(KeyboardKey::KEY_D);
    }
    // pub fn update(&mut self, rl: &RaylibHandle, camera: &Camera2D) {
    //     let mouse_pos = rl.get_mouse_position();
    //     let mouse_pos_in_world = rl.get_screen_to_world2D(mouse_pos, *camera);
    //
    //     self.move_intent = Vector2::default();
    //     if rl.is_key_down(KeyboardKey::KEY_W) {
    //         self.move_intent.y -= 1.0;
    //     }
    //     if rl.is_key_down(KeyboardKey::KEY_S) {
    //         self.move_intent.y += 1.0;
    //     }
    //     if rl.is_key_down(KeyboardKey::KEY_A) {
    //         self.move_intent.x -= 1.0;
    //     }
    //     if rl.is_key_down(KeyboardKey::KEY_D) {
    //         self.move_intent.x += 1.0;
    //     }
    //
    //     if rl.is_key_pressed(KeyboardKey::KEY_E) {
    //         self.interact_intent = true;
    //     }
    //
    //     if rl.is_key_pressed(KeyboardKey::KEY_R) && self.spawn_intent.is_none() {
    //         self.spawn_intent = Some(mouse_pos_in_world);
    //     }
    //
    //     if rl.is_key_pressed(KeyboardKey::KEY_B) {
    //         if self.build_intent.is_none() {
    //             self.build_intent =
    //                 Some(BuildIntent::Initial(BuildingType::Test, mouse_pos_in_world));
    //         }
    //     }
    //
    //     match self.build_intent {
    //         Some(BuildIntent::Ghost(entity_id, _)) => {
    //             self.build_intent = Some(BuildIntent::Ghost(entity_id, mouse_pos_in_world));
    //         }
    //         _ => {} // noop
    //     }
    //
    //     if rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
    //         match self.build_intent {
    //             Some(BuildIntent::Ghost(entity_id, _)) => {
    //                 self.build_intent = Some(BuildIntent::Confirmed(entity_id));
    //             }
    //             _ => {} // noop
    //         }
    //     }
    //
    //     if rl.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
    //         if let Some(BuildIntent::Ghost(entity_id, _)) = self.build_intent {
    //             self.build_intent = Some(BuildIntent::Canceled(entity_id));
    //         } else {
    //             self.close_intent = true;
    //         }
    //     }
    //
    //     if rl.window_should_close() {
    //         self.close_intent = true;
    //     }
    // }
}
