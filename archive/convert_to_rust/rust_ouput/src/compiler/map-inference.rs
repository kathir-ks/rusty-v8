// Converted from V8 C++ source files:
// Header: map-inference.h
// Implementation: map-inference.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(unused_variables)]

use std::cell::RefCell;
use std::rc::Rc;
use std::any::Any;

//use crate::v8::internal::HeapObjectRef;
//use crate::v8::internal::Local;
//use crate::v8::internal::Value;
//use crate::v8::internal::StringView;
//use crate::v8::internal::Maybe;
//use crate::v8::internal::Args;
//use crate::v8::internal::Int64Representation;

pub struct Map {}
pub struct MapRef {}
impl MapRef {
    pub fn equals(&self, other: MapRef) -> bool {
        true
    }

    pub fn instance_type(&self) -> InstanceType {
        InstanceType::JS_OBJECT_TYPE
    }
    pub fn is_stable(&self) -> bool {
        true
    }
}

pub struct HeapObjectRef {}

pub struct Local<'a, T> {
    _marker: std::marker::PhantomData<&'a T>,
}
impl<'a, T> Local<'a, T> {
    pub fn new() -> Self {
        Local{
            _marker: std::marker::PhantomData,
        }
    }
}

pub trait Value {}
pub struct StringView {}
pub struct Maybe<T> {
    _marker: std::marker::PhantomData<T>,
}
pub struct Args {}
pub struct Int64Representation {}
pub struct CFunction {}

impl<T> Maybe<T> {
    pub fn Just(value: T) -> Maybe<T> {
        Maybe {
            _marker: std::marker::PhantomData,
        }
    }
}

pub struct Zone {

}
impl Zone {
    pub fn new() -> Self {
        Zone{}
    }
}
pub struct CompilationDependencies {}

pub struct FeedbackSource {}
impl FeedbackSource {
    pub fn IsValid(&self) -> bool {
        true
    }
}

pub struct JSGraph {}

impl JSGraph {
    pub fn graph(&self) -> &Graph {
        &Graph{}
    }
    pub fn simplified(&self) -> &Simplified {
        &Simplified{}
    }
}

pub struct Simplified {}

pub struct NodeProperties {}
impl NodeProperties {
    pub fn InferMapsUnsafe(
        broker: &JSHeapBroker,
        object: *mut Node,
        effect: Effect,
        maps: &mut ZoneRefSet<Map>,
    ) -> i32 {
        0
    }
    pub const kUnreliableMaps: i32 = 1;
    pub const kNoMaps: i32 = 2;
}

pub struct Effect {
    value: i32,
}

pub struct Control {
    node: *mut Node
}

impl Control {
    pub fn new() -> Self {
        Control{
            node: std::ptr::null_mut()
        }
    }
}
pub struct JSHeapBroker {}

pub struct Node {}

pub struct Graph {}
impl Graph {
    pub fn NewNode(&self, _op: CheckMaps, _object: *mut Node, _effect: Effect, _control: Control) -> Effect {
        Effect{value: 0}
    }
}

#[derive(PartialEq, Eq)]
pub enum InstanceType {
    JS_OBJECT_TYPE,
    STRING_TYPE,
    SYMBOL_TYPE,
}

pub struct InstanceTypeChecker {}
impl InstanceTypeChecker {
    pub fn IsJSReceiver(instance_type: InstanceType) -> bool {
        instance_type == InstanceType::JS_OBJECT_TYPE
    }
    pub fn IsString(instance_type: InstanceType) -> bool {
        instance_type == InstanceType::STRING_TYPE
    }
}

pub struct ZoneRefSet<T> {
    items: Vec<T>,
}

impl<T> ZoneRefSet<T> {
    pub fn new() -> Self {
        ZoneRefSet { items: Vec::new() }
    }
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
    pub fn size(&self) -> usize {
        self.items.len()
    }
    pub fn at(&self, index: usize) -> &T {
        &self.items[index]
    }
    pub fn begin(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
    pub fn end(&self) -> std::slice::Iter<T> {
        self.items.iter()
    }
    pub fn push(&mut self, item: T) {
        self.items.push(item);
    }
    pub fn clear(&mut self) {
        self.items.clear();
    }
}

impl<'a, T> IntoIterator for &'a ZoneRefSet<T> {
    type Item = &'a T;
    type IntoIter = std::slice::Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.iter()
    }
}

pub struct CheckMaps {
    flag: CheckMapsFlag,
    maps: ZoneRefSet<Map>,
    feedback: FeedbackSource
}

impl CheckMaps {
    pub fn new(flag: CheckMapsFlag, maps: ZoneRefSet<Map>, feedback: FeedbackSource) -> Self {
        CheckMaps{
            flag,
            maps,
            feedback
        }
    }
}
#[derive(PartialEq, Eq)]
pub enum CheckMapsFlag {
    kNone,
}

pub struct Reducer {}

impl Reducer {
    pub fn NoChange() -> Reduction {
        Reduction {}
    }
}

pub struct Reduction {}

#[derive(Debug)]
enum MapInferenceError {
    GenericError,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum MapsState {
    kReliableOrGuarded,
    kUnreliableDontNeedGuard,
    kUnreliableNeedGuard,
}

pub struct MapInference<'a> {
    broker_: &'a JSHeapBroker,
    object_: *mut Node,
    maps_: ZoneRefSet<Map>,
    maps_state_: MapsState,
}

impl<'a> MapInference<'a> {
    pub fn new(broker: &'a JSHeapBroker, object: *mut Node, effect: Effect) -> Self {
        let mut maps_ = ZoneRefSet::new();
        let result =
            NodeProperties::InferMapsUnsafe(broker, object, effect, &mut maps_);
        let maps_state_ = if result == NodeProperties::kUnreliableMaps {
            MapsState::kUnreliableDontNeedGuard
        } else {
            MapsState::kReliableOrGuarded
        };
        let is_empty = maps_.is_empty();
        let result_eq = result == NodeProperties::kNoMaps;
        
        MapInference {
            broker_: broker,
            object_: object,
            maps_: maps_,
            maps_state_: maps_state_,
        }
    }

    fn safe(&self) -> bool {
        self.maps_state_ != MapsState::kUnreliableNeedGuard
    }

    fn set_need_guard_if_unreliable(&mut self) {
        if self.have_maps() && self.maps_state_ == MapsState::kUnreliableDontNeedGuard {
            self.maps_state_ = MapsState::kUnreliableNeedGuard;
        }
    }

    fn set_guarded(&mut self) {
        self.maps_state_ = MapsState::kReliableOrGuarded;
    }

    pub fn have_maps(&self) -> bool {
        !self.maps_.is_empty()
    }

    pub fn all_of_instance_types_are_js_receiver(&self) -> bool {
        self.all_of_instance_types_unsafe(InstanceTypeChecker::IsJSReceiver)
    }

    pub fn all_of_instance_types_are(&self, instance_type: InstanceType) -> bool {
        assert!(
            !InstanceTypeChecker::IsString(instance_type),
            "instance_type must not be a String type."
        );
        self.all_of_instance_types_unsafe(|other| instance_type == other)
    }

    pub fn any_of_instance_types_are(&self, instance_type: InstanceType) -> bool {
        assert!(
            !InstanceTypeChecker::IsString(instance_type),
            "instance_type must not be a String type."
        );
        self.any_of_instance_types_unsafe(|other| instance_type == other)
    }

    pub fn all_of_instance_types<F>(&mut self, f: F) -> bool
    where
        F: Fn(InstanceType) -> bool,
    {
        self.set_need_guard_if_unreliable();
        self.all_of_instance_types_unsafe(f)
    }

    fn all_of_instance_types_unsafe<F>(&self, f: F) -> bool
    where
        F: Fn(InstanceType) -> bool,
    {
        assert!(self.have_maps());
        let instance_type = |map: &MapRef| f(map.instance_type());
        self.maps_.begin().all(|map| instance_type(map))
    }

    fn any_of_instance_types_unsafe<F>(&self, f: F) -> bool
    where
        F: Fn(InstanceType) -> bool,
    {
        assert!(self.have_maps());

        let instance_type = |map: &MapRef| f(map.instance_type());

        self.maps_.begin().any(|map| instance_type(map))
    }

    pub fn get_maps(&mut self) -> &ZoneRefSet<Map> {
        self.set_need_guard_if_unreliable();
        &self.maps_
    }

    pub fn is(&self, expected_map: MapRef) -> bool {
        if !self.have_maps() {
            return false;
        }
        if self.maps_.size() != 1 {
            return false;
        }
        self.maps_.at(0).equals(expected_map)
    }

    pub fn insert_map_checks(
        &mut self,
        jsgraph: &JSGraph,
        effect: &mut Effect,
        control: Control,
        feedback: &FeedbackSource,
    ) {
        assert!(self.have_maps());
        assert!(feedback.IsValid());
        *effect = jsgraph.graph().NewNode(
            CheckMaps::new(CheckMapsFlag::kNone, self.maps_.clone(), feedback.clone()),
            self.object_,
            *effect,
            control,
        );
        self.set_guarded();
    }

    pub fn rely_on_maps_via_stability(&mut self, dependencies: &mut CompilationDependencies) -> bool {
        assert!(self.have_maps());
        self.rely_on_maps_helper(dependencies, None, None, Control::new(), &FeedbackSource{})
    }

    pub fn rely_on_maps_prefer_stability(
        &mut self,
        dependencies: &mut CompilationDependencies,
        jsgraph: &JSGraph,
        effect: &mut Effect,
        control: Control,
        feedback: &FeedbackSource,
    ) -> bool {
        assert!(self.have_maps());
        if self.safe() {
            return false;
        }
        if self.rely_on_maps_via_stability(dependencies) {
            return true;
        }
        assert!(self.rely_on_maps_helper(
            &mut CompilationDependencies{},
            Some(jsgraph),
            Some(effect),
            control,
            feedback,
        ));
        false
    }

    fn rely_on_maps_helper(
        &mut self,
        dependencies: &mut CompilationDependencies,
        jsgraph: Option<&JSGraph>,
        effect: Option<&mut Effect>,
        control: Control,
        feedback: &FeedbackSource,
    ) -> bool {
        if self.safe() {
            return true;
        }

        let is_stable = |map: &MapRef| map.is_stable();
        if dependencies != &mut CompilationDependencies{}
            && self.maps_.begin().all(|map| is_stable(map))
        {
            for map in self.maps_.begin() {
                dependencies.DependOnStableMap(map);
            }
            self.set_guarded();
            return true;
        } else if feedback.IsValid() {
            self.insert_map_checks(
                jsgraph.unwrap(),
                effect.unwrap(),
                control,
                feedback,
            );
            return true;
        } else {
            return false;
        }
    }

    pub fn no_change(&mut self) -> Reduction {
        self.set_guarded();
        self.maps_.clear(); // Just to make some CHECKs fail if {this} gets used after.
        Reducer::NoChange()
    }
}

impl<'a> Drop for MapInference<'a> {
    fn drop(&mut self) {
        assert!(self.safe());
    }
}

impl CompilationDependencies {
    pub fn DependOnStableMap(&mut self, _map: MapRef) {}
}
