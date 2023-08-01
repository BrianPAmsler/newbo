use std::{rc::Rc, cell::RefCell, collections::VecDeque};
pub mod components;

use components::Component;

use self::components::{CompRc, Collider};

use super::{Vector3, Engine, Vector2};

pub struct GameObject {
    name: String,
    pos: Vector3,
    rot: Vector3,
    scale: Vector3,
    grounded: bool,
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
            grounded: false,
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
        // Idk if this is gonna be needed.
    }

    pub(in crate::game_engine) fn update(&mut self, delta_time: f32, engine: &mut Engine) {
        // Idk if this is gonna be needed.
    }

    pub(in crate::game_engine) fn fixed_update(&mut self, delta_time: f32, engine: &mut Engine) {
        // Idk if this is gonna be needed.
    }

    pub(in crate::game_engine) fn render(&mut self, delta_time: f32, engine: &mut Engine) {
        // Idk if this is gonna be needed.
    }

    pub fn is_grounded(&self) -> bool {
        self.grounded
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
            let t = c.as_ptr();
            // This should be fine since the contents of r are not being used and any borrow issues will still panic at the downcast_rc function call
            let r: Option<&C> = unsafe {(*t).downcast_ref()};

            if r.is_some() {
                return CompRc::downcast_rc(c);
            }
        }

        None
    }

    pub fn get_components<C: Component>(&self) -> Vec<CompRc<C>> {
        let mut vec = Vec::new();

        for c in &self.components {
            let t = c.as_ptr();
            // This should be fine since the contents of r are not being used and any borrow issues will still panic at the downcast_rc function call
            let r: Option<&C> = unsafe {(*t).downcast_ref()}; 

            if r.is_some() {
                vec.push(CompRc::downcast_rc(c).unwrap());
            }
        }

        vec
    }

    pub fn get_components_in_children<C: Component>(&self) -> Vec<CompRc<C>> {
        let mut vec = self.get_components();
        let children = self.get_all_children();

        for child in children {
            vec.extend(child.borrow().get_components().into_iter());
        }

        vec
    }

    pub fn get_all_components(&self) -> Vec<Rc<RefCell<dyn Component>>> {
        self.components.clone()
    }

    pub fn move_and_collide(obj: &Rc<RefCell<GameObject>>, offset: Vector3, engine: &mut Engine) {
        obj.borrow_mut().grounded = false;
        obj.borrow_mut().pos += offset;

        let c = obj.borrow().get_component();

        if c.is_some() {
            let c = c.unwrap();

            let all_colliders = engine.get_root_object().borrow().get_components_in_children::<Collider>();
            for collider in all_colliders {
                if !collider.ptr_eq(&c) {
                    let o_owner = collider.borrow().get_owner();
                    let s = &mut c.borrow_mut().hitbox;
                    let o = &mut collider.borrow_mut().hitbox;

                    s.pos = Vector2 { x: obj.borrow().pos.x, y: obj.borrow().pos.y };
                    o.pos = Vector2 { x: o_owner.borrow().pos.x, y: o_owner.borrow().pos.y };

                    let push = s.collide(o);

                    if push.is_some() {
                        obj.borrow_mut().pos += Vector3 { x: push.unwrap().x, y: push.unwrap().y, z: 0.0 };
                        if push.unwrap().y > 0.0 {
                            obj.borrow_mut().grounded = true;
                        }
                    }
                }
            }
        }
    }
}