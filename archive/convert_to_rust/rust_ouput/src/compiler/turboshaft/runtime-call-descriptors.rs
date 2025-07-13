// Converted from V8 C++ source files:
// Header: runtime-call-descriptors.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod runtime_call_descriptors {
    use std::any::Any;
    use std::rc::Rc;

    use crate::compiler::turboshaft::operations::TSCallDescriptor;
    use crate::compiler::turboshaft::utils::V8;
    use crate::execution::isolate::JSAny;
    use crate::execution::messages::Handle;
    use crate::execution::stack_guard::StackGuard;
    use crate::include::v8_maybe::Maybe;
    use crate::execution::arguments::Object;

    pub struct RuntimeCallDescriptor {}

    impl RuntimeCallDescriptor {
        pub struct Abort {}
        impl Abort {
            pub const FUNCTION: i32 = 1;
            pub type Arguments = (V<Smi>,);
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;

        }

        pub struct BigIntUnaryOp {}
        impl BigIntUnaryOp {
            pub const FUNCTION: i32 = 2;
            pub type Arguments = (V<BigInt>, V<Smi>);
            pub type ResultType = V<BigInt>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct DateCurrentTime {}
        impl DateCurrentTime {
            pub const FUNCTION: i32 = 3;
            pub type Arguments = ();
            pub type ResultType = V<Number>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct DebugPrint {}
        impl DebugPrint {
            pub const FUNCTION: i32 = 4;
            pub type Arguments = (V<Object>,);
            pub type ResultType = Void;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct StackGuard {}
        impl StackGuard {
            pub const FUNCTION: i32 = 5;
            pub type Arguments = ();
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct StackGuardWithGap {}
        impl StackGuardWithGap {
            pub const FUNCTION: i32 = 6;
            pub type Arguments = (V<Smi>,);
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct HandleNoHeapWritesInterrupts {}
        impl HandleNoHeapWritesInterrupts {
            pub const FUNCTION: i32 = 7;
            pub type Arguments = ();
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 4;
        }

        pub struct PropagateException {}
        impl PropagateException {
            pub const FUNCTION: i32 = 8;
            pub type Arguments = ();
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ReThrow {}
        impl ReThrow {
            pub const FUNCTION: i32 = 9;
            pub type Arguments = (V<Object>,);
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 2;
        }

        pub struct StringCharCodeAt {}
        impl StringCharCodeAt {
            pub const FUNCTION: i32 = 10;
            pub type Arguments = (V<String>, V<Number>);
            pub type ResultType = V<Smi>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        #[cfg(feature = "V8_INTL_SUPPORT")]
        pub struct StringToUpperCaseIntl {}
        #[cfg(feature = "V8_INTL_SUPPORT")]
        impl StringToUpperCaseIntl {
            pub const FUNCTION: i32 = 11;
            pub type Arguments = (V<String>,);
            pub type ResultType = V<String>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct SymbolDescriptiveString {}
        impl SymbolDescriptiveString {
            pub const FUNCTION: i32 = 12;
            pub type Arguments = (V<Symbol>,);
            pub type ResultType = V<String>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 1;
        }

        pub struct TerminateExecution {}
        impl TerminateExecution {
            pub const FUNCTION: i32 = 13;
            pub type Arguments = ();
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 1;
        }

        pub struct TransitionElementsKind {}
        impl TransitionElementsKind {
            pub const FUNCTION: i32 = 14;
            pub type Arguments = (V<HeapObject>, V<Map>);
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct TryMigrateInstance {}
        impl TryMigrateInstance {
            pub const FUNCTION: i32 = 15;
            pub type Arguments = (V<HeapObject>,);
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct TryMigrateInstanceAndMarkMapAsMigrationTarget {}
        impl TryMigrateInstanceAndMarkMapAsMigrationTarget {
            pub const FUNCTION: i32 = 16;
            pub type Arguments = (V<HeapObject>,);
            pub type ResultType = V<Object>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 1 | 2;
        }

        pub struct ThrowAccessedUninitializedVariable {}
        impl ThrowAccessedUninitializedVariable {
            pub const FUNCTION: i32 = 17;
            pub type Arguments = (V<Object>,);
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowConstructorReturnedNonObject {}
        impl ThrowConstructorReturnedNonObject {
            pub const FUNCTION: i32 = 18;
            pub type Arguments = ();
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowNotSuperConstructor {}
        impl ThrowNotSuperConstructor {
            pub const FUNCTION: i32 = 19;
            pub type Arguments = (V<Object>, V<Object>);
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowSuperAlreadyCalledError {}
        impl ThrowSuperAlreadyCalledError {
            pub const FUNCTION: i32 = 20;
            pub type Arguments = ();
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowSuperNotCalled {}
        impl ThrowSuperNotCalled {
            pub const FUNCTION: i32 = 21;
            pub type Arguments = ();
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowCalledNonCallable {}
        impl ThrowCalledNonCallable {
            pub const FUNCTION: i32 = 22;
            pub type Arguments = (V<Object>,);
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct ThrowInvalidStringLength {}
        impl ThrowInvalidStringLength {
            pub const FUNCTION: i32 = 23;
            pub type Arguments = ();
            pub type ResultType = Never;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }

        pub struct NewClosure {}
        impl NewClosure {
            pub const FUNCTION: i32 = 24;
            pub type Arguments = (V<SharedFunctionInfo>, V<FeedbackCell>);
            pub type ResultType = V<JSFunction>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 2;
        }

        pub struct NewClosure_Tenured {}
        impl NewClosure_Tenured {
            pub const FUNCTION: i32 = 25;
            pub type Arguments = (V<SharedFunctionInfo>, V<FeedbackCell>);
            pub type ResultType = V<JSFunction>;
            pub const NEEDS_FRAME_STATE: bool = false;
            pub const PROPERTIES: i32 = 2;
        }

        pub struct HasInPrototypeChain {}
        impl HasInPrototypeChain {
            pub const FUNCTION: i32 = 26;
            pub type Arguments = (V<Object>, V<HeapObject>);
            pub type ResultType = V<Boolean>;
            pub const NEEDS_FRAME_STATE: bool = true;
            pub const PROPERTIES: i32 = 0;
        }
    }

    pub struct V<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    pub struct Void {}
    pub struct Never {}
    pub struct Smi {}
    pub struct BigInt {}
    pub struct Number {}
    pub struct Object {}
    pub struct String {}
    #[cfg(feature = "V8_INTL_SUPPORT")]
    pub struct Symbol {}
    pub struct HeapObject {}
    pub struct Map {}
    pub struct Boolean {}
    pub struct SharedFunctionInfo {}
    pub struct FeedbackCell {}
    pub struct JSFunction {}

}
