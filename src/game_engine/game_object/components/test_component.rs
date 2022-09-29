use crate::game_engine::{game_object::GameObject, self};

use super::Component;

#[derive(Default)]
pub struct TestComponent {
    count: i64,
    fixed_count: i64,
    last_count: i64,
    last_fixed_count: i64
}

impl Component for TestComponent {
    fn update(&mut self, _delta_time: f32, _owner: &GameObject) {
        self.count += 1;
        let mut current_tick = 0i64;
        game_engine::Graphics::get_gl_time(&mut current_tick);

        let delta = current_tick - self.last_count;

        if delta >= game_engine::NS_PER_S {
            let fps = (self.count * game_engine::NS_PER_S * 100) / delta;
            println!("FPS: {}.{}\n", fps / 100, fps % 100);

            self.count = 0;
            self.last_count = current_tick;
        }
    }

    fn fixed_update(&mut self, _delta_time: f32, _owner: &GameObject) {
        self.fixed_count += 1;
        let mut current_tick = 0i64;
        game_engine::Graphics::get_gl_time(&mut current_tick);

        let delta = current_tick - self.last_fixed_count;

        if delta >= game_engine::NS_PER_S {
            let fps = (self.fixed_count * game_engine::NS_PER_S * 100) / delta;
            println!("Fixed FPS: {}.{}\n", fps / 100, fps % 100);

            self.fixed_count = 0;
            self.last_fixed_count = current_tick;
        }
    }
}