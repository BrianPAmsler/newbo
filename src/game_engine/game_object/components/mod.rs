mod test_component;
mod wasdy;
mod sprite;
mod collider;

use std::{rc::Rc, cell::{RefCell, RefMut, Ref}, marker::PhantomData};

use downcast_rs::{Downcast, impl_downcast};
pub use test_component::TestComponent;
pub use wasdy::WASDy;
pub use sprite::SpriteComponent;
pub use collider::Collider;

use crate::game_engine::Engine;

use super::GameObject;

pub struct TickInfo<'a> {
    pub(in crate::game_engine) delta_time: f64,
    pub(in crate::game_engine) engine: &'a mut Engine
}

pub trait Component: Downcast {
    fn init(&mut self, _engine: &mut Engine, _owner: Rc<RefCell<GameObject>>) {}
    fn update(&mut self, _info: TickInfo, _owner: Rc<RefCell<GameObject>>) {}
    fn fixed_update(&mut self, _info: TickInfo, _owner: Rc<RefCell<GameObject>>) {}
    fn render(&mut self, _info: TickInfo, _owner: Rc<RefCell<GameObject>>) {}
}

impl_downcast!(Component);

pub struct CompRc<C: Component> {
    rc: Rc<RefCell<dyn Component>>,
    _pd: PhantomData<C>
}

impl<C: Component> CompRc<C> {
    pub fn downcast_rc(rc: &Rc<RefCell<dyn Component>>) -> Option<CompRc<C>> {
        let downcast = {
            let temp = rc.borrow();
            let op: Option<&C> = temp.downcast_ref();

            op.is_some()
        };

        if !downcast {
            return None;
        }

        Some(CompRc { rc: rc.clone(),_pd: PhantomData })
    }

    pub fn borrow(&self) -> Ref<C> {
        let borrow = self.rc.borrow();

        Ref::map(borrow, |x| {
            x.downcast_ref().unwrap()
        })
    }

    pub fn borrow_mut(&self) -> RefMut<C> {
        let borrow = self.rc.borrow_mut();

        RefMut::map(borrow, |x| {
            x.downcast_mut().unwrap()
        })
    }

    pub fn take_rc(self) -> Rc<RefCell<dyn Component>> {
        self.rc
    }
}