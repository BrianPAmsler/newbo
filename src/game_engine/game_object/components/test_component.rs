use std::{rc::Rc, cell::RefCell};

use crate::game_engine::{game_object::GameObject, self, Vector3};

use super::{Component, TickInfo};

#[derive(Default)]
pub struct TestComponent {
    count: i64,
    fixed_count: i64,
    last_update: f32,
    last_fixed_update: f32
}

impl Component for TestComponent {
    fn update(&mut self, _tick_info: TickInfo, _owner: Rc<RefCell<GameObject>>) {
        self.count += 1;
        let current_tick = game_engine::Graphics::get_glfw_time();

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f32 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }
    }

    fn fixed_update(&mut self, _tick_info: TickInfo, _owner: Rc<RefCell<GameObject>>) {
        self.fixed_count += 1;
        let current_tick = game_engine::Graphics::get_glfw_time();

        let delta = current_tick - self.last_fixed_update;

        if delta >= 1.0 {
            let fps = self.fixed_count as f32 / delta;
            println!("Fixed FPS: {}\n", fps);

            self.fixed_count = 0;
            self.last_fixed_update = current_tick;
        }
    }
}