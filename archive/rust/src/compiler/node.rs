// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    fmt,
    fmt::Display,
    mem::size_of,
    ops::{Deref, DerefMut},
    ptr,
    ptr::NonNull,
};

// Mock Zone and Operator related structs/enums/functions
// because these are not part of the provided C++ file but are dependencies.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct NodeId(u32);

impl NodeId {
    pub fn new(id: u32) -> Self {
        NodeId(id)
    }
}

impl Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Operator {
    mnemonic: &'static str,
}

impl Operator {
    pub const fn new(mnemonic: &'static str) -> Self {
        Operator { mnemonic }
    }

    pub fn mnemonic(&self) -> &'static str {
        self.mnemonic
    }
}

impl Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.mnemonic)
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum IrOpcode {
    kBranch,
    // Add other opcodes as necessary
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Edge {
    from: *const Node,
    to: *const Node,
    index: usize,
}

impl Edge {
    fn from(&self) -> &Node {
        unsafe { &*self.from }
    }
    fn to(&self) -> &Node {
        unsafe { &*self.to }
    }
    fn index(&self) -> usize {
        self.index
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Type {}

impl Type {
    pub fn new() -> Self {
        Type {}
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Zone {
    supports_compression: bool,
}

impl Zone {
    pub fn new(supports_compression: bool) -> Self {
        Zone {
            supports_compression,
        }
    }

    pub fn supports_compression(&self) -> bool {
        self.supports_compression
    }

    pub fn allocate<T>(&self, size: usize) -> *mut T {
        let layout = std::alloc::Layout::from_size_align(size, std::mem::align_of::<T>()).unwrap();
        unsafe {
            let ptr = std::alloc::alloc(layout) as *mut T;
            if ptr.is_null() {
                panic!("Zone allocation failed");
            }
            ptr
        }
    }
}

impl Drop for Zone {
    fn drop(&mut self) {
        // No deallocation is actually needed here as we're simply
        // modeling the C++ Zone which manages the memory.
    }
}

macro_rules! CHECK {
    ($x:expr) => {
        if !$x {
            panic!("Check failed: {}", stringify!($x));
        }
    };
}

macro_rules! CHECK_EQ {
    ($x:expr, $y:expr) => {
        if $x != $y {
            panic!("Check failed: {} != {}", stringify!($x), stringify!($y));
        }
    };
}

macro_rules! CHECK_IMPLIES {
    ($x:expr, $y:expr) => {
        if $x {
            CHECK!($y);
        }
    };
}

macro_rules! DCHECK {
    ($x:expr) => {
        if cfg!(debug_assertions) {
            if !$x {
                panic!("Debug check failed: {}", stringify!($x));
            }
        }
    };
}

macro_rules! DCHECK_EQ {
    ($x:expr, $y:expr) => {
        if cfg!(debug_assertions) {
            if $x != $y {
                panic!("Debug check failed: {} != {}", stringify!($x), stringify!($y));
            }
        }
    };
}

macro_rules! DCHECK_NE {
    ($x:expr, $y:expr) => {
        if cfg!(debug_assertions) {
            if $x == $y {
                panic!("Debug check failed: {} != {}", stringify!($x), stringify!($y));
            }
        }
    };
}

macro_rules! DCHECK_LE {
    ($x:expr, $y:expr) => {
        if cfg!(debug_assertions) {
            if $x > $y {
                panic!("Debug check failed: {} <= {}", stringify!($x), stringify!($y));
            }
        }
    };
}

macro_rules! DCHECK_LT {
    ($x:expr, $y:expr) => {
        if cfg!(debug_assertions) {
            if $x >= $y {
                panic!("Debug check failed: {} < {}", stringify!($x), stringify!($y));
            }
        }
    };
}

macro_rules! DCHECK_NOT_NULL {
    ($x:expr) => {
        if cfg!(debug_assertions) {
            if ($x).is_null() {
                panic!("Debug check failed: {} is not null", stringify!($x));
            }
        }
    };
}

const kCompressGraphZone: bool = true;
const kMaxInlineCapacity: usize = 8;

/// Represents a pointer to a Node within a Zone.
#[derive(Debug, Copy, Clone)]
struct ZoneNodePtr(*mut Node);

impl ZoneNodePtr {
    fn new(node: *mut Node) -> Self {
        ZoneNodePtr(node)
    }
}

impl Deref for ZoneNodePtr {
    type Target = *mut Node;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ZoneNodePtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Represents a pointer to OutOfLineInputs within a Zone.
#[derive(Debug, Copy, Clone)]
struct ZoneOutOfLineInputsPtr(*mut OutOfLineInputs);

impl Deref for ZoneOutOfLineInputsPtr {
    type Target = *mut OutOfLineInputs;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for ZoneOutOfLineInputsPtr {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// A structure that holds the input nodes when they exceed the inline capacity.
#[derive(Debug)]
struct OutOfLineInputs {
    node_: *mut Node,
    capacity_: usize,
    count_: usize,
    inputs_: Vec<ZoneNodePtr>,
    uses_: Vec<Use>,
}

impl OutOfLineInputs {
    /// Creates a new `OutOfLineInputs` instance with the given capacity.
    fn new(zone: &Zone, capacity: usize) -> *mut Self {
        let size = size_of::<OutOfLineInputs>() + capacity * (size_of::<*mut Node>() + size_of::<Use>());
        let raw_buffer = zone.allocate::<OutOfLineInputs>(size);
        unsafe {
            let outline = raw_buffer.add(capacity * size_of::<Use>()) as *mut OutOfLineInputs;
            (*outline).capacity_ = capacity;
            (*outline).count_ = 0;

            let inputs_ptr = zone.allocate::<ZoneNodePtr>(capacity);
            let uses_ptr = zone.allocate::<Use>(capacity);

            (*outline).inputs_ = Vec::from_raw_parts(inputs_ptr, 0, capacity);
            (*outline).uses_ = Vec::from_raw_parts(uses_ptr, 0, capacity);

            outline
        }
    }

    /// Extracts inputs from old use and input pointers and copies them to this out-of-line storage.
    fn extract_from(&mut self, old_use_ptr: &mut Use, old_input_ptr: &mut ZoneNodePtr, count: usize) {
        DCHECK_GE!(count, 0);

        // Extract the inputs from the old use and input pointers and copy them
        // to this out-of-line-storage.
        let mut new_use_ptr = self.uses_.as_mut_ptr().offset((self.uses_.len() - 1) as isize);
        let mut new_input_ptr = self.inputs_.as_mut_ptr();

        CHECK_IMPLIES!(count > 0, Use::InputIndexField::is_valid(count - 1));
        for current in 0..count {
            unsafe {
                (*new_use_ptr).bit_field_ = Use::InputIndexField::encode(current) | Use::InlineField::encode(false);
                DCHECK_EQ!(old_input_ptr, &mut *old_use_ptr.input_ptr());
                DCHECK_EQ!(&mut *new_input_ptr, &mut *(*new_use_ptr).input_ptr());
                let old_to = **old_input_ptr;
                if !old_to.is_null() {
                    **old_input_ptr = ptr::null_mut();
                    (*old_to).remove_use(old_use_ptr);
                    **new_input_ptr = old_to;
                    (*old_to).append_use(&mut *new_use_ptr);
                } else {
                    **new_input_ptr = ptr::null_mut();
                }

                *old_input_ptr = ZoneNodePtr::new(old_input_ptr.0.wrapping_add(1));
                *new_input_ptr = ZoneNodePtr::new(new_input_ptr.wrapping_add(1));
                *old_use_ptr = Use {
                    bit_field_: old_use_ptr.bit_field_,
                    next: old_use_ptr.next,
                    prev: old_use_ptr.prev,
                };
                *new_use_ptr = Use {
                    bit_field_: new_use_ptr.bit_field_,
                    next: new_use_ptr.next,
                    prev: new_use_ptr.prev,
                };
                new_use_ptr = new_use_ptr.wrapping_sub(1);
            }
        }
        self.count_ = count;
    }

    /// Returns a pointer to the beginning of the input array.
    fn inputs(&mut self) -> *mut ZoneNodePtr {
        self.inputs_.as_mut_ptr()
    }
}

/// Represents a use of a node.
#[derive(Debug, Copy, Clone)]
struct Use {
    bit_field_: usize,
    next: *mut Use,
    prev: *mut Use,
}

impl Use {
    // Nested structs/enums for bitfield access
    #[allow(non_snake_case)]
    struct InputIndexField {}
    impl InputIndexField {
        const kShift: usize = 0;
        const kMask: usize = (1 << 10) - 1; // Example mask, adjust as needed
        const kMax: usize = Self::kMask;

        fn encode(value: usize) -> usize {
            (value & Self::kMask) << Self::kShift
        }

        fn decode(bits: usize) -> usize {
            (bits >> Self::kShift) & Self::kMask
        }

        fn is_valid(value: usize) -> bool {
            value <= Self::kMax
        }
    }

    #[allow(non_snake_case)]
    struct InlineField {}
    impl InlineField {
        const kShift: usize = 10;
        const kMask: usize = 1 << Self::kShift;

        fn encode(value: bool) -> usize {
            (value as usize) << Self::kShift
        }

        fn decode(bits: usize) -> bool {
            ((bits >> Self::kShift) & 1) != 0
        }
    }

    fn input_index(&self) -> usize {
        InputIndexField::decode(self.bit_field_)
    }

    fn is_inline(&self) -> bool {
        InlineField::decode(self.bit_field_)
    }

    fn input_ptr(&mut self) -> *mut ZoneNodePtr {
        todo!()
    }
}

// These structs are just type tags for Zone::Allocate<T>(size_t) calls.
struct NodeWithOutOfLineInputs {}
struct NodeWithInLineInputs {}

#[derive(Debug)]
pub struct Node {
    op_: *const Operator,
    mark_: u8,
    bit_field_: usize,
    first_use_: *mut Use,
    inputs_: InputsStorage,
    type_: Type,
}

#[derive(Debug)]
enum InputsStorage {
    Inline {
        count: usize,
        capacity: usize,
        inputs: Vec<ZoneNodePtr>,
        uses: Vec<Use>,
    },
    OutOfLine {
        outline_inputs: *mut OutOfLineInputs,
    },
}

impl Node {
    #[allow(non_snake_case)]
    struct IdField {}
    impl IdField {
        const kShift: usize = 0;
        const kMask: usize = (1 << 20) - 1; // Example mask, adjust as needed
        const kMax: usize = Self::kMask;

        fn encode(value: NodeId) -> usize {
            (value.0 as usize & Self::kMask) << Self::kShift
        }

        fn decode(bits: usize) -> NodeId {
            NodeId(((bits >> Self::kShift) & Self::kMask) as u32)
        }

        fn is_valid(value: NodeId) -> bool {
            (value.0 as usize) <= Self::kMax
        }
    }

    #[allow(non_snake_case)]
    struct InlineCountField {}
    impl InlineCountField {
        const kShift: usize = 20;
        const kMask: usize = (1 << 4) - 1;
        const kMax: usize = Self::kMask;

        fn encode(value: usize) -> usize {
            (value & Self::kMask) << Self::kShift
        }

        fn decode(bits: usize) -> usize {
            (bits >> Self::kShift) & Self::kMask
        }

        fn update(bits: usize, value: usize) -> usize {
            (bits & !(Self::kMask << Self::kShift)) | ((value & Self::kMask) << Self::kShift)
        }
    }

    #[allow(non_snake_case)]
    struct InlineCapacityField {}
    impl InlineCapacityField {
        const kShift: usize = 24;
        const kMask: usize = (1 << 4) - 1;
        const kMax: usize = Self::kMask;

        fn encode(value: usize) -> usize {
            (value & Self::kMask) << Self::kShift
        }

        fn decode(bits: usize) -> usize {
            (bits >> Self::kShift) & Self::kMask
        }
    }

    const kOutlineMarker: usize = InlineCountField::kMax;

    /// Creates a new `Node` instance.
    fn new(id: NodeId, op: *const Operator, inline_count: usize, inline_capacity: usize) -> Self {
        // Check that the id didn't overflow.
        static_assert!(IdField::kMax < std::u32::MAX as usize);
        CHECK!(IdField::is_valid(id));

        // Inputs must either be out of line or within the inline capacity.
        DCHECK!(inline_count == Self::kOutlineMarker || inline_count <= inline_capacity);
        DCHECK_LE!(inline_capacity, kMaxInlineCapacity);

        Node {
            op_: op,
            mark_: 0,
            bit_field_: IdField::encode(id) | InlineCountField::encode(inline_count) | InlineCapacityField::encode(inline_capacity),
            first_use_: ptr::null_mut(),
            inputs_: InputsStorage::Inline {
                count: inline_count,
                capacity: inline_capacity,
                inputs: Vec::new(),
                uses: Vec::new(),
            },
            type_: Type::new(),
        }
    }

    fn new_impl<NodePtrT>(
        zone: &Zone,
        id: NodeId,
        op: *const Operator,
        input_count: usize,
        inputs: *const ZoneNodePtr,
        has_extensible_inputs: bool,
    ) -> *mut Self {
        // Node uses compressed pointers, so zone must support pointer compression.
        CHECK_IMPLIES!(kCompressGraphZone, zone.supports_compression());
        DCHECK_GE!(input_count, 0);

        unsafe {
            // Verify that none of the inputs are {nullptr}.
            for i in 0..input_count {
                if (*inputs.add(i)).0.is_null() {
                    panic!(
                        "Node::New() Error: #{}:{} is nullptr",
                        id,
                        (*op).mnemonic()
                    );
                }
            }
        }

        let mut input_ptr: *mut ZoneNodePtr;
        let mut use_ptr: *mut Use;
        let node: *mut Node;
        let is_inline: bool;

        if input_count > kMaxInlineCapacity {
            // Allocate out-of-line inputs.
            let capacity = if has_extensible_inputs {
                input_count + kMaxInlineCapacity
            } else {
                input_count
            };
            let outline = OutOfLineInputs::new(zone, capacity);

            // Allocate node, with space for OutOfLineInputs pointer.
            let node_buffer = zone.allocate::<NodeWithOutOfLineInputs>(size_of::<Node>() + size_of::<ZoneOutOfLineInputsPtr>());
            node = node_buffer as *mut Node;
            unsafe {
                ptr::write(node, Node {
                    op_: op,
                    mark_: 0,
                    bit_field_: IdField::encode(id) | InlineCountField::encode(Self::kOutlineMarker),
                    first_use_: ptr::null_mut(),
                    inputs_: InputsStorage::OutOfLine {
                        outline_inputs: outline,
                    },
                    type_: Type::new(),
                });
            }

            unsafe {
                (*node).set_outline_inputs(ZoneOutOfLineInputsPtr(outline));
                (*outline).node_ = node;
                (*outline).count_ = input_count;

                input_ptr = (*outline).inputs();
                use_ptr = (*outline).uses_.as_mut_ptr();
            }
            is_inline = false;
        } else {
            // Allocate node with inline inputs. Capacity must be at least 1 so that
            // an OutOfLineInputs pointer can be stored when inputs are added later.
            let mut capacity = std::cmp::max(1, input_count);
            if has_extensible_inputs {
                const max: usize = kMaxInlineCapacity;
                capacity = std::cmp::min(input_count + 3, max);
            }

            let size = size_of::<Node>() + capacity * (size_of::<ZoneNodePtr>() + size_of::<Use>());
            let raw_buffer = zone.allocate::<NodeWithInLineInputs>(size);
            let node_buffer = raw_buffer as *mut Node;

            unsafe {
                ptr::write(node_buffer, Node {
                    op_: op,
                    mark_: 0,
                    bit_field_: IdField::encode(id) | InlineCountField::encode(input_count) | InlineCapacityField::encode(capacity),
                    first_use_: ptr::null_mut(),
                    inputs_: InputsStorage::Inline {
                        count: input_count,
                        capacity: capacity,
                        inputs: Vec::from_raw_parts(zone.allocate::<ZoneNodePtr>(capacity), 0, capacity),
                        uses: Vec::from_raw_parts(zone.allocate::<Use>(capacity), 0, capacity),
                    },
                    type_: Type::new(),
                });

                input_ptr = (*node_buffer).inline_inputs();
                use_ptr = (*node_buffer).inline_uses();
            }
            is_inline = true;
        }

        // Initialize the input pointers and the uses.
        CHECK_IMPLIES!(input_count > 0, Use::InputIndexField::is_valid(input_count - 1));
        for current in 0..input_count {
            unsafe {
                let to = *inputs.add(current);
                *input_ptr.add(current) = to;
                let use_ = use_ptr.sub(input_count - 1 - current);
                (*use_).bit_field_ = Use::InputIndexField::encode(current) | Use::InlineField::encode(is_inline);
                (*to).append_use(&mut *use_);
            }
        }

        unsafe {
            (*node).verify();
            node
        }
    }

    /// Creates a new `Node` instance.
    pub fn new_node(
        zone: &Zone,
        id: NodeId,
        op: *const Operator,
        input_count: usize,
        inputs: *const ZoneNodePtr,
        has_extensible_inputs: bool,
    ) -> *mut Self {
        Self::new_impl::<ZoneNodePtr>(zone, id, op, input_count, inputs, has_extensible_inputs)
    }

    /// Clones a `Node` instance.
    pub fn clone_node(zone: &Zone, id: NodeId, node: &Node) -> *mut Node {
        let input_count = node.input_count();
        let inputs = if node.has_inline_inputs() {
            node.inline_inputs()
        } else {
            unsafe { (*node.outline_inputs().0).inputs() }
        };

        let clone = Self::new_impl(zone, id, node.op(), input_count, inputs, false);
        unsafe {
            (*clone).set_type(node.type_());
        }
        clone
    }

    /// Kills this node by nulling all inputs and removing all uses.
    pub fn kill(&mut self) {
        DCHECK_NOT_NULL!(self.op());
        self.null_all_inputs();
        DCHECK!(self.uses().is_empty());
    }

    /// Appends an input to this node.
    pub fn append_input(&mut self, zone: &Zone, new_to: *mut Node) {
        DCHECK_NOT_NULL!(zone);
        DCHECK_NOT_NULL!(new_to);

        match &mut self.inputs_ {
            InputsStorage::Inline {
                count,
                capacity,
                inputs,
                uses,
            } => {
                if *count < *capacity {
                    // Append inline input.
                    self.bit_field_ = InlineCountField::update(self.bit_field_, *count + 1);
                    unsafe {
                        *self.get_input_ptr(*count) = ZoneNodePtr(new_to);
                        let use_ = self.get_use_ptr(*count);
                        static_assert!(InlineCapacityField::kMax <= Use::InputIndexField::kMax);
                        (*use_).bit_field_ = Use::InputIndexField::encode(*count) | Use::InlineField::encode(true);
                        (*new_to).append_use(&mut *use_);
                    }
                    *count += 1;
                } else {
                    // Append out-of-line input.
                    let input_count = self.input_count();
                    unsafe {
                        let outline = match InlineCountField::decode(self.bit_field_) {
                            Self::kOutlineMarker => {
                                // use current out of line inputs.
                                let outline = match &mut self.inputs_ {
                                    InputsStorage::OutOfLine { outline_inputs } => &mut **outline_inputs,
                                    _ => unreachable!()
                                };
                                if input_count >= outline.capacity_ {
                                    // out of space in out-of-line inputs.
                                    let outline = OutOfLineInputs::new(zone, input_count * 2 + 3);
                                    (*outline).node_ = self;

                                    let mut input_ptr = self.get_input_ptr(0);
                                    let mut use_ptr = self.get_use_ptr(0);

                                    outline.extract_from(&mut *use_ptr, &mut *input_ptr, input_count);
                                    self.set_outline_inputs(ZoneOutOfLineInputsPtr(outline));
                                    outline
                                } else {
                                    outline
                                }
                            }
                            _ => {
                                // switch to out of line inputs.
                                let outline = OutOfLineInputs::new(zone, input_count * 2 + 3);
                                (*outline).node_ = self;

                                let mut input_ptr = self.get_input_ptr(0);
                                let mut use_ptr = self.get_use_ptr(0);

                                outline.extract_from(&mut *use_ptr, &mut *input_ptr, input_count);
                                self.bit_field_ = InlineCountField::update(self.bit_field_, Self::kOutlineMarker);
                                self.set_outline_inputs(ZoneOutOfLineInputsPtr(outline));
                                outline
                            }
                        };
                        outline.count_ += 1;

                        *self.get_input_ptr(input_count) = ZoneNodePtr(new_to);
                        let use_ = self.get_use_ptr(input_count);
                        CHECK!(Use::InputIndexField::is_valid(input_count));
                        (*use_).bit_field_ = Use::InputIndexField::encode(input_count) | Use::InlineField::encode(false);
                        (*new_to).append_use(&mut *use_);

                        match &mut self.inputs_ {
                            InputsStorage::OutOfLine { outline_inputs } => (**outline_inputs).count_ += 1,
                            _ => unreachable!()
                        }
                    }
                }
            }
            InputsStorage::OutOfLine { .. } => {
                // Append out-of-line input.
                let input_count = self.input_count();
                unsafe {
                    let outline = match InlineCountField::decode(self.bit_field_) {
                        Self::kOutlineMarker => {
                            // use current out of line inputs.
                            let outline = match &mut self.inputs_ {
                                InputsStorage::OutOfLine { outline_inputs } => &mut **outline_inputs,
                                _ => unreachable!()
                            };
                            if input_count >= outline.capacity_ {
                                // out of space in out-of-line inputs.
                                let outline = OutOfLineInputs::new(zone, input_count * 2 + 3);
                                (*outline).node_ = self;

                                let mut input_ptr = self.get_input_ptr(0);
                                let mut use_ptr = self.get_use_ptr(0);

                                outline.extract_from(&mut *use_ptr, &mut *input_ptr, input_count);
                                self.set_outline_inputs(ZoneOutOfLineInputsPtr(outline));
                                outline
                            } else {
                                outline
                            }
                        }
                        _ => {
                            // switch to out of line inputs.
                            let outline = OutOfLineInputs::new(zone, input_count * 2 + 3);
                            (*outline).node_ = self;

                            let mut input_ptr = self.get_input_ptr(0);
                            let mut use_ptr = self.get_use_ptr(0);

                            outline.extract_from(&mut *use_ptr, &mut *input_ptr, input_count);
                            self.bit_field_ = InlineCountField::update(self.bit_field_, Self::kOutlineMarker);
                            self.set_outline_inputs(ZoneOutOfLineInputsPtr(outline));
                            outline
                        }
                    };
                    outline.count_ += 1;

                    *self.get_input_ptr(input_count) = ZoneNodePtr(new_to);
                    let use_ = self.get_use_ptr(input_count);
                    CHECK!(Use::InputIndexField::is_valid(input_count));
                    (*use_).bit_field_ = Use::InputIndexField::encode(input_count) | Use::InlineField::encode(false);
                    (*new_to).append_use(&mut *use_);

                    match &mut self.inputs_ {
                        InputsStorage::OutOfLine { outline_inputs } => (**outline_inputs).count_ += 1,
                        _ => unreachable!()
                    }
                }
            }
        }

        self.verify();
    }

    /// Inserts an input at the given index.
    pub fn insert_input(&mut self, zone: &Zone, index: usize, new_to: *mut Node) {
        DCHECK_NOT_NULL!(zone);
        DCHECK_LE!(0, index);
        DCHECK_LT!(index, self.input_count());
        unsafe {
            self.append_input(zone, self.input_at(self.input_count() - 1));
            for i in (index + 1..self.input_count()).rev() {
                self.replace_input(i, self.input_at(i - 1));
            }
            self.replace_input(index, new_to);
        }
        self.verify();
    }

    /// Inserts multiple inputs at the given index.
    pub fn insert_inputs(&mut self, zone: &Zone, index: usize, count: usize) {
        DCHECK_NOT_NULL!(zone);
        DCHECK_LE!(0, index);
        DCHECK_LT!(0, count);
        DCHECK_LT!(index, self.input_count());
        for _ in 0..count {
            unsafe {
                self.append_input(zone, self.input_at(std::cmp::max(self.input_count() - count, 0)));
            }
        }
        for i in (std::cmp::max(index, count)..self.input_count() - count).rev() {
            unsafe {
                self.replace_input(i + count, self.input_at(i));
            }
        }
        for i in 0..count {
            self.replace_input(index + i, ptr::null_mut());
        }
        self.verify();
    }

    /// Removes the input at the given index.
    pub fn remove_input(&mut self, index: usize) -> *mut Node {
        DCHECK_LE!(0, index);
        DCHECK_LT!(index, self.input_count());
        let result = unsafe { self.input_at(index) };
        for i in index..self.input_count() - 1 {
            unsafe {
                self.replace_input(i, self.input_at(i + 1));
            }
        }
        self.trim_input_count(self.input_count() - 1);
        self.verify();
        result
    }

    /// Clears a range of inputs.
    pub fn clear_inputs(&mut self, start: usize, count: usize) {
        unsafe {
            let mut input_ptr = self.get_input_ptr(start);
            let mut use_ptr = self.get_use_ptr(start);
            for _ in 0..count {
                DCHECK_EQ!(input_ptr, (*use_ptr).input_ptr());
                let input = *input_ptr;
                *input_ptr = ZoneNodePtr(ptr::null_mut());
                if !input.is_null() {
                    (*input).remove_use(&mut *use_ptr);
                }
                input_ptr = input_ptr.add(1);
                use_ptr = use_ptr.sub(1);
            }
        }
        self.verify();
    }

    /// Nulls all inputs of this node.
    pub fn null_all_inputs(&mut self) {
        self.clear_inputs(0, self.input_count());
    }

    /// Trims the input count of this node.
    pub fn trim_input_count(&mut self, new_input_count: usize) {
        let current_count = self.input_count();
        DCHECK_LE!(new_input_count, current_count);
        if new_input_count == current_count {
            return; // Nothing to do.
        }
        self.clear_inputs(new_input_count, current_count - new_input_count);

        match &mut self.inputs_ {
            InputsStorage::Inline { count, .. } => {
                self.bit_field_ = InlineCountField::update(self.bit_field_, new_input_count);
                *count = new_input_count;
            }
            InputsStorage::OutOfLine { outline_inputs } => unsafe {
                (**outline_inputs).count_ = new_input_count;
            },
        }
    }

    /// Ensures that the input count of this node is at least the given value.
    pub fn ensure_input_count(&mut self, zone: &Zone, new_input_count: usize) {
        let current_count = self.input_count();
        DCHECK_NE!(current_count, 0);
        if current_count > new_input_count {
            self.trim_input_count(new_input_count);
        } else if current_count < new_input_count {
            unsafe {
                let dummy = self.input_at(current_count - 1);
                let mut current_count_mut = current_count;
                while current_count_mut < new_input_count {
                    self.append_input(zone, dummy);
                    current_count_mut += 1;
                }
            }
        }
    }

    /// Returns the number of uses of this node.
    pub fn use_count(&self) -> usize