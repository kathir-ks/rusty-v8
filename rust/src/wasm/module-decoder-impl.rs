// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Determine if cfg attribute should be enabled in source
// #![cfg(feature = "enable_webassembly")]

pub mod wasm {
    use std::fmt;
    use std::sync::{Arc, Mutex};

    // use base::platform::wrappers; // TODO: Add equivalent wrapper crate
    // use logging::counters; // TODO: Add logging crate
    // use strings::unicode; // TODO: Add equivalent unicode crate
    // use utils::ostreams; // TODO: Add equivalent ostreams crate
    // use wasm::canonical_types; // TODO: Add equivalent canonical types crate
    // use wasm::constant_expression_interface; // TODO: Add equivalent constant expression interface crate
    // use wasm::function_body_decoder_impl; // TODO: Add equivalent function body decoder impl crate
    // use wasm::module_decoder; // TODO: Add equivalent module decoder crate
    // use wasm::wasm_engine; // TODO: Add equivalent wasm engine crate
    // use wasm::wasm_module; // TODO: Add equivalent wasm module crate
    // use wasm::wasm_subtyping; // TODO: Add equivalent wasm subtyping crate
    // use wasm::well_known_imports; // TODO: Add equivalent well known imports crate
    // use crate::base::vector::Vector;
    // use crate::strings::unibrow::Utf8;
    // use crate::strings::unibrow::Wtf8;

    macro_rules! TRACE {
        ($($arg:tt)*) => {
            if crate::V8_FLAGS.trace_wasm_decoder {
                println!($($arg)*);
            }
        };
    }

    pub(crate) const K_NAME_STRING: &str = "name";
    pub(crate) const K_SOURCE_MAPPING_URL_STRING: &str = "sourceMappingURL";
    pub(crate) const K_INST_TRACE_STRING: &str = "metadata.code.trace_inst";
    pub(crate) const K_COMPILATION_HINTS_STRING: &str = "compilationHints";
    pub(crate) const K_BRANCH_HINTS_STRING: &str = "metadata.code.branch_hint";
    pub(crate) const K_DEBUG_INFO_STRING: &str = ".debug_info";
    pub(crate) const K_EXTERNAL_DEBUG_INFO_STRING: &str = "external_debug_info";
    pub(crate) const K_BUILD_ID_STRING: &str = "build_id";

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub enum ImportExportKindCode {
        ExternalFunction,
        ExternalTable,
        ExternalMemory,
        ExternalGlobal,
        ExternalTag,
    }

    pub fn external_kind_name(kind: ImportExportKindCode) -> &'static str {
        match kind {
            ImportExportKindCode::ExternalFunction => "function",
            ImportExportKindCode::ExternalTable => "table",
            ImportExportKindCode::ExternalMemory => "memory",
            ImportExportKindCode::ExternalGlobal => "global",
            ImportExportKindCode::ExternalTag => "tag",
        }
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    pub struct WireBytesRef {
        pub offset: u32,
        pub length: u32,
    }

    impl WireBytesRef {
        pub fn new(offset: u32, length: u32) -> Self {
            WireBytesRef { offset, length }
        }
    }

    // TODO: Add unibrow and Utf8Variant enums and implementations, and implement validate_utf8
    pub fn validate_utf8(_decoder: &mut Decoder, _string: WireBytesRef) -> bool {
        // unibrow::Utf8::ValidateEncoding(
        //     decoder.start() + decoder.GetBufferRelativeOffset(string.offset()),
        //     string.length(),
        // )
        true // Placeholder
    }

    // TODO: Add Utf8Variant grammar enum
    pub fn consume_string(
        decoder: &mut Decoder,
        _grammar: Utf8Variant,
        name: &str,
        tracer: &mut dyn ITracer,
    ) -> WireBytesRef {
        if !tracer.is_no_trace() {
            tracer.description(name);
            tracer.description(" ");
        }
        let length = decoder.consume_u32v("length", tracer);
        if !tracer.is_no_trace() {
            tracer.description(": ");
            tracer.description(length.to_string().as_str());
            tracer.next_line();
        }
        let offset = decoder.pc_offset();
        let string_start = decoder.pc();

        // Consume bytes before validation to guarantee that the string is not oob.
        if length > 0 {
            if !tracer.is_no_trace() {
                tracer.bytes(decoder.pc(), length);
                tracer.description(name);
                tracer.description(": ");
                tracer.description(
                    std::str::from_utf8(decoder.pc()).unwrap_or("<invalid utf8>"),
                ); // TODO: Safer conversion here
                tracer.next_line();
            }
            decoder.consume_bytes(length, name);
            if decoder.ok {
                // switch grammar {
                //     Utf8Variant::kLossyUtf8 => {}
                //     Utf8Variant::kUtf8 => {
                //         if !Utf8::ValidateEncoding(string_start, length) {
                //             decoder.errorf(string_start, "%s: no valid UTF-8 string", name);
                //         }
                //     }
                //     Utf8Variant::kWtf8 => {
                //         if !Wtf8::ValidateEncoding(string_start, length) {
                //             decoder.errorf(string_start, "%s: no valid WTF-8 string", name);
                //         }
                //     }
                //     Utf8Variant::kUtf8NoTrap => unreachable!(),
                // }
            }
        }

        WireBytesRef {
            offset,
            length: if decoder.failed() { 0 } else { length },
        }
    }

    pub fn consume_string_no_trace(
        decoder: &mut Decoder,
        grammar: Utf8Variant,
        name: &str,
    ) -> WireBytesRef {
        consume_string(decoder, grammar, name, &mut NoTrace {})
    }

    pub fn consume_utf8_string(
        decoder: &mut Decoder,
        name: &str,
        tracer: &mut dyn ITracer,
    ) -> WireBytesRef {
        consume_string(decoder, Utf8Variant::Utf8, name, tracer)
    }

    pub fn identify_unknown_section_internal(
        decoder: &mut Decoder,
        tracer: &mut dyn ITracer,
    ) -> SectionCode {
        let string = consume_utf8_string(decoder, "section name", tracer);
        if decoder.failed() {
            return SectionCode::UnknownSectionCode;
        }
        let section_name_start = decoder.start().as_ptr().wrapping_add(decoder.get_buffer_relative_offset(string.offset) as usize);

        TRACE!(
            "  +%d  section name        : \"%.*s\"\n",
            section_name_start as usize - decoder.start().as_ptr() as usize,
            if string.length < 20 {
                string.length
            } else {
                20
            },
            std::str::from_utf8(&decoder.start()[(decoder.get_buffer_relative_offset(string.offset) as usize)..(decoder.get_buffer_relative_offset(string.offset) as usize + string.length as usize)]).unwrap_or("<invalid utf8>")
        );

        let special_sections: [(fn(&str) -> bool, SectionCode); 8] = [
            (|s| s == K_NAME_STRING, SectionCode::NameSectionCode),
            (
                |s| s == K_SOURCE_MAPPING_URL_STRING,
                SectionCode::SourceMappingURLSectionCode,
            ),
            (|s| s == K_INST_TRACE_STRING, SectionCode::InstTraceSectionCode),
            (
                |s| s == K_COMPILATION_HINTS_STRING,
                SectionCode::CompilationHintsSectionCode,
            ),
            (
                |s| s == K_BRANCH_HINTS_STRING,
                SectionCode::BranchHintsSectionCode,
            ),
            (|s| s == K_DEBUG_INFO_STRING, SectionCode::DebugInfoSectionCode),
            (
                |s| s == K_EXTERNAL_DEBUG_INFO_STRING,
                SectionCode::ExternalDebugInfoSectionCode,
            ),
            (|s| s == K_BUILD_ID_STRING, SectionCode::BuildIdSectionCode),
        ];

        let name_vec = unsafe {
            std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                section_name_start,
                string.length as usize,
            ))
        };

        for special_section in &special_sections {
            if (special_section.0)(name_vec) {
                return special_section.1;
            }
        }

        SectionCode::UnknownSectionCode
    }

    pub struct WasmSectionIterator<'a> {
        decoder: &'a mut Decoder,
        tracer: &'a mut dyn ITracer,
        section_code: SectionCode,
        section_start: *const u8,
        payload_start: *const u8,
        section_end: *const u8,
    }

    impl<'a> WasmSectionIterator<'a> {
        pub fn new(decoder: &'a mut Decoder, tracer: &'a mut dyn ITracer) -> Self {
            let mut iterator = WasmSectionIterator {
                decoder,
                tracer,
                section_code: SectionCode::UnknownSectionCode,
                section_start: std::ptr::null(),
                payload_start: std::ptr::null(),
                section_end: std::ptr::null(),
            };
            iterator.next();
            iterator
        }

        pub fn more(&self) -> bool {
            self.decoder.ok && self.decoder.more()
        }

        pub fn section_code(&self) -> SectionCode {
            self.section_code
        }

        pub fn section_start(&self) -> *const u8 {
            self.section_start
        }

        pub fn section_length(&self) -> u32 {
            unsafe { self.section_end.offset_from(self.section_start) as u32 }
        }

        pub fn payload(&self) -> &[u8] {
            let len = self.payload_length();
            unsafe { std::slice::from_raw_parts(self.payload_start, len as usize) }
        }

        pub fn payload_start(&self) -> *const u8 {
            self.payload_start
        }

        pub fn payload_length(&self) -> u32 {
            unsafe { self.section_end.offset_from(self.payload_start) as u32 }
        }

        pub fn section_end(&self) -> *const u8 {
            self.section_end
        }

        pub fn advance(&mut self, move_to_section_end: bool) {
            if move_to_section_end && self.decoder.pc() < self.section_end {
                self.decoder.consume_bytes(
                    unsafe { self.section_end.offset_from(self.decoder.pc()) as u32 },
                    "section payload",
                );
            }
            if self.decoder.pc() != self.section_end {
                let msg = if self.decoder.pc() < self.section_end {
                    "shorter"
                } else {
                    "longer"
                };
                self.decoder.errorf(
                    self.decoder.pc(),
                    &format!(
                        "section was {} than expected size ({} bytes expected, {} decoded)",
                        msg,
                        self.section_length(),
                        unsafe { self.decoder.pc().offset_from(self.section_start) }
                    ),
                );
            }
            self.next();
        }

        fn next(&mut self) {
            if !self.decoder.more() {
                self.section_code = SectionCode::UnknownSectionCode;
                return;
            }
            self.section_start = self.decoder.pc();

            if !self.tracer.is_no_trace() {
                self.tracer.next_line();
            }
            let section_code = self.decoder.consume_u8("section kind", self.tracer);
            if !self.tracer.is_no_trace() {
                self.tracer.description(": ");
                self.tracer.description(section_name(SectionCode::from(section_code)));
                self.tracer.next_line();
            }

            let section_length = self.decoder.consume_u32v("section length", self.tracer);
            if !self.tracer.is_no_trace() {
                self.tracer.description(section_length.to_string().as_str());
                self.tracer.next_line();
            }

            self.payload_start = self.decoder.pc();
            self.section_end = unsafe { self.payload_start.add(section_length as usize) };

            if section_length > self.decoder.available_bytes() {
                self.decoder.errorf(
                    self.section_start,
                    &format!(
                        "section (code {}, \"{}\") extends past end of the module (length {}, remaining bytes {})",
                        section_code,
                        section_name(SectionCode::from(section_code)),
                        section_length,
                        self.decoder.available_bytes()
                    ),
                );
                self.section_end = self.payload_start;
            }

            let mut section_code_enum = SectionCode::from(section_code);

            if section_code_enum == SectionCode::UnknownSectionCode {
                let module_end = self.decoder.end();
                self.decoder.set_end(self.section_end);
                section_code_enum = identify_unknown_section_internal(self.decoder, self.tracer);
                if self.decoder.ok() {
                    self.decoder.set_end(module_end);
                }
                self.payload_start = self.decoder.pc();
            } else if !is_valid_section_code(section_code_enum) {
                self.decoder.errorf(
                    self.decoder.pc(),
                    &format!("unknown section code #0x{:02x}", section_code),
                );
            }

            self.section_code = if self.decoder.failed() {
                SectionCode::UnknownSectionCode
            } else {
                section_code_enum
            };

            if self.section_code == SectionCode::UnknownSectionCode
                && self.section_end > self.decoder.pc()
            {
                let remaining =
                    unsafe { self.section_end.offset_from(self.decoder.pc()) as u32 };
                self.decoder.consume_bytes(remaining, "section payload");
            }
        }
    }

    pub fn dump_module(module_bytes: &[u8], ok: bool) {
        // TODO: Implement module dumping functionality (file writing, etc.)
        // For now, just print a message
        println!(
            "Module dumping {} ({} bytes). Implementation pending.",
            if ok { "succeeded" } else { "failed" },
            module_bytes.len()
        );
    }

    /// The main logic for decoding the bytes of a module.
    pub struct ModuleDecoderImpl {
        decoder: Decoder,
        enabled_features: WasmEnabledFeatures,
        detected_features: *mut WasmDetectedFeatures, //Mutable pointer to WasmDetectedFeatures
        module_: Arc<Mutex<WasmModule>>,
        module_start: *const u8,
        module_end: *const u8,
        tracer: Box<dyn ITracer>,
        next_ordered_section_: SectionCode,
        seen_unordered_sections_: u32,
        init_expr_zone_: Zone,
        inst_traces_: Vec<(u32, u32, u32)>,
    }

    impl ModuleDecoderImpl {
        pub fn new(
            enabled_features: WasmEnabledFeatures,
            wire_bytes: &[u8],
            origin: ModuleOrigin,
            detected_features: *mut WasmDetectedFeatures, //Mutable pointer to WasmDetectedFeatures
            tracer: Box<dyn ITracer>,
        ) -> Self {
            ModuleDecoderImpl {
                decoder: Decoder::new(wire_bytes, 0),
                enabled_features,
                detected_features,
                module_: Arc::new(Mutex::new(WasmModule::new(origin))),
                module_start: wire_bytes.as_ptr(),
                module_end: unsafe { wire_bytes.as_ptr().add(wire_bytes.len()) },
                tracer,
                next_ordered_section_: SectionCode::TypeSectionCode,
                seen_unordered_sections_: 0,
                init_expr_zone_: Zone::new("InitExprZone"),
                inst_traces_: Vec::new(),
            }
        }

        pub fn on_first_error(&mut self) {
            self.decoder.pc = self.decoder.end;
        }

        pub fn decode_module_header(&mut self, bytes: &[u8]) {
            if self.decoder.failed() {
                return;
            }
            self.decoder.reset(bytes, 0);

            let pos = self.decoder.pc;
            let magic_word = self.decoder.consume_u32("wasm magic", &mut *self.tracer);
            self.tracer.next_line();
            if magic_word != KWASM_MAGIC {
                self.decoder.errorf(
                    pos,
                    &format!(
                        "expected magic word 0x{:08x}, found 0x{:08x}",
                        KWASM_MAGIC, magic_word
                    ),
                );
            }

            let pos = self.decoder.pc;
            {
                let magic_version = self.decoder.consume_u32("wasm version", &mut *self.tracer);
                self.tracer.next_line();
                if magic_version != KWASM_VERSION {
                    self.decoder.errorf(
                        pos,
                        &format!(
                            "expected version 0x{:08x}, found 0x{:08x}",
                            KWASM_VERSION, magic_version
                        ),
                    );
                }
            }
        }

        pub fn check_section_order(&mut self, section_code: SectionCode) -> bool {
            // Check the order of ordered sections.
            if section_code >= SectionCode::FirstSectionInModule
                && section_code < SectionCode::FirstUnorderedSection
            {
                if section_code < self.next_ordered_section_ {
                    self.decoder.errorf(
                        self.decoder.pc(),
                        &format!("unexpected section <{}>", section_name(section_code)),
                    );
                    return false;
                }
                self.next_ordered_section_ = SectionCode::from(section_code as u8 + 1);
                return true;
            }

            // Ignore ordering problems in unknown / custom sections. Even allow them to
            // appear multiple times. As optional sections we use them on a "best
            // effort" basis.
            if section_code == SectionCode::UnknownSectionCode {
                return true;
            }
            if section_code > SectionCode::LastKnownModuleSection {
                return true;
            }

            // The rest is standardized unordered sections; they are checked more
            // thoroughly..
            // assert!(SectionCode::FirstUnorderedSection <= section_code);
            // assert!(SectionCode::LastKnownModuleSection >= section_code);

            // Check that unordered sections don't appear multiple times.
            if self.has_seen_unordered_section(section_code) {
                self.decoder.errorf(
                    self.decoder.pc(),
                    &format!("Multiple {} sections not allowed", section_name(section_code)),
                );
                return false;
            }
            self.set_seen_unordered_section(section_code);

            // Define a helper to ensure that sections <= {before} appear before the
            // current unordered section, and everything >= {after} appears after it.
            let mut check_order =
                |before: SectionCode, after: SectionCode| -> bool {
                    // assert!(before < after);
                    if self.next_ordered_section_ > after {
                        self.decoder.errorf(
                            self.decoder.pc(),
                            &format!(
                                "The {} section must appear before the {} section",
                                section_name(section_code),
                                section_name(after)
                            ),
                        );
                        return false;
                    }
                    if self.next_ordered_section_ <= before {
                        self.next_ordered_section_ = SectionCode::from(before as u8 + 1);
                    }
                    true
                };

            // Now check the ordering constraints of specific unordered sections.
            match section_code {
                SectionCode::DataCountSectionCode => {
                    check_order(SectionCode::ElementSectionCode, SectionCode::CodeSectionCode)
                }
                SectionCode::TagSectionCode => {
                    check_order(SectionCode::MemorySectionCode, SectionCode::GlobalSectionCode)
                }
                SectionCode::StringRefSectionCode => {
                    // TODO(12868): If there's a tag section, assert that we're after the
                    // tag section.
                    check_order(SectionCode::MemorySectionCode, SectionCode::GlobalSectionCode)
                }
                SectionCode::InstTraceSectionCode => {
                    // Custom section following code.metadata tool convention containing
                    // offsets specifying where trace marks should be emitted.
                    // Be lenient with placement of instruction trace section. All except
                    // first occurrence after function section and before code section are
                    // ignored.
                    true
                }
                _ => true,
            }
        }

        pub fn decode_section(&mut self, section_code: SectionCode, bytes: &[u8], offset: u32) {
            if self.decoder.failed() {
                return;
            }
            self.decoder.reset(bytes, offset);
            TRACE!("Section: {}\n", section_name(section_code));
            TRACE!(
                "Decode Section {:p} - {:p}\n",
                bytes.as_ptr(),
                unsafe { bytes.as_ptr().add(bytes.len()) }
            );

            if !self.check_section_order(section_code) {
                return;
            }

            match section_code {
                SectionCode::UnknownSectionCode => {}
                SectionCode::TypeSectionCode => self.decode_type_section(),
                SectionCode::ImportSectionCode => self.decode_import_section(),
                SectionCode::FunctionSectionCode => self.decode_function_section(),
                SectionCode::TableSectionCode => self.decode_table_section(),
                SectionCode::MemorySectionCode => self.decode_memory_section(),
                SectionCode::GlobalSectionCode => self.decode_global_section(),
                SectionCode::ExportSectionCode => self.decode_export_section(),
                SectionCode::StartSectionCode => self.decode_start_section(),
                SectionCode::CodeSectionCode => self.decode_code_section(),
                SectionCode::ElementSectionCode => self.decode_element_section(),
                SectionCode::DataSectionCode => self.decode_data_section(),
                SectionCode::NameSectionCode => self.decode_name_section(),
                SectionCode::SourceMappingURLSectionCode => {
                    self.decode_source_mapping_url_section()
                }
                SectionCode::DebugInfoSectionCode => {
                    let mut module = self.module_.lock().unwrap();
                    module.debug_symbols.insert(
                        WasmDebugSymbolsType::EmbeddedDWARF,
                        DebugSymbols {
                            symbol_type: WasmDebugSymbolsType::EmbeddedDWARF,
                            bytes_: Vec::new(), //TODO: Pass bytes
                        },
                    );
                    self.decoder
                        .consume_bytes((self.decoder.end as usize - self.decoder.start as usize) as u32, ".debug_info");
                }
                SectionCode::ExternalDebugInfoSectionCode => {
                    self.decode_external_debug_info_section()
                }
                SectionCode::BuildIdSectionCode => self.decode_build_id_section(),
                SectionCode::InstTraceSectionCode => {
                    if self.enabled_features.has_instruction_tracing() {
                        self.decode_inst_trace_section();
                    } else {
                        // Ignore this section when feature is disabled. It is an optional
                        // custom section anyways.
                        self.decoder
                            .consume_bytes((self.decoder.end as usize - self.decoder.start as usize) as u32, "");
                    }
                }
                SectionCode::CompilationHintsSectionCode => {
                    if self.enabled_features.has_compilation_hints() {
                        self.decode_compilation_hints_section();
                    } else {
                        // Ignore this section when feature was disabled. It is an optional
                        // custom section anyways.
                        self.decoder
                            .consume_bytes((self.decoder.end as usize - self.decoder.start as usize) as u32, "");
                    }
                }
                SectionCode::BranchHintsSectionCode => {
                    if self.enabled_features.has_branch_hinting() {
                        self.decode_branch_hints_section();
                    } else {
                        // Ignore this section when feature was disabled. It is an optional
                        // custom section anyways.
                        self.decoder
                            .consume_bytes((self.decoder.end as usize - self.decoder.start as usize) as u32, "");
                    }
                }
                SectionCode::DataCountSectionCode => self.decode_data_count_section(),
                SectionCode::TagSectionCode => self.decode_tag_section(),
                SectionCode::StringRefSectionCode => {
                    if self.enabled_features.has_stringref() {
                        self.decode_string_ref_section();
                    } else {
                        self.decoder.errorf(
                            self.decoder.pc(),
                            &format!(
                                "unexpected section <{}> (enable with --experimental-wasm-stringref)",
                                section_name(section_code)
                            ),
                        );
                    }
                }
                _ => {
                    self.decoder.errorf(
                        self.decoder.pc(),
                        &format!("unexpected section <{}>", section_name(section_code)),
                    );
                    return;
                }
            }

            if self.decoder.pc() != bytes.as_ptr().wrapping_add(bytes.len()) {
                let msg = if self.decoder.pc() < bytes.as_ptr().wrapping_add(bytes.len()) {
                    "shorter"
                } else {
                    "longer"
                };
                self.decoder.errorf(
                    self.decoder.pc(),
                    &format!(
                        "section was {} than expected size ({} bytes expected, {} decoded)",
                        msg,
                        bytes.len(),
                        unsafe { self.decoder.pc().offset_from(bytes.as_ptr()) }
                    ),
                );
            }
        }

        pub fn type_kind_name(kind: u8) -> &'static str {
            match kind {
                K_WASM_FUNCTION_TYPE_CODE => "func",
                K_WASM_STRUCT_TYPE_CODE => "struct",
                K_WASM_ARRAY_TYPE_CODE => "array",
                K_WASM_CONT_TYPE_CODE => "cont",
                _ => "unknown",
            }
        }

        fn consume_base_type_definition(&mut self) -> TypeDefinition {
            let is_final = true;
            let mut shared = false;

            let kind = self.decoder.consume_u8(" kind", &mut *self.tracer);
            if !self.tracer.is_no_trace() {
                self.tracer.description(": ");
            }
            if kind == K_SHARED_FLAG_CODE {
                if !V8_FLAGS.experimental_wasm_shared {
                    self.decoder.errorf(
                        unsafe { self.decoder.pc().offset(-1) },
                        &format!(
                            "unknown type form: {}, enable with --experimental-wasm-shared",
                            kind
                        ),
                    );
                    return TypeDefinition::default();
                }
                shared = true;
                self.module_.lock().unwrap().has_shared_part = true;
                let kind = self.decoder.consume_u8("shared ", &mut *self.tracer);
                if !self.tracer.is_no_trace() {
                    self.tracer.description(ModuleDecoderImpl::type_kind_name(kind));
                }
            } else {
                if !self.tracer.is_no_trace() {
                    self.tracer.description(ModuleDecoderImpl::type_kind_name(kind));
                }
            }

            match kind {
                K_WASM_FUNCTION_TYPE_CODE => {
                    let sig = self.consume_sig(&self.module_.lock().unwrap().signature_zone);
                    if sig.is_none() {
                        assert!(!self.decoder.ok());
                        return TypeDefinition::default();
                    }
                    TypeDefinition {
                        kind: TypeDefinitionKind::Function,
                        function_sig: sig.unwrap(),
                        supertype: k_no_super_type,
                        is_final,
                        shared,
                        struct_type: None,
                        array_type: None,
                        cont_type: None,
                    }
                }
                K_WASM_STRUCT_TYPE_CODE => {
                    self.module_.lock().unwrap().is_wasm_gc = true;
                    let struct_type = self.consume_struct(&self.module_.lock().unwrap().signature_zone);
                    if struct_type.is_none() {
                        assert!(!self.decoder.ok());
                        return TypeDefinition::default();
                    }
                    TypeDefinition {
                        kind: TypeDefinitionKind::Struct,
                        struct_type: struct_type,
                        supertype: k_no_super_type,
                        is_final,
                        shared,
                        function_sig: None,
                        array_type: None,
                        cont_type: None,
                    }
                }
                K_WASM_ARRAY_TYPE_CODE => {
                    self.module_.lock().unwrap().is_wasm_gc = true;
                    let array_type = self.consume_array(&self.module_.lock().unwrap().signature_zone);
                    if array_type.is_none() {
                        assert!(!self.decoder.ok());
                        return TypeDefinition::default();
                    }
                    TypeDefinition {
                        kind: TypeDefinitionKind::Array,
                        array_type: array_type,
                        supertype: k_no_super_type,
                        is_final,
                        shared,
                        function_sig: None,
                        struct_type: None,
                        cont_type: None,
                    }
                }
                K_WASM_CONT_TYPE_CODE => {
                    if !self.enabled_features.has_wasmfx() {
                        self.decoder.error(
                            unsafe { self.decoder.pc().offset(-1) },
                            "core stack switching not enabled (enable with --experimental-wasm-wasmfx)",
                        );
                    }

                    let pos = self.decoder.pc();
                    let hp = self.consume_heap_type();

                    if !hp.is_index {
                        self.decoder.error(pos, "cont type must refer to a signature index");
                        return TypeDefinition::default();
                    }

                    let cont_type = self.module_.lock().unwrap().signature_zone.new_cont_type(hp.ref_index);
                    TypeDefinition {
                        kind: TypeDefinitionKind::Cont,
                        cont_type: Some(cont_type),
                        supertype: k_no_super_type,
                        is_final,
                        shared,
                        function_sig: None,
                        struct_type: None,
                        array_type: None,
                    }
                }
                _ => {
                    if !self.tracer.is_no_trace() {
                        self.tracer.next_line();
                    }
                    self.decoder.errorf(
                        unsafe { self.decoder.pc().offset(-1) },
                        &format!("unknown type form: {}", kind),
                    );
                    return TypeDefinition::default();
                }
            }
        }

        // {current_type_index} is the index of the type that's being decoded.
        // Any supertype must have a lower index.
        fn consume_subtype_definition(&mut self, current_type_index: usize) -> TypeDefinition {
            let kind = *unsafe { self.decoder.pc() };
            if kind == K_WASM_SUBTYPE_CODE || kind == K_WASM_SUBTYPE_FINAL_CODE {
                self.module_.lock().unwrap().is_wasm_gc = true;
                let is_final = kind == K_WASM_SUBTYPE_FINAL_CODE;
                let is_final_string = if is_final { "subtype final" } else { "subtype extensible" };
                self.decoder.consume_bytes(1, is_final_string);

                const MAXIMUM_SUPERTYPES: u32 = 1;
                let supertype_count = self.decoder.consume_count("supertype count", MAXIMUM_SUPERTYPES as usize);
                let mut supertype: u32 = k_no_super_type_index;
                if supertype_count == 1 {
                    supertype = self.decoder.consume_u32v("supertype", &mut *self.tracer);
                    if supertype as usize >= current_type_index {
                        self.decoder.errorf(
                            self.decoder.pc(),
                            &format!("type {}: invalid supertype {}", current_type_index, supertype),
                        );
                        return TypeDefinition::default();
                    }
                    if !self.tracer.is_no_trace() {
                        self.tracer.description(supertype.to_string().as_str());
                        self.tracer.next_line();
                    }