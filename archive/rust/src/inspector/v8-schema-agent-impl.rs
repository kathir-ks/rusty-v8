// Copyright 2016 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod protocol {
    pub mod schema {
        pub type Domain = std::collections::HashMap<String, serde_json::Value>;

        pub fn domain_from_json(json: &serde_json::Value) -> Option<Domain> {
            if let serde_json::Value::Object(map) = json {
                Some(map.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            } else {
                None
            }
        }

    }
}

pub mod v8_inspector {
    use crate::protocol;
    use std::collections::HashMap;

    pub trait FrontendChannel {
        fn send_protocol_notification(
            &self,
            domain: String,
            method: String,
            params: Option<HashMap<String, serde_json::Value>>,
        );
    }

    pub struct Response {
        success: bool,
        message: Option<String>,
    }

    impl Response {
        pub fn success() -> Self {
            Response {
                success: true,
                message: None,
            }
        }

        pub fn error(message: String) -> Self {
            Response {
                success: false,
                message: Some(message),
            }
        }

        pub fn is_success(&self) -> bool {
            self.success
        }

        pub fn message(&self) -> Option<&String> {
            self.message.as_ref()
        }
    }

    pub trait V8InspectorSessionImpl {
        fn supported_domains_impl(&self) -> Vec<protocol::schema::Domain>;
    }

    pub struct V8SchemaAgentImpl<'a> {
        m_session: &'a dyn V8InspectorSessionImpl,
        m_frontend: &'a dyn FrontendChannel,
    }

    impl<'a> V8SchemaAgentImpl<'a> {
        pub fn new(
            session: &'a dyn V8InspectorSessionImpl,
            frontend_channel: &'a dyn FrontendChannel,
        ) -> Self {
            V8SchemaAgentImpl {
                m_session: session,
                m_frontend: frontend_channel,
            }
        }

        pub fn get_domains(&self) -> Response {
            Response::Success()
        }

        pub fn get_domains_result(&self) -> Vec<protocol::schema::Domain> {
            self.m_session.supported_domains_impl()
        }
    }
}