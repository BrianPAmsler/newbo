use std::{rc::Rc, cell::RefCell};
use glfw::Key;

use crate::game_engine::game_object::GameObject;

use super::{Component, SpriteComponent};

pub struct WASDy {
    pub speed: f32
}

impl Component for WASDy {
    fn init(&mut self, _engine: &mut crate::game_engine::Engine, _owner: Rc<RefCell<GameObject>>) {}

    fn update(&mut self, _info: super::TickInfo, mut _owner: Rc<RefCell<GameObject>>) {
        if _info.engine.get_key(Key::W) {
            _owner.borrow_mut().pos.y += self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::A) {
            _owner.borrow_mut().pos.x -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::S) {
            _owner.borrow_mut().pos.y -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::D) {
            _owner.borrow_mut().pos.x += self.speed * _info.delta_time as f32;
        }

        if _info.engine.get_key(Key::Y) {
            let rc = _owner.borrow().get_component::<SpriteComponent>().unwrap();
            let mut sprite = rc.borrow_mut();

            sprite.sprite.w += 0.1 * _info.delta_time as f32;
        }
    }

    fn fixed_update(&mut self, _info: super::TickInfo, _owner: Rc<RefCell<GameObject>>) {}

    fn render(&mut self, _info: super::TickInfo, _owner: Rc<RefCell<GameObject>>) {}
}