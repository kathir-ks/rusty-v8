// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::{
    collections::HashSet,
    fmt::{self, Write},
    io::{self, Cursor, Read},
    mem::MaybeUninit,
    num::NonZeroU32,
    ops::Range,
    string::FromUtf8Error,
};

// Placeholder for v8::debug::DisassemblyCollector. Requires custom implementation.
pub struct DisassemblyCollector {
    lines: Vec<(String, usize)>,
}

impl DisassemblyCollector {
    pub fn new() -> Self {
        DisassemblyCollector { lines: Vec::new() }
    }

    pub fn reserve_line_count(&mut self, count: usize) {
        self.lines.reserve(count);
    }

    pub fn add_line(&mut self, data: &str, _len: usize, bytecode_offset: usize) {
        self.lines.push((data.to_string(), bytecode_offset));
    }
}

// Placeholder for v8_flags.wasm_disassembly_max_mb. Requires custom implementation.
const WASM_DISASSEMBLY_MAX_MB: usize = 32; // Or any appropriate default value.

// Placeholder for AccountingAllocator. Requires custom implementation.
pub struct AccountingAllocator {}

impl AccountingAllocator {
    pub fn new() -> Self {
        AccountingAllocator {}
    }
}

// Placeholder for Zone. Requires custom implementation.
pub struct Zone<'a> {
    _allocator: &'a AccountingAllocator,
    _name: &'static str,
}

impl<'a> Zone<'a> {
    pub fn new(allocator: &'a AccountingAllocator, name: &'static str) -> Self {
        Zone {
            _allocator: allocator,
            _name: name,
        }
    }
}

// Placeholder for WasmModule. Requires custom implementation.
pub struct WasmModule {
    pub functions: Vec<WasmFunction>,
    pub num_imported_functions: u32,
    pub types: Vec<TypeDefinition>,
    pub import_table: Vec<WasmImport>,
    pub tables: Vec<WasmTable>,
    pub memories: Vec<WasmMemory>,
    pub tags: Vec<WasmTag>,
    pub globals: Vec<WasmGlobal>,
    pub elem_segments: Vec<WasmElemSegment>,
    pub data_segments: Vec<WasmDataSegment>,
    pub start_function_index: i32,
    pub name: WireBytesRef,
    pub stringref_literals: Vec<WasmStringRefLiteral>,
}

impl WasmModule {
    pub fn type_at(&self, index: usize) -> &TypeDefinition {
        &self.types[index]
    }

    pub fn type_(&self, index: ModuleTypeIndex) -> &TypeDefinition {
        &self.types[index.index as usize]
    }

    pub fn has_signature(&self, index: ModuleTypeIndex) -> bool {
        index.index < self.types.len() as u32
    }

    pub fn signature(&self, index: ModuleTypeIndex) -> &FunctionSig {
        self.types[index.index as usize].function_sig
    }
}

// Placeholder for WasmFunction. Requires custom implementation.
pub struct WasmFunction {
    pub sig_index: ModuleTypeIndex,
    pub code: FunctionBody,
    pub exported: bool,
}

// Placeholder for WasmImport. Requires custom implementation.
#[derive(Debug)]
pub struct WasmImport {
    pub kind: ImportExportKindCode,
    pub module_name: WireBytesRef,
    pub field_name: WireBytesRef,
    pub index: u32,
}

// Placeholder for WasmTable. Requires custom implementation.
#[derive(Debug)]
pub struct WasmTable {
    pub initial_size: u32,
    pub maximum_size: u32,
    pub has_maximum_size: bool,
    pub exported: bool,
    pub imported: bool,
    pub type_: ValueType,
    pub shared: bool,
}

// Placeholder for WasmMemory. Requires custom implementation.
#[derive(Debug)]
pub struct WasmMemory {
    pub initial_pages: u32,
    pub maximum_pages: u32,
    pub has_maximum_pages: bool,
    pub exported: bool,
    pub imported: bool,
    pub is_shared: bool,
}

impl WasmMemory {
    pub fn is_memory64(&self) -> bool {
        false // Replace with correct logic.
    }
}

// Placeholder for WasmGlobal. Requires custom implementation.
#[derive(Debug)]
pub struct WasmGlobal {
    pub mutability: bool,
    pub type_: ValueType,
    pub exported: bool,
    pub imported: bool,
    pub init: ConstantExpression,
    pub shared: bool,
}

// Placeholder for WasmElemSegment. Requires custom implementation.
#[derive(Debug)]
pub struct WasmElemSegment {
    pub status: ElemSegmentStatus,
    pub table_index: u32,
    pub offset: ConstantExpression,
    pub type: ValueType,
    pub element_count: usize,
    pub elements_wire_bytes_offset: u32,
    pub shared: bool,
}

// Placeholder for WasmDataSegment. Requires custom implementation.
#[derive(Debug)]
pub struct WasmDataSegment {
    pub active: bool,
    pub memory_index: u32,
    pub dest_addr: ConstantExpression,
    pub source: WireBytesRef,
    pub shared: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ElemSegmentStatus {
    kStatusActive,
    kStatusDeclarative,
}

// Placeholder for WasmTag. Requires custom implementation.
#[derive(Debug)]
pub struct WasmTag {
    pub sig: FunctionSig,
}

// Placeholder for WasmStringRefLiteral. Requires custom implementation.
#[derive(Debug, Copy, Clone)]
pub struct WasmStringRefLiteral {
    pub source: WireBytesRef,
}

// Placeholder for WireBytesRef. Requires custom implementation.
#[derive(Debug, Copy, Clone)]
pub struct WireBytesRef {
    offset_: u32,
    length_: u32,
}

impl WireBytesRef {
    pub fn new(offset: u32, length: u32) -> Self {
        WireBytesRef {
            offset_: offset,
            length_: length,
        }
    }
    pub fn offset(&self) -> u32 {
        self.offset_
    }
    pub fn end_offset(&self) -> u32 {
        self.offset_ + self.length_
    }
    pub fn length(&self) -> u32 {
        self.length_
    }
}

// Placeholder for FunctionBody. Requires custom implementation.
pub struct FunctionBody {
    offset_: u32,
    length_: u32,
}

impl FunctionBody {
    pub fn new(offset: u32, length: u32) -> Self {
        FunctionBody {
            offset_: offset,
            length_: length,
        }
    }
    pub fn offset(&self) -> u32 {
        self.offset_
    }
    pub fn length(&self) -> u32 {
        self.length_
    }
}

// Placeholder for FunctionSig. Requires custom implementation.
#[derive(Debug)]
pub struct FunctionSig {
    params: Vec<ValueType>,
    returns: Vec<ValueType>,
    pub is_shared: bool,
}

impl FunctionSig {
    pub fn new(params: Vec<ValueType>, returns: Vec<ValueType>, is_shared: bool) -> Self {
        FunctionSig {
            params,
            returns,
            is_shared,
        }
    }

    pub fn parameter_count(&self) -> u32 {
        self.params.len() as u32
    }

    pub fn return_count(&self) -> usize {
        self.returns.len()
    }

    pub fn get_param(&self, i: u32) -> ValueType {
        self.params[i as usize]
    }

    pub fn get_return(&self, i: usize) -> ValueType {
        self.returns[i]
    }
}

// Placeholder for ModuleTypeIndex. Requires custom implementation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ModuleTypeIndex {
    pub index: u32,
}

// Placeholder for ValueType. Requires custom implementation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct ValueType {
    kind: ValueTypeKind,
}

impl ValueType {
    pub const I32: Self = ValueType {
        kind: ValueTypeKind::I32,
    };
    pub const I64: Self = ValueType {
        kind: ValueTypeKind::I64,
    };
    pub fn RefMaybeNull(heap_type: HeapType, nullable: Nullable) -> Self {
        ValueType {
            kind: ValueTypeKind::Ref(heap_type, nullable),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum ValueTypeKind {
    I32,
    I64,
    F32,
    F64,
    Ref(HeapType, Nullable),
}

// Placeholder for HeapType. Requires custom implementation.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub struct HeapType {
    kind: HeapTypeKind,
}

impl HeapType {
    pub fn Index(index: u32, is_shared: bool, kind: RefTypeKind) -> Self {
        HeapType {
            kind: HeapTypeKind::Index(index, is_shared, kind),
        }
    }

    pub fn is_index(&self) -> bool {
        matches!(self.kind, HeapTypeKind::Index(_, _, _))
    }

    pub fn ref_index(&self) -> u32 {
        match self.kind {
            HeapTypeKind::Index(index, _, _) => index,
            _ => panic!("Not an index type"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum HeapTypeKind {
    Func,
    Any,
    None,
    Eq,
    Struct,
    Array,
    Data,
    I31,
    String,
    Extern,
    Concrete(u32), // For type indices
    Index(u32, bool, RefTypeKind), // type index, is_shared, RefTypeKind
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Nullable {
    kNullable,
    kNonNullable,
}

// Placeholder for RefTypeKind
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RefTypeKind {
    kRefOther,
    kRefI31,
    kRefString,
}

// Placeholder for TypeDefinition. Requires custom implementation.
#[derive(Debug)]
pub struct TypeDefinition {
    pub supertype: u32,
    pub is_final: bool,
    pub kind: TypeDefinitionKind,
    pub array_type: *const ArrayType,
    pub struct_type: *const StructType,
    pub function_sig: *const FunctionSig,
    pub is_shared: bool,
}

#[derive(Debug, PartialEq)]
pub enum TypeDefinitionKind {
    Function,
    Struct,
    Array,
}

// Placeholder for StructType. Requires custom implementation.
#[derive(Debug)]
pub struct StructType {
    fields: Vec<(ValueType, bool)>, // (type, mutability)
}

impl StructType {
    pub fn new(fields: Vec<(ValueType, bool)>) -> Self {
        StructType { fields }
    }

    pub fn field_count(&self) -> u32 {
        self.fields.len() as u32
    }

    pub fn field(&self, i: u32) -> ValueType {
        self.fields[i as usize].0
    }

    pub fn mutability(&self, i: u32) -> bool {
        self.fields[i as usize].1
    }
}

// Placeholder for ArrayType. Requires custom implementation.
#[derive(Debug)]
pub struct ArrayType {
    element_type: ValueType,
    mutability: bool,
}

impl ArrayType {
    pub fn new(element_type: ValueType, mutability: bool) -> Self {
        ArrayType {
            element_type,
            mutability,
        }
    }

    pub fn element_type(&self) -> ValueType {
        self.element_type
    }

    pub fn mutability(&self) -> bool {
        self.mutability
    }
}

// Placeholder for ConstantExpression. Requires custom implementation.
#[derive(Debug, Copy, Clone)]
pub enum ConstantExpression {
    Empty,
    I32Const(i32),
    RefNull(HeapType),
    RefFunc(u32),
    WireBytesRef(WireBytesRef),
}

impl ConstantExpression {
    pub fn kind(&self) -> ConstantExpressionKind {
        match self {
            ConstantExpression::Empty => ConstantExpressionKind::kEmpty,
            ConstantExpression::I32Const(_) => ConstantExpressionKind::kI32Const,
            ConstantExpression::RefNull(_) => ConstantExpressionKind::kRefNull,
            ConstantExpression::RefFunc(_) => ConstantExpressionKind::kRefFunc,
            ConstantExpression::WireBytesRef(_) => ConstantExpressionKind::kWireBytesRef,
        }
    }

    pub fn i32_value(&self) -> i32 {
        match self {
            ConstantExpression::I32Const(value) => *value,
            _ => panic!("Not an I32Const expression"),
        }
    }

    pub fn index(&self) -> u32 {
        match self {
            ConstantExpression::RefFunc(index) => *index,
            _ => panic!("Not a RefFunc expression"),
        }
    }

    pub fn type_(&self) -> HeapType {
        match self {
            ConstantExpression::RefNull(heap_type) => *heap_type,
            _ => panic!("Not a RefNull expression"),
        }
    }

    pub fn wire_bytes_ref(&self) -> WireBytesRef {
        match self {
            ConstantExpression::WireBytesRef(wire_bytes_ref) => *wire_bytes_ref,
            _ => panic!("Not a WireBytesRef expression"),
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ConstantExpressionKind {
    kEmpty,
    kI32Const,
    kRefNull,
    kRefFunc,
    kWireBytesRef,
}

// Placeholder for ImportExportKindCode. Requires custom implementation.
#[derive(Debug, Copy, Clone, PartialEq)]
pub enum ImportExportKindCode {
    kExternalTable,
    kExternalFunction,
    kExternalGlobal,
    kExternalMemory,
    kExternalTag,
}

// Placeholder for ModuleWireBytes. Requires custom implementation.
pub struct ModuleWireBytes<'a> {
    bytes: &'a [u8],
}

impl<'a> ModuleWireBytes<'a> {
    pub fn new(bytes: &'a [u8]) -> Self {
        ModuleWireBytes { bytes }
    }

    pub fn module_bytes(&self) -> &'a [u8] {
        self.bytes
    }

    pub fn get_function_bytes(&self, func: &WasmFunction) -> &'a [u8] {
        &self.bytes[func.code.offset() as usize..(func.code.offset() + func.code.length()) as usize]
    }

    pub fn start(&self) -> *const u8 {
        self.bytes.as_ptr()
    }

    pub fn length(&self) -> usize {
        self.bytes.len()
    }
}

// Placeholder for NamesProvider. Requires custom implementation.
pub struct NamesProvider<'a> {
    module: &'a WasmModule,
    wire_bytes: &'a [u8],
}

impl<'a> NamesProvider<'a> {
    pub const kDevTools: u32 = 0; // Replace with appropriate value.

    pub fn new(module: &'a WasmModule, wire_bytes: &'a [u8]) -> Self {
        NamesProvider { module, wire_bytes }
    }

    pub fn print_function_name<W: Write>(
        &self,
        out: &mut W,
        index: u32,
        _dev_tools: u32,
        indices_as_comments: IndexAsComment,
    ) {
        write!(out, "$func{}", index).unwrap();
        if indices_as_comments == IndexAsComment::kIndicesAsComments {
            write!(out, " (func {})", index).unwrap();
        }
    }

    pub fn print_local_name<W: Write>(&self, out: &mut W, func_index: u32, index: u32) {
        write!(out, "$local{}@func{}", index, func_index).unwrap();
    }

    pub fn print_value_type<W: Write>(&self, out: &mut W, value_type: ValueType) {
        match value_type.kind {
            ValueTypeKind::I32 => write!(out, "i32").unwrap(),
            ValueTypeKind::I64 => write!(out, "i64").unwrap(),
            ValueTypeKind::F32 => write!(out, "f32").unwrap(),
            ValueTypeKind::F64 => write!(out, "f64").unwrap(),
            ValueTypeKind::Ref(heap_type, nullable) => {
                write!(out, "(ref ").unwrap();
                self.print_heap_type(out, heap_type);
                match nullable {
                    Nullable::kNullable => write!(out, " null").unwrap(),
                    Nullable::kNonNullable => {}
                };
                write!(out, ")").unwrap()
            }
        }
    }

    pub fn print_global_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$global{}", index).unwrap();
    }

    pub fn print_type_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$type{}", index).unwrap();
    }

    pub fn print_table_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$table{}", index).unwrap();
    }

    pub fn print_memory_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$memory{}", index).unwrap();
    }

    pub fn print_data_segment_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$data{}", index).unwrap();
    }

    pub fn print_element_segment_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$elem{}", index).unwrap();
    }

    pub fn print_tag_name<W: Write>(&self, out: &mut W, index: u32) {
        write!(out, "$tag{}", index).unwrap();
    }

    pub fn print_heap_type<W: Write>(&self, out: &mut W, heap_type: HeapType) {
        match heap_type.kind {
            HeapTypeKind::Func => write!(out, "func").unwrap(),
            HeapTypeKind::Any => write!(out, "any").unwrap(),
            HeapTypeKind::None => write!(out, "none").unwrap(),
            HeapTypeKind::Eq => write!(out, "eq").unwrap(),
            HeapTypeKind::Struct => write!(out, "struct").unwrap(),
            HeapTypeKind::Array => write!(out, "array").unwrap(),
            HeapTypeKind::Data => write!(out, "data").unwrap(),
            HeapTypeKind::I31 => write!(out, "i31").unwrap(),
            HeapTypeKind::String => write!(out, "string").unwrap(),
            HeapTypeKind::Extern => write!(out, "extern").unwrap(),
            HeapTypeKind::Concrete(index) => write!(out, "type {}", index).unwrap(),
            HeapTypeKind::Index(index, is_shared, kind) => {
                write!(out, "index {} (shared: {}, kind: {:?})", index, is_shared, kind).unwrap()
            }
        }
    }

    pub fn print_field_name<W: Write>(&self, out: &mut W, struct_index: u32, field_index: u32) {
        write!(out, "$field{}@struct{}", field_index, struct_index).unwrap();
    }

    pub fn print_label_name<W: Write>(
        &self,
        out: &mut W,
        func_index: u32,
        name_section_index: u32,
        label_index: u32,
    ) {
        write!(
            out,
            "$label{}@func{}@name_section{}",
            label_index, func_index, name_section_index
        )
        .unwrap();
    }
}

// Placeholder for WasmError. Requires custom implementation.
#[derive(Debug)]
pub struct WasmError {
    message: String,
    offset: usize,
}

impl WasmError {
    pub fn new(message: String, offset: usize) -> Self {
        WasmError { message, offset }
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn offset(&self) -> usize {
        self.offset
    }
}

// Placeholder for ModuleResult. Requires custom implementation.
pub struct ModuleResult {
    module: Option<Box<WasmModule>>,
    error: Option<WasmError>,
}

impl ModuleResult {
    pub fn ok(module: Box<WasmModule>) -> Self {
        ModuleResult {
            module: Some(module),
            error: None,
        }
    }

    pub fn err(error: WasmError) -> Self {
        ModuleResult {
            module: None,
            error: Some(error),
        }
    }

    pub fn failed(&self) -> bool {
        self.error.is_some()
    }

    pub fn error(&self) -> &WasmError {
        self.error.as_ref().unwrap()
    }

    pub fn value(self) -> Box<WasmModule> {
        self.module.unwrap()
    }
}

// Placeholder for DecodeWasmModuleForDisassembler. Requires custom implementation.
fn decode_wasm_module_for_disassembler(
    wire_bytes: &[u8],
    offsets: &mut OffsetsProvider,
) -> ModuleResult {
    // Dummy implementation for the sake of compilation
    let mut module = WasmModule {
        functions: Vec::new(),
        num_imported_functions: 0,
        types: Vec::new(),
        import_table: Vec::new(),
        tables: Vec::new(),
        memories: Vec::new(),
        tags: Vec::new(),
        globals: Vec::new(),
        elem_segments: Vec::new(),
        data_segments: Vec::new(),
        start_function_index: -1,
        name: WireBytesRef::new(0, 0),
        stringref_literals: Vec::new(),
    };

    offsets.collect_offsets(&module, wire_bytes);
    ModuleResult::ok(Box::new(module))
}

// Placeholder for AllocateOffsetsProvider. Requires custom implementation.
fn allocate_offsets_provider() -> Box<OffsetsProvider> {
    Box::new(OffsetsProvider::new())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum IndexAsComment {
    kNoIndicesAsComments,
    kIndicesAsComments,
}

// Public interface.
pub fn disassemble_module(
    module: &WasmModule,
    wire_bytes: ModuleWireBytes,
    names: &mut NamesProvider,
    collector: &mut DisassemblyCollector,
    function_body_offsets: &mut Vec<i32>,
) {
    let mut out = MultiLineStringBuilder::new();
    let allocator = AccountingAllocator::new();
    let mut md = ModuleDisassembler::new(
        &mut out,
        module,
        names,
        wire_bytes,
        &allocator,
        None, // offsets
        Some(function_body_offsets),
    );
    md.print_module(Indentation::new(0, 2), WASM_DISASSEMBLY_MAX_MB);
    out.to_disassembly_collector(collector);
}

pub fn disassemble_bytes(
    wire_bytes: &[u8],
    collector: &mut DisassemblyCollector,
    function_body_offsets: &mut Vec<i32>,
) {
    let mut offsets = allocate_offsets_provider();
    let result = decode_wasm_module_for_disassembler(wire_bytes, &mut offsets);

    let mut out = MultiLineStringBuilder::new();
    let allocator = AccountingAllocator::new();
    match result {
        ModuleResult {
            module: None,
            error: Some(error),
        } => {
            write!(
                out,
                "Decoding error: {} at offset {}",
                error.message(),
                error.offset()
            )
            .unwrap();
            out.to_disassembly_collector(collector);
            return;
        }
        ModuleResult {
            module: Some(module),
            error: None,
        } => {
            let mut names = NamesProvider::new(&module, wire_bytes);
            let module_bytes = ModuleWireBytes::new(wire_bytes);

            let mut md = ModuleDisassembler::new(
                &mut out,
                &module,
                &mut names,
                module_bytes,
                &allocator,
                Some(offsets), //offsets
                Some(function_body_offsets),
            );
            md.print_module(Indentation::new(0, 2), WASM_DISASSEMBLY_MAX_MB);
            out.to_disassembly_collector(collector);
        }
        _ => {
            panic!("Invalid ModuleResult State")
        }
    };
}

// MultiLineStringBuilder implementation.
#[derive(Debug)]
struct MultiLineStringBuilder {
    lines: Vec<Line>,
    current_line: String,
    current_line_bytecode_offset: usize,
}

#[derive(Debug, Clone)]
struct Line {
    data: String,
    len: usize,
    bytecode_offset: usize,
}

impl MultiLineStringBuilder {
    fn new() -> Self {
        MultiLineStringBuilder {
            lines: Vec::new(),
            current_line: String::new(),
            current_line_bytecode_offset: 0,
        }
    }

    fn next_line(&mut self, bytecode_offset: usize) {
        if !self.current_line.is_empty() {
            self.lines.push(Line {
                data: self.current_line.clone(),
                len: self.current_line.len() + 1,
                bytecode_offset: self.current_line_bytecode_offset,
            });
            self.current_line.clear();
        }
        self.current_line_bytecode_offset = bytecode_offset;
    }

    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current_line.push_str(s);
        Ok(())
    }

    fn to_disassembly_collector(&mut self, collector: &mut DisassemblyCollector) {
        if !self.current_line.is_empty() {
            self.next_line(0); // Finalize the last line.
        }
        collector.reserve_line_count(self.lines.len());
        for line in &self.lines {
            collector.add_line(&line.data, line.len, line.bytecode_offset);
        }
    }

    fn length(&self) -> usize {
        self.lines.iter().map(|l| l.len).sum()
    }

    fn set_current_line_bytecode_offset(&mut self, offset: usize) {
        self.current_line_bytecode_offset = offset;
    }

    fn current_line_bytecode_offset(&self) -> usize {
        self.current_line_bytecode_offset
    }

    fn approximate_size_mb(&self) -> f64 {
        (self.length() as f64) / (1024.0 * 1024.0)
    }

    fn patch_label(&mut self, label_info: &LabelInfo, label_start: *const u8) {
        // Find the line and patch it
        for line in &mut self.lines {
            // Check if the label start address falls within the line's data
            if line.data.as_ptr() as *const u8 <= label_start
                && label_start < (line.data.as_ptr() as *const u8).wrapping_add(line.data.len())
            {
                // Calculate the offset within the line
                let offset = unsafe { label_start.offset_from(line.data.as_ptr() as *const u8) };
                if offset >= 0 {
                    // Overwrite the label in the line's data
                    let start_index = offset as usize;
                    let end_index = start_index + label_info.length as usize;
                    let replacement = String::from_utf8(
                        (0..label_info.length)
                            .map(|_| b' ')
                            .collect::<Vec<u8>>(),
                    )
                    .unwrap(); // Create spaces to maintain length.
                    line.data.replace_range(start_index..end_index, &replacement);
                    return; // Exit after patching.
                }
            }
        }
    }
}

impl Write for MultiLineStringBuilder {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.current_line.push_str(s);
        Ok(())
    }
}

// DisassembleFunctionImpl implementation.
fn disassemble_function_impl<W: Write>(
    module: &WasmModule,
    func_index: i32,
    function_body: &[u8],
    module_bytes: ModuleWireBytes,
    names: &mut NamesProvider,
    os: &mut W,
    offsets: Option<&mut Vec<u32>>,
) {
    let mut sb = MultiLineStringBuilder::new();
    let func = &module.functions[func_index as usize];
    let allocator = AccountingAllocator::new();
    let zone = Zone::new(&allocator, "Wasm disassembler");
    let shared = module.type_(func.sig_index).is_shared;
    let mut detected = WasmDetectedFeatures::new();
    let mut d = FunctionBodyDisassembler::new(
        &zone,
        module,
        func_index as u32,
        shared,
        &mut detected,
        &func.sig,
        function_body,
        func.code.offset() as usize,
        module_bytes,
        names,
    );

    d.decode_as_wat(&mut sb, Indentation::new(0, 2), FunctionHeader::kPrintHeader);
    let print_offsets = false;
    sb.write_to(os, print_offsets, offsets);
}

// DisassembleFunction implementation.
pub fn disassemble_function<W: Write>(
    module: &WasmModule,
    func_index: i32,
    wire_bytes: &[u8],
    names: &mut NamesProvider,
    os: &mut W,
) {
    assert!(
        func_index < module.functions.len() as i32
            && func_index >= module.num_imported_functions as i32
    );
    let module_bytes = ModuleWireBytes::new(wire_bytes);
    let code =
        &module_bytes.bytes[module.functions[func_index as usize].code.offset() as usize..];
    let mut collect_offsets: Option<Vec<u32>> = None;
    disassemble_function_impl(
        module,
        func_index,
        code,
        module_bytes,
        names,
        os,
        collect_offsets.as_mut(),
    );
}

// DisassembleFunction overload implementation.
pub fn disassemble_function_with_offsets<W: Write>(
    module: &WasmModule,
    func_index: i32,
    function_body: &[u8],
    maybe_wire_bytes: &[u8],
    function_body_offset: u32,
    os: &mut W,
    offsets: &mut Vec<u32>,
) {
    assert!(
        func_index < module.functions.len() as i32
            && func_index >= module.num_imported_functions as i32
    );
    let mut fake_names = NamesProvider::new(module, maybe_wire_bytes);
    disassemble_function_impl(
        module,
        func_index,
        function_body,
        ModuleWireBytes::new(&[]),
        &mut fake_names,
        os,
        Some(offsets),
    );
}

// Helpers.
const HEX_CHARS: &[u8] = b"0123456789abcdef";
const UPPER_HEX_CHARS: &[u8] = b"0123456789ABCDEF";

// Returns the log2 of the alignment, e.g. "4" means 2<<4 == 16 bytes.
// This is the same format as used in .wasm binary modules.
fn get_default_alignment(opcode: WasmOpcode) -> u32 {
    match opcode {
        WasmOpcode::kExprS128LoadMem | WasmOpcode::kExprS128StoreMem => 4,
        WasmOpcode::kExprS128Load8x8