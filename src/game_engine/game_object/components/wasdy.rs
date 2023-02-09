use glfw::Key;

use crate::game_engine::game_object::GameObjRef;

use super::{Component, SpriteComponent};

pub struct WASDy {
    pub speed: f32
}

impl Component for WASDy {
    fn init(&mut self, _engine: &mut crate::game_engine::Engine, _owner: GameObjRef) {}

    fn update(&mut self, _info: super::TickInfo, mut _owner: GameObjRef) {
        if _info.engine.get_key(Key::W) {
            _owner.obj.pos.y += self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::A) {
            _owner.obj.pos.x -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::S) {
            _owner.obj.pos.y -= self.speed * _info.delta_time as f32;
        }
        if _info.engine.get_key(Key::D) {
            _owner.obj.pos.x += self.speed * _info.delta_time as f32;
        }
    }

    fn fixed_update(&mut self, _info: super::TickInfo, _owner: GameObjRef) {}

    fn render(&mut self, _info: super::TickInfo, _owner: GameObjRef) {}
}