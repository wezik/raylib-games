use raylib::{color::Color, prelude::{RaylibDraw, RaylibDrawHandle}};

use crate::{systems::{building_system::BuildingType, event_bus::Event}, Game};

#[derive(Default, Debug)]
pub struct UiSystem {
    building_selector: Option<BuildingSelector>,
}

#[derive(Default, Debug)]
struct BuildingSelector {
    selected: usize,
    options: Vec<BuildingType>,
}

impl UiSystem {
    pub fn update(&mut self, game: &mut Game) {
        if game.input_state.build_pressed {
            self.building_selector = Some(BuildingSelector {
                selected: 0,
                options: BuildingType::all(),
            });
            game.popup_active = true;
            game.event_bus.push(Event::BuildingMenuOpened);
        }

        let Some(building_selector) = &mut self.building_selector else { return };

        if game.input_state.confirm_pressed {
            let Some(selected) = building_selector.options.get(building_selector.selected) else { return };
            game.event_bus.push(Event::BuildingSelected(selected.clone()));
            self.building_selector = None;
            game.popup_active = false;
        } else if game.input_state.cancel_pressed {
            self.building_selector = None;
            game.popup_active = false;
        } else if game.input_state.move_left_pressed {
            building_selector.selected = (building_selector.selected + building_selector.options.len() - 1) % building_selector.options.len();
        } else if game.input_state.move_right_pressed {
            building_selector.selected = (building_selector.selected + 1) % building_selector.options.len();
        }
    }

    pub fn draw(&mut self, game: &Game, d: &mut RaylibDrawHandle) {
        if !game.popup_active { return }

        let Some(building_selector) = &self.building_selector else { return };
        d.draw_rectangle(0, 0, 800, 800, Color::BLACK);
        for (i, _building_type) in BuildingType::all().iter().enumerate() {
            let px = i as f32 * 100.0;
            if i == building_selector.selected {
                d.draw_rectangle(px as i32, 0, 80, 80, Color::RED);
            } else {
                d.draw_rectangle(px as i32, 0, 80, 80, Color::WHITE);
            }
        }
    }
}
