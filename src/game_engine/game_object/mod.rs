use std::{rc::Rc, cell::RefCell, collections::VecDeque};
pub mod components;

use components::Component;

use self::components::{TickInfo, CompRc};

use super::{Vector3, Engine};

pub struct GameObject {
    name: String,
    pos: Vector3,
    rot: Vector3,
    scale: Vector3,
    components: Vec<Rc<RefCell<dyn Component>>>,
    children: Vec<Rc<RefCell<GameObject>>>,
    parent: Option<Rc<RefCell<GameObject>>>
}

impl std::fmt::Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameObject{{\"{}\"}}", self.name)
    }
}

impl AsRef<GameObject> for GameObject {
    fn as_ref(&self) -> &GameObject {
        self
    }
}

impl GameObject {
    pub fn create_empty(name: String, parent: Option<Rc<RefCell<GameObject>>>) -> Rc<RefCell<GameObject>> {
        let new_obj = Rc::new(RefCell::new(GameObject {
            name,
            pos: Vector3::ZERO,
            rot: Vector3::ZERO,
            scale: Vector3::ONE,
            components: Vec::new(),
            children: Vec::new(),
            parent: None
        }));

        GameObject::set_parent(new_obj.clone(), parent);

        new_obj
    }

    pub fn get_pos(&self) -> Vector3 {
        self.pos
    }

    pub fn set_pos(&mut self, pos: Vector3) {
        self.pos = pos;
    }

    pub(in crate::game_engine) fn init(&mut self, engine: &mut Engine) {
        let comps: Vec<Rc<RefCell<dyn Component>>> = self.components.iter_mut().map(|x| x.clone()).collect();
        for cmp in comps {
            cmp.borrow_mut().init(engine, self);
        }
    }

    pub(in crate::game_engine) fn update(&mut self, delta_time: f64, engine: &mut Engine) {
        let comps: Vec<Rc<RefCell<dyn Component>>> = self.components.iter_mut().map(|x| x.clone()).collect();
        for cmp in comps {
            cmp.borrow_mut().update(TickInfo { delta_time: delta_time, engine }, self);
        }
    }

    pub(in crate::game_engine) fn fixed_update(&mut self, delta_time: f64, engine: &mut Engine) {
        let comps: Vec<Rc<RefCell<dyn Component>>> = self.components.iter_mut().map(|x| x.clone()).collect();
        for cmp in comps {
            cmp.borrow_mut().fixed_update(TickInfo { delta_time: delta_time, engine }, self);
        }
    }

    pub(in crate::game_engine) fn render(&mut self, delta_time: f64, engine: &mut Engine) {
        let comps: Vec<Rc<RefCell<dyn Component>>> = self.components.iter_mut().map(|x| x.clone()).collect();
        for cmp in comps {
            cmp.borrow_mut().render(TickInfo { delta_time: delta_time, engine }, self);
        }
    }

    pub fn add_component<C: Component>(&mut self, component: C) {
        self.components.push(Rc::new(RefCell::new(component)));
    }

    pub fn get_parent(&self) -> Option<Rc<RefCell<GameObject>>> {
        match &self.parent {
            Some(parent) => Some(parent.clone()),
            None => None
        }
    }

    pub fn set_parent(slf: Rc<RefCell<GameObject>>, parent: Option<Rc<RefCell<GameObject>>>) {
        {
            let old_parent = &slf.borrow().parent;

            match old_parent {
                Some(p) => p.borrow_mut().remove_child(&*slf.borrow().as_ref()),
                None => ()
            }
        }

        match parent {
            Some(p) => {
                p.borrow_mut().add_child(slf.clone());
                slf.borrow_mut().parent = Some(p);
            },
            None => slf.borrow_mut().parent = None
        }
    }

    pub fn get_children(&self) -> Vec<Rc<RefCell<GameObject>>> {
        let mut v = Vec::new();
        v.reserve(self.children.len());

        for c in &self.children {
            v.push(c.clone());
        }

        v
    }

    pub fn get_all_children(&self) -> Vec<Rc<RefCell<GameObject>>> {
        let mut v = Vec::new();

        // BFS
        let mut q = VecDeque::new();
        for c in &self.children {
            q.push_back(c.clone());
            v.push(c.clone());
        }

        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            v.reserve(current.borrow().children.len());

            for c in &current.borrow().children {
                q.push_back(c.clone());
                v.push(c.clone())
            }
        }

        v
    }

    fn add_child(&mut self, child: Rc<RefCell<GameObject>>) {
        self.children.push(child.clone());
    }

    pub fn remove_child(&mut self, child: *const GameObject) {
        let mut idx = -1;
        for (i, c) in self.children.iter().enumerate() {
            if c.borrow().as_ref() as *const GameObject == child {
                idx = i as i32;
                break;
            }
        }

        if idx == -1 {
            panic!("Child does not exist! This shouldn't happen!");
        }

        self.children.remove(idx as usize);
    }

    pub fn get_component<C: Component>(&self) -> Option<CompRc<C>> {
        for c in &self.components {
            let t = c.borrow();
            let r: Option<&C> = t.downcast_ref();

            if r.is_some() {
                return CompRc::downcast_rc(c);
            }
        }

        None
    }

    pub fn get_components<C: Component>(&self) -> Vec<CompRc<C>> {
        let mut vec = Vec::new();

        for c in &self.components {
            let t = c.borrow();
            let r: Option<&C> = t.downcast_ref();

            if r.is_some() {
                vec.push(CompRc::downcast_rc(c).unwrap());
            }
        }

        vec
    }
}
