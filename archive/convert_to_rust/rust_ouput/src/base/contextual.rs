// Converted from V8 C++ source files:
// Header: contextual.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::cell::RefCell;
    use std::marker::PhantomData;
    use std::ops::{Deref, DerefMut};
    use std::thread_local;

    pub struct V8_EXPORT_PRIVATE {}
    pub struct V8_NODISCARD {}

    pub trait ContextualVariableBase {
        type VarT;
        fn get_top<'a>() -> &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Self::VarT> + 'static>>>>;
    }

    pub trait ContextualScopeTrait {
        type VarT;
        fn value(&mut self) -> &mut Self::VarT;
    }

    pub struct ContextualScope<T> {
        value: T,
    }

    impl<T> ContextualScope<T> {
        pub fn new(value: T) -> Self {
            ContextualScope { value }
        }
    }

    impl<T> ContextualScopeTrait for ContextualScope<T> {
        type VarT = T;
        fn value(&mut self) -> &mut Self::VarT {
            &mut self.value
        }
    }

    pub struct ScopeGuard<'a, Derived: ContextualVariableBase> {
        previous: Option<Box<dyn ContextualScopeTrait<VarT = Derived::VarT> + 'static>>,
        top_key: &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Derived::VarT> + 'static>>>>,
        _phantom: PhantomData<Derived>,
    }

    impl<'a, Derived: ContextualVariableBase> ScopeGuard<'a, Derived> {
        pub fn new<F>(top_key: &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Derived::VarT> + 'static>>>>, constructor: F) -> Self
        where
            F: FnOnce() -> Box<dyn ContextualScopeTrait<VarT = Derived::VarT> + 'static>,
        {
            let mut top = top_key.borrow_mut();
            let previous = top.take();
            *top = Some(constructor());
            ScopeGuard {
                previous,
                top_key,
                _phantom: PhantomData,
            }
        }
    }

    impl<'a, Derived: ContextualVariableBase> Drop for ScopeGuard<'a, Derived> {
        fn drop(&mut self) {
            let mut top = self.top_key.borrow_mut();
            *top = self.previous.take();
        }
    }

    pub struct ContextualVariable<Derived, VarType> {
        _phantom: PhantomData<(Derived, VarType)>,
    }

    impl<Derived, VarType> ContextualVariable<Derived, VarType> {
        pub fn get<'a>(top_key: &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = VarType> + 'static>>>>> -> impl DerefMut<Target = VarType> + 'a
        where
            Derived: ContextualVariableBase<VarT = VarType>,
        {
            ContextualVariableRefMut {
                top_key,
                _phantom: PhantomData,
            }
        }

        pub fn has_scope(top_key: &thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = VarType> + 'static>>>>>) -> bool
        where
            Derived: ContextualVariableBase<VarT = VarType>,
        {
            top_key.borrow().is_some()
        }
    }

    pub struct ContextualVariableRefMut<'a, VarType> {
        top_key: &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = VarType> + 'static>>>>,
        _phantom: PhantomData<VarType>,
    }

    impl<'a, VarType> Deref for ContextualVariableRefMut<'a, VarType> {
        type Target = VarType;

        fn deref(&self) -> &Self::Target {
            let top = self.top_key.borrow();
            let scope = top.as_ref().expect("No active scope");
            let value = scope.value();
            unsafe { &*(value as *mut VarType) }
        }
    }

    impl<'a, VarType> DerefMut for ContextualVariableRefMut<'a, VarType> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            let mut top = self.top_key.borrow_mut();
            let scope = top.as_mut().expect("No active scope");
            scope.value()
        }
    }

    #[macro_export]
    macro_rules! declare_contextual_variable {
        ($VarName:ident, $($VarType:tt)*) => {
            pub struct $VarName {}
            impl ContextualVariableBase for $VarName {
                type VarT = $($VarType)*;
                fn get_top<'a>() -> &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Self::VarT> + 'static>>>{
                    thread_local! {
                        static TOP: RefCell<Option<Box<dyn ContextualScopeTrait<VarT = $($VarType)*> + 'static>>> = RefCell::new(None);
                    }
                    &TOP
                }
            }
        };
    }

    #[macro_export]
    macro_rules! export_contextual_variable {
        ($VarName:ident) => {
            impl $VarName {
                fn exported_top<'a>() -> &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = <$VarName as ContextualVariableBase>::VarT> + 'static>>>> {
                    <$VarName as ContextualVariableBase>::get_top()
                }
            }
        };
    }

    pub struct ContextualClass<T>(PhantomData<T>);

    impl<T> ContextualClass<T> {
        pub fn new() -> Self {
            ContextualClass(PhantomData)
        }
    }

    impl<T> ContextualVariableBase for T {
        type VarT = T;
        fn get_top<'a>() -> &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Self::VarT> + 'static>>>>{
            thread_local! {
                static TOP: RefCell<Option<Box<dyn ContextualScopeTrait<VarT = T> + 'static>>> = RefCell::new(None);
            }
            &TOP
        }
    }

    pub struct ContextualVariableWithDefault<Derived, VarType, const DEFAULT: VarType> {
        _phantom: PhantomData<(Derived, VarType)>,
    }

    impl<Derived, VarType, const DEFAULT: VarType> ContextualVariableWithDefault<Derived, VarType, const DEFAULT>
    where
        Derived: ContextualVariableBase<VarT = VarType>,
        VarType: Copy + 'static,
    {
        pub fn get<'a>() -> impl DerefMut<Target = VarType> + 'a {
            ContextualVariableWithDefaultRefMut::<Derived, VarType, DEFAULT> {
                _phantom: PhantomData,
            }
        }
    }

    struct ContextualVariableWithDefaultRefMut<'a, Derived, VarType, const DEFAULT: VarType>
    where
        Derived: ContextualVariableBase<VarT = VarType>,
        VarType: Copy + 'static,
    {
        _phantom: PhantomData<(&'a Derived, VarType)>,
    }

    impl<'a, Derived, VarType, const DEFAULT: VarType> Deref for ContextualVariableWithDefaultRefMut<'a, Derived, VarType, const DEFAULT>
    where
        Derived: ContextualVariableBase<VarT = VarType>,
        VarType: Copy + 'static,
    {
        type Target = VarType;

        fn deref(&self) -> &Self::Target {
            if <Derived as ContextualVariableBase>::get_top().with(|top| top.borrow().is_some()) {
                let top = <Derived as ContextualVariableBase>::get_top();
                let borrowed = top.borrow();
                let scope = borrowed.as_ref().unwrap();
                let value = scope.value();
                unsafe { &*(value as *mut VarType) }
            } else {
                &DEFAULT
            }
        }
    }

    impl<'a, Derived, VarType, const DEFAULT: VarType> DerefMut for ContextualVariableWithDefaultRefMut<'a, Derived, VarType, const DEFAULT>
    where
        Derived: ContextualVariableBase<VarT = VarType>,
        VarType: Copy + 'static,
    {
        fn deref_mut(&mut self) -> &mut Self::Target {
            if <Derived as ContextualVariableBase>::get_top().with(|top| top.borrow().is_some()) {
                let top = <Derived as ContextualVariableBase>::get_top();
                let mut borrowed = top.borrow_mut();
                let scope = borrowed.as_mut().unwrap();
                scope.value()
            } else {
                panic!("Cannot obtain mutable reference to default value.  A scope must be active.");
            }
        }
    }

    #[macro_export]
    macro_rules! declare_contextual_variable_with_default {
        ($VarName:ident, $($VarType:tt)*, $($Args:expr),*) => {
            pub struct $VarName {}
            impl ContextualVariableBase for $VarName {
                type VarT = $($VarType)*;
                fn get_top<'a>() -> &'a thread_local::LocalKey<RefCell<Option<Box<dyn ContextualScopeTrait<VarT = Self::VarT> + 'static>>>{
                    thread_local! {
                        static TOP: RefCell<Option<Box<dyn ContextualScopeTrait<VarT = $($VarType)*> + 'static>>> = RefCell::new(None);
                    }
                    &TOP
                }
            }
        };
    }
}
