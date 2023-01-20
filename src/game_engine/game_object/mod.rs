use std::{rc::Rc, cell::{RefCell, Ref, RefMut}, collections::VecDeque};
pub mod components;

use components::Component;

use self::components::TickInfo;

use super::{Vector3, Engine};

pub struct _GameObject {
    name: String,
    pos: Vector3,
    rot: Vector3,
    scale: Vector3,
    components: Vec<Box<dyn Component>>,
    children: Vec<GameObject>,
    parent: Option<GameObject>
}

pub struct GameObjRef<'o> {
    obj: &'o mut _GameObject,
    obj_rc: GameObject
}

pub struct GameObject {
    obj: Rc<RefCell<_GameObject>>
}

impl GameObject {
    pub fn share(&self) -> GameObject {
        GameObject { obj: self.obj.clone() }
    }
}

impl std::fmt::Display for GameObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "GameObject{{\"{}\"}}", self.obj.as_ref().borrow().name)
    }
}

impl AsRef<GameObject> for GameObject {
    fn as_ref(&self) -> &GameObject {
        self
    }
}

impl GameObject {
    pub fn create_empty(name: String, parent: Option<GameObject>) -> GameObject {
        let newobj = GameObject { obj: Rc::new(RefCell::new(_GameObject {
            name,
            pos: Vector3::ZERO,
            rot: Vector3::ZERO,
            scale: Vector3::ONE,
            components: Vec::new(),
            children: Vec::new(),
            parent: None 
        }))};
        
        newobj.set_parent(parent);

        newobj
    }

    pub fn get_pos(&self) -> Vector3 {
        self.obj.as_ref().borrow().pos
    }

    pub fn set_pos(&self, pos: Vector3) {
        self.obj.borrow_mut().pos = pos;
    }

    pub(in crate::game_engine) fn init(&self, engine: &mut Engine) {
        let mut rf = self.obj.as_ref().borrow_mut();
        let mrf = &mut *rf;
        let comps: Vec<*mut Box<dyn Component>> = mrf.components.iter_mut().map(|x| x as *mut Box<dyn Component>).collect();
        for cmp in comps {
            // This is the best its gonna get I think
            unsafe { (&mut *cmp).init(engine, GameObjRef { obj: mrf, obj_rc: self.share()}) };
        }
    }

    pub(in crate::game_engine) fn update(&self, delta_time: f64, engine: &mut Engine) {
        let mut rf = self.obj.as_ref().borrow_mut();
        let mrf = &mut *rf;
        let comps: Vec<*mut Box<dyn Component>> = mrf.components.iter_mut().map(|x| x as *mut Box<dyn Component>).collect();
        for cmp in comps {
            // This is the best its gonna get I think
            unsafe { (&mut *cmp).update(TickInfo { delta_time, engine }, GameObjRef { obj: &mut *rf, obj_rc: self.share()} )};
        }
    }

    pub(in crate::game_engine) fn fixed_update(&self, delta_time: f64, engine: &mut Engine) {
        let mut rf = self.obj.as_ref().borrow_mut();
        let mrf = &mut *rf;
        let comps: Vec<*mut Box<dyn Component>> = mrf.components.iter_mut().map(|x| x as *mut Box<dyn Component>).collect();
        for cmp in comps {
            // This is the best its gonna get I think
            unsafe { (&mut *cmp).fixed_update(TickInfo { delta_time, engine }, GameObjRef { obj: &mut *rf, obj_rc: self.share()} )};
        }
    }

    pub(in crate::game_engine) fn render(&self, delta_time: f64, engine: &mut Engine) {
        let mut rf = self.obj.as_ref().borrow_mut();
        let mrf = &mut *rf;
        let comps: Vec<*mut Box<dyn Component>> = mrf.components.iter_mut().map(|x| x as *mut Box<dyn Component>).collect();
        for cmp in comps {
            // This is the best its gonna get I think
            unsafe { (&mut *cmp).render(TickInfo { delta_time, engine }, GameObjRef { obj: &mut *rf, obj_rc: self.share()} )};
        }
    }

    pub fn add_component(&self, component: Box<dyn Component>) {
        self.obj.borrow_mut().components.push(component);
    }

    pub fn get_parent(&self) -> Option<GameObject> {
        match &self.obj.as_ref().borrow().parent {
            Some(parent) => Some(parent.share()),
            None => None
        }
    }

    pub fn set_parent(&self, parent: Option<GameObject>) {
        {
            let old_parent = &self.obj.as_ref().borrow().parent;

            match old_parent {
                Some(p) => p.share().remove_child(self),
                None => ()
            }
        }

        match parent {
            Some(p) => {
                p.add_child(self.share());
                self.obj.borrow_mut().parent = Some(p);
            },
            None => self.obj.borrow_mut().parent = None
        }
    }

    pub fn get_children(&self) -> Vec<GameObject> {
        let mut v = Vec::new();
        v.reserve(self.obj.as_ref().borrow().children.len());

        for c in &self.obj.as_ref().borrow().children {
            v.push(c.share());
        }

        v
    }

    pub fn get_all_children(&self) -> Vec<GameObject> {
        let mut v = Vec::new();

        // BFS
        let mut q = VecDeque::new();
        q.push_back(self.share());
        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            v.reserve(current.obj.as_ref().borrow().children.len());

            for c in &current.obj.as_ref().borrow().children {
                q.push_back(c.share());
                v.push(c.share())
            }
        }

        v
    }

    pub fn add_child(&self, child: GameObject) {
        self.obj.borrow_mut().children.push(child.share());
    }

    pub fn remove_child(&self, child: &GameObject) {
        let mut idx = -1;
        for (i, c) in self.obj.borrow_mut().children.iter().enumerate() {
            if Rc::ptr_eq(&c.obj, &child.obj) {
                idx = i as i32;
                break;
            }
        }

        if idx == 1 {
            panic!("Child does not exist! This shouldn't happen!");
        }

        self.obj.borrow_mut().children.remove(idx as usize);
    }

    pub fn borrow_component<C: Component>(&self) -> Option<Ref<C>> {
        let b = self.obj.borrow();
        let out = Ref::filter_map(b, |rf| {
            for c in &rf.components {
                let r = c.downcast_ref();
                if r.is_some() {
                    return r;
                }
            }

            None
        });

        out.ok()
    }
    
    pub fn borrow_component_mut<C: Component>(&self) -> Option<RefMut<C>> {
        let b = self.obj.borrow_mut();
        let out = RefMut::filter_map(b, |rf| {
            for c in &mut rf.components {
                let r = c.downcast_mut();
                if r.is_some() {
                    return r;
                }
            }

            None
        });

        out.ok()
    }
}


impl GameObjRef<'_> {
    pub fn get_pos(&self) -> Vector3 {
        self.obj.pos
    }

    pub fn set_pos(&mut self, pos: Vector3) {
        self.obj.pos = pos;
    }

    pub fn add_component(&mut self, component: Box<dyn Component>) {
        self.obj.components.push(component);
    }

    pub fn get_parent(&self) -> Option<GameObject> {
        match &self.obj.parent {
            Some(parent) => Some(parent.share()),
            None => None
        }
    }

    pub fn set_parent(&mut self, parent: Option<GameObject>) {
        {
            let old_parent = &self.obj.parent;

            match old_parent {
                Some(p) => p.share().remove_child(&self.obj_rc),
                None => ()
            }
        }

        match parent {
            Some(p) => {
                p.add_child(self.obj_rc.share());
                self.obj.parent = Some(p);
            },
            None => self.obj.parent = None
        }
    }

    pub fn get_children(&self) -> Vec<GameObject> {
        let mut v = Vec::new();
        v.reserve(self.obj.children.len());

        for c in &self.obj.children {
            v.push(c.share());
        }

        v
    }

    pub fn get_all_children(&self) -> Vec<GameObject> {
        let mut v = Vec::new();

        // BFS
        let mut q = VecDeque::new();
        q.push_back(self.obj_rc.share());
        while q.len() > 0 {
            let current = q.pop_front().unwrap();
            v.reserve(current.obj.as_ref().borrow().children.len());

            for c in &current.obj.as_ref().borrow().children {
                q.push_back(c.share());
                v.push(c.share())
            }
        }

        v
    }

    pub fn add_child(&mut self, child: GameObject) {
        self.obj.children.push(child.share());
    }

    pub fn remove_child(&mut self, child: &GameObject) {
        let mut idx = -1;
        for (i, c) in self.obj.children.iter().enumerate() {
            if Rc::ptr_eq(&c.obj, &child.obj) {
                idx = i as i32;
                break;
            }
        }

        if idx == 1 {
            panic!("Child does not exist! This shouldn't happen!");
        }

        self.obj.children.remove(idx as usize);
    }

    pub fn borrow_component<C: Component>(&self) -> Option<&C> {
        for c in &self.obj.components {
            let r = c.downcast_ref();
            if r.is_some() {
                return r
            }
        }

        None
    }
    
    pub fn borrow_component_mut<C: Component>(&mut self) -> Option<&mut C> {
        for c in &mut self.obj.components {
            let r = c.downcast_mut();
            if r.is_some() {
                return r;
            }
        }

        None
    }

    pub fn as_rc(&self) -> GameObject {
        self.obj_rc.share()
    }
}