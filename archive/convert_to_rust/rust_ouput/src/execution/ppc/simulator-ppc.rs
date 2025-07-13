// Converted from V8 C++ source files:
// Header: simulator-ppc.h
// Implementation: simulator-ppc.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod ppc {
use std::sync::Mutex;
#[derive(PartialEq, Eq, Hash)]
pub struct Address {}
pub struct Isolate {}
pub struct Instruction {}
pub struct Redirection {}
pub struct V8 {}
pub struct StackVisitor {}
pub mod base {
use std::sync::Mutex;
#[derive(PartialEq, Eq, Hash)]
pub struct Address {}
pub struct Isolate {}
pub struct Instruction {}
pub struct Redirection {}
pub struct V8 {}
pub struct StackVisitor {}
#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutexDeadlockDetectionMode {
    kIgnore,
    kFatal,
}
pub fn set_mutex_deadlock_detection_mode(mode: MutexDeadlockDetectionMode) {}
pub mod bits {
pub fn rotate_left32(value: u32, shift: i32) -> u32 {
value.rotate_left(shift as u32)
}
#[allow(dead_code)]
pub fn signed_mul_high64(x: i64, y: i64) -> i64 {
(x as i128 * y as i128 >> 64) as i64
}
#[allow(dead_code)]
pub fn unsigned_mul_high64(x: u64, y: u64) -> u64 {
(x as u128 * y as u128 >> 64) as u64
}
}
pub mod os {
use std::mem::size_of;
#[cfg(target_os = "linux")]
pub fn activation_frame_alignment() -> usize {
16
}
#[cfg(not(target_os = "linux"))]
pub fn activation_frame_alignment() -> usize {
0
}
#[allow(dead_code)]
pub fn debug_break() {}
}
}
pub mod codegen {
pub mod ppc {
#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Register {
    NoReg,
    R0,
    Sp,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7,
    R8,
    R9,
    R10,
    R11,
    R12,
    R13,
    R14,
    R15,
    R16,
    R17,
    R18,
    R19,
    R20,
    R21,
    R22,
    R23,
    R24,
    R25,
    R26,
    R27,
    R28,
    R29,
    R30,
    Fp,
    KNumGPRs,
}
pub enum DoubleRegister {
    D0,
    D1,
    D2,
    D3,
    D4,
    D5,
    D6,
    D7,
    D8,
    D9,
    D10,
    D11,
    D12,
    D13,
    D14,
    D15,
    D16,
    D17,
    D18,
    D19,
    D20,
    D21,
    D22,
    D23,
    D24,
    D25,
    D26,
    D27,
    D28,
    D29,
    D30,
    D31,
    KNumFPRs,
}
}
}
pub mod heap {
pub mod base {
#[derive(PartialEq, Eq, Hash)]
pub struct Address {}
pub struct StackVisitor {}
}
}
pub mod internal {
use std::sync::Mutex;
#[derive(PartialEq, Eq, Hash)]
pub struct Address {}
pub struct Isolate {
stack_limit: usize,
}
impl Isolate {
pub fn stack_guard(&mut self) -> &mut StackGuard {
&mut StackGuard {
stack_limit: &mut self.stack_limit,
}
}
}
pub struct StackGuard<'a> {
stack_limit: &'a mut usize,
}
impl<'a> StackGuard<'a> {
pub fn adjust_stack_limit_for_simulator(&mut self) {}
}
pub struct Instruction {}
pub struct Redirection {}
pub struct V8 {}
pub struct StackVisitor {}
const kStopCodeMask: u32 = 0x00FFFFFF;
const ABI_RETURNS_OBJECT_PAIRS_IN_REGS: bool = false;
const ABI_USES_FUNCTION_DESCRIPTORS: bool = false;
const ABI_CALL_VIA_IP: bool = false;
pub const kFPRoundingModeMask: i32 = 0x03000000;
pub const kMaxUInt32: u64 = u32::MAX as u64;
pub const kMinInt: i64 = i32::MIN as i64;
pub const kMaxInt: i64 = i32::MAX as i64;
pub const kMinUInt32: u64 = 0;
pub const rtCallRedirInstr: u32 = 0;
pub const kCallRtRedirected: u32 = 0;
pub const kNumRequiredStackFrameSlots: i32 = 2;
pub const kStackFrameExtraParamSlot: usize = 0;
pub const kBitsPerByte: u32 = 8;
pub const VXSOFT: i32 = 28;
pub const VXSQRT: i32 = 29;
pub const VXCVI: i32 = 30;
pub const kStopCode: u32 = 0;
pub const kMaxStopCode: u32 = 0;
pub const kSimd128Size: usize = 16;
pub const kMaxInt8: i16 = i8::MAX as i16;
pub const kMinInt8: i16 = i8::MIN as i16;
pub const kMaxUInt8: i16 = u8::MAX as i16;
pub const kMinInt16: i32 = i16::MIN as i32;
pub const kMaxInt16: i32 = i16::MAX as i32;
pub const kMaxUInt16: i32 = u16::MAX as i32;
const CRWIDTH: i32 = 4;
struct String {}
impl String{
}
}
}
#[allow(dead_code)]
const KB: usize = 1024;
pub const kSystemPointerSize: usize = 8;
pub mod utils {
pub mod ostreams {
pub struct StdoutStream {}
impl StdoutStream {
pub fn new() -> Self {
StdoutStream {}
}
impl StdoutStream {
pub fn print(&mut self, _value: &str) {}
}
}
}
}
pub struct CachePage {
validity_map_: [i8; Self::kValidityMapSize],
data_: [i8; Self::kPageSize],
}
impl CachePage {
pub const LINE_VALID: i32 = 0;
pub const LINE_INVALID: i32 = 1;
pub const kPageShift: i32 = 12;
pub const kPageSize: usize = 1 << Self::kPageShift;
pub const kPageMask: usize = Self::kPageSize - 1;
pub const kLineShift: i32 = 2;
pub const kLineLength: usize = 1 << Self::kLineShift;
pub const kLineMask: usize = Self::kLineLength - 1;
const kValidityMapSize: usize = Self::kPageSize >> Self::kLineShift;
pub fn new() -> Self {
CachePage {
data_: [0; Self::kPageSize],
validity_map_: [Self::LINE_INVALID as i8; Self::kValidityMapSize],
}
}
pub fn validity_byte(&mut self, offset: usize) -> &mut i8 {
&mut self.validity_map_[offset >> Self::kLineShift]
}
pub fn cached_data(&mut self, offset: usize) -> &mut i8 {
&mut self.data_[offset]
}
}
struct GlobalMonitor {
mutex: Mutex<()>,
access_state_: MonitorAccess,
tagged_addr_: usize,
size_: TransactionSize,
thread_id_: ThreadId,
}
impl GlobalMonitor {
fn new() -> Self {
GlobalMonitor {
mutex: Mutex::new(()),
access_state_: MonitorAccess::Open,
tagged_addr_: 0,
size_: TransactionSize::None,
thread_id_: ThreadId::Invalid(),
}
}
fn clear(&mut self) {
self.access_state_ = MonitorAccess::Open;
self.tagged_addr_ = 0;
self.size_ = TransactionSize::None;
self.thread_id_ = ThreadId::Invalid();
}
fn notify_load_excl(&mut self, addr: usize, size: TransactionSize, thread_id: ThreadId) {
self.access_state_ = MonitorAccess::Exclusive;
self.tagged_addr_ = addr;
self.size_ = size;
self.thread_id_ = thread_id;
}
fn notify_store(&mut self, addr: usize, size: TransactionSize, thread_id: ThreadId) {
if self.access_state_ == MonitorAccess::Exclusive {
let transaction_start = addr;
let transaction_end = addr + size as usize;
let exclusive_transaction_start = self.tagged_addr_;
let exclusive_transaction_end = self.tagged_addr_ + self.size_ as usize;
let is_not_overlapped = transaction_end < exclusive_transaction_start || exclusive_transaction_end < transaction_start;
if !is_not_overlapped && self.thread_id_ != thread_id {
self.clear();
}
}
}
fn notify_store_excl(&mut self, addr: usize, size: TransactionSize, thread_id: ThreadId) -> bool {
let permission = self.access_state_ == MonitorAccess::Exclusive && addr == self.tagged_addr_ && self.size_ == size && self.thread_id_ == thread_id;
self.clear();
return permission;
}
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MonitorAccess {
Open,
Exclusive,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum TransactionSize {
None = 0,
Byte = 1,
HalfWord = 2,
Word = 4,
DWord = 8,
}
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct ThreadId {}
impl ThreadId {
const InvalidFn: fn() -> Self = || ThreadId {};
const Invalid: Self = ThreadId {};
}
pub struct Simulator {
instruction_tracing_: bool,
registers_: [i64; 32],
condition_reg_: i32,
fp_condition_reg_: i32,
special_reg_lr_: i64,
special_reg_pc_: i64,
special_reg_ctr_: i64,
special_reg_xer_: i32,
fp_registers_: [f64; 32],
simd_registers_: [SimdrT; 32],
stack_: *mut u8,
pc_modified_: bool,
icount_: i32,
last_debugger_input_: *mut i8,
break_pc_: *mut Instruction,
break_instr_: i32,
isolate_: *mut Isolate,
watched_stops_: [StopCountAndDesc; 256],
}
#[derive(Copy, Clone)]
union SimdrT {
int8: [i8; 16],
uint8: [u8; 16],
int16: [i16; 8],
uint16: [u16; 8],
int32: [i32; 4],
uint32: [u32; 4],
int64: [i64; 2],
uint64: [u64; 2],
f32: [f32; 4],
f64: [f64; 2],
}
#[derive(Copy, Clone)]
struct StopCountAndDesc {
count: u32,
desc: *mut i8,
}
impl Simulator {
const kNumOfWatchedStops: u32 = 256;
const kStopDisabledBit: u32 = 1 << 31;
pub fn new(isolate: *mut Isolate) -> Self {
Simulator {
instruction_tracing_: false,
registers_: [0; 32],
condition_reg_: 0,
fp_condition_reg_: 0,
special_reg_lr_: 0,
special_reg_pc_: 0,
special_reg_ctr_: 0,
special_reg_xer_: 0,
fp_registers_: [0.0; 32],
simd_registers_: [SimdrT { int8: [0; 16] }; 32],
stack_: std::ptr::null_mut(),
pc_modified_: false,
icount_: 0,
last_debugger_input_: std::ptr::null_mut(),
break_pc_: std::ptr::null_mut(),
break_instr_: 0,
isolate_: isolate,
watched_stops_: [StopCountAndDesc { count: 0, desc: std::ptr::null_mut() }; 256],
}
}
pub fn set_register(&mut self, reg: usize, value: i64) {
self.registers_[reg] = value;
}
pub fn get_register(&self, reg: usize) -> i64 {
self.registers_[reg]
}
pub fn set_d_register_from_double(&mut self, dreg: usize, dbl: f64) {
self.fp_registers_[dreg] = dbl;
}
pub fn get_double_from_d_register(&self, dreg: usize) -> f64 {
self.fp_registers_[dreg]
}
pub fn set_d_register(&mut self, dreg: usize, value: i64) {
self.fp_registers_[dreg] = f64::from_bits(value as u64);
}
pub fn get_d_register(&self, dreg: usize) -> i64 {
self.fp_registers_[dreg].to_bits() as i64
}
pub fn set_pc(&mut self, value: i64) {
self.special_reg_pc_ = value;
self.pc_modified_ = true;
}
pub fn get_pc(&self) -> i64 {
self.special_reg_pc_
}
pub fn has_bad_pc(&self) -> bool {
self.special_reg_pc_ == -1 || self.special_reg_pc_ == -2
}
pub fn get_sp(&self) -> Address {
Address {}
}
pub fn get_double_from_register_pair(&self, reg: usize) -> f64 {
0.0
}
pub fn get_lr(&self) -> i64 {
self.special_reg_lr_
}
pub fn instruction_tracing_enabled(&self) -> bool {
self.instruction_tracing_
}
pub fn toggle_instruction_tracing(&mut self) {
self.instruction_tracing_ = !self.instruction_tracing_;
}
pub fn set_cr0(&mut self, _result: i64, _setso: bool) {}
}
}
