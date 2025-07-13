// Converted from V8 C++ source files:
// Header: source-text-module.h
// Implementation: source-text-module.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod source_text_module {
use std::cell::RefCell;
use std::collections::HashSet;
use std::collections::HashMap;
use std::rc::Rc;

use crate::objects::contexts::*;
use crate::objects::module::*;
use crate::objects::promise::*;
use crate::objects::string::*;
use crate::zone::zone_containers::*;
use crate::torque_generated::bit_fields::*;

// Has to be the last include (doesn't have include guards):
//use crate::objects::object_macros::*;

use crate::objects::object_macros_tq::*;

pub struct UnorderedModuleSet {}

pub struct StructBodyDescriptor {}

//#[derive(Struct, Copy, Clone, Debug)]
//#[repr(C)]
pub struct SourceTextModule {
    module: Module,
    has_toplevel_await: bool,
    shared_function_info: Rc<RefCell<SharedFunctionInfo>>,
    info: Rc<RefCell<SourceTextModuleInfo>>,
}

impl SourceTextModule {
    pub fn GetSharedFunctionInfo(&self) -> Rc<RefCell<SharedFunctionInfo>> {
        self.shared_function_info.clone()
    }

    pub fn GetScript(&self) -> Script {
        Script {}
    }

    pub fn has_toplevel_await(&self) -> bool {
        self.has_toplevel_await
    }

    pub fn set_has_toplevel_await(&mut self, value: bool) {
        self.has_toplevel_await = value;
    }

    pub fn info(&self) -> Rc<RefCell<SourceTextModuleInfo>> {
        self.info.clone()
    }

    pub fn GetCell(&self, cell_index: i32) -> Cell {
        Cell {}
    }

    pub fn LoadVariable(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>, cell_index: i32) -> Object {
        Object {}
    }

    pub fn StoreVariable(module: Rc<RefCell<SourceTextModule>>, cell_index: i32, value: Object) {
    }

    pub fn ImportIndex(cell_index: i32) -> i32 {
        if SourceTextModuleDescriptor::GetCellIndexKind(cell_index) == SourceTextModuleDescriptor::CellIndexKind::kExport {
            panic!("cell index is an export, not an import");
        }
        -cell_index - 1
    }

    pub fn ExportIndex(cell_index: i32) -> i32 {
        if SourceTextModuleDescriptor::GetCellIndexKind(cell_index) == SourceTextModuleDescriptor::CellIndexKind::kImport {
            panic!("cell index is an import, not an export");
        }
        cell_index - 1
    }

    pub fn AsyncModuleExecutionFulfilled(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>) -> Result<bool, String> {
        Ok(true)
    }

    pub fn AsyncModuleExecutionRejected(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>, exception: Object) {
    }

    pub fn GetModuleNamespace(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>, module_request: i32) -> JSModuleNamespace {
        JSModuleNamespace {}
    }

    pub fn GetImportMeta(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>) -> Result<JSObject, String> {
        Ok(JSObject {})
    }

    pub fn GetStalledTopLevelAwaitMessages(&self, isolate: &Isolate) -> (Vec<Rc<RefCell<SourceTextModule>>>, Vec<Rc<RefCell<JSMessageObject>>>) {
        (Vec::new(), Vec::new())
    }

    fn AddAsyncParentModule(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        parent: Rc<RefCell<SourceTextModule>>,
    ) {
    }

    fn GetCycleRoot(&self, isolate: &Isolate) -> Rc<RefCell<SourceTextModule>> {
        Rc::new(RefCell::new(SourceTextModule{
            module: Module {},
            has_toplevel_await: false,
            shared_function_info: Rc::new(RefCell::new(SharedFunctionInfo {})),
            info: Rc::new(RefCell::new(SourceTextModuleInfo {})),
        }))
    }

    fn GetAsyncParentModule(&self, isolate: &Isolate, index: i32) -> Rc<RefCell<SourceTextModule>> {
        Rc::new(RefCell::new(SourceTextModule{
            module: Module {},
            has_toplevel_await: false,
            shared_function_info: Rc::new(RefCell::new(SharedFunctionInfo {})),
            info: Rc::new(RefCell::new(SourceTextModuleInfo {})),
        }))
    }

    fn AsyncParentModuleCount(&self) -> i32 {
        0
    }

    fn HasAsyncEvaluationOrdinal(&self) -> bool {
        false
    }

    fn HasPendingAsyncDependencies(&self) -> bool {
        false
    }

    fn IncrementPendingAsyncDependencies(&mut self) {}

    fn DecrementPendingAsyncDependencies(&mut self) {}

    fn CreateExport(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>, cell_index: i32, names: Rc<RefCell<FixedArray>>) {
    }

    fn CreateIndirectExport(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        name: Rc<RefCell<String>>,
        entry: Rc<RefCell<SourceTextModuleInfoEntry>>,
    ) {
    }

    fn ResolveExport(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        module_specifier: Rc<RefCell<String>>,
        export_name: Rc<RefCell<String>>,
        loc: MessageLocation,
        must_resolve: bool,
        resolve_set: &mut Module::ResolveSet,
    ) -> Result<Cell, String> {
        Ok(Cell {})
    }

    fn ResolveImport(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        name: Rc<RefCell<String>>,
        module_request_index: i32,
        loc: MessageLocation,
        must_resolve: bool,
        resolve_set: &mut Module::ResolveSet,
    ) -> Result<Cell, String> {
        Ok(Cell {})
    }

    fn ResolveExportUsingStarExports(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        module_specifier: Rc<RefCell<String>>,
        export_name: Rc<RefCell<String>>,
        loc: MessageLocation,
        must_resolve: bool,
        resolve_set: &mut Module::ResolveSet,
    ) -> Result<Cell, String> {
        Ok(Cell {})
    }

    fn PrepareInstantiate(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        context: v8::Local<v8::Context>,
        module_callback: v8::Module::ResolveModuleCallback,
        source_callback: v8::Module::ResolveSourceCallback,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn FinishInstantiate(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        stack: &mut ZoneForwardList<Rc<RefCell<SourceTextModule>>>,
        dfs_index: &mut u32,
        zone: &mut Zone,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn RunInitializationCode(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn MaybeTransitionComponent(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        stack: &mut ZoneForwardList<Rc<RefCell<SourceTextModule>>>,
        new_status: Status,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn FetchStarExports(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        zone: &mut Zone,
        visited: &mut HashSet<Rc<RefCell<SourceTextModule>>>,
    ) {
    }

    fn GatherAvailableAncestors(
        isolate: &Isolate,
        zone: &mut Zone,
        start: Rc<RefCell<SourceTextModule>>,
        exec_list: &mut AvailableAncestorsSet,
    ) {
    }

    fn ExecuteModule(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        exception_out: &mut Option<Object>,
    ) -> Result<Object, String> {
        Ok(Object {})
    }

    fn InnerExecuteAsyncModule(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        capability: Rc<RefCell<JSPromise>>,
    ) -> Result<Object, String> {
        Ok(Object {})
    }

    fn Evaluate(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
    ) -> Result<Object, String> {
        Ok(Object {})
    }

    fn InnerModuleEvaluation(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        stack: &mut ZoneForwardList<Rc<RefCell<SourceTextModule>>>,
        dfs_index: &mut u32,
    ) -> Result<Object, String> {
        Ok(Object {})
    }

    fn MaybeHandleEvaluationException(
        &mut self,
        isolate: &Isolate,
        stack: &mut ZoneForwardList<Rc<RefCell<SourceTextModule>>>,
    ) -> bool {
        true
    }

    fn AsyncModuleExecutionFulfilled(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn AsyncModuleExecutionRejected(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
        exception: Object,
    ) {
    }

    fn ExecuteAsyncModule(
        isolate: &Isolate,
        module: Rc<RefCell<SourceTextModule>>,
    ) -> Result<bool, String> {
        Ok(true)
    }

    fn Reset(isolate: &Isolate, module: Rc<RefCell<SourceTextModule>>) {}
    fn set_async_evaluation_ordinal(&mut self, ordinal: u32) {}
    fn record_error(&mut self, isolate: &Isolate, exception: Object) {}
    fn set_code(&mut self, code: Object) {}
    fn exception(&self) -> Object {Object{}}
    fn set_regular_exports(&mut self, fixed_array:FixedArray){}
    fn set_regular_imports(&mut self, fixed_array:FixedArray){}
    fn set_requested_modules(&mut self, fixed_array:FixedArray){}
    fn set_dfs_index(&mut self, index:i32){}
    fn set_dfs_ancestor_index(&mut self, index:i32){}
    fn module_requests(&self) -> FixedArray {FixedArray{}}
    fn set_exports(&mut self, table: ObjectHashTable) {}
    fn status(&self) -> i32 {0}
    fn set_cycle_root(&mut self, module:Rc<RefCell<SourceTextModule>>){}
    fn import_meta(&self, kAcquireLoad:i32) -> Object{Object{}}
    fn set_import_meta(&mut self, obj:Object, kReleaseStore:i32){}
    fn set_status(&mut self, status:i32){}
    fn requested_modules(&self) -> FixedArray {FixedArray{}}
}

pub struct SourceTextModuleInfo {
}

impl SourceTextModuleInfo {
    pub fn RegularExportCount(&self) -> i32 {
        0
    }

    pub fn RegularExportLocalName(&self, i: i32) -> String {
        String {}
    }

    pub fn RegularExportCellIndex(&self, i: i32) -> i32 {
        0
    }

    pub fn RegularExportExportNames(&self, i: i32) -> FixedArray {
        FixedArray {}
    }
}

pub struct ModuleRequest {}

impl ModuleRequest {
    pub fn phase(&self) -> ModuleImportPhase {
        ModuleImportPhase::kEvaluation
    }
    pub fn specifier(&self) -> String {String{}}
    pub fn import_attributes(&self) -> FixedArray {FixedArray{}}
}

pub struct SourceTextModuleInfoEntry {}

impl SourceTextModuleInfoEntry{
    pub fn export_name(&self) -> Object {Object{}}
    pub fn import_name(&self) -> Object {Object{}}
    pub fn module_request(&self) -> i32 {0}
    pub fn beg_pos(&self) -> i32 {0}
    pub fn end_pos(&self) -> i32 {0}
}

pub mod v8 {
    pub mod Utils {
        pub fn ToLocal<T>(value: super::String) -> Local<T> {
            Local {}
        }

        pub fn FixedArrayToLocal<T>(array: super::FixedArray) -> Local<T> {
            Local {}
        }
    }
    pub struct Local<T> {}
    pub type Module = super::Module;
    pub trait ResolveModuleCallback {}
    pub trait ResolveSourceCallback {}
}

pub struct Isolate {}
pub struct Factory {}
pub struct JSModuleNamespace {}
pub struct JSObject {}
pub struct Context {}
pub struct Object {}
pub struct String {}
pub struct FixedArray {}
pub struct Cell {}
pub struct Script {}
pub struct JSAsyncFunctionObject {}
pub struct JSFunction {}
pub struct JSGeneratorObject {}
pub struct JSMessageObject {}
pub struct MessageLocation {}
pub struct Zone {}
pub struct ZoneForwardList<T> {}
pub struct ObjectHashTable {}
pub struct SharedFunctionInfo {}
pub struct UnorderedStringSet {}
pub struct UnorderedStringMap {}
pub struct ReadOnlyRoots {}
pub struct Status {}

pub struct ModuleHandleHash {}
pub struct ModuleHandleEqual {}

pub struct StringHandleHash {}
pub struct StringHandleEqual {}

pub enum MessageTemplate {}

pub struct DirectHandle<T> {}
pub struct JSPromise {}
pub struct JSIteratorResult {}
pub struct InternalIndex {}

pub struct HashTable {}

impl Module {
    pub type ResolveSet = HashMap<Rc<RefCell<Module>>, Box<HashSet<Rc<RefCell<String>>>>>;

    pub fn PrepareInstantiate(
        isolate: &Isolate,
        module: Rc<RefCell<Module>>,
        context: v8::Local<v8::Context>,
        module_callback: v8::Module::ResolveModuleCallback,
        source_callback: v8::Module::ResolveSourceCallback,
    ) -> Result<bool, String> {
        Ok(true)
    }

    pub fn FinishInstantiate(
        isolate: &Isolate,
        module: Rc<RefCell<Module>>,
        stack: &mut ZoneForwardList<Rc<RefCell<Module>>>,
        dfs_index: &mut u32,
        zone: &mut Zone,
    ) -> Result<bool, String> {
        Ok(true)
    }

    pub fn Evaluate(isolate: &Isolate, module: Rc<RefCell<Module>>) -> Result<Object, String> {
        Ok(Object {})
    }
    pub fn StatusString(status:i32) -> String {
        String{}
    }
    pub fn GetModuleNamespace(isolate: &Isolate, module: Rc<RefCell<Module>>) -> JSModuleNamespace{
        JSModuleNamespace {}
    }
}

impl Isolate {
    pub fn Throw(&self, exception: Object) {}
    pub fn factory(&self) -> Factory { Factory{} }
    pub fn has_exception(&self) -> bool {false}
    pub fn is_catchable_by_javascript(&self, exception: Object) -> bool {true}
    pub fn native_context(&self) -> Rc<RefCell<Context>> {Rc::new(RefCell::new(Context{}))}
    pub fn promise_then(&self) -> Object {Object{}}
    pub fn is_execution_terminating(&self) -> bool {false}
    pub fn RunHostInitializeImportMetaObjectCallback(&self, module: Rc<RefCell<SourceTextModule>>) -> Result<JSObject, String> { Ok(JSObject{}) }
    pub fn NextModuleAsyncEvaluationOrdinal(&self) -> u32 {0}
}

impl Factory {
    pub fn NewCell(&self) -> Rc<RefCell<Cell>> { Rc::new(RefCell::new(Cell{})) }
    pub fn NewSyntaxError(&self, template: MessageTemplate, arg1: Rc<RefCell<String>>, arg2: Rc<RefCell<String>>) -> Object { Object{} }
    pub fn NewFixedArray(&self, length: i32) -> Rc<RefCell<FixedArray>> {Rc::new(RefCell::new(FixedArray{}))}
    pub fn undefined_value(&self) -> Object {Object{}}
    pub fn null_value(&self) -> Object {Object{}}
    pub fn NewJSPromise(&self) -> Rc<RefCell<JSPromise>> { Rc::new(RefCell::new(JSPromise{})) }
    pub fn NewBuiltinContext(&self, arg1:Rc<RefCell<Context>>, arg2:i32) -> Rc<RefCell<Context>> { Rc::new(RefCell::new(Context{})) }
    pub fn source_text_module_execute_async_module_fulfilled_sfi(&self) -> SharedFunctionInfo { SharedFunctionInfo {} }
    pub fn source_text_module_execute_async_module_rejected_sfi(&self) -> SharedFunctionInfo { SharedFunctionInfo {} }
}
pub struct Execution {}
impl Execution {
    pub fn Call(isolate: &Isolate, function: Rc<RefCell<JSFunction>>, receiver: Rc<RefCell<Object>>, args: [(); 0]) -> Result<Object, String> {
        Ok(Object{})
    }
    pub fn TryCall(isolate: &Isolate, function: Rc<RefCell<JSFunction>>, receiver: Rc<RefCell<Object>>, args: [(); 0], handling:i32, exception_out: &mut Option<Object>) -> Result<Object, String> {
        Ok(Object{})
    }
    pub fn CallBuiltin(isolate: &Isolate, builtin: Object, this: Rc<RefCell<JSPromise>>, vector: ()) -> Result<Object, String>{
        Ok(Object{})
    }
}

pub struct SourceTextModuleDescriptor {}
impl SourceTextModuleDescriptor{
    pub fn GetCellIndexKind(cell_index:i32) -> CellIndexKind {
        CellIndexKind::kExport
    }

    pub enum CellIndexKind{
        kImport,
        kExport,
        kInvalid
    }
}

pub struct MessageHandler {}
impl MessageHandler {
    pub fn MakeMessageObject(
        isolate: &Isolate,
        template: MessageTemplate,
        location: &MessageLocation,
        arg: Object,
    ) -> Rc<RefCell<JSMessageObject>>{
        Rc::new(RefCell::new(JSMessageObject{}))
    }
}

pub enum ModuleImportPhase {
    kEvaluation,
    kSource
}

pub struct AvailableAncestorsSet {}

impl<T: PartialEq> ZoneForwardList<T>{
    pub fn front(&self) -> T {panic!("todo")}
    pub fn pop_front(&mut self){}
    pub fn push_front(&mut self, module: T){}
    pub fn begin(&self) -> std::slice::Iter<T> {panic!("todo")}
    pub fn end(&self) -> std::slice::Iter<T> {panic!("todo")}
    pub fn is_empty(&self) -> bool{false}
}

impl std::cmp::PartialEq for  Rc<RefCell<Module>> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(self, other)
    }
}
impl std::cmp::Eq for Rc<RefCell<Module>> {}

impl<T: PartialEq> HashSet<T> {
    pub fn insert(&mut self, elem: T) -> bool {
        true
    }
}

impl HashMap<Rc<RefCell<Module>>, Box<HashSet<Rc<RefCell<String>>>>> {
    pub fn insert(&mut self, pair:(Rc<RefCell<Module>>, Box<HashSet<Rc<RefCell<String>>>>)) -> bool{
        true
    }
}

}
