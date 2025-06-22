// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// A TraceEventListener is a simple interface that allows subclasses to listen
/// to trace events. This interface is to hide the more complex interactions that
/// the PerfettoConsumer class has to perform. Clients override ParseFromArray()
/// to process traces, e.g. to write them to a file as JSON or for testing
/// purposes.
pub trait TraceEventListener {
    /// Parses a trace event from a byte array.
    fn parse_from_array(&mut self, array: &[u8]);
}