use rand::Rng;
use raylib::{camera::Camera2D, color::Color, ffi::{KeyboardKey, MouseButton}, math::{Rectangle, Vector2}, prelude::{RaylibDraw, RaylibMode2DExt}};

const DIRECTIONS: [(isize, isize); 8] = [
    (-1, 0),
    (-1, 1),
    (-1, -1),
    (0, -1),
    (0, 1),
    (1, -1),
    (1, 0),
    (1, 1),
];

#[derive(Clone)]
struct Cell {
    pub alive: bool,
    pub neighbors: u8,
    pub area: Rectangle,
}

#[derive(Clone)]
struct CellGrid {
    pub cells: Vec<Cell>,
    pub width: usize,
    pub height: usize,
}

impl CellGrid {
    fn new(width: usize, height: usize, cell_size: f32) -> Self {
        let mut cells = vec![];
        for y in 0..height {
            let mut row = vec![];
            for x in 0..width {
                row.push(Cell {
                    alive: false,
                    neighbors: 0,
                    area: Rectangle {
                        x: cell_size * x as f32,
                        y: cell_size * y as f32,
                        width: cell_size,
                        height: cell_size,
                    },
                });
            }
            cells.append(&mut row);
        }

        let mut rng = rand::rng();
        for y in 0..height {
            for x in 0..width {
                let cell = &mut cells[y * width + x];
                cell.alive = rng.random_bool(0.5);
                if cell.alive {
                    for (dx, dy) in DIRECTIONS {
                        let nx = x as isize + dx;
                        let ny = y as isize + dy;

                        if !(0..width as isize).contains(&nx) || !(0..height as isize).contains(&ny) {
                            continue; // out of bounds
                        }

                        cells[ny as usize * width + nx as usize].neighbors += 1;
                    }
                }
            }
        }
        Self{
            cells,
            width,
            height,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> &Cell {
        &self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, is_alive: bool) {
        let previous_alive = self.cells[y * self.width + x].alive;
        match (previous_alive, is_alive) {
            (true, false) | (false, true) => {
                for (dx, dy) in DIRECTIONS {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if !(0..self.width as isize).contains(&nx) || !(0..self.height as isize).contains(&ny) {
                        continue; // out of bounds
                    }
                    let neighbors = self.cells[ny as usize * self.width + nx as usize].neighbors;
                    let new_neighbors = match (neighbors, is_alive) {
                        (0, false) => 0,
                        (_, true) => neighbors + 1,
                        (_, false) => neighbors - 1,
                    };
                    self.cells[ny as usize * self.width + nx as usize].neighbors = new_neighbors;
                }
            },
            _ => {},
        }
        self.cells[y * self.width + x].alive = is_alive;
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }
}

fn main() {
    let (mut rl, thread) = raylib::init().build();

    let screen_width = 800;
    let screen_height = 600;
    rl.set_window_title(&thread, "Game of Life");
    rl.set_window_size(screen_width, screen_height);

    let mut grid = CellGrid::new(500, 500, 10.0);

    let mut accumulated_delta_time = 0.0;
    let mut updates_per_second = 2.0;

    let cell_size = 10.0;
    let mut pause = false;

    let mut camera = Camera2D::default();
    camera.zoom = 2.0;
    camera.offset = Vector2 { x: screen_width as f32 / 2.0, y: screen_height as f32 / 2.0 };
    camera.target = Vector2 { x: cell_size * grid.width() as f32 / 2.0, y: cell_size * grid.height() as f32 / 2.0 };

    while !rl.window_should_close() {
        accumulated_delta_time += rl.get_frame_time();

        let is_plus_down = rl.is_key_down(KeyboardKey::KEY_EQUAL);
        let is_minus_down = rl.is_key_down(KeyboardKey::KEY_MINUS);
        let is_space_pressed = rl.is_key_pressed(KeyboardKey::KEY_SPACE);
        let is_mouse_pressed = rl.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT);
        let is_mouse_right_down = rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT);
        let mouse_wheel = rl.get_mouse_wheel_move_v();

        if is_plus_down {
            updates_per_second += 0.1;
        } else if is_minus_down {
            updates_per_second -= 0.1;
        }

        if is_space_pressed {
            pause = !pause;
        }

        if mouse_wheel.y != 0.0 {
            let new_zoom = camera.zoom + mouse_wheel.y * 0.05;
            camera.zoom = if 0.1 > new_zoom { 0.1 } else { new_zoom };
        }

        if is_mouse_pressed {
            let mouse_pos = rl.get_mouse_position();
            let pos = rl.get_screen_to_world2D(mouse_pos, camera);
            let cell_x = (pos.x / cell_size) as usize;
            let cell_y = (pos.y / cell_size) as usize;
            grid.set(cell_x, cell_y, !grid.get(cell_x, cell_y).alive);
        }

        if is_mouse_right_down {
            camera.target += (rl.get_mouse_delta() / camera.zoom) * -1.0;
        }

        let fixed_delta_time = 1.0 / updates_per_second;

        while accumulated_delta_time >= fixed_delta_time {
            accumulated_delta_time -= fixed_delta_time;
            if pause { break; } // skip updates if paused

            // update
            let snapshot = grid.clone();
            for y in 0..snapshot.height() {
                for x in 0..snapshot.width() {
                    let snapshot_cell = snapshot.get(x, y);
                    let next_cell = match (snapshot_cell.alive, snapshot_cell.neighbors) {
                        (true, 2) | (_, 3) => true,
                        _ => false,
                    };

                    grid.set(x, y, next_cell);
                }
            }
        }

        let fps = rl.get_fps();
        // let font = rl.get_font_default();

        // drawing
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::GRAY);

        // 2D Mode
        {
            let mut d2 = d.begin_mode2D(camera);
            let world_width = screen_width as f32 / camera.zoom;
            let world_height = screen_height as f32 / camera.zoom;
            let camera_bounds = Rectangle {
                x: camera.target.x - world_width / 2.0 - cell_size,
                y: camera.target.y - world_height / 2.0 - cell_size,
                width: world_width + cell_size,
                height: world_height + cell_size,
            };

            for cell in grid.cells.iter() {
                if !camera_bounds.check_collision_recs(&cell.area) {
                    continue; // skip if not in the visible area
                }
                d2.draw_rectangle_rec(cell.area, if cell.alive { Color::GREEN } else { Color::BLACK });
            }
        }

        // Flat mode
        d.draw_text(
            &format!("FPS: {}", fps),
            0,
            0,
            20,
            Color::WHITE,
        );

        d.draw_text(
            &format!("Ticks/s: {:.2}", updates_per_second),
            0,
            25,
            20,
            Color::WHITE,
        );

        if pause {
            d.draw_text(
                &format!("PAUSED"),
                0,
                50,
                20,
                Color::WHITE,
            )
        }
    }
}
