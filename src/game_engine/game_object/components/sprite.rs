use std::marker::PhantomData;

use crate::game_engine::{Sprite, game_object::GameObject};

use super::Component;

pub struct SpriteComponent {
    pub sprite: Sprite,
    pub index: usize,
    _pd: PhantomData<()>
}

impl SpriteComponent {
    pub fn new(sprite_number: usize, sprite_id: i32) -> SpriteComponent {
        SpriteComponent { sprite: Sprite { sprite_id, x: 0.0, y: 0.0, w: 1.0, h: 1.0 }, index: sprite_number, _pd: PhantomData }
    }
}

impl Component for SpriteComponent {
    fn init(&mut self, _engine: &mut crate::game_engine::Engine, _owner: &mut GameObject) {
        let gfx = _engine.get_gfx_mut();

        gfx.update_sprite(self.sprite, self.index);
    }

    fn update(&mut self, _info: super::TickInfo, _owner: &mut GameObject) {
        let engine = _info.engine;
        let gfx = engine.get_gfx_mut();

        let real_sprite = Sprite { sprite_id: self.sprite.sprite_id, x: _owner.pos.x, y: _owner.pos.y, w: self.sprite.w, h: self.sprite.h };

        gfx.update_sprite(real_sprite, self.index);
    }

    fn fixed_update(&mut self, _info: super::TickInfo, _owner: &mut GameObject) {}

    fn render(&mut self, _info: super::TickInfo, _owner: &mut GameObject) {}
}