use crate::game_engine::{game_object::GameObject, Engine};

use super::Component;
use std::{hash::Hash, rc::Rc, cell::RefCell};

pub struct Collider {
    owner: Option<Rc<RefCell<GameObject>>>,
    x: f32,
    y: f32,
    pub w: f32,
    pub h: f32,
    pub on_collide: Option<Box<dyn Fn(&Collider, &Collider)>>
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

    fn update(&mut self, _info: super::TickInfo, _owner: Rc<RefCell<GameObject>>) {
        // This is hacky as fuck i need a better solution.
        self.x = _owner.borrow().pos.x;
        self.y = _owner.borrow().pos.y;
    }
}

impl Collider {
    pub fn new(w: f32, h: f32, on_collide: Option<Box<dyn Fn(&Collider, &Collider)>>) -> Collider {
        Collider { owner: None, x: 0.0, y: 0.0, w, h, on_collide }
    }

    pub fn check_collision(&self, other: &Collider) -> bool {
        if self as *const Collider == other as *const Collider {
            return false;
        }

        let hw = self.w / 2.0;
        let hh = self.h / 2.0;
        let points = [(self.x - hw, self.y - hh), (self.x + hw, self.y - hh), (self.x - hw, self.y + hh), (self.x + hw, self.y + hh)];

        let hw2 = other.w / 2.0;
        let hh2 = other.h / 2.0;
        for point in points {
            //println!("point: ({}, {}), bounds: ({} to {}, {}, {})", point.0, point.1, other.x - hw2, other.x + hw2 , other.y - hh2, other.y + hh2);
            if point.0 > other.x - hw2 && point.0 < other.x + hw2 && point.1 > other.y - hh2 && point.1 < other.y + hh2 {
                return true;
            }
        }

        false
    }

    pub fn move_and_check_collision(&mut self, engine: &mut Engine) {
        let colliders = engine.get_root_object().borrow().get_components_in_children::<Collider>();

        for collider in colliders {
            let rf = collider.borrow();
            
        }
    }
}