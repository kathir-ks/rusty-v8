// Converted from V8 C++ source files:
// Header: v8-schema-agent-impl.h
// Implementation: v8-schema-agent-impl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod protocol {
    pub mod Schema {
        pub struct Domain {
        }
    }
    pub struct Array<T> {
    }
    pub struct DictionaryValue {
    }
}

pub mod v8_inspector {

    use std::ptr;

    use crate::protocol;
    use crate::V8InspectorSessionImpl;
    use crate::Response;

    pub struct V8SchemaAgentImpl {
        m_session: *mut V8InspectorSessionImpl,
        m_frontend: protocol::Schema::Frontend,
    }

    impl V8SchemaAgentImpl {
        pub fn new(session: *mut V8InspectorSessionImpl, frontend_channel: *mut dyn FrontendChannel, state: *mut protocol::DictionaryValue) -> V8SchemaAgentImpl {
            let frontend = protocol::Schema::Frontend {
                frontend_channel: unsafe { &mut *frontend_channel },
            };
            V8SchemaAgentImpl {
                m_session: session,
                m_frontend: frontend,
            }
        }

        pub fn getDomains(&self, result: &mut Option<Box<Vec<Box<protocol::Schema::Domain>>>>) -> Response {
            unsafe {
              let domains = (&mut *self.m_session).supportedDomainsImpl();
              let mut vec = Vec::new();
                for domain in domains {
                    vec.push(Box::new(domain));
                }

                *result = Some(Box::new(vec));
            }
            Response { success: true, message: None }
        }
    }

    impl Drop for V8SchemaAgentImpl {
        fn drop(&mut self) {}
    }

    pub trait FrontendChannel {
    }

    pub mod protocol {
        pub mod Schema {
            #[derive(Debug)]
            pub struct Frontend {
                pub frontend_channel: *mut dyn super::super::FrontendChannel,
            }
        }
    }
}
