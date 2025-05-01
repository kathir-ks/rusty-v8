// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/snapshot/embedded/embedded-file-writer.h
mod embedded_file_writer {
    use std::collections::HashMap;
    use std::vec::Vec;

    // Re-export types used in public interfaces
    pub struct EmbeddedFileWriter {
        pub embedded_variant_: String,
        pub source_positions_: Vec<Vec<u8>>,
        pub label_info_: Vec<Vec<LabelInfo>>,
        pub external_filenames_: HashMap<String, i32>,
        pub external_filenames_by_index_: Vec<String>,
        //pub unwind_infos_: Vec<UnwindInfo>, // Placeholder for actual type
    }

    #[derive(Debug, Clone)]
    pub struct LabelInfo {
        pub offset: u32,
        pub name: String,
    }

    impl EmbeddedFileWriter {
        pub fn new(embedded_variant: String) -> EmbeddedFileWriter {
            EmbeddedFileWriter {
                embedded_variant_: embedded_variant,
                source_positions_: vec![Vec::new(); crate::builtins::Builtins::kBuiltinCount as usize], // Assuming Builtins::kBuiltinCount is a const
                label_info_: vec![Vec::new(); crate::builtins::Builtins::kBuiltinCount as usize], // Assuming Builtins::kBuiltinCount is a const
                external_filenames_: HashMap::new(),
                external_filenames_by_index_: Vec::new(),
                //unwind_infos_: Vec::new(),
            }
        }
        pub fn write_code_section<T: PlatformEmbeddedFileWriterBase>(&self, w: &mut T, blob: &crate::embedded_data::EmbeddedData) {
            w.comment(
                "The embedded blob code section starts here. It contains the builtin",
            );
            w.comment("instruction streams.");
            w.section_text();
        
            #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
            {
                // UMA needs an exposed function-type label at the start of the embedded
                // code section.
                const K_CODE_START_FOR_PROFILER_SYMBOL_NAME: &str =
                    "v8_code_start_for_profiler_";
                const K_DUMMY_FUNCTION_LENGTH: i32 = 1;
                const K_DUMMY_FUNCTION_DATA: i32 = 0xcc;
                w.declare_function_begin(K_CODE_START_FOR_PROFILER_SYMBOL_NAME, K_DUMMY_FUNCTION_LENGTH);
                // The label must not be at the same address as the first builtin, insert
                // padding bytes.
                write_directive_or_separator(w, 0, DataDirective::Byte);
                w.hex_literal(K_DUMMY_FUNCTION_DATA as u8);
                w.newline();
                w.declare_function_end(K_CODE_START_FOR_PROFILER_SYMBOL_NAME);
            }
        
            w.align_to_code_alignment();
            w.declare_symbol_global(self.embedded_blob_code_symbol().as_str());
            w.declare_label_prolog(self.embedded_blob_code_symbol().as_str());
            w.declare_label(self.embedded_blob_code_symbol().as_str());
        
            //static_assert(Builtins::kAllBuiltinsAreIsolateIndependent);
            for embedded_index in 0..crate::builtins::Builtins::kBuiltinCount {
                let builtin = blob.get_builtin_id(embedded_index);
                self.write_builtin(w, blob, builtin);
            }
            w.align_to_page_size_if_needed();
            w.declare_label_epilogue();
            w.newline();
        }
    
        pub fn write_file_epilogue<T: PlatformEmbeddedFileWriterBase>(&self, w: &mut T, blob: &crate::embedded_data::EmbeddedData) {
            {
                let embedded_blob_code_size_symbol =
                    format!("v8_{}_embedded_blob_code_size_", self.embedded_variant_);
        
                w.comment("The size of the embedded blob code in bytes.");
                w.section_ro_data();
                w.align_to_data_alignment();
                w.declare_uint32(embedded_blob_code_size_symbol.as_str(), blob.code_size());
                w.newline();
        
                let embedded_blob_data_size_symbol =
                    format!("v8_{}_embedded_blob_data_size_", self.embedded_variant_);
        
                w.comment("The size of the embedded blob data section in bytes.");
                w.declare_uint32(embedded_blob_data_size_symbol.as_str(), blob.data_size());
                w.newline();
            }
        
            #[cfg(target_os = "windows")]
            {
                let unwind_info_symbol =
                    format!("{}_Builtins_UnwindInfo", self.embedded_variant_);
                
                // w.maybe_emit_unwind_data(
                //     unwind_info_symbol.as_str(),
                //     self.embedded_blob_code_symbol().as_str(),
                //     blob,
                //     &self.unwind_infos_, // Need to convert this to appropriate type
                // );
            }
        
            w.file_epilogue();
        }

        fn write_builtin<T: PlatformEmbeddedFileWriterBase>(&self, w: &mut T, blob: &crate::embedded_data::EmbeddedData, builtin: crate::builtins::Builtin) {
            let is_default_variant = self.embedded_variant_ == crate::embedded_data::K_DEFAULT_EMBEDDED_VARIANT;
    
            let builtin_symbol = if is_default_variant {
                format!("Builtins_{}", crate::builtins::Builtins::name(builtin))
            } else {
                format!("{}_Builtins_{}", self.embedded_variant_, crate::builtins::Builtins::name(builtin))
            };
    
            // Labels created here will show up in backtraces. We check in
            // Isolate::SetEmbeddedBlob that the blob layout remains unchanged, i.e.
            // that labels do not insert bytes into the middle of the blob byte
            // stream.
            w.declare_function_begin(&builtin_symbol, blob.instruction_size_of(builtin) as i32);
            let builtin_id = builtin as usize;
            let current_positions = &self.source_positions_[builtin_id];
            // The code below interleaves bytes of assembly code for the builtin
            // function with source positions at the appropriate offsets.
            //let vpos = current_positions.as_slice();

            let vpos = current_positions.as_slice();
            let mut positions = crate::codegen::SourcePositionTableIterator::new(vpos, crate::codegen::SourcePositionTableIteratorMode::kExternalOnly);
    
            #[cfg(not(debug_assertions))]
            {
                assert!(positions.done()); // Release builds must not contain debug infos.
            }
    
            // Some builtins (InterpreterPushArgsThenFastConstructFunction,
            // JSConstructStubGeneric) have entry points located in the middle of them, we
            // need to store their addresses since they are part of the list of allowed
            // return addresses in the deoptimizer.
            let current_labels = &self.label_info_[builtin_id];
            let mut label_iter = current_labels.iter();
    
            let data = blob.instruction_start_of(builtin);
            let size = blob.padded_instruction_size_of(builtin) as u32;
            let mut i: u32 = 0;
            let mut next_source_pos_offset = if positions.done() {
                size
            } else {
                positions.code_offset() as u32
            };
            let mut next_label_offset = match label_iter.next() {
                Some(label) => label.offset,
                None => size,
            };
            let mut next_offset: u32 = 0;
    
            while i < size {
                if i == next_source_pos_offset {
                    // Write source directive.
                    w.source_info(
                        positions.source_position().external_file_id(),
                        self.get_externally_compiled_filename(
                            positions.source_position().external_file_id(),
                        )
                        .as_str(),
                        positions.source_position().external_line(),
                    );
                    positions.advance();
                    next_source_pos_offset = if positions.done() {
                        size
                    } else {
                        positions.code_offset() as u32
                    };
                    assert!(next_source_pos_offset >= i);
                }
                if i == next_label_offset {
                    if let Some(label) = label_iter.next(){
                        self.write_builtin_labels(w, label.name.clone());
                        next_label_offset = match label_iter.next() {
                            Some(label) => label.offset,
                            None => size,
                        };
                        assert!(next_label_offset >= i);
                    }
                }
    
                next_offset = std::cmp::min(next_source_pos_offset, next_label_offset);
                Self::write_binary_contents_as_inline_assembly(w, &data[i as usize..], (next_offset - i) as u32);
                i = next_offset;
            }
    
            w.declare_function_end(&builtin_symbol);
        }
    
        fn write_builtin_labels<T: PlatformEmbeddedFileWriterBase>(&self, w: &mut T, name: String) {
            w.declare_label(name.as_str());
        }
    
        fn embedded_blob_code_symbol(&self) -> String {
            format!("v8_{}_embedded_blob_code", self.embedded_variant_)
        }

        fn write_binary_contents_as_inline_assembly<T: PlatformEmbeddedFileWriterBase>(w: &mut T, data: &[u8], size: u32) {
            #[cfg(target_os = "zos")]
            {
                // HLASM source must end at column 71 (followed by an optional
                // line-continuation char on column 72), so write the binary data
                // in 32 byte chunks (length 64):
                let chunks = (size + 31) / 32;
                let mut offset: u32 = 0;
                for _i in 0..chunks {
                    w.write_string(" DC x'");
                    for _j in 0..32 {
                        if offset < size {
                            w.write_string(&format!("{:02x}", data[offset as usize]));
                            offset += 1;
                        }
                    }
                    w.write_string("'\n");
                }
            }
            #[cfg(not(target_os = "zos"))]
            {
                let mut current_line_length = 0;
                let mut i: u32 = 0;
        
                // Begin by writing out byte chunks.
                let directive = w.byte_chunk_data_directive();
                let byte_chunk_size = data_directive_size(directive);
                while i + byte_chunk_size as u32 <= size {
                    current_line_length =
                        write_directive_or_separator(w, current_line_length, directive);
                    current_line_length += w.write_byte_chunk(&data[i as usize..]);
                    current_line_length =
                        write_line_end_if_needed(w, current_line_length, byte_chunk_size);
                    i += byte_chunk_size as u32;
                }
                if current_line_length != 0 {
                    w.newline();
                }
                current_line_length = 0;
        
                // Write any trailing bytes one-by-one.
                while i < size {
                    current_line_length =
                        write_directive_or_separator(w, current_line_length, DataDirective::Byte);
                    current_line_length += w.hex_literal(data[i as usize]);
                    current_line_length = write_line_end_if_needed(w, current_line_length, 1);
                    i += 1;
                }
                if current_line_length != 0 {
                    w.newline();
                }
            }
        }
    
        pub fn lookup_or_add_externally_compiled_filename(&mut self, filename: &str) -> i32 {
            if let Some(&id) = self.external_filenames_.get(filename) {
                return id;
            }
            let new_id = self.external_filename_index_to_id(self.external_filenames_.len() as i32);
            self.external_filenames_.insert(filename.to_string(), new_id);
            self.external_filenames_by_index_.push(filename.to_string());
            debug_assert_eq!(
                self.external_filenames_by_index_.len(),
                self.external_filenames_.len()
            );
            new_id
        }
    
        pub fn get_externally_compiled_filename(&self, fileid: i32) -> String {
            let index = self.external_filename_id_to_index(fileid);
            debug_assert!(index >= 0);
            debug_assert!((index as usize) < self.external_filenames_by_index_.len());
        
            self.external_filenames_by_index_[index as usize].clone()
        }
    
        pub fn get_externally_compiled_filename_count(&self) -> i32 {
            self.external_filenames_.len() as i32
        }
    
        fn external_filename_index_to_id(&self, index: i32) -> i32 {
            index + 1
        }
    
        fn external_filename_id_to_index(&self, id: i32) -> i32 {
            id - 1
        }

        pub fn prepare_builtin_source_position_map(&mut self, builtins: &mut crate::builtins::Builtins) {
            for builtin in crate::builtins::Builtins::kFirst..=crate::builtins::Builtins::kLast {
                // Retrieve the SourcePositionTable and copy it.
                let code = builtins.code(builtin);
                if !code.has_source_position_table() {
                    continue;
                }
                let source_position_table = code.source_position_table();
                let data = source_position_table.data().to_vec();
                self.source_positions_[builtin as usize] = data;
            }
        }
    }
    
    fn write_directive_or_separator<T: PlatformEmbeddedFileWriterBase>(
        w: &mut T,
        current_line_length: i32,
        directive: DataDirective,
    ) -> i32 {
        let printed_chars;
        if current_line_length == 0 {
            printed_chars = w.indented_data_directive(directive);
            debug_assert!(printed_chars > 0);
        } else {
            w.write_string(",");
            printed_chars = 1;
            debug_assert_eq!(1, printed_chars);
        }
        current_line_length + printed_chars
    }
    
    fn write_line_end_if_needed<T: PlatformEmbeddedFileWriterBase>(
        w: &mut T,
        current_line_length: i32,
        write_size: i32,
    ) -> i32 {
        const K_TEXT_WIDTH: i32 = 100;
        // Check if adding ',0xFF...FF\n"' would force a line wrap. This doesn't use
        // the actual size of the string to be written to determine this so it's
        // more conservative than strictly needed.
        if current_line_length + ",0x".len() as i32 + write_size * 2 > K_TEXT_WIDTH {
            w.newline();
            0
        } else {
            current_line_length
        }
    }
    
    #[derive(Debug, Copy, Clone)]
    pub enum DataDirective {
        Byte,
        Word,
        Long,
        // Add more as needed
    
        //ByteChunk(usize),
    }

    pub fn data_directive_size(directive: DataDirective) -> i32 {
        match directive {
            DataDirective::Byte => 1,
            DataDirective::Word => 2,
            DataDirective::Long => 4,
            //DataDirective::ByteChunk(size) => size as i32,
        }
    }
    
    // src/snapshot/embedded/platform-embedded-file-writer-base.h
    pub trait PlatformEmbeddedFileWriterBase {
        fn fp(&mut self) -> &mut dyn std::io::Write;
        fn indented_data_directive(&mut self, directive: DataDirective) -> i32;
        fn hex_literal(&mut self, value: u8) -> i32;
        fn newline(&mut self);
        fn comment(&mut self, comment: &str);
        fn section_text(&mut self);
        fn section_ro_data(&mut self);
        fn align_to_data_alignment(&mut self);
        fn declare_uint32(&mut self, symbol: &str, value: u32);
        fn file_epilogue(&mut self);
        fn declare_function_begin(&mut self, name: &str, size: i32);
        fn declare_function_end(&mut self, name: &str);
        fn declare_label(&mut self, name: &str);
        fn declare_symbol_global(&mut self, name: &str);
        fn declare_label_prolog(&mut self, name: &str);
        fn declare_label_epilogue(&mut self);
        fn align_to_code_alignment(&mut self);
        fn align_to_page_size_if_needed(&mut self);
        fn source_info(&mut self, file_id: i32, filename: &str, line: i32);
        fn byte_chunk_data_directive(&mut self) -> DataDirective;
        fn write_byte_chunk(&mut self, data: &[u8]) -> i32;
        fn write_string(&mut self, s: &str);
        //fn maybe_emit_unwind_data(&mut self, symbol: &str, embedded_blob_code_symbol: &str, blob: &crate::embedded_data::EmbeddedData, unwind_infos: &[UnwindInfo]);
    }
}

// src/codegen/source-position-table.h
mod codegen {
    pub enum SourcePositionTableIteratorMode {
        kExternalOnly,
        // Add other modes as needed
    }
    
    pub struct SourcePositionTableIterator<'a> {
        data: &'a [u8],
        index: usize,
        mode: SourcePositionTableIteratorMode,
    }
    
    impl<'a> SourcePositionTableIterator<'a> {
        pub fn new(data: &'a [u8], mode: SourcePositionTableIteratorMode) -> Self {
            SourcePositionTableIterator { data, index: 0, mode }
        }
    
        pub fn done(&self) -> bool {
            self.index >= self.data.len()
        }
    
        pub fn advance(&mut self) {
            if !self.done() {
                self.index += 1; // Simplified, assuming each entry is one byte
            }
        }
    
        pub fn code_offset(&self) -> usize {
            self.index
        }
    
        pub fn source_position(&self) -> SourcePosition {
            // Simplified, assuming the byte represents line number
            SourcePosition {
                external_file_id: 0, // Replace with actual logic
                external_line: self.data[self.index] as i32,
            }
        }
    }
    
    #[derive(Debug, Copy, Clone)]
    pub struct SourcePosition {
        pub external_file_id: i32,
        pub external_line: i32,
    }    
}

// src/objects/code-inl.h
mod code {
    pub struct Code {
        source_position_table: TrustedByteArray,
    }

    impl Code {
        pub fn has_source_position_table(&self) -> bool {
            true
        }
        pub fn source_position_table(&self) -> TrustedByteArray {
            self.source_position_table.clone()
        }
    }

    #[derive(Debug, Clone)]
    pub struct TrustedByteArray {
        data: Vec<u8>
    }

    impl TrustedByteArray {
        pub fn data(&self) -> &[u8] {
            &self.data
        }
    }
}

// src/snapshot/embedded/embedded-data-inl.h
mod embedded_data {
    use crate::builtins::Builtin;

    pub const K_DEFAULT_EMBEDDED_VARIANT: &str = "default";

    pub struct EmbeddedData {
        // Placeholder for data
    }

    impl EmbeddedData {
        pub fn instruction_size_of(&self, _builtin: Builtin) -> i32 {
            10 // Dummy size
        }

        pub fn padded_instruction_size_of(&self, _builtin: Builtin) -> i32 {
            10
        }
    
        pub fn instruction_start_of(&self, _builtin: Builtin) -> Vec<u8> {
            vec![0u8; 10]
        }

        pub fn get_builtin_id(&self, _index: i32) -> Builtin {
            Builtin::kAbort
        }

        pub fn code_size(&self) -> u32 {
            100
        }

        pub fn data_size(&self) -> u32 {
            100
        }
    }
}

// src/flags/flags.h
mod flags {
    // Placeholder for flags
    pub const ENABLE_CONTROL_FLOW_INTEGRITY_BOOL: bool = false;
}

// src/builtins/builtins.h
mod builtins {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Builtin {
        kAbort,
        kAdd,
        kArgumentsAdaptorTrampoline,
        kArrayConstructor,
        kArrayPrototypeJoin,
        kArrayPrototypePop,
        kArrayPrototypePush,
        kArraySpeciesProtector,
        kAsyncFunctionAwaitResume,
        kAsyncFunctionAwaitReject,
        kAsyncGeneratorAwaitReturn,
        kAsyncGeneratorAwaitYield,
        kAsyncGeneratorReject,
        kAsyncGeneratorResolve,
        kAsyncGeneratorReturn,
        kBigIntConstructor,
        kBigIntAsIntN,
        kBigIntAsUintN,
        kBigIntPrototypeValueOf,
        kBooleanConstructor,
        kBooleanPrototypeValueOf,
        kCall,
        kCallFunction,
        kCallWithSpread,
        kCatch,
        kCeil,
        kCheckBigInt,
        kCheckNumber,
        kCheckSymbol,
        kClassConstructor,
        kCodeEntryReturnIC,
        kCollectGarbage,
        kCompileLazyDeoptimizedCode,
        kCompileOptimized_Concurrent,
        kCompileOptimized_NotConcurrent,
        kConstruct,
        kConstructFunction,
        kConstructWithSpread,
        kContinue,
        kCopyDataProperties,
        kCreateAsyncFromSyncIterator,
        kCreateIterResultObject,
        kCreateObject,
        kDateConstructor,
        kDateNow,
        kDateParse,
        kDatePrototypeGetDate,
        kDatePrototypeGetFullYear,
        kDatePrototypeGetHours,
        kDatePrototypeGetMilliseconds,
        kDatePrototypeGetMinutes,
        kDatePrototypeGetMonth,
        kDatePrototypeGetSeconds,
        kDatePrototypeGetTime,
        kDatePrototypeGetTimezoneOffset,
        kDatePrototypeGetUTCDate,
        kDatePrototypeGetUTCFullYear,
        kDatePrototypeGetUTCHours,
        kDatePrototypeGetUTCMilliseconds,
        kDatePrototypeGetUTCMinutes,
        kDatePrototypeGetUTCMonth,
        kDatePrototypeGetUTCSeconds,
        kDatePrototypeSetDate,
        kDatePrototypeSetFullYear,
        kDatePrototypeSetHours,
        kDatePrototypeSetMilliseconds,
        kDatePrototypeSetMinutes,
        kDatePrototypeSetMonth,
        kDatePrototypeSetSeconds,
        kDatePrototypeSetTime,
        kDatePrototypeSetUTCDate,
        kDatePrototypeSetUTCFullYear,
        kDatePrototypeSetUTCHours,
        kDatePrototypeSetUTCMilliseconds,
        kDatePrototypeSetUTCMinutes,
        kDatePrototypeSetUTCMonth,
        kDatePrototypeSetUTCSeconds,
        kDebugBreakAtEntry,
        kDebugBreakAtExit,
        kDecrement,
        kDefault,
        kDefineClass,
        kDefineGetterSetter,
        kDeleteProperty,
        kDiv,
        kDoWhile,
        kElementsAccessor,
        kEnumerableOwnProperties,
        kErrorConstructor,
        kErrorPrototypeToString,
        kExp,
        kExport,
        kFastArrayConstructor,
        kFastNewClosure,
        kFastNewContext,
        kFastNewObject,
        kFastRuntimeIdCount,
        kFastWeakMapConstructor,
        kFastWeakSetConstructor,
        kFindOrderedHashMapEntry,
        kFindOrderedHashSetEntry,
        kFloor,
        kFor,
        kForInContinue,
        kForInDone,
        kForInNext,
        kForOfNext,
        kFrameArguments,
        kFreeArrayElements,
        kFromNumber,
        kFunctionBind,
        kFunctionConstructor,
        kFunctionPrototypeApply,
        kFunctionPrototypeCall,
        kFunctionPrototypeToString,
        kGeneratorClose,
        kGeneratorNext,
        kGeneratorReturn,
        kGeneratorThrow,
        kGetIterator,
        kGetOwnPropertyKeys,
        kGetProperty,
        kGetPropertyWithReceiver,
        kGetSuperConstructor,
        kGetTemplateObject,
        kGlobalThisGetter,
        kGlobalThisSetter,
        kGreaterThan,
        kGreaterThanOrEqual,
        kHandleDebuggerStatement,
        kHasInPrototypeChain,
        kHasProperty,
        kHeapConstant,
        kHiddenFromConsole,
        kIfSuccess,
        kImport,
        kImportMetaGetter,
        kIncrement,
        kIncumbentRealmGetter,
        kInstanceOf,
        kInternalErrorConstructor,
        kInternalErrorPrototypeToString,
        kIsArray,
        kIsConstructor,
        kIsPrivateKey,
        kIsPropertyKey,
        kIsRegExp,
        kJSReturn,
        kJSToBigInt,
        kJSToBoolean,
        kJSToNumber,
        kJSToObject,
        kJSToString,
        kJSToSymbol,
        kJsonParse,
        kJsonStringify,
        kKeyedDeleteProperty,
        kKeyedGetProperty,
        kKeyedHasProperty,
        kKeyedSetProperty,
        kLessThan,
        kLessThanOrEqual,
        kLoadHandler,
        kLog,
        kLookupIterator,
        kMapConstructor,
        kMapPrototypeGet,
        kMapPrototypeSet,
        kMathAcos,
        kMathAcosh,
        kMathAsin,
        kMathAsinh,
        kMathAtan,
        kMathAtan2,
        kMathAtanh,
        kMathCbrt,
        kMathClz32,
        kMathCos,
        kMathCosh,
        kMathExp,
        kMathExpm1,
        kMathFround,
        kMathHypot,
        kMathImul,
        kMathLog,
        kMathLog1p,
        kMathLog10,
        kMathLog2,
        kMathMax,
        kMathMin,
        kMathPow,
        kMathSign,
        kMathSin,
        kMathSinh,
        kMathSqrt,
        kMathTan,
        kMathTanh,
        kMessageGet,
        kMessageGetRaw,
        kMessagePrototypeToString,
        kMod,
        kModuleNormalizeName,
        kMul,
        kNativeErrorConstructor,
        kNativeErrorPrototypeToString,
        kNegate,
        kNewArguments,
        kNewConsString,
        kNewObject,
        kNewPromiseCapability,
        kNewPromiseResolver,
        kNewRegExp,
        kNewString,
        kNewSymbol,
        kNextMicrotask,
        kNoArgumentConstructor,
        kNormalObjectConstructor,
        kNumberConstructor,
        kNumberPrototypeToFixed,
        kNumberPrototypeToPrecision,
        kNumberPrototypeToString,
        kNumberPrototypeValueOf,
        kObjectAssign,
        kObjectConstructor,
        kObjectEntries,
        kObjectFromEntries,
        kObjectGetOwnPropertyDescriptor,
        kObjectGetOwnPropertyNames,
        kObjectGetOwnPropertySymbols,
        kObjectGetPrototypeOf,
        kObjectIs,
        kObjectKeys,
        kObjectPrototypeHasOwnProperty,
        kObjectPrototypeIsPrototypeOf,
        kObjectPrototypePropertyIsEnumerable,
        kObjectPrototypeToString,
        kObjectSetPrototypeOf,
        kObjectValues,
        kOnFatalError,
        kOrdinaryHasInstance,
        kParseInt,
        kParseJson,
        kPerformPromiseAll,
        kPerformPromiseAny,
        kPerformPromiseRace,
        kPerformPromiseThen,
        kPop,
        kPostDecrement,
        kPostIncrement,
        kPreDecrement,
        kPreIncrement,
        kPromiseAll,
        kPromiseAllSettled,
        kPromiseAny,
        kPromiseConstructor,
        kPromisePrototypeCatch,
        kPromisePrototypeFinally,
        kPromisePrototypeThen,
        kPromiseRace,
        kPromiseReject,
        kPromiseResolve,
        kPromiseStaticsHasSpeciesSymbolGetter,
        kProxyConstructor,
        kProxyRevoke,
        kPush,
        kRangeErrorConstructor,
        kRangeErrorPrototypeToString,
        kReferenceErrorConstructor,
        kReferenceErrorPrototypeToString,
        kRegExpConstructor,
        kRegExpExec,
        kRegExpPrototypeCompile,
        kRegExpPrototypeFlagsGetter,
        kRegExpPrototypeSourceGetter,
        kRegExpPrototypeToString,
        kReject,
        kRemoveArrayHoles,
        kResolve,
        kReturn,
        kSameValue,
        kSameValueZero,
        kSetConstructor,
        kSetFunctionName,
        kSetIntegrityLevel,
        kSetPrototype,
        kSetProperty,
        kSetPropertyWithReceiver,
        kSharedArrayBufferConstructor,
        kSharedArrayBufferPrototypeSlice,
        kShift,
        kSignBit,
        kSlice,
        kSmiLexicographicCompare,
        kSome,
        kSpeciesConstructor,
        kSpeciesCreate,
        kSpreadIterable,
        kStrictArguments,
        kStringCharAt,
        kStringCharCodeAt,
        kStringCodePointAt,
        kStringConstructor,
        kStringFromCharCode,
        kStringFromCodePoint,
        kStringIteratorNext,
        kStringPrototypeAnchor,
        kStringPrototypeBig,
        kStringPrototypeBlink,
        kStringPrototypeBold,
        kStringPrototypeCharAt,
        kStringPrototypeCharCodeAt,
        kStringPrototypeCodePointAt,
        kStringPrototypeConcat,
        kStringPrototypeEndsWith,
        kStringPrototypeFixed,
        kStringPrototypeFontcolor,
        kStringPrototypeFontsize,
        kStringPrototypeIncludes,
        kStringPrototypeIndexOf,
        kStringPrototypeItalics,
        kStringPrototypeLastIndexOf,
        kStringPrototypeLink,
        kStringPrototypeLocaleCompare,
        kStringPrototypeMatch,
        kStringPrototypeMatchAll,
        kStringPrototypeNormalize,
        kStringPrototypePadEnd,
        kStringPrototypePadStart,
        kStringPrototypeRepeat,
        kStringPrototypeReplace,
        kStringPrototypeReplaceAll,
        kStringPrototypeSearch,
        kStringPrototypeSlice,
        kStringPrototypeSmall,
        kStringPrototypeSplit,
        kStringPrototypeStartsWith,
        kStringPrototypeStrike,
        kStringPrototypeSub,
        kStringPrototypeSubstring,
        kStringPrototypeSup,
        kStringPrototypeToLocaleLowerCase,
        kStringPrototypeToLocaleUpperCase,
        kStringPrototypeToLowerCase,
        kStringPrototypeToString,
        kStringPrototypeToUpperCase,
        kStringPrototypeTrim,
        kStringPrototypeTrimEnd,
        kStringPrototypeTrimStart,
        kStringPrototypeValueOf,
        kStringPrototype__InternalStringSlice,
        kStringReplaceNonGlobalRegExpWithReceiver,
        kStringSplitWithSingleCharSeparatorAndLimited,
        kStringSplitWithSingleCharSeparatorAndNoLimit,
        kStringSubstringNonNegativeSafeInteger,
        kStringValueOf,
        kSub,
        kSymbolConstructor,
        kSymbolPrototypeToString,
        kSymbolPrototypeValueOf,
        kSyntaxErrorConstructor,
        kSyntaxErrorPrototypeToString,
        kTestUndetectable,
        kThrow,
        kThrowApplyNonFunction,
        kThrowAsyncIteratorMethodNotCallable,
        kThrowCallerArgumentsAccess,
        kThrowConstructedNonCallable,
        kThrowConstructorReturnedNonObject,
        kThrowCreateUnmappedArguments,
        kThrowDerivedConstructorReturnedNonObject,
        kThrowFromAsyncIteratorNext,
        kThrowGeneratorMethodNotCallable,
        kThrowIllegalInvocation,
        kThrowInvalidAsmJsArguments,
        kThrowIteratorMethodNotCallable,
        kThrowNonCallable,
        kThrowNonConstructor,
        kThrowNonExported,
        kThrowNonInitializable,
        kThrowNonObjectCoercible,
        kThrowNotGeneric,
        kThrowNullOrUndefined,
        kThrowPrivateBrandCheckFailed,
        kThrowRedefinition,
        kThrowSameValue,
        kThrowSpreadArgTooBig,
        kThrowSuperAlreadyCalled,
        kThrowSymbolIteratorInvalid,
        kThrowTypeError,
        kThrowTypeErrorIfStrict