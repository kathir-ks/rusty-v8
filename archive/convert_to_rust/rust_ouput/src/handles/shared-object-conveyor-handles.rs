// Converted from V8 C++ source files:
// Header: shared-object-conveyor-handles.h
// Implementation: shared-object-conveyor-handles.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::rc::Rc;
use std::cell::RefCell;

pub struct Isolate {}

impl Isolate {
    pub fn shared_space_isolate(&mut self) -> &mut Isolate {
        self
    }
    pub fn NewPersistentHandles(&mut self) -> Box<PersistentHandles> {
        Box::new(PersistentHandles{})
    }
}

pub struct Handle<T> {
    object: Rc<RefCell<T>>,
}

impl<T> Handle<T> {
    pub fn new(object: T) -> Self {
        Handle {
            object: Rc::new(RefCell::new(object)),
        }
    }

    pub fn clone(&self) -> Self {
        Handle {
            object: self.object.clone(),
        }
    }

    pub fn borrow(&self) -> std::cell::Ref<T> {
        self.object.borrow()
    }

    pub fn borrow_mut(&self) -> std::cell::RefMut<T> {
        self.object.borrow_mut()
    }
}

#[derive(Clone, Copy)]
pub struct HeapObject {
    id: u32,
}

impl HeapObject {
    pub fn new(id: u32) -> Self {
        HeapObject { id }
    }
}

#[derive(Clone, Copy)]
pub struct Tagged<T> {
    object: T,
}

impl<T> Tagged<T> {
    pub fn new(object: T) -> Self {
        Tagged { object }
    }
}

pub struct PersistentHandles {}

impl PersistentHandles {
    pub fn NewHandle(&self, shared_object: Tagged<HeapObject>) -> Handle<HeapObject> {
        Handle::new(shared_object.object)
    }
}

pub struct SharedObjectConveyorHandles {
    persistent_handles_: Box<PersistentHandles>,
    shared_objects_: Vec<Handle<HeapObject>>,
}

impl SharedObjectConveyorHandles {
    pub fn new(isolate: &mut Isolate) -> Self {
        SharedObjectConveyorHandles {
            persistent_handles_: isolate.shared_space_isolate().NewPersistentHandles(),
            shared_objects_: Vec::new(),
        }
    }

    pub fn persist(&mut self, shared_object: Tagged<HeapObject>) -> u32 {
        assert!(Self::is_shared(shared_object));
        let id = self.shared_objects_.len() as u32;
        self.shared_objects_.push(self.persistent_handles_.NewHandle(shared_object));
        id
    }

    pub fn has_persisted(&self, object_id: u32) -> bool {
        object_id < self.shared_objects_.len() as u32
    }

    pub fn get_persisted(&self, object_id: u32) -> Tagged<HeapObject> {
        assert!(self.has_persisted(object_id));
        Tagged::new(*self.shared_objects_[object_id as usize].borrow())
    }

    fn is_shared(shared_object: Tagged<HeapObject>) -> bool {
        true
    }
}
