// Copyright 2022 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::str;
use std::fmt;

//use crate::strings::unicode_decoder::Utf8Decoder;  // Assuming this is available in Rust
use crate::wasm::canonical_types::*;
use crate::wasm::module_decoder::*;
//use crate::wasm::std_object_sizes::*; // Not needed as struct sizes are handled by Rust
use crate::wasm::string_builder::StringBuilder;
use crate::wasm::wasm_code_manager::*;
use crate::wasm::wasm_engine::*;

pub struct NamesProvider<'a> {
    module_: &'a WasmModule,
    wire_bytes_: Vec<u8>,
    mutex_: Mutex<NamesProviderInternal>,
}

struct NamesProviderInternal {
    has_decoded_: bool,
    has_computed_function_import_names_: bool,
    has_computed_import_names_: bool,
    name_section_names_: Option<DecodedNameSection>,
    import_export_function_names_: HashMap<u32, String>,
    import_export_table_names_: HashMap<u32, String>,
    import_export_memory_names_: HashMap<u32, String>,
    import_export_global_names_: HashMap<u32, String>,
    import_export_tag_names_: HashMap<u32, String>,
}

impl<'a> NamesProvider<'a> {
    pub fn new(module_: &'a WasmModule, wire_bytes_: Vec<u8>) -> Self {
        NamesProvider {
            module_,
            wire_bytes_,
            mutex_: Mutex::new(NamesProviderInternal {
                has_decoded_: false,
                has_computed_function_import_names_: false,
                has_computed_import_names_: false,
                name_section_names_: None,
                import_export_function_names_: HashMap::new(),
                import_export_table_names_: HashMap::new(),
                import_export_memory_names_: HashMap::new(),
                import_export_global_names_: HashMap::new(),
                import_export_tag_names_: HashMap::new(),
            }),
        }
    }

    pub fn decode_names_if_not_yet_done(&self) {
        let mut guard = self.mutex_.lock().unwrap();
        if guard.has_decoded_ {
            return;
        }
        guard.has_decoded_ = true;
        guard.name_section_names_ = Some(DecodedNameSection::new(self.wire_bytes_.clone(), self.module_.name_section.clone()));
        self.compute_names_from_imports_exports();
    }

    pub fn compute_function_names_from_imports_exports(&self) {
        let mut guard = self.mutex_.lock().unwrap();
        assert!(!guard.has_computed_function_import_names_);
        guard.has_computed_function_import_names_ = true;

        if self.wire_bytes_.is_empty() {
            return;
        }

        for import in &self.module_.import_table {
            if import.kind != ExternalKind::Function {
                continue;
            }
            if self.module_.lazily_generated_names.has(import.index) {
                continue;
            }
            self.compute_import_name(import, &mut guard.import_export_function_names_);
        }

        for ex in &self.module_.export_table {
            if ex.kind != ExternalKind::Function {
                continue;
            }
            if self.module_.lazily_generated_names.has(ex.index) {
                continue;
            }
            self.compute_export_name(ex, &mut guard.import_export_function_names_);
        }
    }

    pub fn compute_names_from_imports_exports(&self) {
        let mut guard = self.mutex_.lock().unwrap();
        assert!(!guard.has_computed_import_names_);
        guard.has_computed_import_names_ = true;

        if self.wire_bytes_.is_empty() {
            return;
        }

        assert!(guard.has_decoded_);

        let name_section_names = guard.name_section_names_.as_ref().unwrap();

        for import in &self.module_.import_table {
            match import.kind {
                ExternalKind::Function => continue, // Functions are handled separately.
                ExternalKind::Table => {
                    if name_section_names.table_names_.has(import.index) {
                        continue;
                    }
                    self.compute_import_name(import, &mut guard.import_export_table_names_);
                }
                ExternalKind::Memory => {
                    if name_section_names.memory_names_.has(import.index) {
                        continue;
                    }
                    self.compute_import_name(import, &mut guard.import_export_memory_names_);
                }
                ExternalKind::Global => {
                    if name_section_names.global_names_.has(import.index) {
                        continue;
                    }
                    self.compute_import_name(import, &mut guard.import_export_global_names_);
                }
                ExternalKind::Tag => {
                    if name_section_names.tag_names_.has(import.index) {
                        continue;
                    }
                    self.compute_import_name(import, &mut guard.import_export_tag_names_);
                }
            }
        }

        for ex in &self.module_.export_table {
            match ex.kind {
                ExternalKind::Function => continue, // Functions are handled separately.
                ExternalKind::Table => {
                    if name_section_names.table_names_.has(ex.index) {
                        continue;
                    }
                    self.compute_export_name(ex, &mut guard.import_export_table_names_);
                }
                ExternalKind::Memory => {
                    if name_section_names.memory_names_.has(ex.index) {
                        continue;
                    }
                    self.compute_export_name(ex, &mut guard.import_export_memory_names_);
                }
                ExternalKind::Global => {
                    if name_section_names.global_names_.has(ex.index) {
                        continue;
                    }
                    self.compute_export_name(ex, &mut guard.import_export_global_names_);
                }
                ExternalKind::Tag => {
                    if name_section_names.tag_names_.has(ex.index) {
                        continue;
                    }
                    self.compute_export_name(ex, &mut guard.import_export_tag_names_);
                }
            }
        }
    }

    fn compute_import_name(&self, import: &WasmImport, target: &mut HashMap<u32, String>) {
        let mod_start = import.module_name.offset as usize;
        let mod_length = import.module_name.length as usize;
        let field_start = import.field_name.offset as usize;
        let field_length = import.field_name.length as usize;
        let mut buffer = StringBuilder::new();
        buffer.add_char('$');
        sanitize_unicode_name(&mut buffer, &self.wire_bytes_[mod_start..mod_start + mod_length]);
        buffer.add_char('.');
        sanitize_unicode_name(&mut buffer, &self.wire_bytes_[field_start..field_start + field_length]);
        target.insert(import.index, buffer.to_string());
    }

    fn compute_export_name(&self, ex: &WasmExport, target: &mut HashMap<u32, String>) {
        if target.contains_key(&ex.index) {
            return;
        }
        let length = ex.name.length as usize;
        if length == 0 {
            return;
        }
        let mut buffer = StringBuilder::new();
        buffer.add_char('$');
        let offset = ex.name.offset as usize;
        sanitize_unicode_name(&mut buffer, &self.wire_bytes_[offset..offset + length]);
        target.insert(ex.index, buffer.to_string());
    }

    pub fn print_function_name(&self, out: &mut StringBuilder, function_index: u32, behavior: FunctionNamesBehavior, index_as_comment: IndexAsComment) {
        let ref_ = self.module_.lazily_generated_names.lookup_function_name(ModuleWireBytes(&self.wire_bytes_), function_index);
        if ref_.is_set() {
            if behavior == FunctionNamesBehavior::DevTools {
                out.add_char('$');
                self.write_ref(out, &ref_);
                maybe_add_comment(out, function_index, index_as_comment);
            } else {
                self.write_ref(out, &ref_);
            }
            return;
        }

        if behavior == FunctionNamesBehavior::WasmInternal {
            return;
        }

        let mut guard = self.mutex_.lock().unwrap();
        if !guard.has_computed_function_import_names_ {
            self.compute_function_names_from_imports_exports();
        }

        if let Some(name) = guard.import_export_function_names_.get(&function_index) {
            out.add_str(name);
            maybe_add_comment(out, function_index, index_as_comment);
        } else {
            out.add_str(&format!("$func{}", function_index));
        }
    }

    fn write_ref(&self, out: &mut StringBuilder, ref_: &WireBytesRef) {
        let offset = ref_.offset as usize;
        let length = ref_.length as usize;
        out.add_slice(&self.wire_bytes_[offset..offset + length]);
    }

    pub fn print_local_name(&self, out: &mut StringBuilder, function_index: u32, local_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().local_names_, function_index, local_index);

        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            maybe_add_comment(out, local_index, index_as_comment);
        } else {
            out.add_str(&format!("$var{}", local_index));
        }
    }

    pub fn print_label_name(&self, out: &mut StringBuilder, function_index: u32, label_index: u32, fallback_index: u32) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().label_names_, function_index, label_index);

        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
        } else {
            out.add_str(&format!("$label{}", fallback_index));
        }
    }

    pub fn print_type_name(&self, out: &mut StringBuilder, type_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().type_names_, type_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, type_index, index_as_comment);
        }
        out.add_str(&format!("$type{}", type_index));
    }

    pub fn print_table_name(&self, out: &mut StringBuilder, table_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().table_names_, table_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, table_index, index_as_comment);
        }

        if let Some(name) = guard.import_export_table_names_.get(&table_index) {
            out.add_str(name);
            return maybe_add_comment(out, table_index, index_as_comment);
        }
        out.add_str(&format!("$table{}", table_index));
    }

    pub fn print_memory_name(&self, out: &mut StringBuilder, memory_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().memory_names_, memory_index);

        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, memory_index, index_as_comment);
        }

        if let Some(name) = guard.import_export_memory_names_.get(&memory_index) {
            out.add_str(name);
            return maybe_add_comment(out, memory_index, index_as_comment);
        }

        out.add_str(&format!("$memory{}", memory_index));
    }

    pub fn print_global_name(&self, out: &mut StringBuilder, global_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().global_names_, global_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, global_index, index_as_comment);
        }

        if let Some(name) = guard.import_export_global_names_.get(&global_index) {
            out.add_str(name);
            return maybe_add_comment(out, global_index, index_as_comment);
        }

        out.add_str(&format!("$global{}", global_index));
    }

    pub fn print_element_segment_name(&self, out: &mut StringBuilder, element_segment_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().element_segment_names_, element_segment_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            maybe_add_comment(out, element_segment_index, index_as_comment);
        } else {
            out.add_str(&format!("$elem{}", element_segment_index));
        }
    }

    pub fn print_data_segment_name(&self, out: &mut StringBuilder, data_segment_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().data_segment_names_, data_segment_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            maybe_add_comment(out, data_segment_index, index_as_comment);
        } else {
            out.add_str(&format!("$data{}", data_segment_index));
        }
    }

    pub fn print_field_name(&self, out: &mut StringBuilder, struct_index: u32, field_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().field_names_, struct_index, field_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, field_index, index_as_comment);
        }
        out.add_str(&format!("$field{}", field_index));
    }

    pub fn print_tag_name(&self, out: &mut StringBuilder, tag_index: u32, index_as_comment: IndexAsComment) {
        self.decode_names_if_not_yet_done();
        let guard = self.mutex_.lock().unwrap();
        let ref_ = get(&guard.name_section_names_.as_ref().unwrap().tag_names_, tag_index);
        if ref_.is_set() {
            out.add_char('$');
            self.write_ref(out, &ref_);
            return maybe_add_comment(out, tag_index, index_as_comment);
        }
        if let Some(name) = guard.import_export_tag_names_.get(&tag_index) {
            out.add_str(name);
            return maybe_add_comment(out, tag_index, index_as_comment);
        }
        out.add_str(&format!("$tag{}", tag_index));
    }

    pub fn print_heap_type(&self, out: &mut StringBuilder, type_: HeapType) {
        if type_.is_index() {
            self.print_type_name(out, type_.ref_index(), IndexAsComment::No);
        } else {
            out.add_str(type_.name());
        }
    }

    pub fn print_value_type(&self, out: &mut StringBuilder, type_: ValueType) {
        if type_.has_index() {
            out.add_str(if type_.is_nullable() { "(ref null " } else { "(ref " });
            self.print_type_name(out, type_.ref_index(), IndexAsComment::No);
            out.add_char(')');
        } else {
            out.add_str(type_.name());
        }
    }

    pub fn estimate_current_memory_consumption(&self) -> usize {
        let mut result = std::mem::size_of::<NamesProvider>();
        let guard = self.mutex_.lock().unwrap();

        if let Some(names) = &guard.name_section_names_ {
            result += names.local_names_.estimate_current_memory_consumption();
            result += names.label_names_.estimate_current_memory_consumption();
            result += names.type_names_.estimate_current_memory_consumption();
            result += names.table_names_.estimate_current_memory_consumption();
            result += names.memory_names_.estimate_current_memory_consumption();
            result += names.global_names_.estimate_current_memory_consumption();
            result += names.element_segment_names_.estimate_current_memory_consumption();
            result += names.data_segment_names_.estimate_current_memory_consumption();
            result += names.field_names_.estimate_current_memory_consumption();
            result += names.tag_names_.estimate_current_memory_consumption();
        }

        result += string_map_size(&guard.import_export_function_names_);
        result += string_map_size(&guard.import_export_table_names_);
        result += string_map_size(&guard.import_export_memory_names_);
        result += string_map_size(&guard.import_export_global_names_);
        result += string_map_size(&guard.import_export_tag_names_);

        if *crate::v8_flags::trace_wasm_offheap_memory {
            println!("NamesProvider: {}", result);
        }
        result
    }
}

fn string_map_size(map: &HashMap<u32, String>) -> usize {
    let mut result = 0;
    for (_, string) in map {
        result += string.len();
    }
    result + map.len() * (std::mem::size_of::<u32>() + std::mem::size_of::<String>())
}

// Any disallowed characters get replaced with '_'. Reference:
// https://webassembly.github.io/spec/core/text/values.html#text-id
static K_IDENTIFIER_CHAR: [char; 96] = [
    '_', '!', '_', '#', '$',  '%', '&', '\'',  // --
    '_', '_', '*', '+', '_',  '-', '.', '/',   // --
    '0', '1', '2', '3', '4',  '5', '6', '7',   // --
    '8', '9', ':', '_', '<',  '=', '>', '?',   // --
    '@', 'A', 'B', 'C', 'D',  'E', 'F', 'G',   // --
    'H', 'I', 'J', 'K', 'L',  'M', 'N', 'O',   // --
    'P', 'Q', 'R', 'S', 'T',  'U', 'V', 'W',   // --
    'X', 'Y', 'Z', '_', '\\', '_', '^', '_',   // --
    '`', 'a', 'b', 'c', 'd',  'e', 'f', 'g',   // --
    'h', 'i', 'j', 'k', 'l',  'm', 'n', 'o',   // --
    'p', 'q', 'r', 's', 't',  'u', 'v', 'w',   // --
    'x', 'y', 'z', '_', '|',  '_', '~', '_',   // --
];

// To match legacy wasmparser behavior, we emit one '_' per invalid UTF16
// code unit.
// We could decide that we don't care much how exactly non-ASCII names are
// rendered and simplify this to "one '_' per invalid UTF8 byte".
fn sanitize_unicode_name(out: &mut StringBuilder, utf8_src: &[u8]) {
    if utf8_src.is_empty() {
        return;
    }
    //let utf8_data = utf8_src; //utf8_src.to_vec(); //Vector<const uint8_t> utf8_data(utf8_src, length);
    //let decoder = Utf8Decoder::new(utf8_data);
    //let utf16_length = decoder.utf16_length();
    //let mut utf16: Vec<u16> = vec![0u16; utf16_length];
    //decoder.decode(&mut utf16, utf8_data);

    // Below is a simplified UTF-8 to ASCII sanitization.  The more accurate
    // Utf8Decoder based logic is commented out because it relies on
    // `crate::strings::unicode_decoder::Utf8Decoder` which has not been implemented
    // as part of this translation.
    for &byte in utf8_src {
      let c = byte as char;
        if byte < 32 || byte >= 127 {
            out.add_char('_');
        } else {
            let index = (byte - 32) as usize;
            out.add_char(K_IDENTIFIER_CHAR[index]);
        }
    }
}

fn maybe_add_comment(out: &mut StringBuilder, index: u32, add_comment: IndexAsComment) {
    if add_comment == IndexAsComment::Yes {
        out.add_str(&format!(" (;{};)", index));
    }
}

fn get(map: &IndirectNameMap, outer_index: u32, inner_index: u32) -> WireBytesRef {
  if let Some(inner) = map.get(outer_index) {
    get_name_map(inner, inner_index)
  } else {
      WireBytesRef::empty()
  }
}

fn get_name_map(map: &NameMap, index: u32) -> WireBytesRef {
    if let Some(result) = map.get(index) {
        result.clone()
    } else {
        WireBytesRef::empty()
    }
}

// enums
#[derive(PartialEq, Copy, Clone)]
pub enum FunctionNamesBehavior {
    DevTools,
    WasmInternal,
}

#[derive(PartialEq, Copy, Clone)]
pub enum IndexAsComment {
    Yes,
    No,
}

pub struct CanonicalTypeNamesProvider {
  mutex_: Mutex<CanonicalTypeNamesProviderInternal>
}

struct CanonicalTypeNamesProviderInternal {
  payload_size_estimate_: usize,
  type_names_: Vec<String>,
  field_names_: HashMap<u32, Vec<String>>,
}

impl CanonicalTypeNamesProvider {
  pub fn new() -> Self {
    Self {
      mutex_: Mutex::new(CanonicalTypeNamesProviderInternal {
        payload_size_estimate_: 0,
        type_names_: Vec::new(),
        field_names_: HashMap::new(),
      })
    }
  }

  pub fn estimate_current_memory_consumption(&self) -> usize {
    let guard = self.mutex_.lock().unwrap();
    let mut result = std::mem::size_of::<Self>() + guard.payload_size_estimate_;
    result += guard.type_names_.capacity() * std::mem::size_of::<String>();

    for (_ ,vec) in &guard.field_names_ {
      result += vec.capacity() * std::mem::size_of::<String>();
    }

    result += guard.field_names_.len() * (std::mem::size_of::<u32>() + std::mem::size_of::<Vec<String>>());

    if *crate::v8_flags::trace_wasm_offheap_memory {
        println!("CanonicalTypeNamesProvider: {}", result);
    }
    result
  }

  pub fn decode_name_sections(&self) {
    let mut guard = self.mutex_.lock().unwrap();
    guard.type_names_.resize(get_type_canonicalizer().get_current_number_of_types() as usize, "".to_string());
    get_wasm_engine().decode_all_name_sections(self);
  }

  pub fn decode_names(&self, native_module: &NativeModule) {
    let module = native_module.module();
    if module.canonical_typenames_decoded {
      return;
    }
    let mut_module = unsafe { &mut *(module as *const WasmModule as *mut WasmModule) };
    mut_module.canonical_typenames_decoded = true;

    let wire_bytes = native_module.wire_bytes();
    let name_section = module.name_section.clone();
    if name_section.is_empty() {
      return;
    }

    let mut guard = self.mutex_.lock().unwrap();
    let mut added_size = 0;
    decode_canonical_type_names(wire_bytes.clone(), module, &mut guard.type_names_, &mut guard.field_names_, &mut added_size);
    guard.payload_size_estimate_ += added_size;
  }

  pub fn print_type_name(&self, out: &mut StringBuilder, type_index: CanonicalTypeIndex, index_as_comment: IndexAsComment) {
    let guard = self.mutex_.lock().unwrap();
    let index = type_index.index as usize;
    if index > guard.type_names_.len() || guard.type_names_[index].is_empty() {
      drop(guard);
      self.decode_name_sections();
      let guard = self.mutex_.lock().unwrap();
    }

    if index > guard.type_names_.len() || guard.type_names_[index].is_empty() {
      out.add_str(&format!("$canon{}", index));
      return;
    }
    let name = &guard.type_names_[index];
    out.add_char('$');
    out.add_slice(name.as_bytes());
    maybe_add_comment(out, index as u32, index_as_comment);
  }

  pub fn print_value_type(&self, out: &mut StringBuilder, type_: CanonicalValueType) {
    match type_.kind() {
      CanonicalTypeKind::Ref | CanonicalTypeKind::RefNull => {
        if type_.encoding_needs_heap_type() {
          out.add_str(if type_.kind() == CanonicalTypeKind::Ref { "(ref " } else { "(ref null " });
          if type_.has_index() {
            self.print_type_name(out, type_.ref_index(), IndexAsComment::No);
          } else {
            out.add_str(type_.name());
          }
          out.add_char(')');
        } else {
          out.add_str(type_.name());
        }
      }
      _ => {
        out.add_str(wasm::name(type_.kind()));
      }
    }
  }

  pub fn print_field_name(&self, out: &mut StringBuilder, struct_index: CanonicalTypeIndex, field_index: u32) {
    let guard = self.mutex_.lock().unwrap();
    let index = struct_index.index as usize;
    if index > guard.type_names_.len() {
      drop(guard);
      self.decode_name_sections();
    }
    drop(guard);

    let guard = self.mutex_.lock().unwrap();
    if let Some(per_type) = guard.field_names_.get(&index as u32) {
      if field_index < per_type.len() as u32 && !per_type[field_index as usize].is_empty() {
        let name = &per_type[field_index as usize];
        out.add_char('$');
        out.add_slice(name.as_bytes());
        return;
      }
    }
    out.add_str(&format!("$field{}", field_index));
  }
}

pub fn detect_inline_string_threshold() -> usize {
  for i in 0..32 {
    let s = "c".repeat(i);
    let str_ptr = s.as_ptr() as usize;
    let data_ptr = s.as_bytes().as_ptr() as usize;
    if data_ptr < str_ptr || data_ptr >= str_ptr + std::mem::size_of::<String>() {
      return i;
    }
  }
  32
}