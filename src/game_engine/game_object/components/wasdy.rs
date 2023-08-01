use std::{rc::Rc, cell::RefCell};
use glfw::Key;

use crate::game_engine::{game_object::GameObject, Vector3};

use super::{Component, SpriteComponent};

pub struct WASDy {
    pub speed: f32,
    pub velocity: f32,
    pub acc: f32
}

impl Component for WASDy {
    fn init(&mut self, _engine: &mut crate::game_engine::Engine, _owner: Rc<RefCell<GameObject>>) {}

    fn update(&mut self, _info: super::TickInfo, mut _owner: Rc<RefCell<GameObject>>) {
        let mut move_vector = Vector3::ZERO;
        
        if _info.engine.get_key(Key::W) {
            move_vector.y += self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::A) {
            move_vector.x -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::S) {
            move_vector.y -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::D) {
            move_vector.x += self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::Space) && _owner.borrow().is_grounded() {
            self.velocity = self.acc * -0.5;
        }

        if _info.engine.get_key(Key::Right) {
            let cam = _info.engine.get_gfx_mut().get_camera_mut();
            cam.pos += Vector3::new(0.1, 0.0, 0.0) * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::Left) {
            let cam = _info.engine.get_gfx_mut().get_camera_mut();
            cam.pos += Vector3::new(-0.1, 0.0, 0.0) * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::Up) {
            let cam = _info.engine.get_gfx_mut().get_camera_mut();
            cam.pos += Vector3::new(0.0, 0.1, 0.0) * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::Down) {
            let cam = _info.engine.get_gfx_mut().get_camera_mut();
            cam.pos += Vector3::new(0.0, -0.1, 0.0) * _info.delta_time as f32;
        }

        self.velocity += self.acc * _info.delta_time as f32;
        move_vector += (0.0, self.velocity * _info.delta_time as f32, 0.0).into();

        GameObject::move_and_collide(&_owner, move_vector, _info.engine);

        if _owner.borrow().is_grounded() {
            self.velocity = 0.0;
        }
    }

    fn fixed_update(&mut self, _info: super::TickInfo, _owner: Rc<RefCell<GameObject>>) {}

    fn render(&mut self, _info: super::TickInfo, _owner: Rc<RefCell<GameObject>>) {}
}