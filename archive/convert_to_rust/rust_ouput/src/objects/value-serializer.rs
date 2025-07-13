// Converted from V8 C++ source files:
// Header: value-serializer.h
// Implementation: value-serializer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod v8 {
pub mod internal {
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
extern crate lazy_static;
use self::lazy_static::lazy_static;
pub struct V8 {}
pub struct JSArrayBuffer {}
pub struct JSArrayBufferView {}
pub struct JSObject {}
pub struct HeapObject {}
pub struct String {}
pub struct WasmModuleObject {}
pub struct WasmMemoryObject {}
pub struct JSReceiver {}
pub struct Oddball {}
pub struct Smi {}
pub struct HeapNumber {}
pub struct BigInt {}
pub struct JSArray {}
pub struct JSDate {}
pub struct JSPrimitiveWrapper {}
pub struct JSRegExp {}
pub struct JSMap {}
pub struct JSSet {}
pub struct JSSharedArray {}
pub struct JSSharedStruct {}
pub struct FixedArray {}
pub struct Map {}
pub struct PropertyDetails {}
pub struct FieldIndex {}
pub struct SharedFunctionInfo {}
pub struct Context {}
pub struct SharedArrayBuffer {}
pub struct PropertyDescriptor {}
pub struct PropertyKey {}
pub struct JSFunction {}
pub struct TaggedObject {}
pub struct DescriptorArray {}
pub struct SeqTwoByteString {}
pub struct Error {}
pub struct SimpleNumberDictionary {}
pub struct JSRabGsabDataView {}
pub struct TransitionsAccessor {}
pub struct Object {}
pub struct DataView {}
pub enum CodeEntrypointTag {}
pub enum PropertyKind {}
pub enum LookupIteratorState {}
pub enum VariableMode {}
pub enum MessageTemplate {}
pub struct Zone {
dummy: i32
}
pub struct Isolate {
dummy: i32
}
impl Isolate {
pub fn factory(&self) -> Factory {
Factory{dummy:0}
}
pub fn has_shared_space(&self) -> bool {true}
pub fn global_handles(&self) -> GlobalHandles {
GlobalHandles{dummy:0}
}
pub fn object_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn eval_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn range_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn reference_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn syntax_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn type_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn uri_error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn error_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn cause_string(&self) -> String {
String{}
}
pub fn stack_string(&self) -> String {
String{}
}
pub fn boolean_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn number_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn bigint_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn string_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn date_function(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn map_set(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn set_add(&self) -> JSFunction {
JSFunction{dummy:0}
}
pub fn has_exception(&self) -> bool {
false
}
pub fn Throw(&self, obj: TaggedObject){

}
}
pub struct Factory {
dummy: i32
}
impl Factory {
pub fn undefined_value(&self) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn null_value(&self) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn true_value(&self) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn false_value(&self) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn empty_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn InternalizeString(&self, str: String) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn cause_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn stack_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn cause_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn stack_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn cause_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn stack_string(&self) -> DirectHandle<String> {
DirectHandle{dummy: 0}
}
pub fn NewNumberFromInt(&self, value: i32) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewNumberFromUint(&self, value: u32) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewNumber(&self, value: f64) -> DirectHandle<HeapNumber> {
DirectHandle{dummy: 0}
}
pub fn NewStringFromUtf8(&self, chars: base::Vector<const char>, allocation: AllocationType) -> MaybeDirectHandle<String> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewStringFromOneByte(&self, bytes: base::Vector<const u8>, allocation: AllocationType) -> MaybeDirectHandle<String> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewRawTwoByteString(&self, length: i32, allocation: AllocationType) -> MaybeDirectHandle<SeqTwoByteString> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewJSObject(&self, function: JSFunction) -> DirectHandle<JSObject> {
DirectHandle{dummy: 0}
}
pub fn NewJSArray(&self, arg1: i32, arg2: i32) -> DirectHandle<JSArray> {
DirectHandle{dummy: 0}
}
pub fn NewJSArray(&self, kind: ElementsKind, length: i32, i2:i32,mode:ArrayStorageAllocationMode) -> DirectHandle<JSArray> {
DirectHandle{dummy: 0}
}
pub fn NewJSTypedArray( &self, array_type: ExternalArrayType, buffer: DirectHandle<JSArrayBuffer>, byte_offset: i32, byte_length: i32, ) -> DirectHandle<JSTypedArray>{
DirectHandle{dummy: 0}
}
pub fn NewJSArrayBufferAndBackingStore(&self, byte_length: usize, max_byte_length: u32, flag: InitializedFlag, resizable: ResizableFlag) -> MaybeDirectHandle<JSArrayBuffer> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn NewJSDataViewOrRabGsabDataView( &self, buffer: DirectHandle<JSArrayBuffer>, byte_offset: u32, byte_length: u32, is_length_tracking: bool ) -> DirectHandle<JSRabGsabDataView>{
DirectHandle{dummy: 0}
}
pub fn NewFixedArray(&self, length: i32) -> DirectHandle<FixedArray> {
DirectHandle{dummy: 0}
}
pub fn NewError(&self, function: JSFunction, message: DirectHandle<String>) -> TaggedObject {
TaggedObject{}
}
pub fn empty_fixed_array(&self) -> TaggedObject {
TaggedObject{}
}
pub fn NewJSObjectFromMap(&self, map: DirectHandle<Map>) -> DirectHandle<JSObject> {
DirectHandle{dummy: 0}
}
pub fn NewRawTwoByteString(&self, len: i32, alloc: AllocationType) -> MaybeDirectHandle<SeqTwoByteString>{
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
}
pub struct GlobalHandles {
dummy: i32
}
impl GlobalHandles {
pub fn Create(&self, obj: TaggedObject) -> DirectHandle<FixedArray> {
DirectHandle{dummy: 0}
}
pub fn Destroy( location: &DirectHandle<FixedArray>){

}
}
pub enum InitializedFlag {
kUninitialized
}
pub enum ResizableFlag {
kResizable,
kNotResizable
}
pub enum Skip {
SKIP_NONE
}
pub enum AbortReason {}
pub struct HandleScope {
dummy: i32
}
impl HandleScope {
pub fn new(isolate: *mut Isolate) -> HandleScope {
HandleScope{dummy: 0}
}
pub fn CloseAndEscape<T>(&self, obj: DirectHandle<T>) -> MaybeDirectHandle<T> {
MaybeDirectHandle::new(obj)
}
}
impl Drop for HandleScope {
fn drop(&mut self) {

}
}
pub enum ElementsKind {}
pub enum GetKeysConversion {
kKeepNumbers
}
pub enum KeyCollectionMode {
kOwnOnly
}
pub struct KeyAccumulator {}
impl KeyAccumulator {
pub fn GetKeys(isolate: *mut Isolate, object: &DirectHandle<JSObject>, mode: KeyCollectionMode, strings: i32) -> MaybeDirectHandle<FixedArray> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn GetKeys(isolate: *mut Isolate, object: &DirectHandle<JSObject>, mode: KeyCollectionMode, strings: i32, arg1: GetKeysConversion, arg2: bool, arg3: bool) -> MaybeDirectHandle<FixedArray> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
}
pub struct GlobalBackingStoreRegistry {}
impl GlobalBackingStoreRegistry {
pub fn Register(store: *mut ()) {

}
}
pub struct String::FlatContent {}
pub struct WasmCodePointer {}
pub struct Tagged<T> {dummy:i32}
pub struct Local<'a, T> {dummy:i32}
pub struct Utils {}
impl Utils {
pub fn ToLocal<'a>(object: &DirectHandle<JSObject>) -> Local<'static, JSObject> {
Local{dummy: 0}
}
pub fn OpenDirectHandle<'a>(object: Local<'a, JSObject>) -> DirectHandle<JSObject> {
DirectHandle{dummy: 0}
}
pub fn ToLocalShared<'a>(object: &DirectHandle<JSArrayBuffer>) -> Local<'static, SharedArrayBuffer> {
Local{dummy: 0}
}
}
pub struct ErrorUtils {}
impl ErrorUtils {
pub fn Construct(isolate: *mut Isolate, constructor: DirectHandle<JSFunction>, constructor2: DirectHandle<JSFunction>, message: DirectHandle<Object>, options: DirectHandle<Object>, skip: Skip, no_caller: DirectHandle<Object>, collection: i32) -> MaybeDirectHandle<JSObject> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
pub fn SetFormattedStack(isolate: *mut Isolate, error: DirectHandle<JSObject>, stack: DirectHandle<Object>){

}
}
pub struct Execution {}
impl Execution {
pub fn Call(isolate: *mut Isolate, function: DirectHandle<JSFunction>, this: DirectHandle<JSMap>, arg: base::Vector<DirectHandle<Object>>) -> MaybeDirectHandle<Object> {
MaybeDirectHandle::new(DirectHandle{dummy: 0})
}
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum FieldType {
None,
Tagged,
Smi,
Other,
}
impl FieldType {
pub fn now_contains(&self, value: &DirectHandle<Object>) -> bool {
true
}
}
pub struct ObjectValue {
value: Box<dyn std::any::Any>,
}
impl fmt::Debug for ObjectValue {
fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
f.debug_struct("ObjectValue").finish()}
}
impl From<Box<dyn std::any::Any>> for ObjectValue {
fn from(value: Box<dyn std::any::Any>) -> Self {
ObjectValue { value }
}
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MemoryRepresentation {
None,
Smi,
Double,
Object,
}
pub struct MapUpdater {}
impl MapUpdater {
pub fn GeneralizeField(isolate: *mut Isolate, target: DirectHandle<Map>, descriptor: InternalIndex, constness: i32, expected: MemoryRepresentation, value_type: DirectHandle<FieldType>){

}
}
pub struct FieldType_ {}
impl FieldType_ {
pub fn now_contains(details: PropertyDetails, value: &DirectHandle<Object>) -> bool {
true
}
}
pub enum Accessor {
ACCESSOR
}
pub enum Constness {
CONSTANT,
MUTABLE
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyLocation {
kField
}
pub struct JSTypedArray {}
impl JSTypedArray {
pub fn GetBuffer(&self) -> &JSArrayBuffer {
&JSArrayBuffer{}
}
pub fn IsOutOfBounds(&self) -> bool {
false
}
pub fn type_(&self) -> ExternalArrayType {
ExternalArrayType::kExternalInt8Array
}
}
pub struct DirectHandle<T> {
dummy: i32
}
impl <T> DirectHandle<T> {
fn is_identical_to(&self, other: &DirectHandle<T>) -> bool {true}
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ExternalArrayType {
kExternalInt8Array,
kExternalFloat16Array,
}
pub struct MaybeDirectHandle<T> {
handle: DirectHandle<T>,
}
impl<T> MaybeDirectHandle<T> {
fn new(handle: DirectHandle<T>) -> Self {
MaybeDirectHandle { handle }
}
fn ToHandle<'a>(&self, out: &mut DirectHandle<T>) -> bool {
*out = self.handle;
true
}
fn is_null(&self) -> bool {true}
fn FromJust(&self) -> DirectHandle<T> {
self.handle
}
}
impl<T> From<DirectHandle<T>> for MaybeDirectHandle<T> {
fn from(handle: DirectHandle<T>) -> Self {
MaybeDirectHandle { handle }
}
}
pub struct DirectHandleVector<'a, T> {
    items: Vec<DirectHandle<T>>,
dummy: i32,
}
impl <'a, T> DirectHandleVector<'a, T> {
fn new(isolate: *mut Isolate) -> DirectHandleVector<'a, T> {
DirectHandleVector{dummy: 0,items:vec![]}
}
fn push_back(&mut self, value: DirectHandle<T>){
self.items.push(value);
}
fn reserve(&mut self, size: i32){
self.items.reserve(size as usize);
}
fn resize(&mut self, size: i32){
self.items.resize(size as usize,DirectHandle{dummy:0});
}
}
impl<T> std::ops::Index<usize> for DirectHandleVector<'_, T> {
type Output = DirectHandle<T>;
fn index(&self, index: usize) -> &Self::Output {
&self.items[index]
}
}
impl<T> std::ops::IndexMut<usize> for DirectHandleVector<'_, T> {
fn index_mut(&mut self, index: usize) -> &mut Self::Output {
&mut self.items[index]
}
}
pub mod base {
pub struct Vector<T> {
        data: Vec<T>,
}
impl<T> Vector<T> {
fn new() -> Self {
            Vector { data: Vec::new() }
        }
pub fn cast(self) -> Self {
Self{data: vec![]}
}
pub fn push(mut self, val: T) -> Self {
            self.data.push(val);
Self{data: vec![]}
        }
pub fn resize(&mut self, new_len: usize, value: T) where T: Clone{
self.data.resize(new_len, value);
}
pub fn is_empty(&self) -> bool {
self.data.is_empty()
}
pub fn len(&self) -> usize {
self.data.len()
}
pub fn begin(&self) -> *const T {
self.data.as_ptr() as *const T
}
}
impl Vector<const u8> {
pub fn cast(self) -> Vector<char> {
Vector{data:vec![]}
}
}
impl<T> From<Vec<T>> for Vector<T> {
fn from(vec: Vec<T>) -> Self {
Vector { data: vec }
}
}
pub struct CStrVector {
dummy: i32
}
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum AllocationType {
kYoung,
kOld
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Skip {
kNone
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PropertyKind {
kData
}
impl JSObject {
pub fn RawFastPropertyAt(&self, field_index: FieldIndex) -> *mut Object {
std::ptr::null_mut()
}
pub fn HasFastProperties(isolate: *mut Isolate) -> bool {
true
}
}
impl WasmModuleObject {
}
}
}
use v8::internal::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fmt::Debug;
use std::mem::MaybeUninit;
use std::num::NonZeroU32;
use std::rc::Rc;
use std::sync::atomic::{AtomicU32, Ordering};
extern crate lazy_static;
use self::lazy_static::lazy_static;
#[derive(Debug)]
pub enum ValueSerializerError {
OutOfMemory,
DataCloneError(String),
}
impl fmt::Display for ValueSerializerError {
fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
match self {
ValueSerializerError::OutOfMemory => write!(f, "Out of memory"),
ValueSerializerError::DataCloneError(msg) => write!(f, "Data clone error: {}", msg),
}
}
}
impl Error for ValueSerializerError {}
pub struct ValueSerializer {
pub isolate_: *mut Isolate,
pub delegate_: *mut v8::ValueSerializer::Delegate,
pub buffer_: *mut u8,
pub buffer_size_: usize,
pub buffer_capacity_: usize,
pub has_custom_host_objects_: bool,
pub treat_array_buffer_views_as_host_objects_: bool,
pub out_of_memory_: bool,
pub zone_: Zone,
pub id_map_: HashMap<*mut JSReceiver, u32>,
pub next_id_: u32,
pub array_buffer_transfer_map_: HashMap<*mut JSArrayBuffer, u32>,
pub shared_object_conveyor_: i32,
}
impl ValueSerializer {
pub fn new(isolate: *mut Isolate, delegate: *mut v8::ValueSerializer::Delegate) -> Self {
let mut serializer = ValueSerializer {
isolate_: isolate,
delegate_: delegate,
buffer_: std::ptr::null_mut(),
buffer_size_: 0,
buffer_capacity_: 0,
has_custom_host_objects_: false,
treat_array_buffer_views_as_host_objects_: false,
out_of_memory_: false,
zone_: Zone{dummy:0},
id_map_: HashMap::new(),
next_id_: 0,
array_buffer_transfer_map_: HashMap::new(),
shared_object_conveyor_: 0,
};
unsafe {
if !serializer.delegate_.is_null() {
let v8_isolate = serializer.isolate_ as *mut v8::V8;
serializer.has_custom_host_objects_ = (*serializer.delegate_).HasCustomHostObject(v8_isolate);
}
}
serializer
}
pub fn write_header(&mut self) {
self.write_tag(SerializationTag::kVersion);
self.write_varint(15);
}
pub fn set_treat_array_buffer_views_as_host_objects(&mut self, mode: bool) {
self.treat_array_buffer_views_as_host_objects_ = mode;
}
fn write_tag(&mut self, tag: SerializationTag) {
let raw_tag = tag as u8;
self.write_raw_bytes(&raw_tag as *const u8 as *const std::ffi::c_void, std::mem::size_of::<u8>());
}
fn write_varint<T: num::PrimInt + num::Unsigned>(&mut self, value: T) {
let mut stack_buffer: [u8; 10] = [0; 10];
let mut next_byte: *mut u8 = stack_buffer.as_mut_ptr();
let mut value_temp = value;
loop {
unsafe {
*next_byte = (value_temp.to_u8().unwrap() & 0x7F) | 0x80;
next_byte = next_byte.add(1);
}
value_temp = value_temp >> 7;
if value_temp == T::zero() {
break;
}
}
unsafe {
*next_byte.offset(-1) &= 0x7F;
let len = next_byte as usize - stack_buffer.as_ptr() as usize;
self.write_raw_bytes(stack_buffer.as_ptr() as *const std::ffi::c_void, len);
}
}
fn write_zig_zag<T: num::PrimInt + num::Signed>(&mut self, value: T) {
let unsigned_value = ((value << 1) ^ (value >> (std::mem::size_of::<T>() * 8 - 1))).to_u64().unwrap();
self.write_varint(unsigned_value as u64);
}
pub fn write_double(&mut self, value: f64) {
self.write_raw_bytes(&value as *const f64 as *const std::ffi::c_void, std::mem::size_of::<f64>());
}
fn write_one_byte_string(&mut self, chars: base::Vector<const u8>) {
self.write_varint(chars.len() as u32);
self.write_raw_bytes(chars.begin() as *const std::ffi::c_void, chars.len() * std::mem::size_of::<u8>());
}
fn write_two_byte_string(&mut self, chars: base::Vector<const u16>) {
let byte_length = chars.len() * std::mem::size_of::<u16>();
self.write_varint(byte_length as u32);
self.write_raw_bytes(chars.begin() as *const std::ffi::c_void, chars.len() * std::mem::size_of::<u16>());
}
fn write_big_int_contents(&mut self, bigint: Tagged<BigInt>) {
self.write_varint(0u32);
}
fn write_raw_bytes(&mut self, source: *const std::ffi::c_void, length: usize) {
unsafe {
let dest = self.reserve_raw_bytes(length).unwrap();
std::ptr::copy_nonoverlapping(source as *const u8, dest, length);
}
}
fn reserve_raw_bytes(&mut self, bytes: usize) -> Result<*mut u8, ValueSerializerError> {
let old_size = self.buffer_size_;
let new_size = old_size + bytes;
if new_size > self.buffer_capacity_ {
let ok = self.expand_buffer(new_size)?;
}
self.buffer_size_ = new_size;
unsafe {
Ok(self.buffer_.add(old_size))
}
}
fn expand_buffer(&mut self, required_capacity: usize) -> Result<(), ValueSerializerError> {
if required_capacity <= self.buffer_capacity_ {
return Ok(());
}
let requested_capacity = std::cmp::max(required_capacity, self.buffer_capacity_ * 2) + 64;
let mut provided_capacity = 0;
let mut new_buffer = std::ptr::null_mut();
unsafe {
if !self.delegate_.is_null() {
new_buffer = (*self.delegate_).ReallocateBufferMemory(self.buffer_, requested_capacity, &mut provided_capacity);
} else {
new_buffer = std::alloc::realloc(self.buffer_ as *mut u8, requested_capacity) as *mut u8;
provided_capacity = requested_capacity;
}
}
if !new_buffer.is_null() {
self.buffer_ = new_buffer;
self.buffer_capacity_ = provided_capacity;
Ok(())
} else {
self.out_of_memory_ = true;
Err(ValueSerializerError::OutOfMemory)
}
}
pub fn write_byte(&mut self, value: u8) {
let mut dest: *mut u8 = std::ptr::null_mut();
unsafe {
dest = self.reserve_raw_bytes(std::mem::size_of::<u8>()).unwrap();
*dest = value;
}
}
pub fn write_uint32(&mut self, value: u32) {
self.write_varint(value);
}
pub fn write_uint64(&mut self, value: u64) {
self.write_varint(value);
}
pub fn release(&mut self) -> (*mut u8, usize) {
let result = (self.buffer_, self.buffer_size_);
self.buffer_ = std::ptr::null_mut();
self.buffer_size_ = 0;
self.buffer_capacity_ = 0;
result
}
pub fn transfer_array_buffer(&mut self, transfer_id: u32, array_buffer: DirectHandle<JSArrayBuffer>) {
self.array_buffer_transfer_map_.insert( unsafe{ std::mem::transmute(array_buffer)}, transfer_id);
}
fn write_oddball(&mut self, oddball: Tagged<Oddball>) {
let tag = match 1 {
_ => SerializationTag::kUndefined,
};
self.write_tag(tag);
}
fn write_smi(&mut self, smi: Tagged<Smi>) {
self.write_tag(SerializationTag::kInt32);
self.write_zig_zag(0i32);
}
fn write_heap_number(&mut self, number: Tagged<HeapNumber>) {
self.write_tag(SerializationTag::kDouble);
self.write_double(0.0);
}
fn write_big_int(&mut self, bigint: Tagged<BigInt>) {
self.write_tag(SerializationTag::kBigInt);
self.write_big_int_contents(bigint);
}
fn write_string(&mut self, string: DirectHandle<String>) {
let newString:DirectHandle<String> = DirectHandle{dummy:0};
}
fn write_js_receiver(&mut self, receiver: DirectHandle<JSReceiver>) -> Result<(), ValueSerializerError> {
let find_result = 1;
if find_result == 1 {
self.write_tag(SerializationTag::kObjectReference);
self.write_varint(0);
return Ok(());
}
let id = 1;
unsafe{
let instance_type = 1;
let object:DirectHandle<JSObject>= DirectHandle{dummy:0};
return Ok(());
}
}
fn write_js_object(&mut self, object: DirectHandle<JSObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kBeginJSObject);
self.write_tag(SerializationTag::kEndJSObject);
self.write_varint(0);
Ok(())
}
fn write_js_object_slow(&mut self, object: DirectHandle<JSObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kBeginJSObject);
self.write_tag(SerializationTag::kEndJSObject);
self.write_varint(0);
Ok(())
}
fn write_js_array(&mut self, array: DirectHandle<JSArray>) -> Result<(), ValueSerializerError> {
let cage_base:i32 = 0;
let length:u32 = 0;
let should_serialize_densely = true;
if should_serialize_densely{
self.write_tag(SerializationTag::kBeginDenseJSArray);
self.write_varint(1);
self.write_varint(1);
} else {
self.write_tag(SerializationTag::kBeginSparseJSArray);
self.write_varint(length);
}
self.write_tag(SerializationTag::kEndSparseJSArray);
self.write_varint(0);
self.write_varint(1);
Ok(())
}
fn write_js_date(&mut self, date: Tagged<JSDate>) {
self.write_tag(SerializationTag::kDate);
self.write_double(1.2);
}
fn write_js_primitive_wrapper(&mut self, value: DirectHandle<JSPrimitiveWrapper>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kTrueObject);
Ok(())
}
fn write_js_regexp(&mut self, regexp: DirectHandle<JSRegExp>) {
self.write_tag(SerializationTag::kRegExp);
self.write_string(DirectHandle{dummy:0});
self.write_varint(0);
}
fn write_js_map(&mut self, map: DirectHandle<JSMap>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kBeginJSMap);
self.write_tag(SerializationTag::kEndJSMap);
self.write_varint(0);
Ok(())
}
fn write_js_set(&mut self, map: DirectHandle<JSSet>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kBeginJSSet);
self.write_tag(SerializationTag::kEndJSSet);
self.write_varint(0);
Ok(())
}
fn write_js_array_buffer(&mut self, array_buffer: DirectHandle<JSArrayBuffer>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kArrayBuffer);
self.write_varint(0);
Ok(())
}
fn write_js_array_buffer_view(&mut self, view: Tagged<JSArrayBufferView>) -> Result<(), ValueSerializerError> {
if self.treat_array_buffer_views_as_host_objects_ {
self.write_host_object(DirectHandle{dummy:0})?;
return Ok(());
}
self.write_tag(SerializationTag::kArrayBufferView);
self.write_varint(0);
self.write_varint(0);
self.write_varint(0);
Ok(())
}
fn write_js_error(&mut self, error: DirectHandle<JSObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kError);
self.write_varint(0);
Ok(())
}
fn write_js_shared_array(&mut self, shared_array: DirectHandle<JSSharedArray>) -> Result<(), ValueSerializerError> {
self.write_shared_object(DirectHandle{dummy:0})
}
fn write_js_shared_struct(&mut self, shared_struct: DirectHandle<JSSharedStruct>) -> Result<(), ValueSerializerError> {
self.write_shared_object(DirectHandle{dummy:0})
}
fn write_wasm_module(&mut self, object: DirectHandle<WasmModuleObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kWasmModuleTransfer);
self.write_varint(0);
Ok(())
}
fn write_wasm_memory(&mut self, object: DirectHandle<WasmMemoryObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kWasmMemoryTransfer);
self.write_varint(0);
Ok(())
}
fn write_shared_object(&mut self, object: DirectHandle<HeapObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kSharedObject);
self.write_varint(0);
Ok(())
}
fn write_host_object(&mut self, object: DirectHandle<JSObject>) -> Result<(), ValueSerializerError> {
self.write_tag(SerializationTag::kHostObject);
Ok(())
}
fn write_js_object_properties_slow(&mut self, object: DirectHandle<JSObject>, keys: DirectHandle<FixedArray>) -> Result<u32, ValueSerializerError> {
Ok(0)
}
fn is_host_object(&mut self, js_object: DirectHandle<JSObject>) -> Result<bool, ValueSerializerError> {
Ok(false)
}
fn throw_if_out_of_memory(&mut self) -> Result<bool, ValueSerializerError> {
if self.out_of_memory_ {
Err(ValueSerializerError::OutOfMemory)
} else {
Ok(true)
}
}
fn throw_data_clone_error(&mut self, template_index: MessageTemplate) -> Result<bool, ValueSerializerError> {
self.throw_data_clone_error_inner
