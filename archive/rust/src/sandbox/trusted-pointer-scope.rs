#![allow(dead_code)]
#![allow(unused_variables)]

mod sandbox {
    pub mod trusted_pointer_scope {
        use std::cell::RefCell;
        use std::rc::Rc;

        const V8_ENABLE_SANDBOX: bool = true;

        #[derive(PartialEq, Eq)]
        pub enum State {
            kInitial,
            kSuccess,
            kFailure,
        }

        #[derive(PartialEq, Eq)]
        pub enum Storage {
            kEmpty,
            kSingleton,
            kVector,
        }

        pub struct TrustedPointerTableEntry {}

        impl TrustedPointerTableEntry {
            fn overwrite_tag(&mut self, tag: i32) {}
        }
        const kUnpublishedIndirectPointerTag: i32 = 0; // Placeholder value

        pub struct Isolate {
            trusted_pointer_publishing_scope: RefCell<Option<Rc<TrustedPointerPublishingScope>>>,
        }

        impl Isolate {
            pub fn new() -> Self {
                Isolate {
                    trusted_pointer_publishing_scope: RefCell::new(None),
                }
            }
            pub fn trusted_pointer_publishing_scope(
                &self,
            ) -> Option<Rc<TrustedPointerPublishingScope>> {
                self.trusted_pointer_publishing_scope.borrow().clone()
            }

            pub fn set_trusted_pointer_publishing_scope(
                &self,
                scope: Option<Rc<TrustedPointerPublishingScope>>,
            ) {
                *self.trusted_pointer_publishing_scope.borrow_mut() = scope;
            }
        }

        pub struct DisallowJavascriptExecution {}

        pub struct TrustedPointerPublishingScope {
            isolate_: *mut Isolate,
            state_: RefCell<State>,
            storage_: RefCell<Storage>,
            singleton_: RefCell<Option<Box<TrustedPointerTableEntry>>>,
            vector_: RefCell<Option<Vec<Box<TrustedPointerTableEntry>>>>,
        }

        impl TrustedPointerPublishingScope {
            pub fn new(isolate: *mut Isolate, no_js: &DisallowJavascriptExecution) -> Rc<Self> {
                // Nesting TrustedPointerPublishingScopes is not supported for now.
                unsafe {
                    assert!((*isolate).trusted_pointer_publishing_scope().is_none());
                }

                let rc = Rc::new(TrustedPointerPublishingScope {
                    isolate_: isolate,
                    state_: RefCell::new(State::kInitial),
                    storage_: RefCell::new(Storage::kEmpty),
                    singleton_: RefCell::new(None),
                    vector_: RefCell::new(None),
                });
                unsafe {
                    (*isolate).set_trusted_pointer_publishing_scope(Some(rc.clone()));
                }
                rc
            }

            pub fn mark_success(&self) {
                *self.state_.borrow_mut() = State::kSuccess;
            }

            pub fn mark_failure(&self) {
                *self.state_.borrow_mut() = State::kFailure;
            }

            pub fn track_pointer(&self, entry: Box<TrustedPointerTableEntry>) {
                let mut storage = self.storage_.borrow_mut();
                match *storage {
                    Storage::kEmpty => {
                        *self.singleton_.borrow_mut() = Some(entry);
                        *storage = Storage::kSingleton;
                    }
                    Storage::kSingleton => {
                        let previous = self.singleton_.borrow_mut().take().unwrap();
                        let mut vector = Vec::new();
                        vector.reserve(4);
                        vector.push(previous);
                        vector.push(entry);
                        *self.vector_.borrow_mut() = Some(vector);
                        *storage = Storage::kVector;
                    }
                    Storage::kVector => {
                        self.vector_.borrow_mut().as_mut().unwrap().push(entry);
                    }
                }
            }
        }

        impl Drop for TrustedPointerPublishingScope {
            fn drop(&mut self) {
                let state = *self.state_.borrow();
                if state == State::kFailure {
                    let storage = *self.storage_.borrow();
                    match storage {
                        Storage::kSingleton => {
                            if let Some(mut singleton) = self.singleton_.borrow_mut().take() {
                                singleton.overwrite_tag(kUnpublishedIndirectPointerTag);
                            }
                        }
                        Storage::kVector => {
                            if let Some(vector) = self.vector_.borrow_mut().take() {
                                for mut entry in vector {
                                    entry.overwrite_tag(kUnpublishedIndirectPointerTag);
                                }
                            }
                        }
                        Storage::kEmpty => {}
                    }
                } else {
                    // If this assert fails, you probably forgot to call {MarkSuccess()}.
                    assert_eq!(state, State::kSuccess);
                }

                unsafe {
                    assert_eq!(
                        Rc::as_ptr(
                            (*self.isolate_)
                                .trusted_pointer_publishing_scope()
                                .as_ref()
                                .unwrap()
                        ),
                        self
                    );
                    (*self.isolate_).set_trusted_pointer_publishing_scope(None);
                }
            }
        }

        pub struct DisableTrustedPointerPublishingScope {
            isolate_: *mut Isolate,
            saved_: Option<Rc<TrustedPointerPublishingScope>>,
        }

        impl DisableTrustedPointerPublishingScope {
            pub fn new(isolate: *mut Isolate) -> Self {
                unsafe {
                    let saved = (*isolate).trusted_pointer_publishing_scope();
                    if saved.is_some() {
                        (*isolate).set_trusted_pointer_publishing_scope(None);
                    }
                    DisableTrustedPointerPublishingScope {
                        isolate_: isolate,
                        saved_: saved,
                    }
                }
            }
        }

        impl Drop for DisableTrustedPointerPublishingScope {
            fn drop(&mut self) {
                unsafe {
                    if self.saved_.is_some() {
                        (*self.isolate_).set_trusted_pointer_publishing_scope(self.saved_.clone());
                    }
                }
            }
        }
    }
}