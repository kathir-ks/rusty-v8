// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/inspector/v8-console-agent-impl.h (Converted to Rust module definition)
pub mod v8_console_agent_impl {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::inspector::protocol::dictionary_value::DictionaryValue;
    use crate::inspector::protocol::frontend_channel::FrontendChannel;
    use crate::inspector::protocol::response::Response;
    use crate::inspector::v8_console_message::V8ConsoleMessage;
    use crate::inspector::v8_console_message::V8MessageOrigin;
    use crate::inspector::v8_console_message_storage::V8ConsoleMessageStorage;
    use crate::inspector::v8_inspector_impl::V8InspectorImpl;
    use crate::inspector::v8_inspector_session_impl::V8InspectorSessionImpl;

    pub struct V8ConsoleAgentImpl {
        session: *mut V8InspectorSessionImpl, // Raw pointer, needs careful handling. Consider alternatives like Rc<RefCell<>> if ownership/mutation is needed.
        state: Rc<RefCell<DictionaryValue>>, // Assuming DictionaryValue needs interior mutability
        frontend: *mut dyn FrontendChannel, // Raw pointer, needs careful handling.  Using a trait object.
        enabled: bool,
    }

    mod console_agent_state {
        pub const console_enabled: &str = "consoleEnabled";
    }

    impl V8ConsoleAgentImpl {
        pub fn new(
            session: *mut V8InspectorSessionImpl,
            frontend_channel: *mut dyn FrontendChannel,
            state: Rc<RefCell<DictionaryValue>>,
        ) -> Self {
            V8ConsoleAgentImpl {
                session,
                state,
                frontend: frontend_channel,
                enabled: false,
            }
        }

        pub fn enable(&mut self) -> Response {
            if self.enabled {
                return Response::Success();
            }
            self.state.borrow_mut().set_boolean(console_agent_state::console_enabled.to_string(), true);
            self.enabled = true;
            self.report_all_messages();
            Response::Success()
        }

        pub fn disable(&mut self) -> Response {
            if !self.enabled {
                return Response::Success();
            }
            self.state.borrow_mut().set_boolean(console_agent_state::console_enabled.to_string(), false);
            self.enabled = false;
            Response::Success()
        }

        pub fn clear_messages(&self) -> Response {
            Response::Success()
        }

        pub fn restore(&mut self) {
            if !self.state.borrow().boolean_property(console_agent_state::console_enabled.to_string(), false) {
                return;
            }
            self.enable();
        }

        pub fn message_added(&mut self, message: &V8ConsoleMessage) {
            if self.enabled {
                self.report_message(message, true);
            }
        }

        pub fn enabled(&self) -> bool {
            self.enabled
        }

        fn report_all_messages(&mut self) {
            // Assuming `inspector()` returns a raw pointer.  Needs careful handling.
            let session = unsafe { &mut *self.session };
            let inspector = session.inspector();
            let storage = inspector.ensure_console_message_storage(session.context_group_id());

            for message in storage.messages() {
                if message.origin() == V8MessageOrigin::kConsole {
                    if !self.report_message(message, false) {
                        return;
                    }
                }
            }
        }

        fn report_message(&mut self, message: &V8ConsoleMessage, generate_preview: bool) -> bool {
            assert_eq!(message.origin(), V8MessageOrigin::kConsole);
            let frontend = unsafe { &mut *self.frontend };
            message.report_to_frontend(frontend);
            frontend.flush();

            let session = unsafe { &mut *self.session };
            let inspector = session.inspector();
            inspector.has_console_message_storage(session.context_group_id())
        }
    }

    impl Drop for V8ConsoleAgentImpl {
        fn drop(&mut self) {
            // Handle any necessary cleanup here, especially for raw pointers.
            // For example, if `frontend` needs to be dropped in a specific way,
            // that should be done here.
            // SAFETY: Ensure that dropping the raw pointers is safe and doesn't lead to double frees or use-after-free.
        }
    }
}

// src/inspector/protocol/Protocol.h
pub mod protocol {
    pub mod response {
        #[derive(Debug, PartialEq)]
        pub enum Response {
            Success(),
            Error(String),
        }
    }

    pub mod dictionary_value {
        use std::collections::HashMap;
        use std::cell::RefCell;

        #[derive(Debug, Default, Clone)]
        pub struct DictionaryValue {
            data: RefCell<HashMap<String, Value>>,
        }

        #[derive(Debug, Clone)]
        pub enum Value {
            Boolean(bool),
            String(String),
            Number(f64),
            Null,
        }

        impl DictionaryValue {
            pub fn new() -> Self {
                DictionaryValue {
                    data: RefCell::new(HashMap::new()),
                }
            }

            pub fn set_boolean(&mut self, key: String, value: bool) {
                self.data.borrow_mut().insert(key, Value::Boolean(value));
            }

            pub fn set_string(&mut self, key: String, value: String) {
                self.data.borrow_mut().insert(key, Value::String(value));
            }

            pub fn boolean_property(&self, key: String, default_value: bool) -> bool {
                self.data.borrow().get(&key).map_or(default_value, |value| {
                    match value {
                        Value::Boolean(b) => *b,
                        _ => default_value,
                    }
                })
            }
        }
    }

    pub mod frontend_channel {
        pub trait FrontendChannel {
            fn send_protocol_notification(&mut self, message: String);
            fn flush(&mut self);
        }
    }
}

// src/inspector/v8-console-message.h
pub mod v8_console_message {
    use crate::inspector::protocol::frontend_channel::FrontendChannel;

    #[derive(Debug, PartialEq, Eq)]
    pub enum V8MessageOrigin {
        Console,
    }

    pub struct V8ConsoleMessage {
        origin: V8MessageOrigin,
    }

    impl V8ConsoleMessage {
        pub fn new(origin: V8MessageOrigin) -> Self {
            V8ConsoleMessage { origin }
        }

        pub fn origin(&self) -> V8MessageOrigin {
            self.origin
        }

        pub fn report_to_frontend(&self, frontend: &mut dyn FrontendChannel) {
            frontend.send_protocol_notification("Console message reported".to_string());
        }
    }
}

// src/inspector/v8-inspector-impl.h
pub mod v8_inspector_impl {
    use std::collections::HashMap;
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::inspector::v8_console_message_storage::V8ConsoleMessageStorage;

    pub struct V8InspectorImpl {
        console_message_storage: RefCell<HashMap<i32, Rc<V8ConsoleMessageStorage>>>,
    }

    impl V8InspectorImpl {
        pub fn new() -> Self {
            V8InspectorImpl {
                console_message_storage: RefCell::new(HashMap::new()),
            }
        }

        pub fn ensure_console_message_storage(&self, context_group_id: i32) -> Rc<V8ConsoleMessageStorage> {
            let mut storage = self.console_message_storage.borrow_mut();
            storage.entry(context_group_id).or_insert_with(|| Rc::new(V8ConsoleMessageStorage::new())).clone()
        }

        pub fn has_console_message_storage(&self, context_group_id: i32) -> bool {
            self.console_message_storage.borrow().contains_key(&context_group_id)
        }
    }
}

// src/inspector/v8-inspector-session-impl.h
pub mod v8_inspector_session_impl {
    use crate::inspector::v8_inspector_impl::V8InspectorImpl;

    pub struct V8InspectorSessionImpl {
        inspector: *mut V8InspectorImpl, //Raw pointer needs careful handling
        context_group_id: i32,
    }

    impl V8InspectorSessionImpl {
        pub fn new(inspector: *mut V8InspectorImpl, context_group_id: i32) -> Self {
            V8InspectorSessionImpl { inspector, context_group_id }
        }

        pub fn inspector(&mut self) -> &mut V8InspectorImpl {
            unsafe { &mut *self.inspector }
        }

        pub fn context_group_id(&self) -> i32 {
            self.context_group_id
        }
    }
}

// src/inspector/v8-stack-trace-impl.h
pub mod v8_stack_trace_impl {
    // Placeholder, as the original file is empty.
    pub struct V8StackTraceImpl {}
}

// src/inspector/v8-console-message-storage.h
pub mod v8_console_message_storage {
    use std::cell::RefCell;
    use std::rc::Rc;

    use crate::inspector::v8_console_message::V8ConsoleMessage;

    pub struct V8ConsoleMessageStorage {
        messages: RefCell<Vec<Rc<V8ConsoleMessage>>>,
    }

    impl V8ConsoleMessageStorage {
        pub fn new() -> Self {
            V8ConsoleMessageStorage {
                messages: RefCell::new(Vec::new()),
            }
        }

        pub fn add_message(&self, message: Rc<V8ConsoleMessage>) {
            self.messages.borrow_mut().push(message);
        }

        pub fn messages(&self) -> Vec<Rc<V8ConsoleMessage>> {
            self.messages.borrow().clone()
        }
    }
}