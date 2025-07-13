// Converted from V8 C++ source files:
// Header: js-promise-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use crate::v8::internal::AsyncTaskIdBits;
use crate::v8::internal::IsolateForSandbox;
use crate::v8::internal::Object;
use crate::v8::internal::Tagged;
use crate::v8::internal::V8;
use std::io;
pub mod v8 {
pub mod internal {
use crate::v8::internal::Object;
use crate::v8::internal::Tagged;
pub struct AsyncTaskIdBits {}
impl AsyncTaskIdBits {
const kMax: u32 = 0xFFFFFFFF;
fn decode(_flags: u32) -> u32 {
1
}
}
pub struct AsyncTaskIdBits {}
pub struct Object {}
pub struct Tagged<T> {
dummy: i32,
}
pub struct V8 {}
impl<T> Tagged<T> {
pub fn cast(&self) -> &T {
}
}
pub struct IsolateForSandbox {}
pub struct Code {}
}
}
mod objects {
pub struct JSPromise {
flags: u32,
reactions_or_result: i32,
}
impl JSPromise {
pub fn flags(&self) -> u32 {
self.flags
}
pub fn set_flags(&mut self, flags: u32) {
self.flags = flags;
}
pub fn reactions_or_result(&self) -> i32 {
self.reactions_or_result
}
}
}
pub mod objects_inl {
use crate::objects::JSPromise;
pub trait InternalJSPromise {
fn has_handler(&self) -> bool;
fn set_has_handler(&mut self, value: bool);
fn is_silent(&self) -> bool;
fn set_is_silent(&mut self, value: bool);
}
impl InternalJSPromise for JSPromise {
fn has_handler(&self) -> bool {
(self.flags & (1 << 0)) != 0
}
fn set_has_handler(&mut self, value: bool) {
if value {
self.flags |= 1 << 0;
} else {
self.flags &= !(1 << 0);
}
}
fn is_silent(&self) -> bool {
(self.flags & (1 << 1)) != 0
}
fn set_is_silent(&mut self, value: bool) {
if value {
self.flags |= 1 << 1;
} else {
self.flags &= !(1 << 1);
}
}
}
}
pub mod promise {
const kPending: i32 = 0;
}
pub mod js_promise {
use crate::objects::JSPromise;
use crate::v8::internal::AsyncTaskIdBits;
use crate::v8::internal::Object;
use crate::v8::internal::Tagged;
use crate::v8::internal::V8;
use crate::promise;
const kInvalidAsyncTaskId: u32 = 0;
pub trait InternalJSPromise {
fn get_next_async_task_id(async_task_id: u32) -> u32;
fn has_async_task_id(&self) -> bool;
fn async_task_id(&self) -> u32;
fn set_async_task_id(&mut self, id: u32);
fn result(&self) -> Tagged<Object>;
fn reactions(&self) -> Tagged<Object>;
}
impl InternalJSPromise for JSPromise {
fn get_next_async_task_id(mut async_task_id: u32) -> u32 {
loop {
async_task_id = async_task_id.wrapping_add(1);
async_task_id &= AsyncTaskIdBits::kMax;
if async_task_id != kInvalidAsyncTaskId {
break;
}
}
async_task_id
}
fn has_async_task_id(&self) -> bool {
self.async_task_id() != kInvalidAsyncTaskId
}
fn async_task_id(&self) -> u32 {
AsyncTaskIdBits::decode(self.flags())
}
fn set_async_task_id(&mut self, id: u32) {
self.set_flags(id);
}
fn result(&self) -> Tagged<Object> {
assert_ne!(promise::kPending, self.reactions_or_result());
Tagged { dummy: 1 }
}
fn reactions(&self) -> Tagged<Object> {
assert_eq!(promise::kPending, self.reactions_or_result());
Tagged { dummy: 1 }
}
}
}
