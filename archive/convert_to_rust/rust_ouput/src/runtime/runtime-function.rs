// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-function.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use crate::v8::v8 as V8;
    use crate::v8::code as code;
    use crate::v8::v8 as v8;
    use crate::v8::HandleScope as HandleScope;
    use crate::v8::SealHandleScope as SealHandleScope;
    use crate::v8::BitField64 as BitField64;

    //use crate::builtins::accessors;
    //use crate::execution::isolate_inl;
    //use crate::heap::heap_inl;

    pub struct Isolate {}

    impl Isolate {
        pub fn heap(&self) -> Heap {
            Heap{}
        }
    }

    pub struct Heap {}

    impl Heap {
        pub fn ToBoolean(&self, value: bool) -> bool {
            value
        }
    }

    pub struct JSReceiver {}
    pub struct JSFunction {}

    impl JSFunction {
        pub fn shared(&self) -> SharedFunctionInfo {
            SharedFunctionInfo{}
        }
    }

    pub struct SharedFunctionInfo {}

    impl SharedFunctionInfo {
        pub fn script(&self) -> Object {
            Object{}
        }

        pub fn GetSourceCode(_isolate: &Isolate, _shared: &SharedFunctionInfo) -> Box<String> {
            Box::new("".to_string())
        }

        pub fn StartPosition(&self) -> i32 {
            0
        }

        pub fn IsApiFunction(&self) -> bool {
            false
        }
    }

    pub struct Script {}

    impl Script {
        pub fn source(&self) -> String {
            "".to_string()
        }
        pub fn id(&self) -> i32 {
            0
        }
    }

    pub struct Object {}

    pub struct ReadOnlyRoots {}

    impl ReadOnlyRoots {
        pub fn undefined_value(&self) -> Object {
            Object{}
        }
    }

    pub struct Smi {}

    impl Smi {
        pub fn FromInt(value: i32) -> i32 {
            value
        }
    }

    pub struct DirectHandle<T> {
        value: T,
    }

    impl<T> DirectHandle<T> {
        pub fn new(value: T) -> Self {
            DirectHandle { value }
        }

        pub fn get(&self) -> &T {
            &self.value
        }
    }

    pub struct DirectHandleVector<T> {
        data: Vec<T>,
    }

    impl<T> DirectHandleVector<T> {
        pub fn new(isolate: &Isolate, capacity: usize) -> Self {
            DirectHandleVector {
                data: Vec::with_capacity(capacity),
            }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn get(&self, index: usize) -> &T {
            &self.data[index]
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }
    }

    pub mod base {
        pub struct Vector<T> {
            data: Vec<T>,
        }

        impl<T> Vector<T> {
            pub fn of(data: Vec<T>) -> Self {
                Vector { data }
            }
        }

        pub struct VectorOf {}

        impl VectorOf {
            pub fn arguments<T>(arguments: &DirectHandleVector<T>) -> Vec<&T> {
                arguments.data.iter().collect()
            }
        }
    }

    pub struct Execution {}

    impl Execution {
        pub fn Call(
            _isolate: &Isolate,
            _target: &DirectHandle<Object>,
            _receiver: &DirectHandle<Object>,
            _arguments: Vec<&Object>,
        ) -> Result<Object, String> {
            Ok(Object{})
        }
    }

    pub type RuntimeArguments = Vec<Object>;

    macro_rules! RUNTIME_FUNCTION {
        ($name:ident) => {
            pub fn $name(isolate: &mut Isolate, args: &RuntimeArguments) -> Result<Object, String> {
                println!("called {}", stringify!($name));
                // Implementations go here
                Ok(Object{})
            }
        };
    }

    pub fn IsJSFunction(_obj: &JSReceiver) -> bool {
        true
    }

    pub fn IsScript(_obj: &Object) -> bool {
        true
    }

    pub fn Cast<T>(_obj: &JSReceiver) -> &JSFunction {
        //TODO: implement casting logic
        unsafe { std::mem::transmute(_obj) }
    }

    RUNTIME_FUNCTION!(Runtime_FunctionGetScriptSource);

    RUNTIME_FUNCTION!(Runtime_FunctionGetScriptId);

    RUNTIME_FUNCTION!(Runtime_FunctionGetSourceCode);

    RUNTIME_FUNCTION!(Runtime_FunctionGetScriptSourcePosition);

    RUNTIME_FUNCTION!(Runtime_FunctionIsAPIFunction);

    RUNTIME_FUNCTION!(Runtime_Call);
}
