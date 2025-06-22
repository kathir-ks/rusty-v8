// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod contextual {
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};

    // {ContextualVariable} provides a clean alternative to a global variable.
    // The contextual variable is mutable, and supports managing the value of
    // a variable in a well-nested fashion via the {Scope} class.
    // {ContextualVariable} only stores a pointer to the current value, which
    // is stored in a {Scope} object. The most recent value can be retrieved
    // via Get(). Because only {Scope} has actual storage, there must be at
    // least one active {Scope} (i.e. in a surrounding C++ scope), whenever Get()
    // is called.
    // Note that contextual variables must only be used from the same thread,
    // i.e. {Scope} and Get() have to be in the same thread.
    pub struct ContextualVariable<Derived, VarType> {
        _derived: PhantomData<Derived>,
        _var_type: PhantomData<VarType>,
    }

    impl<Derived, VarType> ContextualVariable<Derived, VarType>
    where
        Derived: 'static,
        VarType: 'static,
    {
        thread_local! {
            static TOP: RefCell<Option<*mut Scope<Derived, VarType>>> = RefCell::new(None);
        }

        // A {Scope} contains a new object of type {VarType} and gives
        // ContextualVariable::Get() access to it. Upon destruction, the contextual
        // variable is restored to the state before the {Scope} was created. Scopes
        // have to follow a stack discipline:  A {Scope} has to be destructed before
        // any older scope is destructed.
        pub struct Scope<'a> {
            value: VarType,
            previous: Option<*mut Scope<'a, Derived, VarType>>,
            _phantom: PhantomData<&'a mut VarType>,
        }

        impl<'a, Derived, VarType> Scope<'a, Derived, VarType>
        where
            Derived: 'static,
            VarType: 'static,
        {
            pub fn new(value: VarType) -> Self {
                let previous = ContextualVariable::<Derived, VarType>::TOP.with(|top| {
                    top.borrow().map(|ptr| ptr)
                });
                ContextualVariable::<Derived, VarType>::TOP.with(|top| {
                    *top.borrow_mut() = Some(unsafe { std::mem::transmute( &value as *const VarType as *mut Scope<Derived, VarType> ) });
                });
                Scope {
                    value,
                    previous,
                    _phantom: PhantomData,
                }
            }
            pub fn value(&mut self) -> &mut VarType {
              &mut self.value
            }
        }

        impl<'a, Derived, VarType> Drop for Scope<'a, Derived, VarType>
        where
            Derived: 'static,
            VarType: 'static,
        {
            fn drop(&mut self) {
                ContextualVariable::<Derived, VarType>::TOP.with(|top| {
                    let mut borrowed_top = top.borrow_mut();
                    // Ensure stack discipline.
                    assert_eq!(
                        borrowed_top.map(|ptr| ptr as *const _),
                        Some(&self.value as *const VarType as *const _)
                    );
                    *borrowed_top = self.previous;
                });
            }
        }

        pub fn get() -> impl DerefMut<Target = VarType> + Deref<Target = VarType> {
            assert!(Self::has_scope());
            struct Guard<'a, Derived, VarType> {
                _phantom: PhantomData<&'a mut VarType>,
                data : *mut VarType,
                _d: PhantomData<&'a Derived>
            }
            impl<'a, Derived, VarType> Guard<'a, Derived, VarType> {
                unsafe fn new() -> Guard<'a, Derived, VarType>{
                    let data = Self::top().value as *mut VarType;
                    Guard{
                        _phantom: PhantomData,
                        data,
                        _d: PhantomData
                    }
                }
            }

            impl<'a, Derived, VarType> Deref for Guard<'a, Derived, VarType> {
                type Target = VarType;
                fn deref(&self) -> &Self::Target {
                    unsafe { &*self.data}
                }
            }
            impl<'a, Derived, VarType> DerefMut for Guard<'a, Derived, VarType> {
                fn deref_mut(&mut self) -> &mut Self::Target {
                    unsafe { &mut *self.data}
                }
            }
            impl<'a, Derived, VarType> Drop for Guard<'a, Derived, VarType> {
                fn drop(&mut self) {}
            }

            unsafe { Guard::new() }
        }

        fn top<'a>() -> &'a mut Scope<'a, Derived, VarType>{
            let ptr = ContextualVariable::<Derived, VarType>::TOP.with(|top| {
                *top.borrow_mut()
            });
            if let Some(value) = ptr{
                unsafe { &mut *value }
            } else {
                panic!("no scope found")
            }
        }

        pub fn has_scope() -> bool {
            ContextualVariable::<Derived, VarType>::TOP.with(|top| top.borrow().is_some())
        }
    }

    // Usage: DECLARE_CONTEXTUAL_VARIABLE(VarName, VarType)
    #[macro_export]
    macro_rules! declare_contextual_variable {
        ($VarName:ident, $VarType:ty) => {
            struct $VarName;
            impl $VarName {
                fn _new() -> Self {
                    Self {}
                }
            }
            impl ::v8::base::contextual::ContextualVariable<$VarName, $VarType> {

            }
        };
    }

    // Contextual variables that are accessed in tests need to be
    // exported. For this, place the following macro in the global namespace inside
    // of a .cc file.
    // This functionality is not directly translatable as it relies on exporting symbols
    // from a dynamic library, which is not a common pattern in Rust.
    // #[macro_export]
    // macro_rules! export_contextual_variable {
    //     ($VarName:ident) => {
    //         // Implementation would go here, but requires more context
    //         // and dynamic linking features.
    //     };
    // }

    // By inheriting from {ContextualClass} a class can become a contextual variable
    // of itself, which is very similar to a singleton.
    pub type ContextualClass<T> = ContextualVariable<T, T>;

    // {ContextualVariableWithDefault} is similar to a {ContextualVariable},
    // with the difference that a default value is used if there is no active
    // {Scope} object.
    pub struct ContextualVariableWithDefault<Derived, VarType, const DEFAULT: VarType> {
        base: ContextualVariable<Derived, VarType>,
        _derived: PhantomData<Derived>,
        _var_type: PhantomData<VarType>,
    }

    impl<Derived, VarType, const DEFAULT: VarType>
        ContextualVariableWithDefault<Derived, VarType, { DEFAULT }>
    where
        Derived: 'static,
        VarType: 'static + Copy,
    {
        thread_local! {
            static DEFAULT_VALUE: VarType = DEFAULT;
        }
        pub fn get() -> VarType {
            if ContextualVariable::<Derived, VarType>::has_scope() {
                *ContextualVariable::<Derived, VarType>::get()
            } else {
                Self::DEFAULT_VALUE.with(|&value| value)
            }
        }
    }

    // Usage: DECLARE_CONTEXTUAL_VARIABLE_WITH_DEFAULT(VarName, VarType, Args...)
    #[macro_export]
    macro_rules! declare_contextual_variable_with_default {
        ($VarName:ident, $VarType:ty, $Default:expr) => {
            struct $VarName;

            impl $VarName {
                fn _new() -> Self {
                    Self {}
                }
            }

            impl ::v8::base::contextual::ContextualVariableWithDefault<
                $VarName,
                $VarType,
                {$Default},
            > {
            }
        };
    }
}