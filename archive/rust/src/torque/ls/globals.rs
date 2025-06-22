// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::fs::File;
use std::io::Write;
use std::sync::{Mutex, OnceLock};

/// When the language server is run by VS code, stdout can not be seen, as it is
/// used as the communication channel. For debugging purposes a simple
/// Log class is added, that allows writing diagnostics to a file configurable
/// via command line flag.
pub struct Logger {
    enabled: bool,
    logfile: Option<Mutex<File>>,
}

impl Logger {
    const DEFAULT: Logger = Logger {
        enabled: false,
        logfile: None,
    };

    /// Retrieves a thread-safe, lazily-initialized, static instance of `Logger`.
    fn instance() -> &'static OnceLock<Logger> {
        static INSTANCE: OnceLock<Logger> = OnceLock::new();
        &INSTANCE
    }

    /// Enables the logger and sets the path to the log file.
    pub fn enable(path: String) {
        let _ = Logger::instance().get_or_init(|| Logger::DEFAULT);

        let mut logger = Logger::instance().get().unwrap();

        let result = File::create(path);

        match result {
            Ok(file) => {
                logger.enabled = true;
                logger.logfile = Some(Mutex::new(file));
            }
            Err(e) => {
                eprintln!("Error opening log file: {}", e);
            }
        }
    }

    /// Logs the given arguments to the log file if the logger is enabled.
    pub fn log<T: std::fmt::Display>(args: T) {
        let _ = Logger::instance().get_or_init(|| Logger::DEFAULT);
        let logger = Logger::instance().get().unwrap();

        if logger.enabled {
            if let Some(ref logfile_mutex) = logger.logfile {
                if let Ok(mut logfile) = logfile_mutex.lock() {
                    let _ = writeln!(logfile, "{}", args);
                    let _ = logfile.flush();
                } else {
                    eprintln!("Failed to acquire lock on logfile");
                }
            }
        }
    }

    fn enabled() -> bool {
        let _ = Logger::instance().get_or_init(|| Logger::DEFAULT);
        let logger = Logger::instance().get().unwrap();

        logger.enabled
    }
}

// TODO: implement contextual variables with thread local storage or similar
pub struct TorqueFileList(Vec<String>);

impl TorqueFileList {
    pub fn new() -> Self {
        TorqueFileList(Vec::new())
    }

    pub fn get(&self) -> &Vec<String> {
        &self.0
    }

    pub fn get_mut(&mut self) -> &mut Vec<String> {
        &mut self.0
    }
}