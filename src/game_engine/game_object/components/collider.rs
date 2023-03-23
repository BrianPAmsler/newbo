use crate::game_engine::{game_object::GameObject, Engine, Polygon};

use super::Component;
use std::{hash::Hash, rc::Rc, cell::RefCell};

pub struct Collider {
    owner: Option<Rc<RefCell<GameObject>>>,
    pub hitbox: Polygon
}

impl PartialEq for Collider {
    fn eq(&self, other: &Self) -> bool {
        self as *const Collider == other as *const Collider
    }
}

impl Eq for Collider {}

impl Hash for Collider {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let ptr = self as *const Collider;
        ptr.hash(state);
    }
}

impl Component for Collider {
    fn init(&mut self, _engine: &mut Engine, _owner: Rc<RefCell<GameObject>>) {
        self.owner = Some(Rc::clone(&_owner));
    }
}

impl Collider {
    pub fn new(hitbox: Polygon) -> Collider {
        Collider { owner: None, hitbox }
    }

    pub fn get_owner(&self) -> Rc<RefCell<GameObject>> {
        self.owner.as_ref().unwrap().clone()
    }
}