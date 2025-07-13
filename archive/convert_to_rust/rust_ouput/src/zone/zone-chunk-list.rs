// Converted from V8 C++ source files:
// Header: zone-chunk-list.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
pub mod iterator {
pub struct iterator {}
}
}
pub mod common {
pub mod globals {}
}
pub mod utils {
pub mod memcopy {
pub fn MemCopy(dest: *mut u8, src: *const u8, num: usize) {
unsafe {
std::ptr::copy_nonoverlapping(src, dest, num);
}
}
}
}
pub mod zone {
pub use crate::v8::internal::Zone;
}
pub mod v8 {
pub mod internal {
use std::{
cmp::min,
marker::PhantomData,
mem::MaybeUninit,
ptr::null_mut,
};
use crate::{
base::iterator,
utils::memcopy::MemCopy,
};
#[derive(Debug)]
pub enum ZoneAllocationError {
OutOfMemory,
}
#[derive(Debug)]
pub struct Zone {
}
impl Zone {
pub fn new() -> Self {
Zone { }
}
pub fn allocate<T>(&mut self) -> *mut T {
unsafe {
let layout = std::alloc::Layout::new::<T>();
let ptr = std::alloc::alloc(layout);
if ptr.is_null() {
panic!("Failed to allocate memory in Zone");
}
ptr as *mut T
}
}
pub fn allocate_bytes(&mut self, size: usize) -> *mut u8 {
unsafe {
let layout = std::alloc::Layout::from_size_align(size, 1).unwrap();
let ptr = std::alloc::alloc(layout);
if ptr.is_null() {
panic!("Failed to allocate memory in Zone");
}
ptr
}
}
}
pub struct ZoneObject {}
impl ZoneObject {
pub fn new() -> Self {
ZoneObject {}
}
}
#[derive(PartialEq, Eq, Debug)]
pub struct ZoneChunkListIterator<T, const BACKWARDS: bool, const MODIFIABLE: bool> {
current_: *mut Chunk<T>,
position_: u32,
_marker: PhantomData<T>,
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> Clone for ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {
fn clone(&self) -> Self {
ZoneChunkListIterator {
current_: self.current_,
position_: self.position_,
_marker: PhantomData,
}
}
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> Copy for ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {
}
pub struct ZoneChunkList<T> {
zone_: *mut Zone,
size_: usize,
front_: *mut Chunk<T>,
last_nonempty_: *mut Chunk<T>,
_marker: PhantomData<T>,
}
impl<T> Drop for ZoneChunkList<T> {
fn drop(&mut self) {
unsafe {
let mut current = self.front_;
while !current.is_null() {
let next = (*current).next_;
let layout = std::alloc::Layout::new::<Chunk<T>>();
std::alloc::dealloc(current as *mut u8, layout);
current = next;
}
}
}
}
impl<T> ZoneChunkList<T> {
pub const K_INITIAL_CHUNK_CAPACITY: u32 = 8;
pub const K_MAX_CHUNK_CAPACITY: u32 = 256;
pub fn new(zone: *mut Zone) -> Self {
ZoneChunkList {
zone_: zone,
size_: 0,
front_: null_mut(),
last_nonempty_: null_mut(),
_marker: PhantomData,
}
}
pub fn size(&self) -> usize {
self.size_
}
pub fn empty(&self) -> bool {
self.size() == 0
}
pub fn front(&mut self) -> &mut T {
unsafe { &mut *self.begin() }
}
pub fn front_const(&self) -> &T {
unsafe { &*self.begin_const() }
}
pub fn back(&mut self) -> &mut T {
unsafe {
if self.size_ == 0 {
panic!("Cannot get back of empty list");
}
&mut *self.rbegin()
}
}
pub fn back_const(&self) -> &T {
unsafe {
if self.size_ == 0 {
panic!("Cannot get back of empty list");
}
&*self.rbegin_const()
}
}
pub fn push_back(&mut self, item: &T)
where
T: Copy,
{
unsafe {
if self.last_nonempty_.is_null() {
self.front_ = self.new_chunk(Self::K_INITIAL_CHUNK_CAPACITY);
self.last_nonempty_ = self.front_;
} else if !(*self.last_nonempty_).next_.is_null() && (*self.last_nonempty_).full() {
self.last_nonempty_ = (*self.last_nonempty_).next_;
} else if (*self.last_nonempty_).full() {
let chunk = self.new_chunk(Self::next_chunk_capacity((*self.last_nonempty_).capacity_));
(*self.last_nonempty_).next_ = chunk;
(*chunk).previous_ = self.last_nonempty_;
self.last_nonempty_ = chunk;
}
(*self.last_nonempty_).items()[(*self.last_nonempty_).position_ as usize] = *item;
(*self.last_nonempty_).position_ += 1;
self.size_ += 1;
}
}
pub fn push_front(&mut self, item: &T)
where
T: Copy,
{
unsafe {
if self.front_.is_null() {
self.front_ = self.new_chunk(Self::K_INITIAL_CHUNK_CAPACITY);
self.last_nonempty_ = self.front_;
} else if (*self.front_).full() {
let chunk = self.new_chunk(Self::next_chunk_capacity((*self.front_).capacity_));
(*self.front_).previous_ = chunk;
(*chunk).next_ = self.front_;
self.front_ = chunk;
}
let end = (*self.front_).items().add((*self.front_).position_ as usize);
let start = (*self.front_).items();
std::ptr::copy(start, start.add(1), (*self.front_).position_ as usize);
(*self.front_).items()[0] = *item;
(*self.front_).position_ += 1;
self.size_ += 1;
}
}
pub fn rewind(&mut self, limit: usize) {
if limit >= self.size() {
return;
}
let seek_result = self.seek_index(limit);
unsafe {
(*seek_result.chunk_).position_ = seek_result.chunk_index_ as u32;
self.last_nonempty_ = seek_result.chunk_;
let mut current = (*seek_result.chunk_).next_;
while !current.is_null() {
(*current).position_ = 0;
current = (*current).next_;
}
}
self.size_ = limit;
}
pub fn find(&mut self, index: usize) -> ZoneChunkListIterator<T, false, true> {
let seek_result = self.seek_index(index);
ZoneChunkListIterator {
current_: seek_result.chunk_,
position_: seek_result.chunk_index_ as u32,
_marker: PhantomData,
}
}
pub fn find_const(&self, index: usize) -> ZoneChunkListIterator<T, false, false> {
let seek_result = self.seek_index(index);
ZoneChunkListIterator {
current_: seek_result.chunk_ as *mut Chunk<T>,
position_: seek_result.chunk_index_ as u32,
_marker: PhantomData,
}
}
pub fn split_at(&mut self, mut split_begin: ZoneChunkListIterator<T, false, true>) -> ZoneChunkList<T>
where
T: Copy,
{
let mut result = ZoneChunkList::new(self.zone_);
if split_begin == self.end() {
return result;
}
if split_begin == self.begin() {
std::mem::swap(self, &mut result);
return result;
}
unsafe {
let split_chunk = split_begin.current_;
let chunk_split_begin = (*split_chunk).items().add(split_begin.position_ as usize);
let chunk_split_end = (*split_chunk).items().add((*split_chunk).position_ as usize);
let new_chunk_size = (chunk_split_end as usize - chunk_split_begin as usize) / std::mem::size_of::<T>();
let new_chunk_capacity = min(
Self::K_MAX_CHUNK_CAPACITY as usize,
new_chunk_size.next_power_of_two(),
);
let new_chunk = self.new_chunk(new_chunk_capacity as u32);
std::ptr::copy_nonoverlapping(
chunk_split_begin,
(*new_chunk).items(),
new_chunk_size * std::mem::size_of::<T>(),
);
(*new_chunk).position_ = new_chunk_size as u32;
(*split_chunk).position_ = split_begin.position_;
result.front_ = new_chunk;
result.last_nonempty_ = if self.last_nonempty_ == split_chunk {
new_chunk
} else {
self.last_nonempty_
};
(*new_chunk).next_ = (*split_chunk).next_;
if !(*new_chunk).next_.is_null() {
(*(*new_chunk).next_).previous_ = new_chunk;
}
self.last_nonempty_ = split_chunk;
(*split_chunk).next_ = null_mut();
let mut new_size = 0;
let mut chunk = self.front_;
while chunk != split_chunk {
new_size += (*chunk).size() as usize;
chunk = (*chunk).next_;
}
new_size += (*split_chunk).size() as usize;
result.size_ = self.size() - new_size;
self.size_ = new_size;
}
return result;
}
pub fn append(&mut self, other: &mut ZoneChunkList<T>)
where
T: Copy,
{
unsafe {
if other.front_.is_null() {
return;
}
if self.last_nonempty_.is_null() {
self.front_ = other.front_;
} else {
(*self.last_nonempty_).next_ = other.front_;
(*other.front_).previous_ = self.last_nonempty_;
}
self.last_nonempty_ = other.last_nonempty_;
self.size_ += other.size_;
other.front_ = null_mut();
other.last_nonempty_ = null_mut();
other.size_ = 0;
}
}
pub fn copy_to(&self, ptr: *mut T)
where
T: Copy,
{
unsafe {
let mut current = self.front_;
let mut current_ptr = ptr;
while !current.is_null() {
let start = (*current).items() as *mut u8;
let end = (*current).items().add((*current).position_ as usize) as *mut u8;
let bytes = end as usize - start as usize;
MemCopy(current_ptr as *mut u8, start, bytes);
current_ptr = current_ptr.add((*current).position_ as usize);
current = (*current).next_;
}
}
}
pub fn begin(&mut self) -> ZoneChunkListIterator<T, false, true> {
ZoneChunkListIterator::begin(self)
}
pub fn end(&mut self) -> ZoneChunkListIterator<T, false, true> {
ZoneChunkListIterator::end(self)
}
pub fn rbegin(&mut self) -> ZoneChunkListIterator<T, true, true> {
ZoneChunkListIterator::begin(self)
}
pub fn rend(&mut self) -> ZoneChunkListIterator<T, true, true> {
ZoneChunkListIterator::end(self)
}
pub fn begin_const(&self) -> ZoneChunkListIterator<T, false, false> {
ZoneChunkListIterator::begin(self)
}
pub fn end_const(&self) -> ZoneChunkListIterator<T, false, false> {
ZoneChunkListIterator::end(self)
}
pub fn rbegin_const(&self) -> ZoneChunkListIterator<T, true, false> {
ZoneChunkListIterator::begin(self)
}
pub fn rend_const(&self) -> ZoneChunkListIterator<T, true, false> {
ZoneChunkListIterator::end(self)
}
pub fn swap(&mut self, other: &mut ZoneChunkList<T>) {
assert_eq!(self.zone_, other.zone_);
std::mem::swap(&mut self.size_, &mut other.size_);
std::mem::swap(&mut self.front_, &mut other.front_);
std::mem::swap(&mut self.last_nonempty_, &mut other.last_nonempty_);
}
fn new_chunk(&mut self, capacity: u32) -> *mut Chunk<T> {
unsafe {
let memory = (*self.zone_).allocate_bytes(std::mem::size_of::<Chunk<T>>() + (capacity as usize) * std::mem::size_of::<T>()) as *mut Chunk<T>;
(*memory).capacity_ = capacity;
(*memory).position_ = 0;
(*memory).next_ = null_mut();
(*memory).previous_ = null_mut();
memory
}
}
fn next_chunk_capacity(previous_capacity: u32) -> u32 {
min(previous_capacity * 2, Self::K_MAX_CHUNK_CAPACITY)
}
struct SeekResult<T> {
chunk_: *mut Chunk<T>,
chunk_index_: usize,
}
fn seek_index(&self, index: usize) -> SeekResult<T> {
assert!(index < self.size());
let mut current = self.front_;
let mut current_index = index;
unsafe {
while current_index >= (*current).capacity_ as usize {
current_index -= (*current).capacity_ as usize;
current = (*current).next_;
}
assert!(current_index < (*current).capacity_ as usize);
}
SeekResult {
chunk_: current,
chunk_index_: current_index,
}
}
}
#[repr(C)]
pub struct Chunk<T> {
capacity_: u32,
position_: u32,
next_: *mut Chunk<T>,
previous_: *mut Chunk<T>,
_marker: PhantomData<T>,
}
impl<T> Chunk<T> {
pub fn items(&mut self) -> *mut T {
unsafe { (self as *mut Self).add(1) as *mut T }
}
pub fn items_const(&self) -> *const T {
unsafe { (self as *const Self).add(1) as *const T }
}
pub fn size(&self) -> u32 {
assert!(self.position_ <= self.capacity_);
self.position_
}
pub fn empty(&self) -> bool {
self.size() == 0
}
pub fn full(&self) -> bool {
self.size() == self.capacity_
}
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {
fn begin(list: &ZoneChunkList<T>) -> Self {
if !BACKWARDS {
if list.front_.is_null() {
return ZoneChunkListIterator {
current_: null_mut(),
position_: 0,
_marker: PhantomData,
};
}
return ZoneChunkListIterator {
current_: list.front_,
position_: 0,
_marker: PhantomData,
};
}
if list.empty() {
return ZoneChunkListIterator {
current_: null_mut(),
position_: 0,
_marker: PhantomData,
};
}
unsafe {
assert!(!(*list.last_nonempty_).empty());
return ZoneChunkListIterator {
current_: list.last_nonempty_,
position_: (*list.last_nonempty_).position_ - 1,
_marker: PhantomData,
};
}
}
fn end(list: &ZoneChunkList<T>) -> Self {
if BACKWARDS {
return ZoneChunkListIterator {
current_: null_mut(),
position_: 0,
_marker: PhantomData,
};
}
if list.empty() {
return ZoneChunkListIterator {
current_: null_mut(),
position_: 0,
_marker: PhantomData,
};
}
unsafe {
return ZoneChunkListIterator {
current_: (*list.last_nonempty_).next_,
position_: 0,
_marker: PhantomData,
};
}
}
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE>
where T: Copy
{
pub unsafe fn move_<const MOVE_BACKWARD: bool>(&mut self) {
if MOVE_BACKWARD {
if self.position_ == 0 {
if self.current_.is_null() {
return;
}
self.current_ = (*self.current_).previous_;
self.position_ = if !self.current_.is_null() {
(*self.current_).position_ - 1
} else {
0
};
} else {
self.position_ -= 1;
}
} else {
self.position_ += 1;
if self.current_.is_null() {
return;
}
if self.position_ >= (*self.current_).position_ {
self.current_ = (*self.current_).next_;
self.position_ = 0;
}
}
}
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> std::iter::Iterator for ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE>
where T: Copy
{
type Item = T;
fn next(&mut self) -> Option<Self::Item> {
if BACKWARDS {
return None;
}
unsafe {
if self.current_.is_null() {
return None;
}
if self.position_ >= (*self.current_).position_ {
return None;
}
let item = *((*self.current_).items().add(self.position_ as usize));
self.move_::<false>();
return Some(item);
}
}
}
impl<T, const BACKWARDS: bool, const MODIFIABLE: bool> ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE>
where T: Copy
{
}
unsafe impl<T: Send, const BACKWARDS: bool, const MODIFIABLE: bool> Send for ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {}
unsafe impl<T: Sync, const BACKWARDS: bool, const MODIFIABLE: bool> Sync for ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {}
impl<'a, T, const BACKWARDS: bool, const MODIFIABLE: bool> ZoneChunkListIterator<T, BACKWARDS, MODIFIABLE> {
}
impl<T> std::cmp::PartialEq for ZoneChunkListIterator<T, false, true> {
fn eq(&self, other: &Self) -> bool {
self.current_ == other.current_ && self.position_ == other.position_
}
}
impl<T> std::cmp::Eq for ZoneChunkListIterator<T, false, true> {}
}
}
