// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::cell::{RefCell, OnceLock};
use std::collections::{HashMap, HashSet};
use std::rc::Rc;
// use std::sync::{Mutex, OnceLock}; // If thread-safe global context is needed
use std::any::Any;

// Placeholder for Ast, CppBuilder, Declarable, Namespace, SourceId, TorqueMacro
mod ast {
    pub struct Ast {}
}
use ast::Ast;

mod cpp_builder {
    pub struct File {}
}
use cpp_builder::File;

mod declarable {
    pub struct Declarable {}
}
use declarable::Declarable;

mod namespace {
    pub struct Namespace {}
}
use namespace::Namespace;

mod source_id {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub struct SourceId(u32);

    impl SourceId {
        pub const INVALID: SourceId = SourceId(0);
    }
}
use source_id::SourceId;

mod torque_macro {
    pub struct TorqueMacro {}
}
use torque_macro::TorqueMacro;

// Mock globals
mod globals {
    pub type SizeT = usize;
}

use globals::SizeT;

#[derive(Default)]
struct GlobalContextState {
    collect_language_server_data: bool,
    collect_kythe_data: bool,
    force_assert_statements: bool,
    annotate_ir: bool,
    default_namespace: Namespace,
    ast: Ast,
    declarables: Vec<Box<dyn Any>>,
    cpp_includes: HashSet<String>,
    generated_per_file: HashMap<SourceId, PerFileStreams>,
    fresh_ids: HashMap<String, usize>,
    macros_for_cc_output: Vec<(Rc<TorqueMacro>, SourceId)>,
    macros_for_cc_output_set: HashSet<(Rc<TorqueMacro>, SourceId)>,
    macros_for_cc_debug_output: Vec<(Rc<TorqueMacro>, SourceId)>,
    macros_for_cc_debug_output_set: HashSet<(Rc<TorqueMacro>, SourceId)>,
    instance_types_initialized: bool,
}

pub struct GlobalContext {
    state: RefCell<GlobalContextState>,
}

impl GlobalContext {
    fn instance() -> &'static GlobalContext {
        static INSTANCE: OnceLock<GlobalContext> = OnceLock::new();
        INSTANCE.get_or_init(|| GlobalContext { state: RefCell::new(GlobalContextState::default()) })
    }

    pub fn new(ast: Ast) -> Self {
        let mut state = GlobalContextState::default();
        state.ast = ast;
        GlobalContext { state: RefCell::new(state) }
    }

    pub fn get_default_namespace() -> &'static Namespace {
        &Self::instance().state.borrow().default_namespace
    }

    pub fn register_declarable<T: 'static>(d: Box<T>) -> *mut T {
        let mut state = Self::instance().state.borrow_mut();
        let ptr = Box::into_raw(d);
        state.declarables.push(unsafe { Box::from_raw(ptr) });
        ptr
    }

    pub fn all_declarables() -> Vec<&'static dyn Any> {
        let state = Self::instance().state.borrow();
        state.declarables.iter().map(|d| d.as_ref() as &dyn Any).collect()
    }

    pub fn add_cpp_include(include_path: String) {
        Self::instance().state.borrow_mut().cpp_includes.insert(include_path);
    }

    pub fn cpp_includes() -> HashSet<String> {
        Self::instance().state.borrow().cpp_includes.clone()
    }

    pub fn set_collect_language_server_data() {
        Self::instance().state.borrow_mut().collect_language_server_data = true;
    }

    pub fn collect_language_server_data() -> bool {
        Self::instance().state.borrow().collect_language_server_data
    }

    pub fn set_collect_kythe_data() {
        Self::instance().state.borrow_mut().collect_kythe_data = true;
    }

    pub fn collect_kythe_data() -> bool {
        Self::instance().state.borrow().collect_kythe_data
    }

    pub fn set_force_assert_statements() {
        Self::instance().state.borrow_mut().force_assert_statements = true;
    }

    pub fn force_assert_statements() -> bool {
        Self::instance().state.borrow().force_assert_statements
    }

    pub fn set_annotate_ir() {
        Self::instance().state.borrow_mut().annotate_ir = true;
    }

    pub fn annotate_ir() -> bool {
        Self::instance().state.borrow().annotate_ir
    }

    pub fn ast() -> &'static Ast {
        &Self::instance().state.borrow().ast
    }

    pub fn make_unique_name(base: &str) -> String {
        let mut state = Self::instance().state.borrow_mut();
        let count = state.fresh_ids.entry(base.to_string()).or_insert(0);
        let result = format!("{}_{}", base, count);
        *count += 1;
        result
    }

    pub fn generated_per_file(file: SourceId) -> &'static mut PerFileStreams {
        let context = Self::instance();
        let mut state = context.state.borrow_mut();
        state.generated_per_file.entry(file).or_insert_with(|| {
            let mut pfs = PerFileStreams::default();
            pfs.file = file;
            pfs
        })
    }

    pub fn set_instance_types_initialized() {
        let mut state = Self::instance().state.borrow_mut();
        assert!(!state.instance_types_initialized);
        state.instance_types_initialized = true;
    }

    pub fn is_instance_types_initialized() -> bool {
        Self::instance().state.borrow().instance_types_initialized
    }

    pub fn ensure_in_cc_output_list(macro_: Rc<TorqueMacro>, source: SourceId) {
        let context = Self::instance();
        let mut state = context.state.borrow_mut();
        let item = (macro_.clone(), source);
        if state.macros_for_cc_output_set.insert(item) {
            state.macros_for_cc_output.push((macro_.clone(), source));
        }
        Self::ensure_in_cc_debug_output_list(macro_, source);
    }

    pub fn all_macros_for_cc_output() -> Vec<(Rc<TorqueMacro>, SourceId)> {
        Self::instance().state.borrow().macros_for_cc_output.clone()
    }

    fn ensure_in_cc_debug_output_list(macro_: Rc<TorqueMacro>, source: SourceId) {
        let context = Self::instance();
        let mut state = context.state.borrow_mut();
        let item = (macro_.clone(), source);
        if state.macros_for_cc_debug_output_set.insert(item) {
            state.macros_for_cc_debug_output.push((macro_.clone(), source));
        }
    }

    pub fn all_macros_for_cc_debug_output() -> Vec<(Rc<TorqueMacro>, SourceId)> {
        Self::instance().state.borrow().macros_for_cc_debug_output.clone()
    }
}

// PerFileStreams equivalent
#[derive(Default)]
pub struct PerFileStreams {
    pub file: SourceId,
    pub csa_headerfile: String,
    pub csa_header: File,
    pub csa_ccfile: String,
    pub csa_cc: File,
    pub class_definition_headerfile: String,
    pub class_definition_inline_headerfile_macro_declarations: String,
    pub class_definition_inline_headerfile_macro_definitions: String,
    pub class_definition_inline_headerfile: String,
    pub class_definition_ccfile: String,
    pub class_definition_cc: File,
    pub required_builtin_includes: HashSet<SourceId>,
}

pub fn register_declarable<T: 'static>(d: Box<T>) -> *mut T {
    GlobalContext::instance().register_declarable(d)
}

// TargetArchitecture equivalent
struct TargetArchitectureState {
    tagged_size: SizeT,
    raw_ptr_size: SizeT,
    smi_tag_and_shift_size: i32,
    external_ptr_size: SizeT,
    cppheap_ptr_size: SizeT,
    trusted_ptr_size: SizeT,
}

pub struct TargetArchitecture {
    state: RefCell<TargetArchitectureState>,
}

impl TargetArchitecture {
    fn instance() -> &'static TargetArchitecture {
        static INSTANCE: OnceLock<TargetArchitecture> = OnceLock::new();
        INSTANCE.get_or_init(|| TargetArchitecture {
            state: RefCell::new(TargetArchitectureState {
                tagged_size: 0,
                raw_ptr_size: 0,
                smi_tag_and_shift_size: 0,
                external_ptr_size: 0,
                cppheap_ptr_size: 0,
                trusted_ptr_size: 0,
            })
        })
    }

    pub fn new(force_32bit: bool) -> Self {
        let tagged_size = if force_32bit { 4 } else { 8 };
        let raw_ptr_size = if force_32bit { 4 } else { 8 };
        let smi_tag_and_shift_size = if force_32bit { 1 } else { 1 };
        let external_ptr_size = if force_32bit { 4 } else { 8 };
        let cppheap_ptr_size = if force_32bit { 4 } else { 8 };
        let trusted_ptr_size = if force_32bit { 4 } else { 8 };

        let state = TargetArchitectureState {
            tagged_size,
            raw_ptr_size,
            smi_tag_and_shift_size,
            external_ptr_size,
            cppheap_ptr_size,
            trusted_ptr_size,
        };

        TargetArchitecture { state: RefCell::new(state) }
    }

    pub fn tagged_size() -> SizeT {
        Self::instance().state.borrow().tagged_size
    }

    pub fn raw_ptr_size() -> SizeT {
        Self::instance().state.borrow().raw_ptr_size
    }

    pub fn external_pointer_size() -> SizeT {
        Self::instance().state.borrow().external_ptr_size
    }

    pub fn cpp_heap_pointer_size() -> SizeT {
        Self::instance().state.borrow().cppheap_ptr_size
    }

    pub fn trusted_pointer_size() -> SizeT {
        Self::instance().state.borrow().trusted_ptr_size
    }

    pub fn protected_pointer_size() -> SizeT {
        Self::tagged_size()
    }

    pub fn max_heap_alignment() -> SizeT {
        Self::tagged_size()
    }

    pub fn are_pointers_compressed() -> bool {
        Self::tagged_size() < Self::raw_ptr_size()
    }

    pub fn smi_tag_and_shift_size() -> i32 {
        Self::instance().state.borrow().smi_tag_and_shift_size
    }
}