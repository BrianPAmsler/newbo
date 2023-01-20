mod test_component;
mod wasdy;
mod sprite;

use downcast_rs::{Downcast, impl_downcast};
pub use test_component::TestComponent;
pub use wasdy::WASDy;
pub use sprite::SpriteComponent;

use crate::game_engine::Engine;

use super::GameObjRef;

pub struct TickInfo<'a> {
    pub(in crate::game_engine) delta_time: f64,
    pub(in crate::game_engine) engine: &'a mut Engine
}

pub trait Component: Downcast {
    fn init(&mut self, _engine: &mut Engine, _owner: GameObjRef) {}
    fn update(&mut self, _info: TickInfo, _owner: GameObjRef) {}
    fn fixed_update(&mut self, _info: TickInfo, _owner: GameObjRef) {}
    fn render(&mut self, _info: TickInfo, _owner: GameObjRef) {}
}

impl_downcast!(Component);