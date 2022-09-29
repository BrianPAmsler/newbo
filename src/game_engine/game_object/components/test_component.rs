use crate::game_engine::{game_object::GameObject, self};

use super::Component;

#[derive(Default)]
pub struct TestComponent {
    count: i64,
    fixed_count: i64,
    last_update: f64,
    last_fixed_update: f64
}

impl Component for TestComponent {
    fn update(&mut self, _delta_time: f64, _owner: &GameObject) {
        self.count += 1;
        let current_tick = game_engine::Graphics::get_glfw_time();

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f64 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }
    }

    fn fixed_update(&mut self, _delta_time: f64, _owner: &GameObject) {
        self.fixed_count += 1;
        let current_tick = game_engine::Graphics::get_glfw_time();

        let delta = current_tick - self.last_fixed_update;

        if delta >= 1.0 {
            let fps = self.fixed_count as f64 / delta;
            println!("Fixed FPS: {}\n", fps);

            self.fixed_count = 0;
            self.last_fixed_update = current_tick;
        }
    }
}