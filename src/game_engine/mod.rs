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
    fixed_tick_duration: f64,
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

        Ok(Engine { running: false, fixed_tick_duration: 1.0 / 60.0, gfx: gfx, root_object: GameObject::create_empty("root", None), offset1: 0.0, offset2: 0.0 })
    }

    pub fn start_game_loop(&mut self) -> Result<(), EngineError> {
        if self.running {
            return Err("Game loop already running!".into());
        }

        self.running = true;

        let mut last_tick = Graphics::get_glfw_time();
        let mut last_fixed_tick: f64;

        let mut fixed_tick_overflow = 0.0;

        last_fixed_tick = last_tick;
        
        let mut should_close = false;

        // Loop until the user closes the window
        while self.gfx.window_alive() {
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
            let current_time = Graphics::get_glfw_time();

            self.game_tick(current_time - last_tick);
            last_tick = current_time;

            let fixed_diff = current_time - last_fixed_tick - self.fixed_tick_duration;

            // Add overflow to adjust for errors in timing
            if fixed_diff + fixed_tick_overflow >= 0.0 {
                fixed_tick_overflow = f64::max(0.0, fixed_diff * 2.0);
                self.fixed_game_tick(current_time - last_fixed_tick);
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

    pub fn set_fixed_tick_rate(&mut self, tickrate:  f64) {
        self.fixed_tick_duration = 1.0 / tickrate;
    }

    pub fn get_root_object(&self) -> GameObject {
        self.root_object.share()
    }

    pub fn get_gfx(&self) -> &Graphics {
        &self.gfx
    }

    fn init(&mut self) {
        let stuff = self.root_object.get_all_children();
        for obj in stuff {
            obj.init(&self);
        }
    }

    fn game_tick(&mut self, delta_time: f64) {
       self.offset1 += 0.01 * delta_time as f32;

       let stuff = self.root_object.get_all_children();
       for obj in stuff {
           obj.update(delta_time, &self);
       }
    }

    fn fixed_game_tick(&mut self, delta_time: f64) {
        self.offset2 -= 0.01 * delta_time as f32;

        let stuff = self.root_object.get_all_children();
        for obj in stuff {
            obj.fixed_update(delta_time, &self);
        }
    }
}