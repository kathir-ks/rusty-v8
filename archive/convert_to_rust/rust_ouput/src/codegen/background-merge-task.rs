// Converted from V8 C++ source files:
// Header: background-merge-task.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod handles {
    pub struct MaybeHandle<T> {
        pub value: Option<T>,
    }

    impl<T> MaybeHandle<T> {
        pub fn new(value: Option<T>) -> Self {
            MaybeHandle { value }
        }

        pub fn empty() -> Self {
            MaybeHandle { value: None }
        }

        pub fn from_handle(handle: &T) -> Self {
            MaybeHandle { value: Some(handle.clone()) }
        }

        pub fn is_empty(&self) -> bool {
            self.value.is_none()
        }

        pub fn to_option(&self) -> Option<T> where T: Clone {
            self.value.clone()
        }
    }
}

use std::rc::Rc;
use std::cell::RefCell;
use std::vec::Vec;

pub struct Isolate {}
pub struct ScriptDetails {}
pub enum LanguageMode {}
pub struct Script {}
pub struct SharedFunctionInfo {}
pub struct String {}
pub struct LocalIsolate {}
pub struct DirectHandle<T> {
    pub value: Rc<RefCell<T>>,
}

impl<T> DirectHandle<T> {
    pub fn new(value: T) -> Self {
        DirectHandle {
            value: Rc::new(RefCell::new(value)),
        }
    }
}

#[derive(Debug)]
pub enum BackgroundMergeError {
    CacheLookupFailed,
    MergeFailed,
    ForegroundCompletionFailed,
}

pub struct BackgroundMergeTask {
    persistent_handles_: Option<Rc<RefCell<PersistentHandles>>>,
    cached_script_: handles::MaybeHandle<Script>,
    toplevel_sfi_from_cached_script_: handles::MaybeHandle<SharedFunctionInfo>,
    used_new_sfis_: Vec<Rc<RefCell<SharedFunctionInfo>>>,
    new_compiled_data_for_cached_sfis_: Vec<NewCompiledDataForCachedSfi>,
    state_: State,
}

struct NewCompiledDataForCachedSfi {
    cached_sfi: Rc<RefCell<SharedFunctionInfo>>,
    new_sfi: Rc<RefCell<SharedFunctionInfo>>,
}

enum State {
    kNotStarted,
    kPendingBackgroundWork,
    kPendingForegroundWork,
    kDone,
}

struct PersistentHandles {}

impl BackgroundMergeTask {
    pub fn new() -> Self {
        BackgroundMergeTask {
            persistent_handles_: None,
            cached_script_: handles::MaybeHandle::empty(),
            toplevel_sfi_from_cached_script_: handles::MaybeHandle::empty(),
            used_new_sfis_: Vec::new(),
            new_compiled_data_for_cached_sfis_: Vec::new(),
            state_: State::kNotStarted,
        }
    }

    pub fn SetUpOnMainThread(&mut self, isolate: &Isolate, source_text: &String,
                             script_details: &ScriptDetails,
                             language_mode: LanguageMode) {
        self.persistent_handles_ = Some(Rc::new(RefCell::new(PersistentHandles {})));
        self.state_ = State::kPendingBackgroundWork;
    }

    pub fn SetUpOnMainThread_cached(&mut self, isolate: &Isolate, cached_script: DirectHandle<Script>) {
        self.persistent_handles_ = Some(Rc::new(RefCell::new(PersistentHandles {})));
        self.cached_script_ = handles::MaybeHandle::from_handle(&*cached_script.value.borrow());
        self.state_ = State::kPendingBackgroundWork;
    }

    pub fn BeginMergeInBackground(&mut self, isolate: &LocalIsolate,
                                 new_script: DirectHandle<Script>) {
        self.toplevel_sfi_from_cached_script_ = handles::MaybeHandle::new(Some(SharedFunctionInfo {}));

        let new_sfi = Rc::new(RefCell::new(SharedFunctionInfo {}));
        self.used_new_sfis_.push(new_sfi.clone());

        let cached_sfi = Rc::new(RefCell::new(SharedFunctionInfo {}));
        self.new_compiled_data_for_cached_sfis_.push(NewCompiledDataForCachedSfi {
            cached_sfi: cached_sfi.clone(),
            new_sfi: new_sfi.clone(),
        });

        self.state_ = State::kPendingForegroundWork;
    }

    pub fn CompleteMergeInForeground(&mut self, isolate: &Isolate, new_script: DirectHandle<Script>) -> Rc<RefCell<SharedFunctionInfo>> {
        self.state_ = State::kDone;
        Rc::new(RefCell::new(SharedFunctionInfo {}))
    }

    pub fn HasPendingBackgroundWork(&self) -> bool {
        match self.state_ {
            State::kPendingBackgroundWork => true,
            _ => false,
        }
    }

    pub fn HasPendingForegroundWork(&self) -> bool {
        match self.state_ {
            State::kPendingForegroundWork => true,
            _ => false,
        }
    }

    pub fn ForceGCDuringNextMergeForTesting() {}
}
