mod test_component;
mod wasdy;
mod sprite;
mod collision_detector;

use downcast_rs::{Downcast, impl_downcast};
pub use test_component::TestComponent;
pub use wasdy::WASDy;
pub use sprite::SpriteComponent;
pub use collision_detector::Collider;

use crate::game_engine::Engine;

use super::GameObject;

pub struct TickInfo<'a> {
    pub(in crate::game_engine) delta_time: f64,
    pub(in crate::game_engine) engine: &'a mut Engine
}

pub trait Component: Downcast {
    fn init(&mut self, _engine: &mut Engine, _owner: &mut GameObject) {}
    fn update(&mut self, _info: TickInfo, _owner: &mut GameObject) {}
    fn fixed_update(&mut self, _info: TickInfo, _owner: &mut GameObject) {}
    fn render(&mut self, _info: TickInfo, _owner: &mut GameObject) {}
}

impl_downcast!(Component);