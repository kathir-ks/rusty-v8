// Converted from V8 C++ source files:
// Header: global-context.h
// Implementation: global-context.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct ContextualClass<T> {}
}

pub mod common {
    pub mod globals {
        pub type SourceId = usize;
        pub struct LineAndColumn {}
        impl LineAndColumn {
            pub fn Invalid() -> Self {
                LineAndColumn {}
            }
        }
    }
}

pub mod torque {
    use std::cell::RefCell;
    use std::collections::{HashMap, HashSet};
    use std::rc::Rc;
    use std::string::String;
    use std::sync::Mutex;

    use super::base;
    use super::common::globals::SourceId;
    use super::common::globals::LineAndColumn;

    const kBaseNamespaceName: &str = "Base";
    const kTaggedSize: usize = 8;
    const kSystemPointerSize: usize = 8;
    const kExternalPointerSlotSize: usize = 8;
    const kCppHeapPointerSlotSize: usize = 8;
    const kTrustedPointerSize: usize = 8;
    const kSmiTagSize: usize = 1;
    const kSmiShiftSize: usize = 1;
    const kApiInt32Size: usize = 4;

    pub struct Ast {}

    pub trait Declaration {}

    pub struct Namespace {
        name: String,
    }

    impl Namespace {
        pub fn new(name: &str) -> Self {
            Namespace {
                name: name.to_string(),
            }
        }
    }

    impl Declaration for Namespace {}

    pub struct GlobalContext {
        collect_language_server_data_: bool,
        collect_kythe_data_: bool,
        force_assert_statements_: bool,
        annotate_ir_: bool,
        default_namespace_: Option<Rc<Namespace>>,
        ast_: Ast,
        declarables_: Vec<Rc<dyn Declaration>>,
        cpp_includes_: HashSet<String>,
        generated_per_file_: HashMap<SourceId, PerFileStreams>,
        fresh_ids_: HashMap<String, usize>,
        macros_for_cc_output_: Vec<(Rc<TorqueMacro>, SourceId)>,
        macros_for_cc_output_set_: HashSet<(Rc<TorqueMacro>, SourceId)>,
        macros_for_cc_debug_output_: Vec<(Rc<TorqueMacro>, SourceId)>,
        macros_for_cc_debug_output_set_: HashSet<(Rc<TorqueMacro>, SourceId)>,
        instance_types_initialized_: bool,
    }

    impl GlobalContext {
        thread_local! {
            static INSTANCE: RefCell<Option<GlobalContext>> = RefCell::new(None);
        }

        pub fn initialize(ast: Ast) {
            GlobalContext::INSTANCE.with(|instance| {
                *instance.borrow_mut() = Some(GlobalContext::new(ast));
            });
        }

        fn new(ast: Ast) -> Self {
            let mut context = GlobalContext {
                collect_language_server_data_: false,
                collect_kythe_data_: false,
                force_assert_statements_: false,
                annotate_ir_: false,
                default_namespace_: None,
                ast_: ast,
                declarables_: Vec::new(),
                cpp_includes_: HashSet::new(),
                generated_per_file_: HashMap::new(),
                fresh_ids_: HashMap::new(),
                macros_for_cc_output_: Vec::new(),
                macros_for_cc_output_set_: HashSet::new(),
                macros_for_cc_debug_output_: Vec::new(),
                macros_for_cc_debug_output_set_: HashSet::new(),
                instance_types_initialized_: false,
            };

            let default_namespace = Rc::new(Namespace::new(kBaseNamespaceName));
            context.default_namespace_ = Some(default_namespace);
            context
        }

        fn get() -> &'static RefCell<Option<GlobalContext>> {
            &GlobalContext::INSTANCE
        }

        pub fn get_default_namespace() -> Rc<Namespace> {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance
                    .as_ref()
                    .unwrap()
                    .default_namespace_
                    .clone()
                    .unwrap()
            })
        }

        fn register_declarable_internal<T: Declaration + 'static>(
            &mut self,
            d: T,
        ) -> Rc<T> {
            let rc = Rc::new(d);
            self.declarables_.push(rc.clone() as Rc<dyn Declaration>);
            rc
        }

        pub fn register_declarable<T: Declaration + 'static>(d: T) -> Rc<T> {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().register_declarable_internal(d)
            })
        }

        pub fn all_declarables() -> Vec<Rc<dyn Declaration>> {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().declarables_.clone()
            })
        }

        pub fn add_cpp_include(include_path: String) {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().cpp_includes_.insert(include_path);
            });
        }

        pub fn cpp_includes() -> HashSet<String> {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().cpp_includes_.clone()
            })
        }

        pub fn set_collect_language_server_data() {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().collect_language_server_data_ = true;
            });
        }

        pub fn collect_language_server_data() -> bool {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().collect_language_server_data_
            })
        }

        pub fn set_collect_kythe_data() {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().collect_kythe_data_ = true;
            });
        }

        pub fn collect_kythe_data() -> bool {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().collect_kythe_data_
            })
        }

        pub fn set_force_assert_statements() {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().force_assert_statements_ = true;
            });
        }

        pub fn force_assert_statements() -> bool {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().force_assert_statements_
            })
        }

        pub fn set_annotate_ir() {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                instance.as_mut().unwrap().annotate_ir_ = true;
            });
        }

        pub fn annotate_ir() -> bool {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().annotate_ir_
            })
        }

        pub fn ast() -> &'static Ast {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                &instance.as_ref().unwrap().ast_
            })
        }

        pub fn make_unique_name(base: &str) -> String {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                let context = instance.as_mut().unwrap();
                let id = context.fresh_ids_.entry(base.to_string()).or_insert(0);
                let result = format!("{}_{}", base, id);
                *id += 1;
                result
            })
        }

        pub fn generated_per_file(file: SourceId) -> PerFileStreams {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                let context = instance.as_mut().unwrap();
                let mut result = context.generated_per_file_.entry(file).or_insert(PerFileStreams::new()).clone();
                result.file = file;
                result
            })
        }

        pub fn set_instance_types_initialized() {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                let context = instance.as_mut().unwrap();
                assert!(!context.instance_types_initialized_);
                context.instance_types_initialized_ = true;
            });
        }

        pub fn is_instance_types_initialized() -> bool {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().instance_types_initialized_
            })
        }

        pub fn ensure_in_cc_output_list(macro_: Rc<TorqueMacro>, source: SourceId) {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                let context = instance.as_mut().unwrap();
                let item = (macro_.clone(), source);
                if context.macros_for_cc_output_set_.insert(item.clone()) {
                    context.macros_for_cc_output_.push(item);
                }
                GlobalContext::ensure_in_cc_debug_output_list(macro_, source);
            });
        }

        pub fn all_macros_for_cc_output() -> Vec<(Rc<TorqueMacro>, SourceId)> {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().macros_for_cc_output_.clone()
            })
        }

        pub fn ensure_in_cc_debug_output_list(macro_: Rc<TorqueMacro>, source: SourceId) {
            GlobalContext::INSTANCE.with(|instance| {
                let mut instance = instance.borrow_mut();
                let context = instance.as_mut().unwrap();
                let item = (macro_.clone(), source);
                if context.macros_for_cc_debug_output_set_.insert(item.clone()) {
                    context.macros_for_cc_debug_output_.push(item);
                }
            });
        }

        pub fn all_macros_for_cc_debug_output() -> Vec<(Rc<TorqueMacro>, SourceId)> {
            GlobalContext::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().macros_for_cc_debug_output_.clone()
            })
        }
    }

    pub fn register_declarable<T: Declaration + 'static>(d: T) -> Rc<T> {
        GlobalContext::INSTANCE.with(|instance| {
            let mut instance = instance.borrow_mut();
            let context = instance.as_mut().unwrap();
            let rc = Rc::new(d);
            context.declarables_.push(rc.clone() as Rc<dyn Declaration>);
            rc
        })
    }

    #[derive(Clone)]
    pub struct PerFileStreams {
        pub file: SourceId,
        pub csa_headerfile: String,
        pub csa_header: cpp::File,
        pub csa_ccfile: String,
        pub csa_cc: cpp::File,
        pub class_definition_headerfile: String,
        pub class_definition_inline_headerfile_macro_declarations: String,
        pub class_definition_inline_headerfile_macro_definitions: String,
        pub class_definition_inline_headerfile: String,
        pub class_definition_ccfile: String,
        pub class_definition_cc: cpp::File,
        pub required_builtin_includes: HashSet<SourceId>,
    }

    impl PerFileStreams {
        pub fn new() -> Self {
            PerFileStreams {
                file: 0,
                csa_headerfile: String::new(),
                csa_header: cpp::File::new(),
                csa_ccfile: String::new(),
                csa_cc: cpp::File::new(),
                class_definition_headerfile: String::new(),
                class_definition_inline_headerfile_macro_declarations: String::new(),
                class_definition_inline_headerfile_macro_definitions: String::new(),
                class_definition_inline_headerfile: String::new(),
                class_definition_ccfile: String::new(),
                class_definition_cc: cpp::File::new(),
                required_builtin_includes: HashSet::new(),
            }
        }
    }

    pub struct CurrentScope {}

    impl CurrentScope {
        pub struct Scope<'a>(&'a ());
    }

    pub struct CurrentSourcePosition {}

    impl CurrentSourcePosition {
        pub struct Scope<'a>(SourcePosition);
    }

    #[derive(Clone, Copy)]
    pub struct SourcePosition {
        pub file: SourceId,
        pub start: LineAndColumn,
        pub end: LineAndColumn,
    }

    pub struct CurrentSourceFile {}

    impl CurrentSourceFile {
        pub fn get() -> SourceId {
            0
        }
    }

    pub struct SmiTagging<const SmiShiftSize: usize>;

    impl<const SmiShiftSize: usize> SmiTagging<SmiShiftSize> {
        const kSmiShiftSize: usize = 1;
    }

    pub struct TargetArchitecture {
        tagged_size_: usize,
        raw_ptr_size_: usize,
        smi_tag_and_shift_size_: usize,
        external_ptr_size_: usize,
        cppheap_ptr_size_: usize,
        trusted_ptr_size_: usize,
    }

    impl TargetArchitecture {
        pub fn new(force_32bit: bool) -> Self {
            TargetArchitecture {
                tagged_size_: if force_32bit { 4 } else { kTaggedSize },
                raw_ptr_size_: if force_32bit { 4 } else { kSystemPointerSize },
                smi_tag_and_shift_size_: kSmiTagSize
                    + (if force_32bit {
                        SmiTagging::<kApiInt32Size>::kSmiShiftSize
                    } else {
                        kSmiShiftSize
                    }),
                external_ptr_size_: if force_32bit { 4 } else { kExternalPointerSlotSize },
                cppheap_ptr_size_: if force_32bit { 4 } else { kCppHeapPointerSlotSize },
                trusted_ptr_size_: if force_32bit { 4 } else { kTrustedPointerSize },
            }
        }

        thread_local! {
            static INSTANCE: RefCell<Option<TargetArchitecture>> = RefCell::new(None);
        }

        pub fn initialize(force_32bit: bool) {
            TargetArchitecture::INSTANCE.with(|instance| {
                *instance.borrow_mut() = Some(TargetArchitecture::new(force_32bit));
            });
        }

         fn get() -> &'static RefCell<Option<TargetArchitecture>> {
            &TargetArchitecture::INSTANCE
        }

        pub fn tagged_size() -> usize {
             TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().tagged_size_
            })
        }

        pub fn raw_ptr_size() -> usize {
            TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().raw_ptr_size_
            })
        }

        pub fn external_pointer_size() -> usize {
            TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().external_ptr_size_
            })
        }

        pub fn cpp_heap_pointer_size() -> usize {
            TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().cppheap_ptr_size_
            })
        }

        pub fn trusted_pointer_size() -> usize {
             TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().trusted_ptr_size_
            })
        }

        pub fn protected_pointer_size() -> usize {
            TargetArchitecture::tagged_size()
        }

        pub fn max_heap_alignment() -> usize {
            TargetArchitecture::tagged_size()
        }

        pub fn are_pointers_compressed() -> bool {
            TargetArchitecture::tagged_size() < TargetArchitecture::raw_ptr_size()
        }

        pub fn smi_tag_and_shift_size() -> i32 {
           TargetArchitecture::INSTANCE.with(|instance| {
                let instance = instance.borrow();
                instance.as_ref().unwrap().smi_tag_and_shift_size_ as i32
            })
        }
    }

    pub struct TorqueMacro {}

    pub mod cpp {
        pub struct File {}

        impl File {
            pub fn new() -> Self {
                File {}
            }
        }
    }
}
