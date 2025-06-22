// Copyright 2013 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod assert_scope {
    use bitflags::bitflags;
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::sync::{Mutex, MutexGuard};

    // Forward declarations.
    pub struct Isolate {} // Placeholder
                         // #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]

    bitflags! {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub struct PerThreadAssertType: u32 {
            const ASSERT_TYPE_IS_VALID_MARKER = 0b00000001;
            const SAFEPOINTS_ASSERT = 0b00000010;
            const HEAP_ALLOCATION_ASSERT = 0b00000100;
            const HANDLE_ALLOCATION_ASSERT = 0b00001000;
            const HANDLE_DEREFERENCE_ASSERT = 0b00010000;
            const HANDLE_USAGE_ON_ALL_THREADS_ASSERT = 0b00100000;
            const CODE_DEPENDENCY_CHANGE_ASSERT = 0b01000000;
            const CODE_ALLOCATION_ASSERT = 0b10000000;
            const GC_MOLE = 0b00000000;
            const POSITION_INFO_SLOW_ASSERT = 0b00000000;
        }
    }

    pub type PerThreadAsserts = PerThreadAssertType;

    // Empty assert scope, used for debug-only scopes in release mode
    // This class is also templated so that it still has distinct instances for each
    // debug scope -- this is necessary for GCMole to be able to recognise
    // DisableGCMole scopes as distinct from other assert scopes.
    #[derive(Debug)]
    pub struct PerThreadAssertScopeEmpty<const ALLOW: bool, const TYPES: u32> {
        _phantom: PhantomData<[(); 0]>, // Zero-sized type to prevent construction
    }

    impl<const ALLOW: bool, const TYPES: u32> PerThreadAssertScopeEmpty<ALLOW, TYPES> {
        pub fn new() -> Self {
            PerThreadAssertScopeEmpty {
                _phantom: PhantomData,
            }
        }

        pub fn release(&mut self) {}
    }

    #[derive(Debug)]
    pub struct PerThreadAssertScope<const ALLOW: bool, const TYPES: u32> {
        old_data_: PerThreadAsserts,
        _phantom: PhantomData<[(); 0]>,
    }

    impl<const ALLOW: bool, const TYPES: u32> PerThreadAssertScope<ALLOW, TYPES> {
        pub fn new() -> Self {
            let old_data_ = PerThreadAsserts::empty(); // Replace with appropriate initial value if needed

            PerThreadAssertScope {
                old_data_,
                _phantom: PhantomData,
            }
        }

        pub fn is_allowed() -> bool {
            ALLOW
        }

        pub fn release(&mut self) {
            // Add release logic here
        }
    }

    // Implement drop to handle any cleanup logic.
    impl<const ALLOW: bool, const TYPES: u32> Drop for PerThreadAssertScope<ALLOW, TYPES> {
        fn drop(&mut self) {
            // Add drop logic here
        }
    }

    // Per-isolate assert scopes.
    macro_rules! per_isolate_assert_scope_declaration {
        ($scope_type:ident) => {
            pub struct $scope_type<'a> {
                isolate_: &'a Isolate,
                old_data_: bool,
            }

            impl<'a> $scope_type<'a> {
                pub fn new(isolate: &'a Isolate) -> Self {
                    Self {
                        isolate_: isolate,
                        old_data_: false,
                    }
                }

                pub fn is_allowed(isolate: &Isolate) -> bool {
                    // Add logic to determine if it's allowed based on the Isolate state.
                    // This is a placeholder implementation.
                    true
                }

                pub fn open(isolate: &Isolate, was_execution_allowed: &mut bool) {
                    // Add logic to open the scope.
                    // This is a placeholder implementation.
                    *was_execution_allowed = true;
                }

                pub fn close(isolate: &Isolate, was_execution_allowed: bool) {
                    // Add logic to close the scope.
                    // This is a placeholder implementation.
                }
            }

            impl<'a> Drop for $scope_type<'a> {
                fn drop(&mut self) {
                    // Add drop logic here
                }
            }
        };
    }

    macro_rules! per_isolate_assert_enable_scope {
        ($enable_type:ident, $_1:ident, $_2:ident, $_3:ident) => {
            per_isolate_assert_scope_declaration!($enable_type);
        };
    }

    macro_rules! per_isolate_assert_disable_scope {
        ($_1:ident, $disable_type:ident, $_2:ident, $_3:ident) => {
            per_isolate_assert_scope_declaration!($disable_type);
        };
    }

    macro_rules! per_isolate_dcheck_type {
        ($v:ident, $enable:expr) => {
            $v!(AllowJavascriptExecution, DisallowJavascriptExecution, javascript_execution_assert, $enable);
            $v!(AllowDeoptimization, DisallowDeoptimization, deoptimization_assert, $enable);
            $v!(AllowCompilation, DisallowCompilation, compilation_assert, $enable);
            $v!(AllowExceptions, DisallowExceptions, no_exception_assert, $enable);
        };
    }

    macro_rules! per_isolate_check_type {
        ($v:ident, $enable:expr) => {
            $v!(NoThrowOnJavascriptExecution, ThrowOnJavascriptExecution, javascript_execution_throws, $enable);
            $v!(NoDumpOnJavascriptExecution, DumpOnJavascriptExecution, javascript_execution_dump, $enable);
        };
    }

    per_isolate_dcheck_type!(per_isolate_assert_enable_scope, true);
    per_isolate_check_type!(per_isolate_assert_enable_scope, true);
    per_isolate_dcheck_type!(per_isolate_assert_disable_scope, false);
    per_isolate_check_type!(per_isolate_assert_disable_scope, false);

    #[cfg(debug_assertions)]
    macro_rules! per_isolate_dcheck_enable_scope {
        ($enable_type:ident, $disable_type:ident, $field:ident, $_:ident) => {
            pub struct $enable_type##DebugOnly<'a> {
                base: $enable_type<'a>,
            }

            impl<'a> $enable_type##DebugOnly<'a> {
                pub fn new(isolate: &'a Isolate) -> Self {
                    Self {
                        base: $enable_type::new(isolate),
                    }
                }
            }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! per_isolate_dcheck_enable_scope {
        ($enable_type:ident, $disable_type:ident, $field:ident, $_:ident) => {
            pub struct $enable_type##DebugOnly<'a> {
                _phantom: PhantomData<&'a Isolate>,
            }

            impl<'a> $enable_type##DebugOnly<'a> {
                pub fn new(_isolate: &'a Isolate) -> Self {
                    Self {
                        _phantom: PhantomData,
                    }
                }
            }
        };
    }

    #[cfg(debug_assertions)]
    macro_rules! per_isolate_dcheck_disable_scope {
        ($enable_type:ident, $disable_type:ident, $field:ident, $_:ident) => {
            pub struct $disable_type##DebugOnly<'a> {
                base: $disable_type<'a>,
            }

            impl<'a> $disable_type##DebugOnly<'a> {
                pub fn new(isolate: &'a Isolate) -> Self {
                    Self {
                        base: $disable_type::new(isolate),
                    }
                }
            }
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! per_isolate_dcheck_disable_scope {
        ($enable_type:ident, $disable_type:ident, $field:ident, $_:ident) => {
            pub struct $disable_type##DebugOnly<'a> {
                _phantom: PhantomData<&'a Isolate>,
            }

            impl<'a> $disable_type##DebugOnly<'a> {
                pub fn new(_isolate: &'a Isolate) -> Self {
                    Self {
                        _phantom: PhantomData,
                    }
                }
            }
        };
    }

    per_isolate_dcheck_type!(per_isolate_dcheck_enable_scope, true);
    per_isolate_dcheck_type!(per_isolate_dcheck_disable_scope, false);

    #[cfg(debug_assertions)]
    type PerThreadAssertScopeDebugOnly<const ALLOW: bool, const TYPES: u32> = PerThreadAssertScope<ALLOW, TYPES>;

    #[cfg(not(debug_assertions))]
    type PerThreadAssertScopeDebugOnly<const ALLOW: bool, const TYPES: u32> = PerThreadAssertScopeEmpty<ALLOW, TYPES>;

    // Per-thread assert scopes.

    // Scope to document where we do not expect handles to be created.
    pub type DisallowHandleAllocation = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::HANDLE_ALLOCATION_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowHandleAllocation.
    pub type AllowHandleAllocation = PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::HANDLE_ALLOCATION_ASSERT.bits() }>;

    // Scope to document where we do not expect safepoints to be entered.
    pub type DisallowSafepoints = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::SAFEPOINTS_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowSafepoints.
    pub type AllowSafepoints = PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::SAFEPOINTS_ASSERT.bits() }>;

    // Scope to document where we do not expect any allocation.
    pub type DisallowHeapAllocation = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::HEAP_ALLOCATION_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowHeapAllocation.
    pub type AllowHeapAllocation = PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::HEAP_ALLOCATION_ASSERT.bits() }>;

    // Like AllowHeapAllocation, but enabled in release builds.
    pub type AllowHeapAllocationInRelease = PerThreadAssertScope<true, { PerThreadAssertType::HEAP_ALLOCATION_ASSERT.bits() }>;

    // Scope to document where we do not expect any handle dereferences.
    pub type DisallowHandleDereference = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::HANDLE_DEREFERENCE_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowHandleDereference.
    pub type AllowHandleDereference = PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::HANDLE_DEREFERENCE_ASSERT.bits() }>;

    // Explicitly allow handle dereference and creation for all threads/isolates on
    // one particular thread.
    pub type AllowHandleUsageOnAllThreads =
        PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::HANDLE_USAGE_ON_ALL_THREADS_ASSERT.bits() }>;

    // Scope to document where we do not expect code dependencies to change.
    pub type DisallowCodeDependencyChange =
        PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::CODE_DEPENDENCY_CHANGE_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowCodeDependencyChange.
    pub type AllowCodeDependencyChange =
        PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::CODE_DEPENDENCY_CHANGE_ASSERT.bits() }>;

    // Scope to document where we do not expect code to be allocated.
    pub type DisallowCodeAllocation = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::CODE_ALLOCATION_ASSERT.bits() }>;

    // Scope to introduce an exception to DisallowCodeAllocation.
    pub type AllowCodeAllocation = PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::CODE_ALLOCATION_ASSERT.bits() }>;

    // Scope to document where we do not expect garbage collections. It differs from
    // DisallowHeapAllocation by also forbidding safepoints.
    pub type DisallowGarbageCollection = PerThreadAssertScopeDebugOnly<false, { (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits() }>;

    // Like DisallowGarbageCollection, but enabled in release builds.
    pub type DisallowGarbageCollectionInRelease =
        PerThreadAssertScope<false, { (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits() }>;

    // Scope to skip gc mole verification in places where we do tricky raw
    // work.
    pub type DisableGCMole = PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::GC_MOLE.bits() }>;

    // Scope to ensure slow path for obtaining position info is not called
    pub type DisallowPositionInfoSlow =
        PerThreadAssertScopeDebugOnly<false, { PerThreadAssertType::POSITION_INFO_SLOW_ASSERT.bits() }>;

    // Scope to add an exception to disallowing position info slow path
    pub type AllowPositionInfoSlow =
        PerThreadAssertScopeDebugOnly<true, { PerThreadAssertType::POSITION_INFO_SLOW_ASSERT.bits() }>;

    #[cfg(debug_assertions)]
    macro_rules! disallow_garbage_collection {
        ($name:ident) => {
            let $name = DisallowGarbageCollection::new();
        };
    }

    #[cfg(not(debug_assertions))]
    macro_rules! disallow_garbage_collection {
        ($name:ident) => {};
    }

    // Scope to introduce an exception to DisallowGarbageCollection.
    pub type AllowGarbageCollection = PerThreadAssertScopeDebugOnly<true, { (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits() }>;

    // Like AllowGarbageCollection, but enabled in release builds.
    pub type AllowGarbageCollectionInRelease =
        PerThreadAssertScope<true, { (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits() }>;

    // Scope to document where we do not expect any access to the heap.
    pub type DisallowHeapAccess = PerThreadAssertScopeDebugOnly<
        false,
        {
            (PerThreadAssertType::CODE_DEPENDENCY_CHANGE_ASSERT
                | PerThreadAssertType::HANDLE_DEREFERENCE_ASSERT
                | PerThreadAssertType::HANDLE_ALLOCATION_ASSERT
                | PerThreadAssertType::HEAP_ALLOCATION_ASSERT)
            .bits()
        },
    >;

    // Scope to introduce an exception to DisallowHeapAccess.
    pub type AllowHeapAccess = PerThreadAssertScopeDebugOnly<
        true,
        {
            (PerThreadAssertType::CODE_DEPENDENCY_CHANGE_ASSERT
                | PerThreadAssertType::HANDLE_DEREFERENCE_ASSERT
                | PerThreadAssertType::HANDLE_ALLOCATION_ASSERT
                | PerThreadAssertType::HEAP_ALLOCATION_ASSERT)
            .bits()
        },
    >;

    pub struct DisallowHeapAccessIf {
        maybe_disallow_: Option<DisallowHeapAccess>,
    }

    impl DisallowHeapAccessIf {
        pub fn new(condition: bool) -> Self {
            let maybe_disallow_ = if condition {
                Some(PerThreadAssertScopeDebugOnly::<false, {
                    (PerThreadAssertType::CODE_DEPENDENCY_CHANGE_ASSERT
                        | PerThreadAssertType::HANDLE_DEREFERENCE_ASSERT
                        | PerThreadAssertType::HANDLE_ALLOCATION_ASSERT
                        | PerThreadAssertType::HEAP_ALLOCATION_ASSERT)
                    .bits()
                }>::new())
            } else {
                None
            };
            DisallowHeapAccessIf { maybe_disallow_ }
        }
    }

    // Like MutexGuard but also asserts that no garbage collection happens while
    // we're holding the mutex.
    pub struct NoGarbageCollectionMutexGuard<'a> {
        guard_: MutexGuard<'a, ()>,
        mutex_: &'a Mutex<()>,
        no_gc_: Option<DisallowGarbageCollection>,
    }

    impl<'a> NoGarbageCollectionMutexGuard<'a> {
        pub fn new(mutex: &'a Mutex<()>) -> Self {
            let guard_ = mutex.lock().unwrap();
            let no_gc_ = Some(PerThreadAssertScopeDebugOnly::<false, {
                (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits()
            }>::new());
            NoGarbageCollectionMutexGuard {
                guard_,
                mutex_: mutex,
                no_gc_,
            }
        }

        pub fn unlock(&mut self) {
            self.mutex_.unlock();
            self.no_gc_.take();
        }

        pub fn lock(&mut self) {
            self.mutex_.lock().unwrap();
            self.no_gc_ = Some(PerThreadAssertScopeDebugOnly::<false, {
                (PerThreadAssertType::SAFEPOINTS_ASSERT | PerThreadAssertType::HEAP_ALLOCATION_ASSERT).bits()
            }>::new());
        }
    }

    impl<'a> Drop for NoGarbageCollectionMutexGuard<'a> {
        fn drop(&mut self) {
            // Release the mutex when dropped
            // This is important because the mutex needs to be unlocked even if the `Unlock` or `Lock` method hasn't called.
            // It should be locked and unlocked in pairs.
            // The guard_ is actually not doing anything here. We only keep this field to make sure the mutex is unlocked when the guard is dropped.
        }
    }

    trait Unlock {
        fn unlock(&mut self);
    }

    impl<'a> Unlock for MutexGuard<'a, ()> {
        fn unlock(&mut self) {}
    }
    // Explicit instantiation declarations are not needed in Rust.
}