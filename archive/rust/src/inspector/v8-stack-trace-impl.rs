// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod v8_stack_trace_impl {
    use std::rc::Weak;
    use std::sync::Arc;
    use std::string::FromUtf16Error;

    pub type String16 = String; // Assuming String16 is a UTF-16 string, represented by Rust's String
    pub type StringView = str; // Assuming StringView is a string slice, represented by Rust's str

    pub trait V8InspectorClient {
        fn base_url() -> String;
    }

    pub mod protocol {
        pub mod Runtime {
            #[derive(Debug, Clone)]
            pub struct CallFrame {
                pub function_name: String,
                pub script_id: String,
                pub url: String,
                pub line_number: i32,
                pub column_number: i32,
            }

            #[derive(Debug, Clone)]
            pub struct StackTrace {
                pub call_frames: Vec<CallFrame>,
                //Potentially include other fields in the future if necessary.
            }
        }
        pub mod API {
            #[derive(Debug, Clone)]
            pub struct StackTrace {
            }
        }
    }

    pub struct StackFrame {
        m_function_name: String16,
        m_script_id: i32,
        m_source_url: String16,
        m_line_number: i32,
        m_column_number: i32,
        m_has_source_url_comment: bool,
    }

    impl StackFrame {
        pub fn new(
            function_name: String16,
            script_id: i32,
            source_url: String16,
            line_number: i32,
            column_number: i32,
            has_source_url_comment: bool,
        ) -> Self {
            StackFrame {
                m_function_name: function_name,
                m_script_id: script_id,
                m_source_url: source_url,
                m_line_number: line_number,
                m_column_number: column_number,
                m_has_source_url_comment: has_source_url_comment,
            }
        }

        pub fn function_name(&self) -> &String16 {
            &self.m_function_name
        }
        pub fn script_id(&self) -> i32 {
            self.m_script_id
        }
        pub fn source_url(&self) -> &String16 {
            &self.m_source_url
        }
        pub fn line_number(&self) -> i32 {
            self.m_line_number
        }
        pub fn column_number(&self) -> i32 {
            self.m_column_number
        }

        pub fn build_inspector_object<T: V8InspectorClient>(
            &self,
            _client: &T,
        ) -> protocol::Runtime::CallFrame {
            protocol::Runtime::CallFrame {
                function_name: self.m_function_name.clone(),
                script_id: self.m_script_id.to_string(),
                url: self.m_source_url.clone(),
                line_number: self.m_line_number,
                column_number: self.m_column_number,
            }
        }

        pub fn is_equal(&self, frame: &StackFrame) -> bool {
            self.m_function_name == frame.m_function_name
                && self.m_script_id == frame.m_script_id
                && self.m_source_url == frame.m_source_url
                && self.m_line_number == frame.m_line_number
                && self.m_column_number == frame.m_column_number
                && self.m_has_source_url_comment == frame.m_has_source_url_comment
        }
    }

    pub trait V8StackTrace {
        fn clone(&self) -> Box<dyn V8StackTrace>;
        fn first_non_empty_source_url(&self) -> StringView;
        fn is_empty(&self) -> bool;
        fn top_source_url(&self) -> StringView;
        fn top_line_number(&self) -> i32;
        fn top_column_number(&self) -> i32;
        fn top_script_id(&self) -> i32;
        fn top_function_name(&self) -> StringView;
        fn build_inspector_object(&self, max_async_depth: i32) -> Option<protocol::Runtime::API::StackTrace>;
        fn to_string(&self) -> Option<StringBuffer>;
        fn frames(&self) -> Vec<V8StackFrame>;
    }

    pub type V8StackFrame = Arc<StackFrame>;

    pub struct V8StackTraceId {
    }

    pub struct V8StackTraceImpl {
        m_frames: Vec<Arc<StackFrame>>,
        m_max_async_depth: i32,
        m_async_parent: Weak<AsyncStackTrace>,
        m_external_parent: V8StackTraceId,
    }

    impl V8StackTraceImpl {
        pub const K_DEFAULT_MAX_CALL_STACK_SIZE_TO_CAPTURE: i32 = 200;

        //This function is not fully translatable as v8::Local<v8::StackTrace> is a V8 type that is unavailable
        //and V8Debugger is not defined.
        pub fn create(
            _debugger: &V8Debugger,
            _stack_trace: (), //v8::Local<v8::StackTrace>,
            _max_stack_size: i32,
        ) -> Self {
            todo!()
        }
        //This function is not fully translatable as V8Debugger is not defined.
        pub fn capture(
            _debugger: &V8Debugger,
            _max_stack_size: i32,
        ) -> Self {
            todo!()
        }

        pub fn new(
            frames: Vec<Arc<StackFrame>>,
            max_async_depth: i32,
            async_parent: Weak<AsyncStackTrace>,
            external_parent: V8StackTraceId,
        ) -> Self {
            V8StackTraceImpl {
                m_frames: frames,
                m_max_async_depth: max_async_depth,
                m_async_parent: async_parent,
                m_external_parent: external_parent,
            }
        }

        pub fn build_inspector_object_impl(
            &self,
            _debugger: &V8Debugger,
        ) -> Option<protocol::Runtime::StackTrace> {
            self.build_inspector_object_impl_with_depth(_debugger, i32::MAX)
        }

        pub fn build_inspector_object_impl_with_depth(
            &self,
            _debugger: &V8Debugger,
            _max_async_depth: i32,
        ) -> Option<protocol::Runtime::StackTrace> {
            let call_frames: Vec<_> = self.m_frames.iter().map(|frame|{
                protocol::Runtime::CallFrame {
                    function_name: frame.function_name().clone(),
                    script_id: frame.script_id().to_string(),
                    url: frame.source_url().clone(),
                    line_number: frame.line_number(),
                    column_number: frame.column_number()
                }
            }).collect();

            Some(protocol::Runtime::StackTrace{
                call_frames: call_frames
            })
        }

        pub fn is_equal_ignoring_top_frame(&self, stack_trace: &V8StackTraceImpl) -> bool {
            if self.m_frames.len() != stack_trace.m_frames.len() {
                return false;
            }

            for i in 1..self.m_frames.len() {
                if !Arc::ptr_eq(&self.m_frames[i], &stack_trace.m_frames[i]) {
                    return false;
                }
            }

            true
        }
    }

    impl V8StackTrace for V8StackTraceImpl {
        fn clone(&self) -> Box<dyn V8StackTrace> {
            Box::new(V8StackTraceImpl {
                m_frames: self.m_frames.clone(),
                m_max_async_depth: self.m_max_async_depth,
                m_async_parent: Weak::clone(&self.m_async_parent),
                m_external_parent: V8StackTraceId {},
            })
        }

        fn first_non_empty_source_url(&self) -> StringView {
            for frame in &self.m_frames {
                if !frame.source_url().is_empty() {
                    return &frame.source_url();
                }
            }
            ""
        }

        fn is_empty(&self) -> bool {
            self.m_frames.is_empty()
        }

        fn top_source_url(&self) -> StringView {
            if let Some(frame) = self.m_frames.first() {
                &frame.source_url()
            } else {
                ""
            }
        }

        fn top_line_number(&self) -> i32 {
            self.m_frames.first().map(|frame| frame.line_number() + 1).unwrap_or(0)
        }

        fn top_column_number(&self) -> i32 {
            self.m_frames.first().map(|frame| frame.column_number() + 1).unwrap_or(0)
        }

        fn top_script_id(&self) -> i32 {
            self.m_frames.first().map(|frame| frame.script_id()).unwrap_or(0)
        }

        fn top_function_name(&self) -> StringView {
            self.m_frames.first().map(|frame| frame.function_name().as_str()).unwrap_or("")
        }

        fn build_inspector_object(&self, max_async_depth: i32) -> Option<protocol::Runtime::API::StackTrace> {
            //protocol::Runtime::API::StackTrace {};
            todo!()
        }

        fn to_string(&self) -> Option<StringBuffer> {
            todo!()
        }

        fn frames(&self) -> Vec<V8StackFrame> {
            self.m_frames.clone()
        }
    }

    struct StackFrameIterator<'a> {
        m_current_it: std::slice::Iter<'a, Arc<StackFrame>>,
        m_current_end: std::slice::Iter<'a, Arc<StackFrame>>,
        m_parent: *mut AsyncStackTrace, // raw pointer because AsyncStackTrace is not fully defined and can't be borrowed safely
    }

    impl<'a> StackFrameIterator<'a> {
        fn new(stack_trace: &'a V8StackTraceImpl) -> Self {
            StackFrameIterator {
                m_current_it: stack_trace.m_frames.iter(),
                m_current_end: stack_trace.m_frames.iter(),
                m_parent: std::ptr::null_mut(), //Parent field cannot be initialized because AsyncStackTrace is not fully defined
            }
        }

        fn next(&mut self) {
            self.m_current_it.next();
        }

        fn frame(&mut self) -> Option<&StackFrame> {
            self.m_current_it.next().map(|arc| arc.as_ref())
        }

        fn done(&self) -> bool {
            self.m_current_it.len() == 0
        }
    }

    pub struct AsyncStackTrace {
        m_id: usize,
        m_description: String16,
        m_frames: Vec<Arc<StackFrame>>,
        m_async_parent: Weak<AsyncStackTrace>,
        m_external_parent: V8StackTraceId,
    }

    impl AsyncStackTrace {
        pub fn new(
            description: String16,
            frames: Vec<Arc<StackFrame>>,
            async_parent: Weak<AsyncStackTrace>,
            external_parent: V8StackTraceId,
        ) -> Self {
            AsyncStackTrace {
                m_id: 0, // Dummy value, needs proper ID generation.
                m_description: description,
                m_frames: frames,
                m_async_parent: async_parent,
                m_external_parent: external_parent,
            }
        }

        //This function is not fully translatable as V8Debugger is not defined.
        pub fn capture(
            _debugger: &V8Debugger,
            description: String16,
            skip_top_frame: bool,
        ) -> Arc<AsyncStackTrace> {
            // Capture the stack trace here and create the AsyncStackTrace object.
            // The original C++ code uses v8::StackTrace::CurrentStackTrace to capture the stack.
            // Because the V8 API is not available in Rust, this part is skipped
            let frames = Vec::new();
            let async_parent = Weak::new();
            let external_parent = V8StackTraceId {};
            Arc::new(AsyncStackTrace::new(description, frames, async_parent, external_parent))
        }

        //This function is not fully translatable as V8Debugger is not defined.
        pub fn store(
            _debugger: &V8Debugger,
            stack: Arc<AsyncStackTrace>
        ) -> usize {
            let stack_ptr = Arc::into_raw(stack);
            stack_ptr as usize
        }

        pub fn build_inspector_object(
            &self,
            _debugger: &V8Debugger,
            max_async_depth: i32,
        ) -> Option<protocol::Runtime::StackTrace> {
            // Implement the logic to build the inspector object based on the stack frames.
            // This is similar to V8StackTraceImpl::buildInspectorObjectImpl.
            let call_frames: Vec<_> = self.m_frames.iter().map(|frame|{
                protocol::Runtime::CallFrame {
                    function_name: frame.function_name().clone(),
                    script_id: frame.script_id().to_string(),
                    url: frame.source_url().clone(),
                    line_number: frame.line_number(),
                    column_number: frame.column_number()
                }
            }).collect();

            Some(protocol::Runtime::StackTrace{
                call_frames: call_frames
            })
        }

        pub fn description(&self) -> &String16 {
            &self.m_description
        }

        pub fn parent(&self) -> Weak<AsyncStackTrace> {
            Weak::clone(&self.m_async_parent)
        }

        pub fn is_empty(&self) -> bool {
            self.m_frames.is_empty()
        }

        pub fn external_parent(&self) -> &V8StackTraceId {
            &self.m_external_parent
        }

        pub fn frames(&self) -> &Vec<Arc<StackFrame>> {
            &self.m_frames
        }
    }

    // Dummy StringBuffer
    pub struct StringBuffer {}
    // Dummy V8Debugger
    pub struct V8Debugger {}
}