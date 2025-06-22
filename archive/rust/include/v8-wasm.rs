// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::sync::Arc;

mod internal {
    pub mod wasm {
        pub struct NativeModule {}
        pub struct StreamingDecoder {}
    }
}

/// An owned byte buffer with associated size.
pub struct OwnedBuffer {
    pub buffer: Box<[u8]>,
    pub size: usize,
}

impl OwnedBuffer {
    pub fn new(buffer: Box<[u8]>, size: usize) -> Self {
        OwnedBuffer { buffer, size }
    }

    pub fn default() -> Self {
        OwnedBuffer {
            buffer: Box::new([]),
            size: 0,
        }
    }
}

/// Wrapper around a compiled WebAssembly module, which is potentially shared by
/// different WasmModuleObjects.
pub struct CompiledWasmModule {
    native_module: Arc<internal::wasm::NativeModule>,
    source_url: String,
}

impl CompiledWasmModule {
    /// Serialize the compiled module. The serialized data does not include the
    /// wire bytes.
    pub fn serialize(&self) -> OwnedBuffer {
        // Placeholder implementation.  Serialization details are V8-internal.
        OwnedBuffer::default()
    }

    /// Get the (wasm-encoded) wire bytes that were used to compile this module.
    pub fn get_wire_bytes_ref(&self) -> &[u8] {
        // Placeholder implementation.  Access to wire bytes is V8-internal.
        &[]
    }

    pub fn source_url(&self) -> &String {
        &self.source_url
    }
}

impl CompiledWasmModule {
    fn new(native_module: Arc<internal::wasm::NativeModule>, source_url: &str) -> Self {
        CompiledWasmModule {
            native_module,
            source_url: source_url.to_string(),
        }
    }
}

// An instance of WebAssembly.Memory.
pub struct WasmMemoryObject {}

impl WasmMemoryObject {
    /// Returns underlying ArrayBuffer.
    pub fn buffer(&self) -> ArrayBuffer {
        // Placeholder implementation.  Integration with V8's ArrayBuffer is required.
        ArrayBuffer {}
    }

    // V8_INLINE static WasmMemoryObject* Cast(Value* value)
    // CheckCast implementation is omitted due to missing Value type.
    // It involves runtime type checking and casting which would depend on
    // a concrete Value type implementation.
}

// An instance of WebAssembly.Module.
pub struct WasmModuleObject {
    compiled_module: CompiledWasmModule,
}

impl WasmModuleObject {
    /// Efficiently re-create a WasmModuleObject, without recompiling, from
    /// a CompiledWasmModule.
    pub fn from_compiled_module(
        compiled_module: &CompiledWasmModule,
    ) -> Result<WasmModuleObject, String> {
        // Placeholder implementation.  Needs Isolate implementation from v8.
        Ok(WasmModuleObject {
            compiled_module: CompiledWasmModule {
                native_module: Arc::new(internal::wasm::NativeModule{}),
                source_url: String::new()
            }
        })
    }

    /// Get the compiled module for this module object. The compiled module can be
    /// shared by several module objects.
    pub fn get_compiled_module(&self) -> &CompiledWasmModule {
        &self.compiled_module
    }

    /// Compile a Wasm module from the provided uncompiled bytes.
    pub fn compile(wire_bytes: &[u8]) -> Result<WasmModuleObject, String> {
        // Placeholder implementation.  Needs Isolate and compilation.
        Ok(WasmModuleObject {
            compiled_module: CompiledWasmModule {
                native_module: Arc::new(internal::wasm::NativeModule{}),
                source_url: String::new()
            }
        })
    }
}

// The V8 interface for WebAssembly streaming compilation. When streaming
// compilation is initiated, V8 passes a {WasmStreaming} object to the embedder
// such that the embedder can pass the input bytes for streaming compilation to
// V8.
pub struct WasmStreaming {
    impl_: Box<WasmStreamingImpl>,
}

pub struct WasmStreamingImpl {}

impl WasmStreaming {
    pub fn new(impl_: Box<WasmStreamingImpl>) -> Self {
        WasmStreaming { impl_ }
    }

    /// Pass a new chunk of bytes to WebAssembly streaming compilation.
    /// The buffer passed into {OnBytesReceived} is owned by the caller.
    pub fn on_bytes_received(&mut self, bytes: &[u8]) {
        // Placeholder implementation.
    }

    /// {Finish} should be called after all received bytes where passed to
    /// {OnBytesReceived} to tell V8 that there will be no more bytes. {Finish}
    /// must not be called after {Abort} has been called already.
    /// If {can_use_compiled_module} is true and {SetCompiledModuleBytes} was
    /// previously called, the compiled module bytes can be used.
    /// If {can_use_compiled_module} is false, the compiled module bytes previously
    /// set by {SetCompiledModuleBytes} should not be used.
    pub fn finish(&mut self, can_use_compiled_module: bool) {
        // Placeholder implementation.
    }

    /// Abort streaming compilation. If {exception} has a value, then the promise
    /// associated with streaming compilation is rejected with that value. If
    /// {exception} does not have value, the promise does not get rejected.
    /// {Abort} must not be called repeatedly, or after {Finish}.
    pub fn abort(&mut self, exception: Option<Value>) {
        // Placeholder implementation.
    }

    /// Passes previously compiled module bytes. This must be called before
    /// {OnBytesReceived}, {Finish}, or {Abort}. Returns true if the module bytes
    /// can be used, false otherwise. The buffer passed via {bytes} and {size}
    /// is owned by the caller. If {SetCompiledModuleBytes} returns true, the
    /// buffer must remain valid until either {Finish} or {Abort} completes.
    /// The compiled module bytes should not be used until {Finish(true)} is
    /// called, because they can be invalidated later by {Finish(false)}.
    pub fn set_compiled_module_bytes(&mut self, bytes: &[u8], size: usize) -> bool {
        // Placeholder implementation.
        true
    }

    /// Sets a callback which is called whenever a significant number of new
    /// functions are ready for serialization.
    pub fn set_more_functions_can_be_serialized_callback(
        &mut self,
        callback: Box<dyn Fn(CompiledWasmModule)>,
    ) {
        // Placeholder implementation.
    }

    /*
     * Sets the UTF-8 encoded source URL for the {Script} object. This must be
     * called before {Finish}.
     */
    pub fn set_url(&mut self, url: &str) {
        // Placeholder implementation.
    }
    // Unpack needs Isolate and Value implementation from v8.
}

// The V8 interface for a WebAssembly memory map descriptor. This is an
// experimental feature that may change and be removed without further
// communication.
pub struct WasmMemoryMapDescriptor {}

impl WasmMemoryMapDescriptor {
    //WasmFileDescriptor alias
    pub type WasmFileDescriptor = i32;

    pub fn new(_fd: WasmFileDescriptor) -> WasmMemoryMapDescriptor{
        //Needs isolate implementation.
        WasmMemoryMapDescriptor{}
    }
}

pub struct ArrayBuffer {}
pub struct Value {}