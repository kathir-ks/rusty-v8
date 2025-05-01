// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod torque_compiler {
    use std::path::PathBuf;

    use std::collections::HashMap;

    pub struct TorqueCompilerOptions {
        pub output_directory: String,
        pub v8_root: String,
        pub collect_language_server_data: bool,
        pub collect_kythe_data: bool,
        // dcheck(...) are only generated for debug builds. To provide
        // language server support for statements inside dchecks, this flag
        // can force generate them.
        pub force_assert_statements: bool,
        // Forge (Google3) can only run 64-bit executables. As Torque runs as part
        // of the build process, we need a "cross-compile" mode when we target 32-bit
        // architectures. Note that this does not needed in Chromium/V8 land, since we
        // always build with the same bit width as the target architecture.
        pub force_32bit_output: bool,
        // Adds extra comments in output that show Torque intermediate representation.
        pub annotate_ir: bool,
        // Strips the v8-root in case the source path contains it as a prefix.
        pub strip_v8_root: bool,
    }

    impl Default for TorqueCompilerOptions {
        fn default() -> Self {
            TorqueCompilerOptions {
                output_directory: String::new(),
                v8_root: String::new(),
                collect_language_server_data: false,
                collect_kythe_data: false,
                force_assert_statements: false,
                force_32bit_output: false,
                annotate_ir: false,
                strip_v8_root: false,
            }
        }
    }

    pub type SourceId = usize;

    pub struct SourcePosition {
        pub source: SourceId,
        pub start: usize,
        pub length: usize,
    }

    pub struct SourceFileMap {
        pub map: HashMap<SourceId, PathBuf>,
    }

    pub struct LanguageServerData {}

    pub struct TorqueMessage {
        pub message: String,
    }

    pub struct TorqueCompilerResult {
        // Map translating SourceIds to filenames. This field is
        // set on errors, so the SourcePosition of the error can be
        // resolved.
        pub source_file_map: Option<SourceFileMap>,

        // Eagerly collected data needed for the LanguageServer.
        // Set the corresponding options flag to enable.
        pub language_server_data: LanguageServerData,

        // Errors collected during compilation.
        pub messages: Vec<TorqueMessage>,
    }

    pub struct TorqueCompilationUnit {
        pub source_file_path: String,
        pub file_content: String,
    }

    // TODO: Implement KytheConsumer
    pub struct KytheConsumer {}

    // V8_EXPORT_PRIVATE TorqueCompilerResult
    // CompileTorque(const std::string& source, TorqueCompilerOptions options);
    extern "C" {
        fn CompileTorque(source: *const std::os::raw::c_char, options: TorqueCompilerOptions) -> TorqueCompilerResult;
    }

    pub fn compile_torque_string(source: &str, options: TorqueCompilerOptions) -> TorqueCompilerResult {
        use std::ffi::CString;
        let c_source = CString::new(source).expect("CString::new failed");
        unsafe {
            CompileTorque(c_source.as_ptr(), options)
        }
    }

    // TorqueCompilerResult CompileTorque(const std::vector<std::string>& files,
    //                                    TorqueCompilerOptions options);
    pub fn compile_torque_files(files: &[String], options: TorqueCompilerOptions) -> TorqueCompilerResult {
        let mut result = TorqueCompilerResult {
            source_file_map: None,
            language_server_data: LanguageServerData {},
            messages: Vec::new(),
        };
        for file in files {
            let res = compile_torque_string(file, options);
            result.messages.extend(res.messages);
        }
        result
    }

    // V8_EXPORT_PRIVATE TorqueCompilerResult CompileTorqueForKythe(
    //     std::vector<TorqueCompilationUnit> units, TorqueCompilerOptions options,
    //     KytheConsumer* kythe_consumer);
    pub fn compile_torque_for_kythe(
        units: Vec<TorqueCompilationUnit>,
        options: TorqueCompilerOptions,
        _kythe_consumer: &KytheConsumer,
    ) -> TorqueCompilerResult {
        let mut result = TorqueCompilerResult {
            source_file_map: None,
            language_server_data: LanguageServerData {},
            messages: Vec::new(),
        };
        for unit in units {
            let res = compile_torque_string(&unit.file_content, options);
            result.messages.extend(res.messages);
        }
        result
    }
}