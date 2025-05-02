// src/objects/transitions.rs

use std::cmp::Ordering;
use std::fmt;
use std::sync::{Mutex, MutexGuard};

//use crate::base::small_vector::SmallVector; // Assuming this is handled by Rust's Vec
//use crate::objects::objects::*; // Define your Rust equivalents for these
//use crate::objects::transitions::*; // Define your Rust equivalents for these
//use crate::utils::utils::*; // Define your Rust equivalents for these

// Define placeholders for types and functions that are not directly translatable.
// Replace these with your actual Rust implementations.

type Isolate<'a> = &'a mut IsolateData; // Placeholder for Isolate
type DirectHandle<'a, T> = &'a mut T; // Placeholder for DirectHandle
type Handle<'a, T> = &'a T;
type Name = String; // Placeholder for Name
type Map = usize; // Placeholder for Map (replace with a proper Rust type)
type Object = usize;
type String = std::string::String;
type Symbol = String; // Placeholder for Symbol
type MaybeObject = Option<usize>; // Placeholder for MaybeObject
type WeakFixedArray = Vec<MaybeObject>; // Placeholder for WeakFixedArray
type PropertyDetails = u32; // Placeholder for PropertyDetails
type TransitionArray = Vec<(Name, MaybeObject)>; // Placeholder for TransitionArray

const K_NOT_FOUND: i32 = -1;
const NONE: u32 = 0;
const FROZEN: u32 = 1;
const SEALED: u32 = 2;
const K_MAX_NUMBER_OF_TRANSITIONS: usize = 256;
const K_PROTO_TRANSITION_HEADER_SIZE: usize = 1;
const K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET: usize = 0;
const K_MAX_CACHED_PROTOTYPE_TRANSITIONS: usize = 64;

struct IsolateData {
    full_transition_array_access: Mutex<()>,
}

impl IsolateData {
    fn full_transition_array_access(&self) -> &Mutex<()> {
        &self.full_transition_array_access
    }
}

#[derive(Debug, PartialEq)]
enum Encoding {
    WeakRef,
    PrototypeInfo,
    Uninitialized,
    MigrationTarget,
    FullTransitionArray,
}

#[derive(Debug, PartialEq)]
enum TransitionKindFlag {
    SimplePropertyTransition,
    SpecialTransition,
    PrototypeTransition,
}

// ReadOnlyRoots struct:
struct ReadOnlyRoots<'a> {
  isolate: &'a Isolate<'a>,
}

impl<'a> ReadOnlyRoots<'a> {
  fn nonextensible_symbol(&self) -> Name {
    "nonextensible_symbol".to_string()
  }
  fn sealed_symbol(&self) -> Name {
    "sealed_symbol".to_string()
  }
  fn frozen_symbol(&self) -> Name {
    "frozen_symbol".to_string()
  }
  fn elements_transition_symbol(&self) -> Name {
    "elements_transition_symbol".to_string()
  }
  fn strict_function_transition_symbol(&self) -> Name {
    "strict_function_transition_symbol".to_string()
  }
  fn empty_weak_fixed_array(&self) -> WeakFixedArray {
    Vec::new()
  }
  fn undefined_value(&self) -> Handle<'a, Object> {
    &0 //replace with correct value later
  }
}

macro_rules! dcheck_ne {
    ($left:expr, $right:expr) => {
        if $left == $right {
            panic!("DCHECK_NE failed: {} != {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! dcheck_eq {
    ($left:expr, $right:expr) => {
        if $left != $right {
            panic!("DCHECK_EQ failed: {} == {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! check_le {
    ($left:expr, $right:expr) => {
        if $left > $right {
            panic!("CHECK_LE failed: {} <= {}", stringify!($left), stringify!($right));
        }
    };
}

macro_rules! check {
    ($condition:expr) => {
        if !$condition {
            panic!("CHECK failed: {}", stringify!($condition));
        }
    };
}

macro_rules! use_ {
    ($x:expr) => {
        let _ = $x;
    };
}

struct Factory<'a> {
  isolate: &'a mut Isolate<'a>
}

impl<'a> Factory<'a> {
  fn new_transition_array(&self, size: usize, _slack: i32) -> DirectHandle<'a, TransitionArray> {
    let mut result = Vec::with_capacity(size);
    for _ in 0..size {
      result.push((String::new(), None));
    }
    &mut result
  }

  fn copy_weak_fixed_array_and_grow(&self, array: &DirectHandle<'a, WeakFixedArray>, grow_by: usize) -> DirectHandle<'a, WeakFixedArray> {
    let mut new_array = array.clone();
    new_array.resize(array.len() + grow_by, None);

    &mut new_array
  }
}

impl<'a> Isolate<'a> {
    fn factory(&'a mut self) -> Factory<'a> {
        Factory { isolate: self }
    }
}

// Placeholder functions
fn is_unique_name(_name: &Name) -> bool {
    true // Replace with actual implementation
}

fn is_symbol(_name: &Name) -> bool {
    false // Replace with actual implementation
}

fn get_target_details(_name: &Name, _target: Map) -> PropertyDetails {
    0 // Replace with actual implementation
}

fn make_weak<T>(_target: &T) -> MaybeObject {
  Some(0) // Replace with actual implementation
}

fn is_undefined<T>(_obj: &T) -> bool {
    false // Replace with actual implementation
}

fn cast<T>(_obj: MaybeObject) -> T {
    0 as T // Replace with actual implementation
}

fn is_map(_heap_object: Map) -> bool {
    false // Replace with actual implementation
}

fn direct_handle<'a, T>(obj: T, _isolate: &Isolate) -> DirectHandle<'a, T> {
  Box::leak(Box::new(obj))
}

struct TransitionsAccessor<'a> {
    isolate_: &'a mut Isolate<'a>,
    map_: Map,
    raw_transitions_: MaybeObject,
    concurrent_access_: bool,
}

impl<'a> TransitionsAccessor<'a> {
    fn encoding(&self) -> Encoding {
        Self::get_encoding(self.isolate_, self.raw_transitions_)
    }

    fn transitions(&self) -> TransitionArray {
        Self::get_transition_array(self.isolate_, self.raw_transitions_)
    }

    // static
    fn get_simple_transition(isolate: &mut Isolate, map: &mut Map) -> Map {
        let raw_transitions = Self::raw_transitions(isolate, map);
        match Self::get_encoding(isolate, raw_transitions) {
            Encoding::WeakRef => cast(raw_transitions.unwrap()), // Assuming GetHeapObjectAssumeWeak is just unwrapping
            _ => 0,
        }
    }

    fn has_simple_transition_to(&self, map: Map) -> bool {
        match self.encoding() {
            Encoding::WeakRef => {
                self.raw_transitions_.unwrap() == map
            }
            Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget | Encoding::FullTransitionArray => {
                false
            }
        }
    }

    // static
    fn insert_helper(isolate: &mut Isolate, map: &mut Map, name: &mut Name, target: &mut Map, flag: TransitionKindFlag) {
        dcheck_ne!(flag, TransitionKindFlag::PrototypeTransition);
        let encoding = Self::get_encoding(isolate, Self::raw_transitions(isolate, map));
        dcheck_ne!(encoding, Encoding::PrototypeInfo);
        let roots = ReadOnlyRoots {isolate};
        //(*target)->SetBackPointer(*map); //Cannot implement due to Rust borrowing rules

        // If the map doesn't have any transitions at all yet, install the new one.
        if encoding == Encoding::Uninitialized || encoding == Encoding::MigrationTarget {
            if flag == TransitionKindFlag::SimplePropertyTransition {
                Self::replace_transitions(isolate, map, make_weak(target));
                return;
            }
            // If the flag requires a full TransitionArray, allocate one.
            let mut result = isolate.factory().new_transition_array(1, 0);
            result[0].0 = name.clone();
            result[0].1 = make_weak(target);
            Self::replace_transitions(isolate, map, &result);
            dcheck_eq!(Self::get_encoding(isolate, Self::raw_transitions(isolate, map)), Encoding::FullTransitionArray);
            return;
        }

        if encoding == Encoding::WeakRef {
            let simple_transition = Self::get_simple_transition(isolate, map);
            dcheck_ne!(simple_transition, 0); // Assuming is_null is checking for 0

            if flag == TransitionKindFlag::SimplePropertyTransition {
              //Cannot implement key getting and details check, assume true
              Self::replace_transitions(isolate, map, make_weak(target));
              return;
            }

            // Otherwise allocate a full TransitionArray with slack for a new entry.
            let mut result = isolate.factory().new_transition_array(1, 1);

            // Reload `simple_transition`. Allocations might have caused it to be
            // cleared.
            let simple_transition = Self::get_simple_transition(isolate, map);
            if simple_transition == 0 {
              result[0].0 = name.clone();
              result[0].1 = make_weak(target);
              Self::replace_transitions(isolate, map, &result);
              dcheck_eq!(Self::get_encoding(isolate, Self::raw_transitions(isolate, map)), Encoding::FullTransitionArray);
              return;
            }

            // Insert the original transition in index 0.
            // TODO: Implement GetSimpleTransitionKey, but for now assume it returns the name
            result[0].0 = "SimpleTransitionKey".to_string();
            result[0].1 = make_weak(&simple_transition);

            // Search for the correct index to insert the new transition.
            let mut insertion_index = 0;
            let index = K_NOT_FOUND;

            use_(index);

            result.resize(2, ("".to_string(), None));
            //result.set_number_of_transitions(2);
            if insertion_index == 0 {
              // If the new transition will be inserted in index 0, move the original
              // transition to index 1.
              result[1].0 = "SimpleTransitionKey".to_string();
              result[1].1 = make_weak(&simple_transition);
            }
            result[insertion_index].0 = name.clone();
            result[insertion_index].1 = make_weak(target);

            //SLOW_DCHECK(result.IsSortedNoDuplicates());
            Self::replace_transitions(isolate, map, &result);
            dcheck_eq!(Self::get_encoding(isolate, Self::raw_transitions(isolate, map)), Encoding::FullTransitionArray);
            return;
        }

        // At this point, we know that the map has a full TransitionArray.
        dcheck_eq!(encoding, Encoding::FullTransitionArray);

        let mut number_of_transitions = 0;
        let mut new_nof = 0;
        let mut insertion_index = K_NOT_FOUND;
        let is_special_transition = flag == TransitionKindFlag::SpecialTransition;
        //DCHECK_EQ(is_special_transition, IsSpecialTransition(roots, *name));
        let details = if is_special_transition {
          0
        } else {
          get_target_details(name, *target)
        };

        {
          //DisallowGarbageCollection no_gc;
          let mut array = Self::get_transition_array(isolate, Self::raw_transitions(isolate, map));
          number_of_transitions = array.len();

          //let index = is_special_transition
          //  ? array.SearchSpecial(Cast<Symbol>(*name), false, &insertion_index)
          //  : array.Search(details.kind(), *name, details.attributes(), &insertion_index);

          let index = K_NOT_FOUND;
          // If an existing entry was found, overwrite it and return.
          if index != K_NOT_FOUND {
            let _mutex_guard = isolate.full_transition_array_access().lock().unwrap();
            array[index].1 = make_weak(target);
            Self::replace_transitions(isolate, map, &array);
            return;
          }

          new_nof = number_of_transitions + 1;
          check_le!(new_nof, K_MAX_NUMBER_OF_TRANSITIONS);
          dcheck_eq!(insertion_index, K_NOT_FOUND);

          // If there is enough capacity, insert new entry into the existing array.
          if new_nof <= array.capacity() {
            let _mutex_guard = isolate.full_transition_array_access().lock().unwrap();
            array.resize(new_nof, ("".to_string(), None));
            //array.SetNumberOfTransitions(new_nof);
            for i in (insertion_index + 1..number_of_transitions + 1).rev() {
              array[i] = array[i - 1].clone();
            }
            array[insertion_index].0 = name.clone();
            array[insertion_index].1 = make_weak(target);
            //SLOW_DCHECK(array.IsSortedNoDuplicates());
            Self::replace_transitions(isolate, map, &array);
            return;
          }
        }

        // We're gonna need a bigger TransitionArray.
        let mut result = isolate.factory().new_transition_array(
          new_nof,
          0, //Map::SlackForArraySize(number_of_transitions, kMaxNumberOfTransitions),
        );

        // The map's transition array may have shrunk during the allocation above as
        // it was weakly traversed, though it is guaranteed not to disappear. Trim the
        // result copy if needed, and recompute variables.
        //{
        //  DisallowGarbageCollection no_gc;
        //  Tagged<TransitionArray> array = GetTransitionArray(isolate, map);
        //  if (array->number_of_transitions() != number_of_transitions) {
        //    DCHECK_LT(array->number_of_transitions(), number_of_transitions);

        //    int index =
        //        is_special_transition
        //            ? array->SearchSpecial(Cast<Symbol>(*name), false, &insertion_index)
        //            : array->Search(details.kind(), *name, details.attributes(),
        //                            &insertion_index);
        //    CHECK_EQ(index, kNotFound);
        //    USE(index);
        //    DCHECK_GE(insertion_index, 0);
        //    DCHECK_LE(insertion_index, number_of_transitions);

        //    number_of_transitions = array->number_of_transitions();
        //    new_nof = number_of_transitions + 1;
        //    result->SetNumberOfTransitions(new_nof);
        //  }

        //  if (array->HasPrototypeTransitions()) {
        //    result->SetPrototypeTransitions(array->GetPrototypeTransitions());
        //  }
        //  if (array->HasSideStepTransitions()) {
        //    result->SetSideStepTransitions(array->GetSideStepTransitions());
        //  }

        //  DCHECK_NE(kNotFound, insertion_index);
        //  for (int i = 0; i < insertion_index; ++i) {
        //    result->Set(i, array->GetKey(i), array->GetRawTarget(i));
        //  }
        //  result->Set(insertion_index, *name, MakeWeak(*target));
        //  for (int i = insertion_index; i < number_of_transitions; ++i) {
        //    result->Set(i + 1, array->GetKey(i), array->GetRawTarget(i));
        //  }

        //  SLOW_DCHECK(result->IsSortedNoDuplicates());
        //  ReplaceTransitions(isolate, map, result);
        //}
        Self::replace_transitions(isolate, map, &result);
    }

    fn search_transition(&self, _name: Name, _kind: u32, _attributes: u32) -> Map {
        match self.encoding() {
            Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget => {
                0
            }
            Encoding::WeakRef => {
              //if (!IsMatchingMap(map, name, kind, attributes)) return Tagged<Map>();
              //return map;
              0
            }
            Encoding::FullTransitionArray => {
              //base::MutexGuardIf guard(isolate_->full_transition_array_access(),
              //                         concurrent_access_);
              //return transitions()->SearchAndGetTarget(kind, name, attributes);
              0
            }
        }
    }

    fn search_special(&self, _name: Symbol) -> Map {
      if self.encoding() != Encoding::FullTransitionArray { return 0 };
      //base::MutexGuardIf guard(isolate_->full_transition_array_access(),
      //                         concurrent_access_);
      //int transition = transitions()->SearchSpecial(name, concurrent_access_);
      //if (transition == kNotFound) return {};
      //return transitions()->GetTarget(transition);
      0
    }

    // static
    fn is_special_transition(roots: &ReadOnlyRoots, name: &Name) -> bool {
        if is_symbol(name) == false { return false };
        name == &roots.nonextensible_symbol() ||
        name == &roots.sealed_symbol() || name == &roots.frozen_symbol() ||
        name == &roots.elements_transition_symbol() ||
        name == &roots.strict_function_transition_symbol()
    }

    fn find_transition_to_field(&self, _name: &mut String) -> Result<Map, ()> {
        //DCHECK(IsInternalizedString(*name));
        //DisallowGarbageCollection no_gc;
        //Tagged<Map> target = SearchTransition(*name, PropertyKind::kData, NONE);
        //if (target.is_null()) return MaybeHandle<Map>();
        //#ifdef DEBUG
        //  PropertyDetails details = target->GetLastDescriptorDetails(isolate_);
        //  DCHECK_EQ(NONE, details.attributes());
        //  DCHECK_EQ(PropertyKind::kData, details.kind());
        //  DCHECK_EQ(PropertyLocation::kField, details.location());
        //#endif
        //return Handle<Map>(target, isolate_);
        Err(())
    }

    fn for_each_transition_to<F>(&self, _name: Name, _callback: F, _no_gc: ())
      where F: Fn(Map) {
      match self.encoding() {
        Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget => {
          return;
        }
        Encoding::WeakRef => {
          //Tagged<Map> target =
          //    Cast<Map>(raw_transitions_.GetHeapObjectAssumeWeak());
          //InternalIndex descriptor = target->LastAdded();
          //Tagged<DescriptorArray> descriptors =
          //    target->instance_descriptors(kRelaxedLoad);
          //Tagged<Name> key = descriptors->GetKey(descriptor);
          //if (key == name) {
          //  callback(target);
          //}
          return;
        }
        Encoding::FullTransitionArray => {
          //base::MutexGuardIf guard(isolate_->full_transition_array_access(),
          //                         concurrent_access_);
          //return transitions()->ForEachTransitionTo(name, callback);
        }
      }
    }

    // static
    fn can_have_more_transitions(isolate: &mut Isolate, map: &mut Map) -> bool {
      //if (map->is_dictionary_map()) return false;
      let raw_transitions = Self::raw_transitions(isolate, map);
      if Self::get_encoding(isolate, raw_transitions) == Encoding::FullTransitionArray {
        //return GetTransitionArray(isolate, raw_transitions)
        //           ->number_of_transitions() < kMaxNumberOfTransitions;
        false
      } else {
        true
      }
    }

    // static
    fn is_matching_map(_target: Map, _name: Name, _kind: u32, _attributes: u32) -> bool {
      //InternalIndex descriptor = target->LastAdded();
      //Tagged<DescriptorArray> descriptors =
      //    target->instance_descriptors(kRelaxedLoad);
      //Tagged<Name> key = descriptors->GetKey(descriptor);
      //if (key != name) return false;
      //return descriptors->GetDetails(descriptor)
      //    .HasKindAndAttributes(kind, attributes);
      false
    }

    fn number_of_transitions(&self) -> usize {
      match self.encoding() {
        Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget => 0,
        Encoding::WeakRef => 1,
        Encoding::FullTransitionArray => {
          self.transitions().len()
        }
      }
    }

    fn has_prototype_transitions(&self) -> bool {
      match self.encoding() {
        Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget | Encoding::WeakRef => false,
        Encoding::FullTransitionArray => {
          let transitions = self.transitions();
          transitions.iter().any(|(name, _)| name == "prototype") // Assuming prototype transition names
        }
      }
    }

    // static
    fn set_migration_target(isolate: &mut Isolate, map: &mut Map, migration_target: Map) {
      //We only cache the migration target for maps with empty transitions for GC's
      //sake.
      if Self::get_encoding(isolate, Self::raw_transitions(isolate, map)) != Encoding::Uninitialized { return; }
      //DCHECK(map->is_deprecated());
      //map->set_raw_transitions(migration_target, kReleaseStore);
    }

    fn get_migration_target(&self) -> Map {
      if self.encoding() == Encoding::MigrationTarget {
        //return Cast<Map>(map_->raw_transitions(kAcquireLoad));
        0
      } else {
        0
      }
    }

    // static
    fn replace_transitions(
      isolate: &mut Isolate,
      map: &mut Map,
      new_transitions: &TransitionArray,
    ) {
      //if (GetEncoding(isolate, map) == kFullTransitionArray) {
      //  CheckNewTransitionsAreConsistent(
      //      isolate, map, new_transitions.GetHeapObjectAssumeStrong());
      //  DCHECK_NE(GetTransitionArray(isolate, map),
      //            new_transitions.GetHeapObjectAssumeStrong());
      //}
      Self::set_raw_transitions(isolate, map, new_transitions);
    }

    // static
    fn ensure_has_full_transition_array(isolate: &mut Isolate, map: &mut Map) {
      let encoding = Self::get_encoding(isolate, Self::raw_transitions(isolate, map));
      if encoding == Encoding::FullTransitionArray { return; }
      let nof = if encoding == Encoding::Uninitialized || encoding == Encoding::MigrationTarget {
        0
      } else {
        1
      };
      let mut result = isolate.factory().new_transition_array(nof, 0);
      // Reload encoding after possible GC.
      let encoding = Self::get_encoding(isolate, Self::raw_transitions(isolate, map));
      if nof == 1 {
        if encoding == Encoding::Uninitialized {
          // If allocation caused GC and cleared the target, trim the new array.
          //result->SetNumberOfTransitions(0);
          result.clear();
        } else {
          // Otherwise populate the new array.
          let target = Self::get_simple_transition(isolate, map);
          //TODO: implement
          result[0].0 = "SimpleTransitionKey".to_string();
          result[0].1 = make_weak(&target);
        }
      }
      Self::replace_transitions(isolate, map, &result);
    }

    fn traverse_transition_tree_internal<F>(&self, callback: F, _no_gc: ())
    where F: Fn(Map) {
      //Static stack size
      let mut stack: Vec<Map> = Vec::new();
      stack.push(self.map_);

      while !stack.is_empty() {
        let current_map = stack.pop().unwrap();

        callback(current_map);

        let raw_transitions = Self::raw_transitions(self.isolate_, current_map);
        let encoding = Self::get_encoding(self.isolate_, raw_transitions);

        match encoding {
          Encoding::PrototypeInfo | Encoding::Uninitialized | Encoding::MigrationTarget => {},
          Encoding::WeakRef => {
            //stack.emplace_back(
            //  Cast<Map>(raw_transitions.GetHeapObjectAssumeWeak()));
          },
          Encoding::FullTransitionArray => {
            //Tagged<TransitionArray> transitions =
            //  Cast<TransitionArray>(raw_transitions.GetHeapObjectAssumeStrong());
            //if (transitions->HasPrototypeTransitions()) {
            //  Tagged<WeakFixedArray> proto_trans =
            //      transitions->GetPrototypeTransitions();
            //  int length =
            //      TransitionArray::NumberOfPrototypeTransitions(proto_trans);
            //  for (int i = 0; i < length; ++i) {
            //    int index = TransitionArray::kProtoTransitionHeaderSize + i;
            //    Tagged<MaybeObject> target = proto_trans->get(index);
            //    Tagged<HeapObject> heap_object;
            //    if (target.GetHeapObjectIfWeak(&heap_object)) {
            //      stack.emplace_back(Cast<Map>(heap_object));
            //    } else {
            //      DCHECK(target.IsCleared());
            //    }
            //  }
            //}
            //ReadOnlyRoots roots(isolate_);
            //for (int i = 0; i < transitions->number_of_transitions(); ++i) {
            //  stack.emplace_back(transitions->GetTarget(i));
            //}
          }
        }
      }
    }

    // static placeholders
    fn raw_transitions(_isolate: &mut Isolate, map: &Map) -> MaybeObject {
      Some(*map) // Replace with actual implementation
    }

    fn get_transition_array(_isolate: &mut Isolate, raw_transitions: MaybeObject) -> TransitionArray {
        //Replace with actual implementation
        TransitionArray::new()
    }

    fn get_simple_transition_key(_map: Map) -> Name {
        "key".to_string() // Replace with actual implementation
    }

    fn set_raw_transitions(_isolate: &mut Isolate, _map: &mut Map, _new_transitions: &TransitionArray) {
        // Replace with actual implementation
    }

    fn get_encoding(_isolate: &mut Isolate, raw_transitions: MaybeObject) -> Encoding {
      match raw_transitions {
        Some(_) => Encoding::FullTransitionArray,
        None => Encoding::Uninitialized,
      }
    }
}

impl fmt::Display for Encoding {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Encoding::WeakRef => write!(f, "WeakRef"),
            Encoding::PrototypeInfo => write!(f, "PrototypeInfo"),
            Encoding::Uninitialized => write!(f, "Uninitialized"),
            Encoding::MigrationTarget => write!(f, "MigrationTarget"),
            Encoding::FullTransitionArray => write!(f, "FullTransitionArray"),
        }
    }
}

// static
impl TransitionArray {
  fn compact_prototype_transition_array(isolate: &mut Isolate, array: &mut WeakFixedArray) -> bool {
    let header = K_PROTO_TRANSITION_HEADER_SIZE;
    let number_of_transitions = Self::number_of_prototype_transitions(array);
    if number_of_transitions == 0 {
      //Empty array cannot be compacted.
      return false;
    }
    let mut new_number_of_transitions = 0;
    for i in 0..number_of_transitions {
      let target = array[header + i];

      if target.is_none() {
        if new_number_of_transitions != i {
          array[header + new_number_of_transitions] = target;
        }
        new_number_of_transitions += 1;
      }
    }
    //Fill slots that became free with undefined value.
    let undefined = *isolate.factory().undefined_value();
    for i in new_number_of_transitions..number_of_transitions {
      array[header + i] = Some(undefined);
    }
    if number_of_transitions != new_number_of_transitions {
      Self::set_number_of_prototype_transitions(array, new_number_of_transitions);
    }
    new_number_of_transitions < number_of_transitions
  }

  // static
  fn grow_prototype_transition_array(
    array: &mut WeakFixedArray,
    new_capacity: usize,
    isolate: &mut Isolate,
  ) -> &mut WeakFixedArray {
    //Grow array by factor 2 up to MaxCachedPrototypeTransitions.
    let capacity = array.len() - K_PROTO_TRANSITION_HEADER_SIZE;
    let new_capacity = std::cmp::min(K_MAX_CACHED_PROTOTYPE_TRANSITIONS, new_capacity);
    dcheck_ne!(new_capacity, capacity);
    let grow_by = new_capacity - capacity;
    let new_array = isolate.factory().copy_weak_fixed_array_and_grow(array, grow_by);

    if capacity < 0 {
      //There was no prototype transitions array before, so the size
      //couldn't be copied. Initialize it explicitly.
      Self::set_number_of_prototype_transitions(new_array, 0);
    }
    new_array
  }

  fn number_of_prototype_transitions(array: &WeakFixedArray) -> usize {
    array.get(K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET).map_or(0, |_| 0)
  }

  fn set_number_of_prototype_transitions(array: &mut WeakFixedArray, value: usize) {
    array[K_PROTO_TRANSITION_NUMBER_OF_ENTRIES_OFFSET] = Some(value); // Replace with correct value
  }
}

impl TransitionsAccessor<'_> {
  // static
  fn put_prototype_transition(
    isolate: &mut Isolate,
    map: &mut Map,
    _prototype: &mut Object,
    target_map: &mut Map,
  ) -> bool {
    //DCHECK_IMPLIES(v8_flags.move_prototype_transitions_first,
    //             IsUndefined(map->GetBackPointer()));
    //DCHECK(IsMap(Cast<HeapObject>(*prototype)->map()));

    //Only the main thread should write to transition arrays.
    //DCHECK_EQ(ThreadId::Current(), isolate->thread_id());

    //It's OK to read the transition array without holding the
    //full_transition_array_access lock in read mode, since this is only called
    //in the main thread, and the main thread is the only writer. In addition, we
    //shouldn't GC while holding the lock, because it will cause a deadlock if a
    //background thread is waiting for the shared mutex outside of a safepoint.

    //Don't cache prototype transition if this map is either shared, or a map of
    //a prototype.
    //if (map->is_prototype_map()) return false;
    //if (map->is_dictionary_map() || !v8_flags.cache_prototype_transitions)
    //return false;

    let header = TransitionArray::K_PROTO_TRANSITION_HEADER_SIZE;

    let mut cache =
        TransitionArray::get_prototype_transitions(isolate, *map); //DirectHandle<WeakFixedArray>
    let capacity = cache.len() - header;
    let transitions = TransitionArray::number_of_prototype_transitions(&cache) + 1;

    if transitions > capacity {
      //Grow the array if compacting it doesn't free space.
      let compacted: bool;
      {
        let _guard = isolate.full_transition_array_access().lock().unwrap();
        //DisallowGarbageCollection no_gc;
        compacted = TransitionArray::compact_prototype_transition_array(isolate, &mut cache);
      }
      if !compacted {
        //if (capacity == TransitionArray::kMaxCachedPrototypeTransitions)
        //return false;

        //cache = TransitionArray::GrowPrototypeTransitionArray(
        //    cache, 2 * transitions, isolate);
        //SetPrototypeTransitions(isolate, map, cache);
      }
    }

    //if (v8_flags.move_prototype_transitions_first) {
    //  target_map->SetBackPointer(*map);
    //}

    //Reload number of transitions as they might have been compacted.
    let last = TransitionArray::number_of_prototype_transitions(&cache);
    let entry = header + last;

    {
      let _guard = isolate.full_transition_array_access().lock().unwrap();
      //DisallowGarbageCollection no_gc;
      //cache->set(entry, MakeWeak(*target_map));
      TransitionArray::set_number_of_prototype_transitions(&mut cache, last + 1);
    }
    true
  }

  // static
  fn get_prototype_transition(
    isolate: &mut Isolate,
    