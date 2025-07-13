// Converted from V8 C++ source files:
// Header: v8-console-agent-impl.h
// Implementation: v8-console-agent-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Console {
        pub trait Backend {
            fn enable(&self) -> Response;
            fn disable(&self) -> Response;
            fn clear_messages(&self) -> Response;
        }
        pub struct Frontend {
            channel: Box<dyn FrontendChannel>,
        }
        impl Frontend {
            pub fn flush(&self) {}
        }
    }
    pub struct Response {
        success: bool,
        error_message: Option<String>,
    }
    impl Response {
        pub fn Success() -> Response {
            Response {
                success: true,
                error_message: None,
            }
        }
        pub fn Error(error_message: String) -> Response {
            Response {
                success: false,
                error_message: Some(error_message),
            }
        }
        pub fn is_success(&self) -> bool {
            self.success
        }
        pub fn error_message(&self) -> &Option<String> {
            &self.error_message
        }
    }
    pub trait FrontendChannel {
        fn send_protocol_notification(&self, message: String);
    }
    pub struct DictionaryValue {}
    impl DictionaryValue {
        pub fn setBoolean(&mut self, _name: &str, _value: bool) {}
        pub fn booleanProperty(&self, _name: &str, _default_value: bool) -> bool {
            false
        }
    }
}
use std::cell::RefCell;
use std::rc::Rc;

pub struct V8ConsoleMessage;
pub struct V8InspectorSessionImpl;
pub struct V8InspectorImpl;
pub enum V8MessageOrigin {
    kConsole,
}
pub struct V8ConsoleMessageStorage {
    messages: Vec<Rc<V8ConsoleMessage>>,
}
impl V8ConsoleMessageStorage {
    pub fn new() -> Self {
        V8ConsoleMessageStorage { messages: Vec::new() }
    }
    pub fn messages(&self) -> &Vec<Rc<V8ConsoleMessage>> {
        &self.messages
    }
}
pub struct V8StackTraceImpl;
pub struct String16;

pub struct V8ConsoleAgentImpl {
    m_session: *mut V8InspectorSessionImpl,
    m_state: Rc<RefCell<protocol::DictionaryValue>>,
    m_frontend: protocol::Console::Frontend,
    m_enabled: bool,
}

impl V8ConsoleAgentImpl {
    pub fn new(
        session: *mut V8InspectorSessionImpl,
        frontend_channel: Box<dyn protocol::FrontendChannel>,
        state: Rc<RefCell<protocol::DictionaryValue>>,
    ) -> V8ConsoleAgentImpl {
        V8ConsoleAgentImpl {
            m_session: session,
            m_state: state,
            m_frontend: protocol::Console::Frontend {
                channel: frontend_channel,
            },
            m_enabled: false,
        }
    }

    pub fn enable(&mut self) -> protocol::Response {
        if self.m_enabled {
            return protocol::Response::Success();
        }
        self.m_state
            .borrow_mut()
            .setBoolean("consoleEnabled", true);
        self.m_enabled = true;
        self.report_all_messages();
        protocol::Response::Success()
    }

    pub fn disable(&mut self) -> protocol::Response {
        if !self.m_enabled {
            return protocol::Response::Success();
        }
        self.m_state
            .borrow_mut()
            .setBoolean("consoleEnabled", false);
        self.m_enabled = false;
        protocol::Response::Success()
    }

    pub fn clear_messages(&mut self) -> protocol::Response {
        protocol::Response::Success()
    }

    pub fn restore(&mut self) {
        if !self
            .m_state
            .borrow()
            .booleanProperty("consoleEnabled", false)
        {
            return;
        }
        self.enable();
    }

    pub fn message_added(&mut self, _message: &V8ConsoleMessage) {
        if self.m_enabled {
            self.report_message(&V8ConsoleMessage {}, true);
        }
    }

    pub fn enabled(&self) -> bool {
        self.m_enabled
    }

    fn report_all_messages(&mut self) {
        let storage = self.ensure_console_message_storage();
        for message in storage.messages() {
            if message.origin() == V8MessageOrigin::kConsole {
                if !self.report_message(&V8ConsoleMessage {}, false) {
                    return;
                }
            }
        }
    }

    fn report_message(&mut self, _message: &V8ConsoleMessage, _generate_preview: bool) -> bool {
        self.m_frontend.flush();
        self.has_console_message_storage()
    }

    fn inspector(&self) -> &V8InspectorImpl {
        unsafe {
            (*self.m_session).inspector()
        }
    }

    fn ensure_console_message_storage(&self) -> V8ConsoleMessageStorage {
        let session = unsafe {& *self.m_session};
        session.inspector().ensure_console_message_storage(session.context_group_id())
    }
    fn has_console_message_storage(&self) -> bool {
        let session = unsafe {& *self.m_session};
        session.inspector().has_console_message_storage(session.context_group_id())
    }
}

impl Drop for V8ConsoleAgentImpl {
    fn drop(&mut self) {}
}

trait ConsoleMessage {
    fn origin(&self) -> V8MessageOrigin;
    fn reportToFrontend(&self, frontend: &protocol::Console::Frontend);
}
impl ConsoleMessage for V8ConsoleMessage {
    fn origin(&self) -> V8MessageOrigin {
        V8MessageOrigin::kConsole
    }
    fn reportToFrontend(&self, frontend: &protocol::Console::Frontend){

    }
}

impl V8InspectorImpl{
    fn ensure_console_message_storage(&self, context_group_id: i32) -> V8ConsoleMessageStorage{
        V8ConsoleMessageStorage::new()
    }
    fn has_console_message_storage(&self, context_group_id: i32) -> bool{
        true
    }
}
impl V8InspectorSessionImpl{
    fn inspector(&self) -> &V8InspectorImpl{
        todo!()
    }
    fn context_group_id(&self) -> i32{
        0
    }
}
