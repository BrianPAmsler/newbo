mod graphics;
mod game_object;
mod vectors;
mod err;

pub use game_object::*;
use glfw::{Key, Action};
pub use vectors::*;

use graphics::*;

const VERTICES: [TerrainVertex; 3] = [[-0.5, -0.5, 0.0, 1.0, 0.0, 0.0], [0.5, -0.5, 0.0, 0.0, 1.0, 0.0], [0.0, 0.5, 0.0, 0.0, 0.0, 1.0]];

pub struct Engine {
    running: bool,
    fixed_tick_duration: i64,
    gfx: Graphics,
    offset1: f32,
    offset2: f32
}

impl Engine {
    pub fn new() -> Engine {
        Graphics::init_gl().unwrap();

        let mut gfx = Graphics::create_window().unwrap();
        gfx.buffer_verticies(&VERTICES);

        Engine { running: false, fixed_tick_duration: 1000000000i64 / 60, gfx: gfx, offset1: 0.0, offset2: 0.0 }
    }

    pub fn start_game_loop(&mut self) {
        if self.running {
            panic!("Game loop already running!");
        }

        self.running = true;

        let mut last_tick: i64 = 0;
        let mut last_fixed_tick: i64;

        Graphics::get_gl_time(&mut last_tick);
        last_fixed_tick = last_tick;

        // Loop until the user closes the window
        while self.gfx.window_alive() {
            let mut should_close = false;
            // Poll for and process events
            for (_, event) in self.gfx.get_window_events() {
                println!("{:?}", event);
                match event {
                    glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                        should_close = true;
                    },
                    _ => {},
                }
            }

            if should_close {
                self.gfx.close_window();
            }

            // Game tick
            let mut current_time: i64 = 0;
            Graphics::get_gl_time(&mut current_time);

            self.game_tick((current_time - last_tick) as f32 / 1000000000f32);
            last_tick = current_time;

            if current_time - last_fixed_tick >= self.fixed_tick_duration {
                self.fixed_game_tick((current_time - last_fixed_tick) as f32 / 1000000000f32);
                last_fixed_tick = current_time;
            }

            // Render
            self.gfx.render(self.offset1, self.offset2);

            // Swap front and back buffers
            self.gfx.swap_window_buffers();
        }

        self.running = false;
    }

    // Default value is 60 tps, so this may never need to be used.
    #[allow(dead_code)]
    pub fn set_fixed_tick_rate(&mut self, tickrate:  u32) {
        self.fixed_tick_duration = 1000000000i64 / tickrate as i64;
    }

    fn game_tick(&mut self, delta_time: f32) {
       self.offset1 += 0.1 * delta_time;
    }

    fn fixed_game_tick(&mut self, delta_time: f32) {
        self.offset2 -= 0.1 * delta_time;
    }
}