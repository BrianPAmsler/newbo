use std::{rc::Rc, cell::RefCell};

use crate::game_engine::{game_object::GameObject, self, Vector3};

use super::{Component, TickInfo};

#[derive(Default)]
pub struct TestComponent {
    count: i64,
    fixed_count: i64,
    last_update: f64,
    last_fixed_update: f64
}

impl Component for TestComponent {
    fn update(&mut self, _tick_info: TickInfo, _owner: Rc<RefCell<GameObject>>) {
        self.count += 1;
        let current_tick = game_engine::Graphics::get_glfw_time();

        let delta = current_tick - self.last_update;

        if delta >= 1.0 {
            let fps = self.count as f64 / delta;
            println!("FPS: {}\n", fps);

            self.count = 0;
            self.last_update = current_tick;
        }

        let cam = _tick_info.engine.get_gfx_mut().get_camera_mut();
        cam.pos += Vector3::new(0.0, 0.0, 0.0) * _tick_info.delta_time as f32;
        cam.rot += Vector3::new(0.0, 0.1, 0.0) * _tick_info.delta_time as f32;
    }

    fn fixed_update(&mut self, _tick_info: TickInfo, _owner: Rc<RefCell<GameObject>>) {
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