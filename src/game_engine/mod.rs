mod graphics;
pub mod game_object;
mod vectors;
mod err;

use game_object::*;
use glfw::{Key, Action};
pub use vectors::*;

use graphics::*;

use self::err::EngineError;

const VERTICES: [TerrainVertex; 3] = [[-0.5, -0.5, 0.0, 1.0, 0.0, 0.0], [0.5, -0.5, 0.0, 0.0, 1.0, 0.0], [0.0, 0.5, 0.0, 0.0, 0.0, 1.0]];

pub struct Engine {
    running: bool,
    fixed_tick_duration: i64,
    gfx: Graphics,
    root_object: GameObject,
    offset1: f32,
    offset2: f32
}

impl Engine {
    pub fn init_engine() -> Result<Engine, EngineError> {
        let result = Graphics::init_gl();
        if result.is_err() {
            return Err(result.err().unwrap());
        }

        let mut gfx = {
            let result = Graphics::create_window();

            if result.is_err() {
                return Err(result.err().unwrap())
            }

            result.unwrap()
        };
        
        gfx.buffer_verticies(&VERTICES);

        Ok(Engine { running: false, fixed_tick_duration: 1000000000i64 / 60, gfx: gfx, root_object: GameObject::create_empty("root", None), offset1: 0.0, offset2: 0.0 })
    }

    pub fn start_game_loop(&mut self) -> Result<(), EngineError> {
        if self.running {
            return Err("Game loop already running!".into());
        }

        self.running = true;

        let mut last_tick: i64 = 0;
        let mut last_fixed_tick: i64;

        Graphics::get_gl_time(&mut last_tick);
        last_fixed_tick = last_tick;
        
        let mut should_close = false;

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
                break;
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

        Ok(())
    }

    pub fn set_fixed_tick_rate(&mut self, tickrate:  u32) {
        self.fixed_tick_duration = 1000000000i64 / tickrate as i64;
    }

    pub fn get_root_object(&self) -> GameObject {
        self.root_object.share()
    }

    fn game_tick(&mut self, delta_time: f32) {
       self.offset1 += 0.1 * delta_time;
    }

    fn fixed_game_tick(&mut self, delta_time: f32) {
        self.offset2 -= 0.1 * delta_time;

        let stuff = self.root_object.get_all_children();
        for obj in stuff {
            obj.func_for_components(&|c| c.update(delta_time));
        }
    }
}