// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod load_elimination {
    use std::any::Any;
    use std::collections::HashMap;
    use std::fmt;
    use std::hash::{Hash, Hasher};
    use std::rc::Rc;
    use std::cell::RefCell;
    use std::sync::Mutex;

    // Placeholder for v8::base
    pub mod base {
        pub trait CompilerSpecific {}
    }

    // Placeholder for v8::codegen
    pub mod codegen {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum MachineRepresentation {
            kNone,
            kWord8,
            kWord16,
            kWord32,
            kWord64,
            kFloat32,
            kFloat64,
            kSimd128,
            kTagged,
            kCompressed,
            kSandboxedPointer,
            kLast, // Not a real representation, only used to count the number of representations
        }
    }

    // Placeholder for v8::common
    pub mod common {
        pub mod globals {
            pub const kMaxUInt32: u64 = u32::MAX as u64;
        }
    }

    // Placeholder for v8::compiler
    pub mod compiler {
        use super::base::*;
        use super::codegen::MachineRepresentation;
        //use super::handles::MaybeHandle;
        use std::cell::RefCell;
        use std::rc::Rc;
        use std::collections::HashMap;
        use std::any::Any;

        pub trait GraphReducer {
            fn reducer_name(&self) -> &'static str;
            fn reduce(&mut self, node: &mut Node) -> Reduction;
        }

        pub struct Reduction {
            pub changed: bool,
            pub replacement: Option<Box<Node>>, // Box to own the node
        }

        impl Reduction {
            pub fn Replace(node: Node) -> Reduction {
                Reduction {
                    changed: true,
                    replacement: Some(Box::new(node)),
                }
            }
            pub fn Changed(node: Node) -> Reduction {
                Reduction {
                    changed: true,
                    replacement: Some(Box::new(node)),
                }
            }
            pub fn kNoChange() -> Reduction {
                Reduction {
                    changed: false,
                    replacement: None,
                }
            }

            pub fn Reduce(node: Node) -> Reduction {
                Reduction {
                    changed: true,
                    replacement: Some(Box::new(node)),
                }
            }
        }

        // Placeholder for SimplifiedOperator
        pub struct SimplifiedOperator {}

        // Placeholder for FieldAccess
        #[derive(Debug, Clone)]
        pub struct FieldAccess {}

        // Placeholder for TFGraph
        pub struct TFGraph {}

        // Placeholder for JSGraph
        pub struct JSGraph {}

        // Placeholder for CommonOperatorBuilder
        pub struct CommonOperatorBuilder {}

        // Placeholder for Isolate
        pub struct Isolate {}

        // Placeholder for Factory
        pub struct Factory {}

        // Placeholder for Map
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Map {}

        // Placeholder for ZoneRefSet
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct ZoneRefSet<T> {
            items: Vec<T>,
        }

        impl<T> ZoneRefSet<T> {
            pub fn new() -> Self {
                ZoneRefSet { items: Vec::new() }
            }

            pub fn insert(&mut self, item: T)
            where
                T: Eq + Hash + Clone,
            {
                if !self.items.contains(&item) {
                    self.items.push(item);
                }
            }

            pub fn contains(&self, item: &T) -> bool
            where
                T: PartialEq,
            {
                self.items.contains(item)
            }
        }

        // Placeholder for Name
        #[derive(Debug, Clone)]
        pub struct Name {}

        // Placeholder for MaybeHandle
        #[derive(Debug, Clone)]
        pub struct MaybeHandle<T> {
            address: usize, // Representing the memory address
            phantom: std::marker::PhantomData<T>,
        }

        impl<T> MaybeHandle<T> {
            pub fn new(address: usize) -> Self {
                MaybeHandle {
                    address,
                    phantom: std::marker::PhantomData,
                }
            }
            pub fn empty() -> Self {
                MaybeHandle {
                    address: 0,
                    phantom: std::marker::PhantomData,
                }
            }

            pub fn address(&self) -> usize {
                self.address
            }
        }

        // Placeholder for ConstFieldInfo
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum ConstFieldInfo {
            None,
            // Add other variants as needed
        }

        // Placeholder for JSHeapBroker
        pub struct JSHeapBroker {}

        // Placeholder for Node
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct NodeId(usize); // Using usize to represent Node ID
        #[derive(Debug, Clone)]
        pub struct Node {
            id: NodeId,
            dead: bool,
        }
        impl Node {
            pub fn new(id: NodeId) -> Self {
                Node {
                    id,
                    dead: false,
                }
            }
            pub fn IsDead(&self) -> bool {
                self.dead
            }
            pub fn GetId(&self) -> NodeId {
                self.id
            }
        }

        // Placeholder for Editor
        pub trait Editor {
            fn ReplaceInput(&mut self, node: &mut Node, index: usize, replacement: &Node);
            fn ReplaceUses(&mut self, node: &mut Node, replacement: &Node);
            fn Kill(&mut self, node: &mut Node);
        }

        // Implement Editor trait for LoadElimination
        impl Editor for LoadElimination {
            fn ReplaceInput(&mut self, node: &mut Node, index: usize, replacement: &Node) {
                //TODO: Implement ReplaceInput
            }
            fn ReplaceUses(&mut self, node: &mut Node, replacement: &Node) {
                //TODO: Implement ReplaceUses
            }
            fn Kill(&mut self, node: &mut Node) {
                //TODO: Implement Kill
                node.dead = true;
            }
        }

        pub trait AdvancedReducer: GraphReducer {}

        pub struct LoadElimination {
            editor: Box<dyn Editor>,
            broker_: *mut JSHeapBroker, // Raw pointer, needs careful management
            node_states_: RefCell<AbstractStateForEffectNodes>,
            jsgraph_: *mut JSGraph, // Raw pointer
        }

        impl LoadElimination {
            pub fn new(
                editor: Box<dyn Editor>,
                broker: *mut JSHeapBroker,
                jsgraph: *mut JSGraph,
                zone: *mut Zone,
            ) -> Self {
                LoadElimination {
                    editor,
                    broker_: broker,
                    node_states_: RefCell::new(AbstractStateForEffectNodes::new(zone)),
                    jsgraph_: jsgraph,
                }
            }

            pub fn reducer_name(&self) -> &'static str {
                "LoadElimination"
            }

            pub fn reduce(&mut self, node: &mut Node) -> Reduction {
                self.Reduce(node)
            }

            fn Reduce(&mut self, node: &mut Node) -> Reduction {
                // Match on the node type and call the appropriate Reduce function.
                // This is just a placeholder implementation.
                // Add more cases based on the original C++ code.
                //println!("Reducing node: {:?}", node);
                Reduction::kNoChange()
            }
        }

        impl AdvancedReducer for LoadElimination {}

        const K_MAX_TRACKED_ELEMENTS: usize = 8;

        #[derive(Debug, Clone)]
        struct Element {
            object: Option<Node>,
            index: Option<Node>,
            value: Option<Node>,
            representation: MachineRepresentation,
        }

        impl Element {
            fn new(
                object: Option<Node>,
                index: Option<Node>,
                value: Option<Node>,
                representation: MachineRepresentation,
            ) -> Self {
                Element {
                    object,
                    index,
                    value,
                    representation,
                }
            }
        }

        pub struct AbstractElements {
            elements_: [Element; K_MAX_TRACKED_ELEMENTS],
            next_index_: usize,
            zone: *mut Zone,
        }

        impl AbstractElements {
            fn new(zone: *mut Zone) -> Self {
                AbstractElements {
                    elements_: [
                        Element::new(
                            None,
                            None,
                            None,
                            MachineRepresentation::kNone
                        );
                        K_MAX_TRACKED_ELEMENTS
                    ],
                    next_index_: 0,
                    zone,
                }
            }

            fn with_element(
                zone: *mut Zone,
                object: Node,
                index: Node,
                value: Node,
                representation: MachineRepresentation,
            ) -> Self {
                let mut abstract_elements = AbstractElements::new(zone);
                abstract_elements.elements_[abstract_elements.next_index_] =
                    Element::new(Some(object), Some(index), Some(value), representation);
                abstract_elements.next_index_ = (abstract_elements.next_index_ + 1) % K_MAX_TRACKED_ELEMENTS;
                abstract_elements
            }

            fn extend(
                &self,
                zone: *mut Zone,
                object: Node,
                index: Node,
                value: Node,
                representation: MachineRepresentation,
            ) -> Box<AbstractElements> {
                let mut new_elements = AbstractElements::new(zone);
                new_elements.elements_.copy_from_slice(&self.elements_);
                new_elements.next_index_ = self.next_index_;
                new_elements.elements_[new_elements.next_index_] =
                    Element::new(Some(object), Some(index), Some(value), representation);
                new_elements.next_index_ = (new_elements.next_index_ + 1) % K_MAX_TRACKED_ELEMENTS;
                Box::new(new_elements)
            }

            fn lookup(
                &self,
                object: &Node,
                index: &Node,
                representation: MachineRepresentation,
            ) -> Option<&Node> {
                for element in &self.elements_ {
                    if let (Some(element_object), Some(element_index), Some(element_value)) =
                        (&element.object, &element.index, &element.value)
                    {
                        if element_object.GetId() == object.GetId()
                            && element_index.GetId() == index.GetId()
                            && element.representation == representation
                        {
                            return Some(element_value);
                        }
                    }
                }
                None
            }

            fn kill(&self, zone: *mut Zone, object: &Node, index: &Node) -> Box<AbstractElements> {
                let mut new_elements = AbstractElements::new(zone);
                new_elements.elements_.copy_from_slice(&self.elements_);
                new_elements.next_index_ = self.next_index_;
                for element in &mut new_elements.elements_ {
                    if let (Some(element_object), Some(element_index)) =
                        (&element.object, &element.index)
                    {
                        if element_object.GetId() == object.GetId()
                            && element_index.GetId() == index.GetId()
                        {
                            element.object = None;
                            element.index = None;
                            element.value = None;
                            element.representation = MachineRepresentation::kNone;
                        }
                    }
                }
                Box::new(new_elements)
            }

            fn equals(&self, that: &AbstractElements) -> bool {
                for i in 0..K_MAX_TRACKED_ELEMENTS {
                    if let (Some(this_object), Some(this_index), Some(this_value)) =
                        (&self.elements_[i].object, &self.elements_[i].index, &self.elements_[i].value)
                    {
                        if let (Some(that_object), Some(that_index), Some(that_value)) =
                            (&that.elements_[i].object, &that.elements_[i].index, &that.elements_[i].value)
                        {
                            if this_object.GetId() != that_object.GetId()
                                || this_index.GetId() != that_index.GetId()
                                || this_value.GetId() != that_value.GetId()
                                || self.elements_[i].representation != that.elements_[i].representation
                            {
                                return false;
                            }
                        } else {
                            return false;
                        }
                    } else {
                        if that.elements_[i].object.is_some()
                            || that.elements_[i].index.is_some()
                            || that.elements_[i].value.is_some()
                        {
                            return false;
                        }
                    }
                }
                true
            }

            fn merge(&self, that: &AbstractElements, zone: *mut Zone) -> Box<AbstractElements> {
                if self.equals(that) {
                    return Box::new(AbstractElements::new(zone)); // Return new empty abstract elements
                }

                // Placeholder implementation - needs a more sophisticated merge strategy
                let mut new_elements = AbstractElements::new(zone);
                new_elements.elements_.copy_from_slice(&self.elements_);
                new_elements.next_index_ = self.next_index_;

                for i in 0..K_MAX_TRACKED_ELEMENTS {
                    if let (Some(that_object), Some(that_index), Some(that_value)) =
                        (&that.elements_[i].object, &that.elements_[i].index, &that.elements_[i].value)
                    {
                        if let (Some(this_object), Some(this_index), Some(this_value)) =
                            (&self.elements_[i].object, &self.elements_[i].index, &self.elements_[i].value)
                        {
                            if this_object.GetId() != that_object.GetId()
                                || this_index.GetId() != that_index.GetId()
                                || this_value.GetId() != that_value.GetId()
                                || self.elements_[i].representation != that.elements_[i].representation
                            {
                                // If elements are different, clear the entry
                                new_elements.elements_[i].object = None;
                                new_elements.elements_[i].index = None;
                                new_elements.elements_[i].value = None;
                                new_elements.elements_[i].representation = MachineRepresentation::kNone;
                            }
                        }
                    } else if self.elements_[i].object.is_some()
                        || self.elements_[i].index.is_some()
                        || self.elements_[i].value.is_some()
                    {
                        new_elements.elements_[i].object = None;
                        new_elements.elements_[i].index = None;
                        new_elements.elements_[i].value = None;
                        new_elements.elements_[i].representation = MachineRepresentation::kNone;
                    }
                }

                Box::new(new_elements)
            }

            fn print(&self) {
                println!("AbstractElements:");
                for (i, element) in self.elements_.iter().enumerate() {
                    println!(
                        "  Element[{}]: Object={:?}, Index={:?}, Value={:?}, Representation={:?}",
                        i, element.object, element.index, element.value, element.representation
                    );
                }
                println!("  Next Index: {}", self.next_index_);
            }
        }

        // Placeholder for AliasStateInfo
        pub struct AliasStateInfo {}

        #[derive(Debug, Clone, PartialEq, Eq)]
        struct FieldInfo {
            value: Option<Node>,
            representation: MachineRepresentation,
            name: MaybeHandle<Name>,
            const_field_info: ConstFieldInfo,
        }

        impl FieldInfo {
            fn new(
                value: Option<Node>,
                representation: MachineRepresentation,
                name: MaybeHandle<Name>,
                const_field_info: ConstFieldInfo,
            ) -> Self {
                FieldInfo {
                    value,
                    representation,
                    name,
                    const_field_info,
                }
            }
        }

        const K_MAX_TRACKED_FIELDS_PER_OBJECT: usize = 32;
        const K_MAX_TRACKED_OBJECTS: usize = 100;
        const K_MAX_TRACKED_FIELDS: usize = 300;

        pub struct AbstractField {
            info_for_node_: RefCell<HashMap<NodeId, FieldInfo>>,
            zone: *mut Zone,
        }

        impl AbstractField {
            fn new(zone: *mut Zone) -> Self {
                AbstractField {
                    info_for_node_: RefCell::new(HashMap::new()),
                    zone,
                }
            }

            fn with_info(zone: *mut Zone, object: Node, info: FieldInfo) -> Self {
                let mut abstract_field = AbstractField::new(zone);
                abstract_field
                    .info_for_node_
                    .borrow_mut()
                    .insert(object.GetId(), info);
                abstract_field
            }

            fn extend(
                &self,
                zone: *mut Zone,
                object: Node,
                info: FieldInfo,
                current_field_count: i32,
            ) -> Box<AbstractField> {
                let mut new_field = AbstractField::new(zone);
                new_field
                    .info_for_node_
                    .borrow_mut()
                    .extend(self.info_for_node_.borrow().clone());

                if (current_field_count >= K_MAX_TRACKED_FIELDS as i32
                    && new_field.info_for_node_.borrow().len() > 0)
                    || new_field.info_for_node_.borrow().len() >= K_MAX_TRACKED_OBJECTS
                {
                    if let Some((first_key, _)) = new_field.info_for_node_.borrow().iter().next() {
                        new_field.info_for_node_.borrow_mut().remove(first_key);
                    }
                }
                new_field.info_for_node_.borrow_mut().insert(object.GetId(), info);
                Box::new(new_field)
            }

            fn lookup(&self, object: &Node) -> Option<FieldInfo> {
                self.info_for_node_.borrow().get(&object.GetId()).cloned()
            }

            fn kill_const(&self, zone: *mut Zone, object: &Node) -> Box<AbstractField> {
                let mut new_field = AbstractField::new(zone);
                new_field
                    .info_for_node_
                    .borrow_mut()
                    .extend(self.info_for_node_.borrow().clone());

                new_field.info_for_node_.borrow_mut().remove(&object.GetId());
                Box::new(new_field)
            }

            fn kill(
                &self,
                zone: *mut Zone,
                alias_info: &AliasStateInfo,
                name: MaybeHandle<Name>,
            ) -> Box<AbstractField> {
                // Placeholder implementation
                let mut new_field = AbstractField::new(zone);
                new_field
                    .info_for_node_
                    .borrow_mut()
                    .extend(self.info_for_node_.borrow().clone());
                // Need AliasStateInfo to determine which objects to kill
                Box::new(new_field)
            }

            fn equals(&self, that: &AbstractField) -> bool {
                self.info_for_node_.borrow().eq(&that.info_for_node_.borrow())
            }

            fn merge(&self, that: &AbstractField, zone: *mut Zone, count: &mut i32) -> Box<AbstractField> {
                if self.equals(that) {
                    return Box::new(AbstractField::new(zone));
                }
                let mut copy = AbstractField::new(zone);

                for (this_object_id, this_second) in self.info_for_node_.borrow().iter() {
                    let this_object_id = *this_object_id;
                    let this_object = Node::new(this_object_id);
                    if this_object.IsDead() {
                        continue;
                    }
                    if let Some(that_second) = that.info_for_node_.borrow().get(&this_object_id) {
                        if *that_second == *this_second {
                            copy.info_for_node_.borrow_mut().insert(this_object_id, this_second.clone());
                            *count += 1;
                        }
                    }
                }

                Box::new(copy)
            }

            fn print(&self) {
                println!("AbstractField:");
                for (node_id, field_info) in self.info_for_node_.borrow().iter() {
                    println!(
                        "  NodeId({}): Value={:?}, Representation={:?}, Name={:?}",
                        node_id.0, field_info.value, field_info.representation, field_info.name
                    );
                }
            }

            fn count(&self) -> i32 {
                self.info_for_node_.borrow().len() as i32
            }
        }

        pub struct AbstractMaps {
            info_for_node_: RefCell<HashMap<NodeId, ZoneRefSet<Map>>>,
            zone: *mut Zone,
        }

        impl AbstractMaps {
            fn new(zone: *mut Zone) -> Self {
                AbstractMaps {
                    info_for_node_: RefCell::new(HashMap::new()),
                    zone,
                }
            }

            fn with_maps(zone: *mut Zone, object: Node, maps: ZoneRefSet<Map>) -> Self {
                let mut abstract_maps = AbstractMaps::new(zone);
                abstract_maps.info_for_node_.borrow_mut().insert(object.GetId(), maps);
                abstract_maps
            }

            fn extend(&self, zone: *mut Zone, object: Node, maps: ZoneRefSet<Map>) -> Box<AbstractMaps> {
                let mut new_maps = AbstractMaps::new(zone);
                new_maps
                    .info_for_node_
                    .borrow_mut()
                    .extend(self.info_for_node_.borrow().clone());
                new_maps.info_for_node_.borrow_mut().insert(object.GetId(), maps);
                Box::new(new_maps)
            }

            fn lookup(&self, object: &Node, object_maps: &mut ZoneRefSet<Map>) -> bool {
                if let Some(maps) = self.info_for_node_.borrow().get(&object.GetId()) {
                    *object_maps = maps.clone();
                    true
                } else {
                    false
                }
            }

            fn kill(&self, zone: *mut Zone, alias_info: &AliasStateInfo) -> Box<AbstractMaps> {
                // Placeholder implementation - needs alias analysis
                let mut new_maps = AbstractMaps::new(zone);
                new_maps
                    .info_for_node_
                    .borrow_mut()
                    .extend(self.info_for_node_.borrow().clone());
                Box::new(new_maps)
            }

            fn equals(&self, that: &AbstractMaps) -> bool {
                self.info_for_node_.borrow().eq(&that.info_for_node_.borrow())
            }

            fn merge(&self, that: &AbstractMaps, zone: *mut Zone) -> Box<AbstractMaps> {
                if self.equals(that) {
                    return Box::new(AbstractMaps::new(zone));
                }

                let mut new_maps = AbstractMaps::new(zone);
                for (node_id, maps) in self.info_for_node_.borrow().iter() {
                    if let Some(that_maps) = that.info_for_node_.borrow().get(node_id) {
                        if maps == that_maps {
                            new_maps.info_for_node_.borrow_mut().insert(*node_id, maps.clone());
                        }
                    }
                }
                Box::new(new_maps)
            }

            fn print(&self) {
                println!("AbstractMaps:");
                for (node_id, maps) in self.info_for_node_.borrow().iter() {
                    println!("  NodeId({}): Maps={:?}", node_id.0, maps);
                }
            }
        }

        #[derive(Debug, Clone, PartialEq, Eq)]
        pub struct IndexRange {
            begin_: i32,
            end_: i32,
        }

        impl IndexRange {
            fn new(begin: i32, size: i32) -> Self {
                assert!(begin >= 0);
                assert!(size >= 1);
                let end = begin + size;
                if end > K_MAX_TRACKED_FIELDS_PER_OBJECT as i32 {
                    return IndexRange::invalid();
                }
                IndexRange {
                    begin_: begin,
                    end_: end,
                }
            }

            fn invalid() -> Self {
                IndexRange {
                    begin_: -1,
                    end_: -1,
                }
            }

            fn is_valid(&self) -> bool {
                self.begin_ != -1 && self.end_ != -1
            }
        }

        pub struct AbstractState {
            elements_: Option<Box<AbstractElements>>,
            fields_: [Option<Box<AbstractField>>; K_MAX_TRACKED_FIELDS_PER_OBJECT],
            const_fields_: [Option<Box<AbstractField>>; K_MAX_TRACKED_FIELDS_PER_OBJECT],
            maps_: Option<Box<AbstractMaps>>,
            const_fields_count_: i32,
            fields_count_: i32,
            zone: *mut Zone,
        }

        impl AbstractState {
            fn new(zone: *mut Zone) -> Self {
                AbstractState {
                    elements_: None,
                    fields_: Default::default(),
                    const_fields_: Default::default(),
                    maps_: None,
                    const_fields_count_: 0,
                    fields_count_: 0,
                    zone,
                }
            }

            fn empty_state() -> &'static AbstractState {
                lazy_static::lazy_static! {
                    static ref EMPTY_STATE: AbstractState = {
                        // Dummy zone for empty_state_
                        let mut zone = Zone::new();
                        AbstractState::new(&mut zone)
                    };
                }
                &EMPTY_STATE
            }

            fn equals(&self, that: &AbstractState) -> bool {
                if self.elements_.is_some() != that.elements_.is_some() {
                    return false;
                }
                if let (Some(elements), Some(that_elements)) = (&self.elements_, &that.elements_) {
                    if !elements.equals(that_elements) {
                        return false;
                    }
                }

                for i in 0..K_MAX_TRACKED_FIELDS_PER_OBJECT {
                    if self.fields_[i].is_some() != that.fields_[i].is_some() {
                        return false;
                    }
                    if let (Some(field), Some(that_field)) = (&self.fields_[i], &that.fields_[i]) {
                        if !field.equals(that_field) {
                            return false;
                        }
                    }

                    if self.const_fields_[i].is_some() != that.const_fields_[i].is_some() {
                        return false;
                    }
                    if let (Some(const_field), Some(that_const_field)) = (&self.const_fields_[i], &that.const_fields_[i]) {
                        if !const_field.equals(that_const_field) {
                            return false;
                        }
                    }
                }

                if self.maps_.is_some() != that.maps_.is_some() {
                    return false;
                }
                if let (Some(maps), Some(that_maps)) = (&self.maps_, &that.maps_) {
                    if !maps.equals(that_maps) {
                        return false;
                    }
                }
                true
            }

            fn merge(&mut self, that: &AbstractState, zone: *mut Zone) {
                if self.elements_.is_none() && that.elements_.is_some() {
                    self.elements_ = Some(Box::new(AbstractElements::new(zone)));
                }
                if let (Some(this_elements), Some(that_elements)) = (&mut self.elements_, &that.elements_) {
                    let merged_elements = this_elements.merge(that_elements, zone);
                    self.elements_ = Some(merged_elements);
                }

                for i in 0..K_MAX_TRACKED_FIELDS_PER_OBJECT {
                    let mut count = 0;
                    if self.fields_[i].is_none() && that.fields_[i].is_some() {
                        self.fields_[i] = Some(Box::new(AbstractField::new(zone)));
                    }
                    if let (Some(this_field), Some(that_field)) = (&mut self.fields_[i], &that.fields_[i]) {
                        let merged_field = this_field.merge(that_field, zone, &mut count);
                        self.fields_[i] = Some(merged_field);
                    }
                    self.fields_count_ += count;

                    count = 0;
                    if self.const_fields_[i].is_none() && that.const_fields_[i].is_some() {
                        self.const_fields_[i] = Some(Box::new(AbstractField::new(zone)));
                    }
                    if let (Some(this_const_field), Some(that_const_field)) = (&mut self.const_fields_[i], &that.const_fields_[i]) {
                        let merged_const_field = this_const_field.merge(that_const_field, zone, &mut count);
                        self.const_fields_[i] = Some(merged_const_field);
                    }
                    self.const_fields_count_ += count;
                }

                if self.maps_.is_none() && that.maps_.is_some() {
                    self.maps_ = Some(Box::new(AbstractMaps::new(zone)));
                }
                if let (Some(this_maps), Some(that_maps)) = (&mut self.maps_, &that.maps_) {
                    let merged_maps = this_maps.merge(that_maps, zone);
                    self.maps_ = Some(merged_maps);
                }
            }

            fn set_maps(&self, zone: *mut Zone, object: Node, maps: ZoneRefSet<Map>) -> Box<AbstractState> {
                let mut new_state = AbstractState::new(zone);
                new_state.elements_ = self.elements_.clone();
                new_state.fields_ = self.fields_.clone();
                new_state.const_fields_ = self.const_fields_.clone();
                new_state.maps_ = Some(Box::new(AbstractMaps::with_maps(zone, object, maps)));
                new_state.const_fields_count_ = self.const_fields_count_;
                new_state.fields_count_ = self.fields_count_;
                Box::new(new_state)
            }

            fn kill_maps(&self, zone: *mut Zone, object: &Node) -> Box<AbstractState> {
                let mut new_state = AbstractState::new(zone);
                new_state.elements_ = self.elements_.clone();
                new_state.fields_ = self.fields_.clone();
                new_state.const_fields_ = self.const_fields_.clone();
                if self.maps_.is_some() {
                    new_state.maps_ = Some(Box::new(AbstractMaps::new(zone)));
                }
                new_state.const_fields_count_ = self.const_fields_count_;
                new_state.fields_count_ = self.fields_count_;
                Box::new(new_state)
            }

            fn kill_maps_aliased(&self, zone: *mut Zone, alias_info: &AliasStateInfo) -> Box<AbstractState> {
                let mut new_state = AbstractState::new(zone);
                new_state.elements_ = self.elements_.clone();
                new_state.fields_ = self.fields_.clone();
                new_state.const_fields_ = self.const_fields_.clone();
                if let Some(maps) = &self.maps_ {
                    new_state.maps_ = Some(maps.kill(zone, alias_info));
                }
                new_state.const_fields_count_ = self.const_fields_count_;
                new_state.fields_count_ = self.fields_count_;
                Box::new(new_state)
            }

            fn lookup_maps(&self, object: &Node, object_maps: &mut ZoneRefSet<Map>) -> bool {
                match &self.maps_ {
                    Some(maps) => maps.lookup(object, object_maps),
                    None => false,
                }
            }

            fn add_field(
                &self,
                zone: *mut Zone,
                object: Node,
                index: IndexRange,
                info: FieldInfo,
            ) -> Box<AbstractState> {
                let mut new_state = AbstractState::new(zone);
                new_state.elements_ = self.elements_.clone();
                new_state.fields_ = self.fields_.clone();
                new_state.const_fields_ = self.const_fields_.clone();
                new_state.