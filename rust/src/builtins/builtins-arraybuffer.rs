// TODO: Add equivalent for #include "src/builtins/builtins-utils-inl.h"
// TODO: Add equivalent for #include "src/builtins/builtins.h"
// TODO: Add equivalent for #include "src/execution/protectors-inl.h"
// TODO: Add equivalent for #include "src/execution/protectors.h"
// TODO: Add equivalent for #include "src/handles/maybe-handles-inl.h"
// TODO: Add equivalent for #include "src/heap/heap-inl.h"
// TODO: Add equivalent for #include "src/logging/counters.h"
// TODO: Add equivalent for #include "src/numbers/conversions.h"
// TODO: Add equivalent for #include "src/objects/js-array-buffer-inl.h"
// TODO: Add equivalent for #include "src/objects/objects-inl.h"

//use std::alloc::Allocator;
use std::cmp;
use std::mem;
use std::sync::atomic::{AtomicU8, Ordering};

macro_rules! check_shared {
    ($expected:expr, $name:expr, $method:expr, $isolate:expr) => {
        if $name.is_shared() != $expected {
            return Err($isolate.new_type_error(
                MessageTemplate::kIncompatibleMethodReceiver,
                $isolate.factory().new_string_from_ascii_checked($method),
                $name.into(),
            ));
        }
    };
}

macro_rules! check_resizable {
    ($expected:expr, $name:expr, $method:expr, $isolate:expr) => {
        if $name.is_resizable_by_js() != $expected {
            return Err($isolate.new_type_error(
                MessageTemplate::kIncompatibleMethodReceiver,
                $isolate.factory().new_string_from_ascii_checked($method),
                $name.into(),
            ));
        }
    };
}

mod internal {
    use super::*;
    use std::ptr::NonNull;

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum SharedFlag {
        kShared,
        kNotShared,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum ResizableFlag {
        kNotResizable,
        kResizable,
    }

    #[derive(PartialEq, Eq, Copy, Clone)]
    pub enum InitializedFlag {
        kZeroInitialized,
        kUninitialized,
    }

    #[derive(Debug)]
    pub enum MessageTemplate {
        kInvalidArrayBufferLength,
        kInvalidArrayBufferMaxLength,
        kArrayBufferAllocationFailed,
        kConstructorNotFunction,
        kIncompatibleMethodReceiver,
        kDetachedOperation,
        kArrayBufferSpeciesThis,
        kSharedArrayBufferSpeciesThis,
        kArrayBufferTooShort,
        kSharedArrayBufferTooShort,
        kInvalidArrayBufferResizeLength,
        kOutOfMemory,
        kDataCloneErrorNonDetachableArrayBuffer,
    }

    pub struct Isolate {
        // TODO: Define the fields of Isolate
        array_buffer_allocator: Box<ArrayBufferAllocator>,
    }

    impl Isolate {
        pub fn new(array_buffer_allocator: ArrayBufferAllocator) -> Self {
            Isolate {
                array_buffer_allocator: Box::new(array_buffer_allocator),
            }
        }
        pub fn array_buffer_allocator(&self) -> &ArrayBufferAllocator {
            &self.array_buffer_allocator
        }

        pub fn new_type_error(&self, _template: MessageTemplate, _arg1: String, _arg2: String) -> Error {
            // Implement type error creation logic
            Error::TypeError(String::from("Type Error"))
        }
        pub fn factory(&self) -> Factory {
            Factory {}
        }
        pub fn count_usage(&self, _feature: UseCounterFeature) {
             // Noop.
        }
        pub fn shared_array_buffer_fun(&self) -> JSFunction {
            JSFunction {}
        }

        pub fn array_buffer_fun(&self) -> JSFunction {
            JSFunction {}
        }

        pub fn heap(&self) -> Heap {
            Heap {}
        }
    }

    pub struct Heap {}

    impl Heap {
        pub fn resize_array_buffer_extension(&self, _extension: Extension, _delta: i64) {
            // Noop.
        }
    }

    pub struct Extension {}

    pub struct Factory {}

    impl Factory {
        pub fn new_string_from_ascii_checked(&self, str: &'static str) -> String {
            str.to_string()
        }

        pub fn new_number(&self, value: f64) -> Object {
            Object::Number(value)
        }
        pub fn max_byte_length_string(&self) -> String {
            String::from("maxByteLength")
        }

        pub fn new_js_array_buffer(&self, backing_store: Box<BackingStore>) -> JSArrayBuffer {
            JSArrayBuffer {
                backing_store: Some(backing_store),
                shared: false,
                resizable: ResizableFlag::kNotResizable,
                byte_length: 0,
                max_byte_length: 0,
                detachable: true,
            }
        }

        pub fn new_js_array_buffer_and_backing_store(
            &self,
            byte_length: usize,
            max_byte_length: usize,
            initialized: InitializedFlag,
            resizable: ResizableFlag,
        ) -> Result<JSArrayBuffer, Error> {
            let mut backing_store = BackingStore::allocate_and_zero(byte_length);

            match initialized {
                InitializedFlag::kZeroInitialized => {
                    unsafe {
                        std::ptr::write_bytes(backing_store.data.as_mut_ptr(), 0, byte_length);
                    }
                }
                InitializedFlag::kUninitialized => {
                    // Do nothing, leave the memory uninitialized.
                }
            }

            Ok(JSArrayBuffer {
                backing_store: Some(Box::new(backing_store)),
                shared: false,
                resizable,
                byte_length,
                max_byte_length,
                detachable: true,
            })
        }

        pub fn new_number_from_size(&self, size: usize) -> Object {
            Object::Number(size as f64)
        }
    }

    pub enum UseCounterFeature {
        kResizableArrayBuffer,
        kGrowableSharedArrayBuffer,
    }

    pub enum Object {
        Number(f64),
        JSArrayBuffer(JSArrayBuffer),
        String(String),
    }

    impl Object {
        pub fn number_value(&self) -> f64 {
            match self {
                Object::Number(value) => *value,
                _ => panic!("Not a number"),
            }
        }

        pub fn to_integer(_isolate: &Isolate, object: &Object) -> Result<Object, Error> {
            match object {
                Object::Number(num) => Ok(Object::Number(num.trunc())), // Truncate towards zero.
                _ => Err(Error::TypeError("Cannot convert non-number to integer".to_string()))
            }
        }

        pub fn same_value(obj1: &Object, obj2: &Object) -> bool {
            match (obj1, obj2) {
                (Object::Number(n1), Object::Number(n2)) => {
                    if n1.is_nan() && n2.is_nan() {
                        true // NaN === NaN in SameValue algorithm.
                    } else {
                        n1 == n2
                    }
                }
                (Object::String(s1), Object::String(s2)) => s1 == s2,
                _ => false,
            }
        }

         //TODO: Implement SpeciesConstructor
        pub fn species_constructor(_isolate: &Isolate, _receiver: &JSReceiver, constructor: &JSFunction) -> Result<Object, Error> {
            // For now, just return the constructor itself
            Ok(Object::JSArrayBuffer(JSArrayBuffer {}))
        }

        pub fn to_index(_isolate: &Isolate, _obj: &Object) -> Result<usize, Error> {
            // Implement ToIndex conversion logic here
            unimplemented!()
        }
    }

    pub struct JSReceiver {}

    pub struct JSFunction {}

    #[derive(Debug)]
    pub enum Error {
        TypeError(String),
        RangeError(String),
    }

    pub struct ArrayBufferAllocator {
        max_allocation_size: usize,
    }

    impl ArrayBufferAllocator {
        pub fn new(max_allocation_size: usize) -> Self {
            ArrayBufferAllocator {
                max_allocation_size,
            }
        }

        pub fn max_allocation_size(&self) -> usize {
            self.max_allocation_size
        }
    }

    pub struct BackingStore {
        data: Vec<u8>,
        max_size: usize,
        shared: bool,
        is_wasm_memory: bool,
        resizable_by_js: bool,
    }

    impl BackingStore {
        pub fn allocate_and_zero(byte_length: usize) -> BackingStore {
            BackingStore {
                data: vec![0u8; byte_length],
                max_size: byte_length,
                shared: false,
                is_wasm_memory: false,
                resizable_by_js: false,
            }
        }
        // TODO: Implement BackingStore::Allocate
        pub fn allocate(
            isolate: &Isolate,
            byte_length: usize,
            shared: SharedFlag,
            initialized: InitializedFlag,
        ) -> Option<Box<BackingStore>> {
            if byte_length > isolate.array_buffer_allocator().max_allocation_size() {
                return None;
            }

            let mut data = vec![0u8; byte_length];

            match initialized {
                InitializedFlag::kZeroInitialized => {
                    // Initialize the buffer with zeros
                    unsafe {
                        std::ptr::write_bytes(data.as_mut_ptr(), 0, byte_length);
                    }
                }
                InitializedFlag::kUninitialized => {
                    // Do nothing, leave the memory uninitialized.
                }
            }

            Some(Box::new(BackingStore {
                data,
                max_size: byte_length,
                shared: shared == SharedFlag::kShared,
                is_wasm_memory: false,
                resizable_by_js: false,
            }))
        }

        // TODO: Implement BackingStore::TryAllocateAndPartiallyCommitMemory
        pub fn try_allocate_and_partially_commit_memory(
            isolate: &Isolate,
            byte_length: usize,
            max_byte_length: usize,
            page_size: usize,
            initial_pages: usize,
            max_pages: usize,
            wasm_memory_flag: WasmMemoryFlag,
            shared: SharedFlag,
        ) -> Option<Box<BackingStore>> {
            if max_byte_length > isolate.array_buffer_allocator().max_allocation_size() {
                return None;
            }

            if byte_length > max_byte_length {
                return None;
            }

            let mut data = vec![0u8; byte_length];
            unsafe {
                std::ptr::write_bytes(data.as_mut_ptr(), 0, byte_length);
            }

            Some(Box::new(BackingStore {
                data,
                max_size: max_byte_length,
                shared: shared == SharedFlag::kShared,
                is_wasm_memory: wasm_memory_flag == WasmMemoryFlag::kWasm,
                resizable_by_js: true,
            }))
        }

        pub fn resize_in_place(&mut self, _isolate: &Isolate, new_byte_length: usize) -> ResizeOrGrowResult {
            if new_byte_length > self.max_size {
                return ResizeOrGrowResult::kFailure;
            }

            self.data.resize(new_byte_length, 0);
            self.data.shrink_to_fit(); // Free up unused capacity.
            ResizeOrGrowResult::kSuccess
        }

        pub fn grow_in_place(&mut self, _isolate: &Isolate, new_byte_length: usize) -> ResizeOrGrowResult {
            if new_byte_length > self.max_size {
                return ResizeOrGrowResult::kFailure;
            }

             if new_byte_length < self.data.len() {
                return ResizeOrGrowResult::kRace;
            }

            self.data.resize(new_byte_length, 0);
            self.data.shrink_to_fit(); // Free up unused capacity.
            ResizeOrGrowResult::kSuccess
        }

        pub fn is_wasm_memory(&self) -> bool {
            self.is_wasm_memory
        }

        pub fn max_byte_length(&self) -> usize {
            self.max_size
        }

        pub fn is_resizable_by_js(&self) -> bool {
            self.resizable_by_js
        }
    }

    pub enum ResizeOrGrowResult {
        kSuccess,
        kFailure,
        kRace,
    }

    pub enum WasmMemoryFlag {
        kWasm,
        kNotWasm,
    }

    pub struct JSArrayBuffer {
        backing_store: Option<Box<BackingStore>>,
        shared: bool,
        resizable: ResizableFlag,
        byte_length: usize,
        max_byte_length: usize,
        detachable: bool,
    }

    impl JSArrayBuffer {
        pub const K_MAX_BYTE_LENGTH: usize = usize::MAX >> 1;
        pub fn setup(&mut self, shared: SharedFlag, resizable: ResizableFlag, backing_store: Option<Box<BackingStore>>, _isolate: &Isolate) {
            self.shared = shared == SharedFlag::kShared;
            self.resizable = resizable;
            self.backing_store = backing_store;
        }

        pub fn is_shared(&self) -> bool {
            self.shared
        }

        pub fn is_resizable_by_js(&self) -> bool {
            self.resizable == ResizableFlag::kResizable
        }

        pub fn was_detached(&self) -> bool {
            self.backing_store.is_none()
        }

        pub fn get_byte_length(&self) -> usize {
            match &self.backing_store {
                Some(store) => store.data.len(),
                None => 0,
            }
        }

        pub fn backing_store(&self) -> *mut u8 {
            match &self.backing_store {
                Some(store) => store.data.as_ptr() as *mut u8,
                None => std::ptr::null_mut(),
            }
        }

        pub fn get_backing_store(&self) -> Option<&BackingStore> {
            self.backing_store.as_deref()
        }

        pub fn max_byte_length(&self) -> usize {
            self.max_byte_length
        }

        pub fn is_detachable(&self) -> bool {
            self.detachable
        }

        pub fn detach(buffer: &mut JSArrayBuffer) -> Result<(), Error> {
            buffer.backing_store = None;
            buffer.byte_length = 0;
            Ok(())
        }

        // TODO: Implement GetResizableBackingStorePageConfigurationImpl
        pub fn get_resizable_backing_store_page_configuration_impl(
            isolate: &Isolate,
            byte_length: usize,
            max_byte_length: usize,
            page_size: &mut usize,
            initial_pages: &mut usize,
            max_pages: &mut usize,
        ) -> Option<MessageTemplate> {
            // This is a placeholder implementation.
            // In a real implementation, you would calculate the page configuration
            // based on the given byte_length, max_byte_length, and system page size.
            // The implementation should also handle potential overflow and range errors.

            *page_size = 4096; // Example page size
            *initial_pages = (byte_length + *page_size - 1) / *page_size; // Round up to nearest page
            *max_pages = (max_byte_length + *page_size - 1) / *page_size; // Round up to nearest page

            if *initial_pages > *max_pages {
                return Some(MessageTemplate::kInvalidArrayBufferMaxLength); // Example error condition
            }

            None // No error
        }

        pub fn extension(&self) -> Extension {
            Extension {}
        }

        pub fn byte_length(&self) -> usize {
            self.byte_length
        }

        pub fn set_byte_length(&mut self, new_byte_length: usize) {
            self.byte_length = new_byte_length;
        }

    }

    // Tries to allocate a BackingStore given the input configuration. Either
    // returns the BackingStore or a message template that should be thrown as
    // RangeError.
    fn try_allocate_backing_store(
        isolate: &Isolate,
        shared: SharedFlag,
        resizable: ResizableFlag,
        length: &Object,
        max_length: &Object,
        initialized: InitializedFlag,
    ) -> Result<(Option<Box<BackingStore>>, Option<MessageTemplate>), Error> {
        //DisallowJavascriptExecution no_js(isolate);

        let mut byte_length: usize = 0;
        let mut max_byte_length: usize = 0;
        let mut backing_store: Option<Box<BackingStore>> = None;

        let max_allocatable = isolate.array_buffer_allocator().max_allocation_size();
        //DCHECK(max_allocatable <= JSArrayBuffer::kMaxByteLength);
        //static_assert(JSArrayBuffer::kMaxByteLength == JSTypedArray::kMaxByteLength);

        match Object::to_integer(isolate, length) {
            Ok(len_obj) => {
                if Object::number_value(&len_obj) < 0.0 {
                    return Ok((None, Some(MessageTemplate::kInvalidArrayBufferLength)));
                }

                if let Ok(size) = Object::number_value(&len_obj) as usize {
                     byte_length = size;
                } else {
                    return Ok((None, Some(MessageTemplate::kInvalidArrayBufferLength)));
                }
                if byte_length > max_allocatable {
                    return Ok((None, Some(MessageTemplate::kInvalidArrayBufferLength)));
                }
            }
            Err(_) => return Ok((None, Some(MessageTemplate::kInvalidArrayBufferLength))),
        }

        match resizable {
            ResizableFlag::kNotResizable => {
                if let Some(bs) = BackingStore::allocate(isolate, byte_length, shared, initialized) {
                    backing_store = Some(bs);
                }
            }
            ResizableFlag::kResizable => {
                match Object::to_integer(isolate, max_length) {
                    Ok(max_len_obj) => {
                        if Object::number_value(&max_len_obj) < 0.0 {
                            return Ok((None, Some(MessageTemplate::kInvalidArrayBufferMaxLength)));
                        }

                        if let Ok(size) = Object::number_value(&max_len_obj) as usize {
                            max_byte_length = size;
                         } else {
                            return Ok((None, Some(MessageTemplate::kInvalidArrayBufferMaxLength)));
                         }

                        if max_byte_length > max_allocatable {
                            return Ok((None, Some(MessageTemplate::kInvalidArrayBufferMaxLength)));
                        }

                        if byte_length > max_byte_length {
                            return Ok((None, Some(MessageTemplate::kInvalidArrayBufferMaxLength)));
                        }

                        let mut page_size: usize = 0;
                        let mut initial_pages: usize = 0;
                        let mut max_pages: usize = 0;
                        let maybe_range_error =
                            JSArrayBuffer::get_resizable_backing_store_page_configuration_impl(
                                isolate,
                                byte_length,
                                max_byte_length,
                                &mut page_size,
                                &mut initial_pages,
                                &mut max_pages,
                            );
                        if maybe_range_error.is_some() {
                            return Ok((None, maybe_range_error));
                        }
                        if let Some(bs) = BackingStore::try_allocate_and_partially_commit_memory(
                            isolate,
                            byte_length,
                            max_byte_length,
                            page_size,
                            initial_pages,
                            max_pages,
                            WasmMemoryFlag::kNotWasm,
                            shared,
                        ) {
                            backing_store = Some(bs);
                         }
                    }
                    Err(_) => return Ok((None, Some(MessageTemplate::kInvalidArrayBufferMaxLength))),
                }
            }
        }

        // Range errors bailed out earlier; only the failing allocation needs to be
        // caught here.
        if backing_store.is_none() {
            return Ok((None, Some(MessageTemplate::kArrayBufferAllocationFailed)));
        }
        Ok((backing_store, None))
    }

    fn construct_buffer(
        isolate: &Isolate,
        target: &JSFunction,
        new_target: &JSReceiver,
        length: &Object,
        max_length: &Object,
        initialized: InitializedFlag,
    ) -> Result<JSArrayBuffer, Error> {
        // We first try to convert the sizes and collect any possible range errors. If
        // no errors are observable we create the BackingStore before the
        // JSArrayBuffer to avoid a complex dance during setup. We then always create
        // the AB before throwing a possible error as the creation is observable.
        let shared = if std::ptr::eq(target, &isolate.array_buffer_fun()) {
            SharedFlag::kNotShared
        } else {
            SharedFlag::kShared
        };
        let resizable = if matches!(max_length, Object::Number(_)) {
            ResizableFlag::kResizable
        } else {
            ResizableFlag::kNotResizable
        };

        // BackingStore allocation may GC which is not observable itself.
        let (backing_store, range_error) =
            try_allocate_backing_store(isolate, shared, resizable, length, max_length, initialized)?;

        let result = JSArrayBuffer {}; // TODO: replace with JSObject::New
        let mut array_buffer = JSArrayBuffer {
            backing_store: None,
            shared: false,
            resizable: ResizableFlag::kNotResizable,
            byte_length: 0,
            max_byte_length: 0,
            detachable: false,
        };

        let backing_store_creation_failed = backing_store.is_none();
        array_buffer.setup(shared, resizable, backing_store, isolate);

        if backing_store_creation_failed {
            if let Some(err) = range_error {
                return Err(Error::RangeError("Failed to construct buffer".to_string())); // TODO: Translate MessageTemplate to string
            }
        }
        Ok(array_buffer)
    }

    // ES #sec-arraybuffer-constructor
    pub fn array_buffer_constructor(args: BuiltinArguments, isolate: &Isolate) -> Result<Object, Error> {
        // HandleScope scope(isolate);
        let target = args.target;

        //DCHECK(*target == target->native_context()->array_buffer_fun() ||
        //       *target == target->native_context()->shared_array_buffer_fun());
        if args.new_target.is_none() {
            // [[Call]]
            return Err(Error::TypeError(
                "Constructor ArrayBuffer requires 'new'".to_string(),
            ));
        }

        // [[Construct]]
        let new_target = JSReceiver {}; //Cast<JSReceiver>(args.new_target());
        let length = args.at_or_undefined(1);
        let number_length = Object::to_integer(isolate, &length)?;

        if Object::number_value(&number_length) < 0.0 {
            return Err(Error::RangeError("Invalid array buffer length".to_string()));
        }

        let options = args.at_or_undefined(2);
        let max_length = js_object_read_from_options_bag(&options, &isolate.factory().max_byte_length_string(), isolate);

        let number_max_length;

        if let Some(max_length_val) = &max_length {
             if std::ptr::eq(&args.target, &isolate.array_buffer_fun()) {
                isolate.count_usage(UseCounterFeature::kResizableArrayBuffer);
            } else {
                isolate.count_usage(UseCounterFeature::kGrowableSharedArrayBuffer);
            }

            number_max_length = Object::to_integer(isolate, max_length_val)?;
        } else {
            number_max_length = Object::Number(0.0); // Arbitrary default if max_length is undefined
        }

        let array_buffer = construct_buffer(
            isolate,
            &args.target,
            &new_target,
            &number_length,
            &number_max_length,
            InitializedFlag::kZeroInitialized,
        )?;

        Ok(Object::JSArrayBuffer(array_buffer))
    }

    // This is a helper to construct an ArrayBuffer with uinitialized memory.
    // This means the caller must ensure the buffer is totally initialized in
    // all cases, or we will expose uinitialized memory to user code.
    pub fn array_buffer_constructor_do_not_initialize(args: BuiltinArguments, isolate: &Isolate) -> Result<Object, Error> {
         //HandleScope scope(isolate);
        let target = isolate.array_buffer_fun();
        let length = args.at_or_undefined(1);

        let array_buffer = construct_buffer(
            isolate,
            &target,
            &target,
            &length,
            &Object::Number(0.0), // Empty object for max_length
            InitializedFlag::kUninitialized,
        )?;

        Ok(Object::JSArrayBuffer(array_buffer))
    }

    fn slice_helper(
        args: BuiltinArguments,
        isolate: &Isolate,
        k_method_name: &'static str,
        is_shared: bool,
    ) -> Result<Object, Error> {
        //HandleScope scope(isolate);
        let start = &args.at(1);
        let end = &args.at_or_undefined(2);

        // * If Type(O) is not Object, throw a TypeError exception.
        // * If O does not have an [[ArrayBufferData]] internal slot, throw a
        //   TypeError exception.
        // CHECK_RECEIVER(JSArrayBuffer, array_buffer, kMethodName);
        let array_buffer = match &args.receiver {
            Object::JSArrayBuffer(ab) => ab,
            _ => {
                return Err(Error::TypeError(
                    "Receiver is not a JSArrayBuffer".to_string(),
                ))
            }
        };

        // * [AB] If IsSharedArrayBuffer(O) is true, throw a TypeError exception.
        // * [SAB] If IsSharedArrayBuffer(O) is false, throw a TypeError exception.
        check_shared!(is_shared, array_buffer, k_method_name, isolate);

        // * [AB] If IsDetachedBuffer(buffer) is true, throw a TypeError exception.
        if !is_shared && array_buffer.was_detached() {
            return Err(Error::TypeError(
                "Cannot perform slice on detached buffer".to_string(),
            ));
        }

        // * [AB] Let len be O.[[ArrayBufferByteLength]].
        // * [SAB] Let len be O.[[ArrayBufferByteLength]].
        let len = array_buffer.get_byte_length() as f64;

        // * Let relativeStart be ? ToInteger(start).
        let relative_start = match Object::to_integer(isolate, start) {
            Ok(obj) => Object::number_value(&obj),
            Err(e) => return Err(e),
        };

        // * If relativeStart < 0, let first be max((len + relativeStart), 0); else
        //   let first be min(relativeStart, len).
        let first = if relative_start < 0.0 {
            cmp::max(len + relative_start, 0.0)
        } else {
            cmp::min(relative_start, len)
        };

        // * If end is undefined, let relativeEnd be len; else let relativeEnd be ?
        //   ToInteger(end).
        let relative_end = if matches!(end, Object::Number(_)) {
            len
        } else {
            match Object::to_integer(isolate, end) {
                Ok(obj) => Object::number_value(&obj),
                Err(e) => return Err(e),
            }
        };

        // * If relativeEnd < 0, let final be max((len + relativeEnd), 0); else let
        //   final be min(relativeEnd, len).
        let final_ = if relative_end < 0.0 {
            cmp::max(len + relative_end, 0.0)
        } else {
            cmp::min(relative_end, len)
        };

        // * Let newLen be max(final-first, 0).
        let new_len = cmp::max(final_ - first, 0.0);
        let new_len_obj = Object::Number(new_len);

        // * [AB] Let ctor be ? SpeciesConstructor(O, %ArrayBuffer%).
        // * [SAB] Let ctor be ? SpeciesConstructor(O, %SharedArrayBuffer%).
        let constructor_fun = if is_shared {
            &isolate.shared_array_buffer_fun()
        } else {
            &isolate.array_buffer_fun()
        };

        let ctor = Object::species_constructor(isolate, &JSReceiver {}, constructor_fun)?;

        // * Let new be ? Construct(ctor, newLen).
        let new_ = match ctor {
            Object::JSArrayBuffer(_ab) => {
                 // TODO: Implement Construct
                Ok(Object::JSArrayBuffer(JSArrayBuffer {}))
            }
            _ => {
                 return Err(Error::TypeError(
                    "Species constructor returned non-object".to_string(),
                ));
            }
        }?;

        // * If new does not have an [[ArrayBufferData]] internal slot, throw a
        //   TypeError exception.
        let new_array_buffer = match &new_ {
            Object::JSArrayBuffer(ab) => ab,
            _ => {
                return Err(Error::TypeError(
                    "Species constructor returned non-ArrayBuffer".to_string(),
                ))
            }
        };

        // * [AB] If IsSharedArrayBuffer(new) is true, throw a TypeError exception.
        // * [SAB] If IsSharedArrayBuffer(new) is false, throw a TypeError exception.
        check_shared!(is_shared, new_array_buffer, k_method_name, isolate);

        // The created ArrayBuffer might or might not be resizable, since the species
        // constructor might return a non-resizable or a resizable buffer.

        // * [AB] If IsDetachedBuffer(new) is true, throw a TypeError exception.
        if !is_shared && new_array_buffer.was_detached() {
            return Err(Error::TypeError(
                "Species constructor returned detached ArrayBuffer".to_string(),
            ));
        }

        // * [AB] If SameValue(new, O) is true, throw a TypeError exception.
        if !is_shared && Object::same_value(&new_, &args.receiver) {
            return Err(Error::TypeError(
                "ArrayBuffer.prototype.slice cannot be called on the same ArrayBuffer".to_string(),
            ));
        }

        // * [SAB] If new.[[ArrayBufferData]] and O.[[ArrayBufferData]] are the same
        //         Shared Data Block values, throw a TypeError exception.
        // TODO: Implement check for same backing store
        if is_shared {
            // TODO: Reimplement same-backing-store check
            //if new_array_buffer.backing_store() == array_buffer.backing_store() {
            //    return Err(Error::TypeError("SharedArrayBuffer.prototype.slice cannot be called on the same SharedArrayBuffer".to_string()));
            //}
        }

        // * If new.[[ArrayBufferByteLength]] < newLen, throw a TypeError exception.
        let new_array_buffer_byte_length = new_array_buffer.get_byte_length() as f64;
        if new_array_buffer_byte_length < new_len {
            return Err(Error::TypeError(
                "ArrayBuffer.prototype.slice created too-short ArrayBuffer".to_string(),
            ));
        }

        // * [AB] NOTE: Side-effects of the above steps may have detached O.
        // * [AB] If IsDetachedBuffer(O) is true, throw a TypeError exception.
        if !is_shared && array_buffer.was_detached() {
            return Err(Error::TypeError(
                "Original ArrayBuffer detached during species constructor call".to_string(),
            ));
        }

        // * Let fromBuf be O.[[ArrayBufferData]].
        // * Let toBuf be new.[[ArrayBufferData]].
        // * Perform CopyDataBlockBytes(toBuf, 0, fromBuf, first, newLen).
        let first_size = first as usize;
        let new_len_size = new_len as usize;

        if new_len_size != 0 {
            let mut from_data = array_buffer.backing_store() as *mut u8;
            let mut to_data = new_array_buffer.backing_store() as *mut u8;
             if from_data.is_null() {
                return Err(Error::TypeError("from_