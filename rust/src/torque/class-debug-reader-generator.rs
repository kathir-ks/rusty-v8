// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: add appropriate Rust crates

//use std::optional::Optional;
use std::collections::HashSet;
use std::string::String;

// TODO: Import flags, implementation_visitor, and type_oracle
//use crate::flags;
//use crate::torque::implementation_visitor;
//use crate::torque::type_oracle;

const KTQ_OBJECT_OVERRIDE_DECLS: &str = r#"  
  //std::vector<std::unique_ptr<ObjectProperty>> GetProperties(
  //    d::MemoryAccessor accessor) const override;
  fn get_properties(&self, accessor: MemoryAccessor) -> Vec<ObjectProperty>;
  fn get_name(&self) -> &'static str;
  fn visit(&self, visitor: &mut TqObjectVisitor) ;
  fn is_superclass_of(&self, other: &dyn TqObject) -> bool;
"#;

enum TypeStorage {
    AsStoredInHeap,
    Uncompressed,
}

// An iterator for use in ValueTypeFieldsRange.
struct ValueTypeFieldIterator<'a> {
    type_: &'a Type,
    index_: usize,
}

impl<'a> ValueTypeFieldIterator<'a> {
    fn new(type_: &'a Type, index_: usize) -> Self {
        ValueTypeFieldIterator { type_, index_ }
    }

    struct Result<'b> {
        name_and_type: NameAndType,
        pos: SourcePosition,
        offset_bytes: usize,
        num_bits: i32,
        shift_bits: i32,
    }
    
    fn current(&self) -> Result<'a> {
        if let Some(struct_type) = self.type_.struct_supertype() {
            let field = &struct_type.fields()[self.index_];
            return Result {
                name_and_type: field.name_and_type.clone(),
                pos: field.pos.clone(),
                offset_bytes: *field.offset.as_ref().unwrap(),
                num_bits: 0,
                shift_bits: 0,
            };
        }
        let mut type_ = self.type_;
        let mut bitfield_start_offset = 0;

        //TODO: Implement TypeOracle
        //if let Some(type_wrapped_in_smi) =
        //    Type::match_unary_generic(type_, TypeOracle::get_smi_tagged_generic())
        //{
        //    type_ = type_wrapped_in_smi;
        //    bitfield_start_offset = TargetArchitecture::smi_tag_and_shift_size();
        //}

        if let Some(bit_field_struct_type) = type_.dynamic_cast_bit_field_struct_type() {
            let field = &bit_field_struct_type.fields()[self.index_];
            return Result {
                name_and_type: field.name_and_type.clone(),
                pos: field.pos.clone(),
                offset_bytes: 0,
                num_bits: field.num_bits,
                shift_bits: field.offset + bitfield_start_offset,
            };
        }
        panic!("UNREACHABLE");
    }
}

impl<'a> Iterator for ValueTypeFieldIterator<'a> {
    type Item = ValueTypeFieldIterator<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        self.index_ += 1;
        Some(ValueTypeFieldIterator::new(self.type_, self.index_))
    }
}

impl<'a> PartialEq for ValueTypeFieldIterator<'a> {
    fn eq(&self, other: &Self) -> bool {
        self.type_ as *const _ == other.type_ as *const _ && self.index_ == other.index_
    }
}

// A way to iterate over the fields of structs or bitfield structs. For other
// types, the iterators returned from begin() and end() are immediately equal.
struct ValueTypeFieldsRange<'a> {
    type_: &'a Type,
}

impl<'a> ValueTypeFieldsRange<'a> {
    fn new(type_: &'a Type) -> Self {
        ValueTypeFieldsRange { type_ }
    }

    fn begin(&self) -> ValueTypeFieldIterator<'a> {
        ValueTypeFieldIterator::new(self.type_, 0)
    }

    fn end(&self) -> ValueTypeFieldIterator<'a> {
        let mut index = 0;
        //TODO: Implement struct supertype
        //if let Some(struct_type) = self.type_.struct_supertype() {
        //    if *struct_type
        //        != TypeOracle::get_float64_or_undefined_or_hole_type()
        //    {
        //        index = struct_type.fields().len();
        //    }
        //}

        let mut type_ = self.type_;

        //TODO: Implement match_unary_generic
        //if let Some(type_wrapped_in_smi) =
        //    Type::match_unary_generic(type_, TypeOracle::get_smi_tagged_generic())
        //{
        //    type_ = type_wrapped_in_smi;
        //}

        if let Some(bit_field_struct_type) = type_.dynamic_cast_bit_field_struct_type() {
            index = bit_field_struct_type.fields().len();
        }
        ValueTypeFieldIterator::new(self.type_, index)
    }
}

// A convenient way to keep track of several different ways that we might need
// to represent a field's type in the generated C++.
struct DebugFieldType<'a> {
    name_and_type_: &'a NameAndType,
    pos_: &'a SourcePosition,
}

impl<'a> DebugFieldType<'a> {
    fn new(field: &'a Field) -> Self {
        DebugFieldType {
            name_and_type_: &field.name_and_type,
            pos_: &field.pos,
        }
    }

    fn from_name_and_type(name_and_type: &'a NameAndType, pos: &'a SourcePosition) -> Self {
        DebugFieldType {
            name_and_type_: name_and_type,
            pos_: pos,
        }
    }

    fn is_tagged(&self) -> bool {
        //TODO: implement IsSubtypeOf
        //self.name_and_type_
        //    .type
        //    .is_subtype_of(TypeOracle::get_tagged_type())
        true
    }

    // Returns the type that should be used for this field's value within code
    // that is compiled as part of the debug helper library. In particular, this
    // simplifies any tagged type to a plain uintptr_t because the debug helper
    // compiles without most of the V8 runtime code.
    fn get_value_type(&self, storage: TypeStorage) -> String {
        if self.is_tagged() {
            match storage {
                TypeStorage::AsStoredInHeap => "i::Tagged_t".to_string(),
                TypeStorage::Uncompressed => "uintptr_t".to_string(),
            }
        } else {
            // We can't emit a useful error at this point if the constexpr type name is
            // wrong, but we can include a comment that might be helpful.
            self.get_original_type(storage)
                + " /*Failing? Ensure constexpr type name is correct, and the "
                + "necessary #include is in any .tq file*/"
        }
    }

    // Returns the type that should be used to represent a field's type to
    // debugging tools that have full V8 symbols. The types returned from this
    // method are resolveable in the v8::internal namespace and may refer to
    // object types that are not included in the compilation of the debug helper
    // library.
    fn get_original_type(&self, storage: TypeStorage) -> String {
        //TODO: Implement StructSupertype
        //if self.name_and_type_.type.struct_supertype().is_some() {
        //    // There's no meaningful type we could use here, because the V8 symbols
        //    // don't have any definition of a C++ struct matching this struct type.
        //    return "".to_string();
        //}
        if self.is_tagged() {
            //TODO: implement ClassSupertype, GetGeneratedTNodeTypeName
            //if let Some(field_class_type) = self.name_and_type_.type.class_supertype() {
            //    let result = "v8::internal::".to_string()
            //        + &field_class_type.get_generated_t_node_type_name();
            //    if let TypeStorage::AsStoredInHeap = storage {
            //        return "v8::internal::TaggedMember<".to_string() + &result + ">";
            //    }
            //    return result;
            //} else {
            //    let result = "v8::internal::Object".to_string();
            //    if let TypeStorage::AsStoredInHeap = storage {
            //        return "v8::internal::TaggedMember<".to_string() + &result + ">";
            //    }
            //    return result;
            //}
            "v8::internal::Object".to_string()
        } else {
            self.name_and_type_.type.get_constexpr_generated_type_name()
        }
    }

    // Returns a C++ expression that evaluates to a string (type `const char*`)
    // containing the name of the field's type. The types returned from this
    // method are resolveable in the v8::internal namespace and may refer to
    // object types that are not included in the compilation of the debug helper
    // library.
    fn get_type_string(&self, storage: TypeStorage) -> String {
        //TODO: implement IsStructType
        //if self.is_tagged() || self.name_and_type_.type.is_struct_type() {
        //    // Wrap up the original type in a string literal.
        //    return "\"".to_string() + &self.get_original_type(storage) + "\"";
        //}

        // We require constexpr type names to be resolvable in the v8::internal
        // namespace, according to the contract in debug-helper.h. In order to
        // verify at compile time that constexpr type names are resolvable, we use
        // the type name as a dummy template parameter to a function that just
        // returns its parameter.
        "CheckTypeName<".to_string()
            + &self.get_value_type(storage)
            + ">(\""
            + &self.get_original_type(storage)
            + "\")"
    }

    // Returns the field's size in bytes.
    fn get_size(&self) -> usize {
        //TODO: Implement SizeOf
        //if let Some(opt_size) = SizeOf(self.name_and_type_.type) {
        //    return opt_size.0;
        //} else {
        //    Error("Size required for type ", self.name_and_type_.type.to_string()).position(self.pos_);
        //    return 0;
        //}
        8
    }

    // Returns the name of the function for getting this field's address.
    fn get_address_getter(&self) -> String {
        "Get".to_string() + &camelify_string(&self.name_and_type_.name) + "Address"
    }
}

// Emits a function to get the address of a field within a class, based on the
// member variable {address_}, which is a tagged pointer. Example
// implementation:
//
// uintptr_t TqFixedArray::GetObjectsAddress() const {
//   return address_ - i::kHeapObjectTag + 16;
// }
fn generate_field_address_accessor(
    field: &Field,
    class_name: &str,
    h_contents: &mut String,
    cc_contents: &mut String,
) {
    let debug_field_type = DebugFieldType::new(field);

    let address_getter = debug_field_type.get_address_getter();

    h_contents.push_str(&format!("  uintptr_t {}() const;\n", address_getter));
    cc_contents.push_str(&format!(
        "\nuintptr_t Tq{}::{}() const {{\n",
        class_name, address_getter
    ));
    //TODO: Implement kHeapObjectTag
    cc_contents.push_str(&format!(
        "  return address_ - 0 /*i::kHeapObjectTag*/ + {};\n",
        field.offset.unwrap()
    ));
    cc_contents.push_str("}\n");
}

// Emits a function to get the value of a field, or the value from an indexed
// position within an array field, based on the member variable {address_},
// which is a tagged pointer, and the parameter {accessor}, a function pointer
// that allows for fetching memory from the debuggee. The returned result
// includes both a "validity", indicating whether the memory could be fetched,
// and the fetched value. If the field contains tagged data, then these
// functions call EnsureDecompressed to expand compressed data. Example:
//
// Value<uintptr_t> TqMap::GetPrototypeValue(d::MemoryAccessor accessor) const {
//   i::Tagged_t value{};
//   d::MemoryAccessResult validity = accessor(
//       GetPrototypeAddress(),
//       reinterpret_cast<uint8_t*>(&value),
//       sizeof(value));
//   return {validity, EnsureDecompressed(value, address_)};
// }
//
// For array fields, an offset parameter is included. Example:
//
// Value<uintptr_t> TqFixedArray::GetObjectsValue(d::MemoryAccessor accessor,
//                                                size_t offset) const {
//   i::Tagged_t value{};
//   d::MemoryAccessResult validity = accessor(
//       GetObjectsAddress() + offset * sizeof(value),
//       reinterpret_cast<uint8_t*>(&value),
//       sizeof(value));
//   return {validity, EnsureDecompressed(value, address_)};
// }
fn generate_field_value_accessor(
    field: &Field,
    class_name: &str,
    h_contents: &mut String,
    cc_contents: &mut String,
) {
    // Currently not implemented for struct fields.
    //TODO: implement StructSupertype
    //if field.name_and_type.type.struct_supertype().is_some() {
    //    return;
    //}

    let debug_field_type = DebugFieldType::new(field);

    let address_getter = debug_field_type.get_address_getter();
    let field_getter = "Get".to_string() + &camelify_string(&field.name_and_type.name) + "Value";

    let mut index_param = String::new();
    let mut index_offset = String::new();
    if field.index.is_some() {
        index_param = ", size_t offset".to_string();
        index_offset = " + offset * sizeof(value)".to_string();
    }

    let field_value_type = debug_field_type.get_value_type(TypeStorage::Uncompressed);
    h_contents.push_str(&format!(
        "  Value<{}> {}(d::MemoryAccessor accessor {}) const;\n",
        field_value_type, field_getter, index_param
    ));
    cc_contents.push_str(&format!(
        "\nValue<{}> Tq{}::{}(d::MemoryAccessor accessor{}) const {{\n",
        field_value_type, class_name, field_getter, index_param
    ));
    cc_contents.push_str(&format!(
        "  {} value{{}};\n",
        debug_field_type.get_value_type(TypeStorage::AsStoredInHeap)
    ));
    cc_contents.push_str(&format!(
        "  d::MemoryAccessResult validity = accessor({}(){}, reinterpret_cast<uint8_t*>(&value), sizeof(value));\n",
        address_getter, index_offset
    ));
    
    //TODO: Implement MAP_PACKING
    //if field_getter == "GetMapValue" {
    //    cc_contents.push_str("  value = i::MapWord::Unpack(value);\n");
    //}

    cc_contents.push_str(&format!(
        "  return {{validity, {}}};\n",
        if debug_field_type.is_tagged() {
            "EnsureDecompressed(value, address_)".to_string()
        } else {
            "value".to_string()
        }
    ));
    cc_contents.push_str("}\n");
}

// Emits a portion of the member function GetProperties that is responsible for
// adding data about the current field to a result vector called "result".
// Example output:
//
// std::vector<std::unique_ptr<StructProperty>> prototype_struct_field_list;
// result.push_back(std::make_unique<ObjectProperty>(
//     "prototype",                                     // Field name
//     "v8::internal::HeapObject",                      // Field type
//     "v8::internal::HeapObject",                      // Decompressed type
//     GetPrototypeAddress(),                           // Field address
//     1,                                               // Number of values
//     8,                                               // Size of value
//     std::move(prototype_struct_field_list),          // Struct fields
//     d::PropertyKind::kSingle));                      // Field kind
//
// In builds with pointer compression enabled, the field type for tagged values
// is "v8::internal::TaggedValue" (a four-byte class) and the decompressed type
// is a normal Object subclass that describes the expanded eight-byte type.
//
// If the field is an array, then its length is fetched from the debuggee. This
// could fail if the debuggee has incomplete memory, so the "validity" from that
// fetch is used to determine the result PropertyKind, which will say whether
// the array's length is known.
//
// If the field's type is a struct, then a local variable is created and filled
// with descriptions of each of the struct's fields. The type and decompressed
// type in the ObjectProperty are set to the empty string, to indicate to the
// caller that the struct fields vector should be used instead.
//
// The following example is an array of structs, so it uses both of the optional
// components described above:
//
// std::vector<std::unique_ptr<StructProperty>> descriptors_struct_field_list;
// descriptors_struct_field_list.push_back(std::make_unique<StructProperty>(
//     "key",                                // Struct field name
//     "v8::internal::PrimitiveHeapObject",  // Struct field type
//     "v8::internal::PrimitiveHeapObject",  // Struct field decompressed type
//     0,                                    // Byte offset within struct data
//     0,                                    // Bitfield size (0=not a bitfield)
//     0));                                  // Bitfield shift
// // The line above is repeated for other struct fields. Omitted here.
// // Fetch the slice.
// auto indexed_field_slice_descriptors =
//     TqDebugFieldSliceDescriptorArrayDescriptors(accessor, address_);
// if (indexed_field_slice_descriptors.validity == d::MemoryAccessResult::kOk) {
//   result.push_back(std::make_unique<ObjectProperty>(
//     "descriptors",                                 // Field name
//     "",                                            // Field type
//     "",                                            // Decompressed type
//     address_ - i::kHeapObjectTag +
//     std::get<1>(indexed_field_slice_descriptors.value), // Field address
//     std::get<2>(indexed_field_slice_descriptors.value), // Number of values
//     12,                                            // Size of value
//     std::move(descriptors_struct_field_list),      // Struct fields
//     GetArrayKind(indexed_field_slice_descriptors.validity)));  // Field kind
// }
fn generate_get_props_chunk_for_field(
    field: &Field,
    get_props_impl: &mut String,
    class_name: &str,
) {
    let debug_field_type = DebugFieldType::new(field);

    // If the current field is a struct or bitfield struct, create a vector
    // describing its fields. Otherwise this vector will be empty.
    let struct_field_list_name = field.name_and_type.name.clone() + "_struct_field_list";
    get_props_impl.push_str(&format!(
        "  std::vector<std::unique_ptr<StructProperty>> {};\n",
        struct_field_list_name
    ));
    for struct_field in ValueTypeFieldsRange::new(&field.name_and_type.type) {
        let struct_field_val = struct_field.current();
        let struct_field_type = DebugFieldType::from_name_and_type(
            &struct_field_val.name_and_type,
            &struct_field_val.pos,
        );

        get_props_impl.push_str(&format!(
            "  {}.push_back(std::make_unique<StructProperty>(\"{}\", {}, {}, {}, {}, {}));\n",
            struct_field_list_name,
            struct_field_val.name_and_type.name,
            struct_field_type.get_type_string(TypeStorage::AsStoredInHeap),
            struct_field_val.offset_bytes,
            struct_field_val.num_bits,
            struct_field_val.shift_bits,
            //TODO: Check the necessity
            "{}"
        ));
    }
    let struct_field_list = "std::move(".to_string() + &struct_field_list_name + ")";

    // The number of values and property kind for non-indexed properties:
    let count_value = "1".to_string();
    let property_kind = "d::PropertyKind::kSingle".to_string();

    // If the field is indexed, emit a fetch of the array length, and change
    // count_value and property_kind to be the correct values for an array.
    if field.index.is_some() {
        let indexed_field_slice_name = "indexed_field_slice_".to_string() + &field.name_and_type.name;
        get_props_impl.push_str(&format!(
            "  auto {} = TqDebugFieldSlice{}{}(accessor, address_);\n",
            indexed_field_slice_name,
            class_name,
            camelify_string(&field.name_and_type.name)
        ));
        let validity = indexed_field_slice_name.clone() + ".validity";
        let value = indexed_field_slice_name.clone() + ".value";
        let property_kind_local = "GetArrayKind(".to_string() + &validity + ")";

        get_props_impl.push_str(&format!(
            "  if ({} == d::MemoryAccessResult::kOk) {{\n",
            validity
        ));
        get_props_impl.push_str(&format!(
            "    result.push_back(std::make_unique<ObjectProperty>(\"{}\", {}, address_ - i::kHeapObjectTag + std::get<1>({}), std::get<2>({}), {}, {}, {}));\n",
            field.name_and_type.name,
            debug_field_type.get_type_string(TypeStorage::AsStoredInHeap),
            value,
            value,
            debug_field_type.get_size(),
            struct_field_list,
            property_kind_local
        ));
        get_props_impl.push_str("  }\n");
        return;
    }
    get_props_impl.push_str(&format!(
        "  result.push_back(std::make_unique<ObjectProperty>(\"{}\", {}, {}, {}, {}, {}, {}));\n",
        field.name_and_type.name,
        debug_field_type.get_type_string(TypeStorage::AsStoredInHeap),
        debug_field_type.get_address_getter(),
        count_value,
        debug_field_type.get_size(),
        struct_field_list,
        property_kind
    ));
}

// For any Torque-defined class Foo, this function generates a class TqFoo which
// allows for convenient inspection of objects of type Foo in a crash dump or
// time travel session (where we can't just run the object printer). The
// generated class looks something like this:
//
// class TqFoo : public TqParentOfFoo {
//  public:
//   // {address} is an uncompressed tagged pointer.
//   inline TqFoo(uintptr_t address) : TqParentOfFoo(address) {}
//
//   // Creates and returns a list of this object's properties.
//   std::vector<std::unique_ptr<ObjectProperty>> GetProperties(
//       d::MemoryAccessor accessor) const override;
//
//   // Returns the name of this class, "v8::internal::Foo".
//   const char* GetName() const override;
//
//   // Visitor pattern; implementation just calls visitor->VisitFoo(this).
//   void Visit(TqObjectVisitor* visitor) const override;
//
//   // Returns whether Foo is a superclass of the other object's type.
//   bool IsSuperclassOf(const TqObject* other) const override;
//
//   // Field accessors omitted here (see other comments above).
// };
//
// Four output streams are written:
//
// h_contents:  A header file which gets the class definition above.
// cc_contents: A cc file which gets implementations of that class's members.
// visitor:     A stream that is accumulating the definition of the class
//              TqObjectVisitor. Each class Foo gets its own virtual method
//              VisitFoo in TqObjectVisitor.
fn generate_class_debug_reader(
    type_: &ClassType,
    h_contents: &mut String,
    cc_contents: &mut String,
    visitor: &mut String,
    done: &mut HashSet<*const ClassType>,
) {
    // Make sure each class only gets generated once.
    if !done.insert(type_ as *const ClassType) {
        return;
    }
    //TODO: Implement GetSuperClass()
    //let super_type = type_.GetSuperClass();
    let super_type: Option<&ClassType> = None;

    // We must emit the classes in dependency order. If the super class hasn't
    // been emitted yet, go handle it first.
    if let Some(super_type) = super_type {
        generate_class_debug_reader(super_type, h_contents, cc_contents, visitor, done);
    }

    // Classes with undefined layout don't grant any particular value here and may
    // not correspond with actual C++ classes, so skip them.
    if type_.has_undefined_layout() {
        return;
    }

    let name = type_.name.clone();
    let super_name = super_type.map(|t| t.name.clone()).unwrap_or("Object".to_string());
    h_contents.push_str(&format!("\nstruct Tq{} : public Tq{} {{\n", name, super_name));
    h_contents.push_str(" public:\n");
    h_contents.push_str(&format!("  Tq{}(uintptr_t address) : Tq{}(address) {{}}\n", name, super_name));
    h_contents.push_str(KTQ_OBJECT_OVERRIDE_DECLS);

    cc_contents.push_str(&format!("\nconst char* Tq{}::get_name() const {{\n", name));
    cc_contents.push_str(&format!("  return \"v8::internal::{}\";\n", name));
    cc_contents.push_str("}\n");

    cc_contents.push_str(&format!("\nvoid Tq{}::visit(TqObjectVisitor* visitor) {{\n", name));
    cc_contents.push_str(&format!("  visitor->Visit{}(this);\n", name));
    cc_contents.push_str("}\n");

    cc_contents.push_str(&format!("\nbool Tq{}::is_superclass_of(const TqObject* other) const {{\n", name));
    cc_contents.push_str(&format!(
        "  return get_name() != other->get_name() && dynamic_cast::<Tq{}>(other) != nullptr;\n",
        name
    ));
    cc_contents.push_str("}\n");

    // By default, the visitor method for this class just calls the visitor method
    // for this class's parent. This allows custom visitors to only override a few
    // classes they care about without needing to know about the entire hierarchy.
    visitor.push_str(&format!("  virtual void Visit{}(const Tq{}* object) {{\n", name, name));
    visitor.push_str(&format!("    Visit{}(object);\n", super_name));
    visitor.push_str("  }\n");

    let mut get_props_impl = String::new();

    for field in &type_.fields {
        //TODO: implement GetVoidType
        //if field.name_and_type.type == TypeOracle::get_void_type() {
        //    continue;
        //}
        if field.offset.is_some() {
            generate_field_address_accessor(field, &name, h_contents, cc_contents);
            generate_field_value_accessor(field, &name, h_contents, cc_contents);
        }
        generate_get_props_chunk_for_field(field, &mut get_props_impl, &name);
    }

    h_contents.push_str("};\n");

    cc_contents.push_str(&format!(
        "\nstd::vector<std::unique_ptr<ObjectProperty>> Tq{}::GetProperties(d::MemoryAccessor accessor) const {{\n",
        name
    ));
    // Start by getting the fields from the parent class.
    cc_contents.push_str(&format!(
        "  std::vector<std::unique_ptr<ObjectProperty>> result = Tq{}::GetProperties(accessor);\n",
        super_name
    ));
    // Then add the fields from this class.
    cc_contents.push_str(&get_props_impl);
    cc_contents.push_str("  return result;\n");
    cc_contents.push_str("}\n");
}

//TODO: Implement
//void ImplementationVisitor::GenerateClassDebugReaders(
//    const std::string& output_directory) {
//    const std::string file_name = "class-debug-readers";
//    std::stringstream h_contents;
//    std::stringstream cc_contents;
//    h_contents << "// Provides the ability to read object properties in\n";
//    h_contents << "// postmortem or remote scenarios, where the debuggee's\n";
//    h_contents << "// memory is not part of the current process's address\n";
//    h_contents << "// space and must be read using a callback function.\n\n";
//    {
//        IncludeGuardScope include_guard(h_contents, file_name + ".h");
//
//        h_contents << "#include <cstdint>\n";
//        h_contents << "#include <vector>\n\n";
//
//        for (const std::string& include_path : GlobalContext::CppIncludes()) {
//            h_contents << "#include " << StringLiteralQuote(include_path) << "\n";
//        }
//
//        h_contents
//            << "\n#include \"tools/debug_helper/debug-helper-internal.h\"\n\n";
//
//        const char* kWingdiWorkaround =
//            "// Unset a wingdi.h macro that causes conflicts.\n"
//            "#ifdef GetBValue\n"
//            "#undef GetBValue\n"
//            "#endif\n\n";
//
//        h_contents << kWingdiWorkaround;
//
//        cc_contents << "#include \"torque-generated/" << file_name << ".h\"\n\n";
//        cc_contents << "#include \"src/objects/all-objects-inl.h\"\n";
//        cc_contents << "#include \"torque-generated/debug-macros.h\"\n\n";
//        cc_contents << kWingdiWorkaround;
//        cc_contents << "namespace i = v8::internal;\n\n";
//
//        NamespaceScope h_namespaces(h_contents,
//                                    {"v8", "internal", "debug_helper_internal"});
//        NamespaceScope cc_namespaces(cc_contents,
//                                     {"v8", "internal", "debug_helper_internal"});
//
//        std::stringstream visitor;
//        visitor << "\nclass TqObjectVisitor {\n";
//        visitor << " public:\n";
//        visitor << "  virtual void VisitObject(const TqObject* object) {}\n";
//
//        std::unordered_set<const ClassType*> done;
//        for (const ClassType* type : TypeOracle::GetClasses()) {
//            GenerateClassDebugReader(*type, h_contents, cc_contents, visitor, &done);
//        }
//
//        visitor << "};\n";
//        h_contents << visitor.str();
//    }
//    WriteFile(output_directory + "/" + file_name + ".h", h_contents.str());
//    WriteFile(output_directory + "/" + file_name + ".cc", cc_contents.str());
//}

// --- Helper Structs and Enums ---

trait TqObject {
    fn get_name(&self) -> &'static str;
}

struct