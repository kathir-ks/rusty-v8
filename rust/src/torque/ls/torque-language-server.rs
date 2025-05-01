// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::env;
use std::ffi::CStr;
use std::os::raw::c_char;

mod logger;
mod torque_file_list;
mod language_server_data;
mod source_positions;
mod diagnostics_files;
mod message_handler;
mod message_pipe;
mod json; // Placeholder for json handling crate/module

use logger::Logger;
use torque_file_list::TorqueFileList;
use language_server_data::LanguageServerData;
use source_positions::SourceFileMap;
use diagnostics_files::DiagnosticsFiles;
use message_handler::handle_message;
use message_pipe::{read_message, write_message};
use json::JsonValue; // Assuming a JsonValue type is defined in the json module

fn wrapped_main(args: &[String]) -> Result<(), Box<dyn std::error::Error>> {
    let _log_scope = Logger::Scope::new();
    let _files_scope = TorqueFileList::Scope::new();
    let _server_data_scope = LanguageServerData::Scope::new();
    let _source_file_map_scope = SourceFileMap::Scope::new("");
    let _diagnostics_files_scope = DiagnosticsFiles::Scope::new();

    let mut i = 1;
    while i < args.len() {
        if args[i] == "-l" {
            if i + 1 < args.len() {
                Logger::enable(&args[i + 1]);
                i += 1;
            }
            break;
        }
        i += 1;
    }

    loop {
        let message = read_message()?;

        // TODO(szuend): We should probably offload the actual message handling
        //               (even the parsing) to a background thread, so we can
        //               keep receiving messages. We might also receive
        //               $/cancelRequests or contet updates, that require restarts.
        handle_message(message, &write_message)?;
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    wrapped_main(&args)
}