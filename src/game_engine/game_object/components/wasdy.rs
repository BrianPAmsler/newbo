use glfw::Key;

use crate::game_engine::game_object::GameObjRef;

use super::{Component, SpriteComponent};

pub struct WASDy {
    pub speed: f32
}

impl Component for WASDy {
    fn init(&mut self, _engine: &mut crate::game_engine::Engine, _owner: GameObjRef) {}

    fn update(&mut self, _info: super::TickInfo, mut _owner: GameObjRef) {
        let sprite = &mut _owner.borrow_component_mut::<SpriteComponent>().unwrap().sprite;

        if _info.engine.get_key(Key::W) {
            sprite.y += self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::A) {
            sprite.x -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::S) {
            sprite.y -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::D) {
            sprite.x += self.speed * _info.delta_time as f32;
        }
    }

    fn fixed_update(&mut self, _info: super::TickInfo, _owner: GameObjRef) {}

    fn render(&mut self, _info: super::TickInfo, _owner: GameObjRef) {}
}