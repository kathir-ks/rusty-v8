// Converted from V8 C++ source files:
// Header: source-position-table.h
// Implementation: source-position-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub struct BitField8<T, const OFFSET: usize, const SIZE: usize> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T, const OFFSET: usize, const SIZE: usize> BitField8<T, OFFSET, SIZE> {
        pub const kMax: u8 = (1 << SIZE) - 1;
        pub const kMask: u8 = Self::kMax << OFFSET;
        pub const kSize: usize = SIZE;

        pub fn encode(value: T) -> u8
        where
            T: Into<u8>,
        {
            (value.into() << OFFSET) & Self::kMask
        }

        pub fn decode(bits: u8) -> T
        where
            T: From<u8>,
        {
            ((bits & Self::kMask) >> OFFSET).into()
        }
    }
    pub struct Vector<T> {
        _phantom: std::marker::PhantomData<T>,
    }
    impl<T> Vector<T> {
        pub fn length(&self) -> i32 {
            0
        }
    }

    pub struct OwnedVector<T> {
        data: Vec<T>,
    }

    impl<T: Copy> OwnedVector<T> {
        pub fn new() -> Self {
            OwnedVector { data: Vec::new() }
        }

        pub fn push(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn data(&self) -> &[T] {
            &self.data
        }

        pub fn as_vector(&self) -> Vector<T> {
            Vector {
                _phantom: std::marker::PhantomData,
            }
        }
    }

    impl OwnedVector<u8> {
        pub fn from_vec(vec: Vec<u8>) -> Self {
            OwnedVector { data: vec }
        }
    }

    pub fn OwnedCopyOf(bytes: &ZoneVector<u8>) -> OwnedVector<u8> {
        OwnedVector {
            data: bytes.data.clone(),
        }
    }
}

pub mod codegen {
    use crate::codegen::assembler::SourcePosition;

    pub struct TrustedByteArray {}
    pub mod assembler {
        #[derive(Clone, Copy)]
        pub struct SourcePosition {
            raw_value: i64,
        }
        impl SourcePosition {
            pub fn IsKnown(&self) -> bool {
                true
            }
            pub fn FromRaw(raw: i64) -> Self {
                SourcePosition { raw_value: raw }
            }
            pub fn raw(&self) -> i64 {
                self.raw_value
            }
            pub fn IsJavaScript(&self) -> bool {
                true
            }
            pub fn IsExternal(&self) -> bool {
                false
            }
        }
    }
}

pub mod common {
    pub mod assert_scope {
        pub struct AssertScope {}
    }
    pub mod checks {
        pub fn unreachable() -> ! {
            panic!("unreachable");
        }
    }
    pub mod globals {
        pub const kBitsPerByte: i32 = 8;
        pub const kFunctionEntryBytecodeOffset: i32 = 0;
    }
}
pub mod heap {
    use crate::codegen::TrustedByteArray;
    pub struct LocalFactory {}

    impl LocalFactory {
        pub fn NewTrustedByteArray(&self, size: i32) -> Handle<TrustedByteArray> {
            Handle {
                _phantom: std::marker::PhantomData,
            }
        }
        pub fn empty_trusted_byte_array(&self) -> Handle<TrustedByteArray> {
            Handle {
                _phantom: std::marker::PhantomData,
            }
        }
    }
}
pub mod objects {
    pub struct Object {}
    pub mod objects_inl {
        pub struct ObjectInl {}
    }
}

pub mod zone {
    use std::vec::Vec;
    pub struct Zone {}
    pub struct ZoneVector<T> {
        data: Vec<T>,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> ZoneVector<T> {
        pub fn new(_zone: &Zone) -> Self {
            ZoneVector {
                data: Vec::new(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn push_back(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn empty(&self) -> bool {
            self.data.is_empty()
        }

        pub fn size(&self) -> usize {
            self.data.len()
        }

        pub fn data(&self) -> &[T] {
            &self.data
        }

        pub fn begin(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }

        pub fn iter(&self) -> std::slice::Iter<T> {
            self.data.iter()
        }
    }

    pub struct ZoneList<T> {
        _phantom: std::marker::PhantomData<T>,
    }
}

pub struct V8_EXPORT_PRIVATE {}
#[derive(Clone, Copy)]
pub struct Tagged<T> {
    _phantom: std::marker::PhantomData<T>,
}

#[derive(Clone, Copy)]
pub struct Handle<T> {
    _phantom: std::marker::PhantomData<T>,
}
impl<T> Handle<T> {
    pub fn is_null(&self) -> bool {
        true
    }
}

pub struct Isolate {
    factory: heap::LocalFactory,
}
impl Isolate {
    pub fn factory(&mut self) -> &mut heap::LocalFactory {
        &mut self.factory
    }
}
pub struct LocalIsolate {
    factory: heap::LocalFactory,
}
impl LocalIsolate {
    pub fn factory(&mut self) -> &mut heap::LocalFactory {
        &mut self.factory
    }
}
fn MemCopy(dest: *mut u8, src: &[u8], size: usize) {
    let dest_slice = unsafe { std::slice::from_raw_parts_mut(dest, size) };
    dest_slice.copy_from_slice(src);
}
struct PositionTableEntry {
    source_position: i64,
    code_offset: i32,
    is_statement: bool,
}

impl PositionTableEntry {
    fn new() -> Self {
        PositionTableEntry {
            source_position: 0,
            code_offset: crate::common::globals::kFunctionEntryBytecodeOffset,
            is_statement: false,
        }
    }

    fn new_with_values(offset: i32, source: i64, statement: bool) -> Self {
        PositionTableEntry {
            source_position: source,
            code_offset: offset,
            is_statement: statement,
        }
    }
}

pub struct SourcePositionTableBuilder {
    mode_: RecordingMode,
    bytes_: zone::ZoneVector<u8>,
    #[cfg(debug_assertions)]
    raw_entries_: zone::ZoneVector<PositionTableEntry>,
    previous_: PositionTableEntry,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum RecordingMode {
    OMIT_SOURCE_POSITIONS,
    LAZY_SOURCE_POSITIONS,
    RECORD_SOURCE_POSITIONS,
}

impl SourcePositionTableBuilder {
    pub fn new(zone: &mut zone::Zone, mode: RecordingMode) -> Self {
        SourcePositionTableBuilder {
            mode_: mode,
            bytes_: zone::ZoneVector::new(zone),
            #[cfg(debug_assertions)]
            raw_entries_: zone::ZoneVector::new(zone),
            previous_: PositionTableEntry::new(),
        }
    }

    pub fn AddPosition(
        &mut self,
        code_offset: usize,
        source_position: codegen::assembler::SourcePosition,
        is_statement: bool,
    ) {
        if self.Omit() {
            return;
        }
        assert!(source_position.IsKnown());
        let offset = code_offset as i32;
        self.AddEntry(PositionTableEntry::new_with_values(
            offset,
            source_position.raw(),
            is_statement,
        ));
    }

    fn AddEntry(&mut self, entry: PositionTableEntry) {
        let mut tmp = entry;
        SubtractFromEntry(&mut tmp, &self.previous_);
        EncodeEntry(&mut self.bytes_, &tmp);
        self.previous_ = entry;

        #[cfg(debug_assertions)]
        self.raw_entries_.push_back(entry);
    }

    pub fn ToSourcePositionTable<IsolateT>(
        &mut self,
        isolate: &mut IsolateT,
    ) -> Handle<codegen::TrustedByteArray>
    where
        IsolateT: FactoryInterface,
    {
        if self.bytes_.empty() {
            return isolate.factory().empty_trusted_byte_array();
        }
        assert!(!self.Omit());

        let table = isolate
            .factory()
            .NewTrustedByteArray(self.bytes_.size() as i32);
        MemCopy(
            unsafe { std::mem::transmute(table) },
            self.bytes_.data(),
            self.bytes_.size(),
        );

        #[cfg(debug_assertions)]
        {
            let mut it = SourcePositionTableIterator::new_all(table);
            CheckTableEquals(&self.raw_entries_, &mut it);
            self.mode_ = RecordingMode::OMIT_SOURCE_POSITIONS;
        }

        table
    }

    pub fn ToSourcePositionTableVector(&mut self) -> base::OwnedVector<u8> {
        if self.bytes_.empty() {
            return base::OwnedVector::new();
        }
        assert!(!self.Omit());

        let table = base::OwnedCopyOf(&self.bytes_);

        #[cfg(debug_assertions)]
        {
            let mut it = SourcePositionTableIterator::new_vector_all(table.as_vector());
            CheckTableEquals(&self.raw_entries_, &mut it);
            self.mode_ = RecordingMode::OMIT_SOURCE_POSITIONS;
        }

        table
    }

    pub fn Omit(&self) -> bool {
        self.mode_ != RecordingMode::RECORD_SOURCE_POSITIONS
    }

    pub fn Lazy(&self) -> bool {
        self.mode_ == RecordingMode::LAZY_SOURCE_POSITIONS
    }
}
trait FactoryInterface {
    fn factory(&mut self) -> &mut heap::LocalFactory;
}
impl FactoryInterface for Isolate {
    fn factory(&mut self) -> &mut heap::LocalFactory {
        &mut self.factory
    }
}
impl FactoryInterface for LocalIsolate {
    fn factory(&mut self) -> &mut heap::LocalFactory {
        &mut self.factory
    }
}

struct PositionTableEntryIterator<'a> {
    raw_table_: &'a [u8],
    index_: usize,
    current_: PositionTableEntry,
}

impl<'a> PositionTableEntryIterator<'a> {
    fn new(raw_table: &'a [u8]) -> Self {
        PositionTableEntryIterator {
            raw_table_: raw_table,
            index_: 0,
            current_: PositionTableEntry::new(),
        }
    }

    fn next(&mut self) -> Option<PositionTableEntry> {
        if self.index_ >= self.raw_table_.len() {
            return None;
        }

        let mut tmp = PositionTableEntry::new();
        DecodeEntryToPositionTableEntry(self.raw_table_, &mut self.index_, &mut tmp);
        AddAndSetEntry(&mut self.current_, &tmp);
        Some(self.current_)
    }
}

pub struct SourcePositionTableIterator {
    raw_table_: Vec<u8>,
    table_: Handle<codegen::TrustedByteArray>,
    index_: i32,
    current_: PositionTableEntry,
    iteration_filter_: IterationFilter,
    function_entry_filter_: FunctionEntryFilter,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum IterationFilter {
    kJavaScriptOnly = 0,
    kExternalOnly = 1,
    kAll = 2,
}

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum FunctionEntryFilter {
    kSkipFunctionEntry = 0,
    kDontSkipFunctionEntry = 1,
}
pub struct IndexAndPositionState {
    index_: i32,
    position_: PositionTableEntry,
    iteration_filter_: IterationFilter,
    function_entry_filter_: FunctionEntryFilter,
}

impl SourcePositionTableIterator {
    fn new(
        byte_array: Handle<codegen::TrustedByteArray>,
        iteration_filter: IterationFilter,
        function_entry_filter: FunctionEntryFilter,
    ) -> Self {
        let mut iterator = SourcePositionTableIterator {
            raw_table_: Vec::new(),
            table_: byte_array,
            index_: 0,
            current_: PositionTableEntry::new(),
            iteration_filter_: iteration_filter,
            function_entry_filter_: function_entry_filter,
        };
        iterator.Initialize();
        iterator
    }
    fn new_all(byte_array: Handle<codegen::TrustedByteArray>) -> Self {
        SourcePositionTableIterator::new(
            byte_array,
            IterationFilter::kAll,
            FunctionEntryFilter::kDontSkipFunctionEntry,
        )
    }
    fn new_vector_all(bytes: base::Vector<u8>) -> Self {
        SourcePositionTableIterator {
            raw_table_: Vec::new(),
            table_: Handle {
                _phantom: std::marker::PhantomData,
            },
            index_: 0,
            current_: PositionTableEntry::new(),
            iteration_filter_: IterationFilter::kAll,
            function_entry_filter_: FunctionEntryFilter::kDontSkipFunctionEntry,
        }
    }

    fn new_tagged(
        byte_array: Tagged<codegen::TrustedByteArray>,
        iteration_filter: IterationFilter,
        function_entry_filter: FunctionEntryFilter,
    ) -> Self {
        let mut iterator = SourcePositionTableIterator {
            raw_table_: VectorFromByteArrayToVec(byte_array),
            table_: Handle {
                _phantom: std::marker::PhantomData,
            },
            index_: 0,
            current_: PositionTableEntry::new(),
            iteration_filter_: iteration_filter,
            function_entry_filter_: function_entry_filter,
        };
        iterator.Initialize();
        iterator
    }

    fn new_vector(
        bytes: base::Vector<u8>,
        iteration_filter: IterationFilter,
        function_entry_filter: FunctionEntryFilter,
    ) -> Self {
        let mut iterator = SourcePositionTableIterator {
            raw_table_: Vec::new(),
            table_: Handle {
                _phantom: std::marker::PhantomData,
            },
            index_: 0,
            current_: PositionTableEntry::new(),
            iteration_filter_: iteration_filter,
            function_entry_filter_: function_entry_filter,
        };
        iterator.Initialize();
        iterator
    }

    fn Initialize(&mut self) {
        self.Advance();
        if self.function_entry_filter_ == FunctionEntryFilter::kSkipFunctionEntry
            && self.current_.code_offset == crate::common::globals::kFunctionEntryBytecodeOffset
            && !self.done()
        {
            self.Advance();
        }
    }

    pub fn Advance(&mut self) {
        let bytes = if self.table_.is_null() {
            &self.raw_table_
        } else {
            return;
        };
        if self.done() {
            return;
        }

        let mut filter_satisfied = false;
        while !self.done() && !filter_satisfied {
            if self.index_ >= bytes.len() as i32 {
                self.index_ = Self::kDone;
            } else {
                let mut tmp = PositionTableEntry::new();
                DecodeEntryToPositionTableEntry(bytes, &mut self.index_ as &mut i32, &mut tmp);
                AddAndSetEntry(&mut self.current_, &tmp);
                let p = self.source_position();
                filter_satisfied = (self.iteration_filter_ == IterationFilter::kAll)
                    || (self.iteration_filter_ == IterationFilter::kJavaScriptOnly
                        && p.IsJavaScript())
                    || (self.iteration_filter_ == IterationFilter::kExternalOnly && p.IsExternal());
            }
        }
    }

    pub fn code_offset(&self) -> i32 {
        assert!(!self.done());
        self.current_.code_offset
    }

    pub fn source_position(&self) -> codegen::assembler::SourcePosition {
        assert!(!self.done());
        codegen::assembler::SourcePosition::FromRaw(self.current_.source_position)
    }

    pub fn is_statement(&self) -> bool {
        assert!(!self.done());
        self.current_.is_statement
    }

    pub fn done(&self) -> bool {
        self.index_ == Self::kDone
    }

    pub fn GetState(&self) -> IndexAndPositionState {
        IndexAndPositionState {
            index_: self.index_,
            position_: self.current_.clone(),
            iteration_filter_: self.iteration_filter_,
            function_entry_filter_: self.function_entry_filter_,
        }
    }

    pub fn RestoreState(&mut self, saved_state: &IndexAndPositionState) {
        self.index_ = saved_state.index_;
        self.current_ = saved_state.position_.clone();
        self.iteration_filter_ = saved_state.iteration_filter_;
        self.function_entry_filter_ = saved_state.function_entry_filter_;
    }

    const kDone: i32 = -1;
}

fn AddAndSetEntry(value: &mut PositionTableEntry, other: &PositionTableEntry) {
    value.code_offset += other.code_offset;
    if value.code_offset != crate::common::globals::kFunctionEntryBytecodeOffset {
        assert!(value.code_offset >= 0);
    }
    value.source_position += other.source_position;
    assert!(value.source_position >= 0);
    value.is_statement = other.is_statement;
}

fn SubtractFromEntry(value: &mut PositionTableEntry, other: &PositionTableEntry) {
    value.code_offset -= other.code_offset;
    value.source_position -= other.source_position;
}

fn EncodeInt(bytes: &mut zone::ZoneVector<u8>, mut value: i64) {
    // Zig-zag encoding.
    let kShift = (std::mem::size_of::<i64>() as i32) * crate::common::globals::kBitsPerByte - 1;
    value = ((value as u64 << 1) ^ (value >> kShift)) as i64;
    assert!(value >= 0);
    let mut encoded = value as u64;
    let mut more;
    loop {
        more = encoded > base::BitField8::<u8, 0, 7>::kMax as u64;
        let current =
            base::BitField8::<bool, 7, 1>::encode(more) | base::BitField8::<u32, 0, 7>::encode((encoded & base::BitField8::<u8, 0, 7>::kMask as u64) as u32);
        bytes.push_back(current);
        encoded >>= base::BitField8::<u8, 0, 7>::kSize;
        if !more {
            break;
        }
    }
}

fn EncodeEntry(bytes: &mut zone::ZoneVector<u8>, entry: &PositionTableEntry) {
    assert!(entry.code_offset >= 0);
    EncodeInt(
        bytes,
        if entry.is_statement {
            entry.code_offset as i64
        } else {
            -(entry.code_offset as i64) - 1
        },
    );
    EncodeInt(bytes, entry.source_position);
}

fn DecodeInt<T>(bytes: &[u8], index: &mut i32) -> i64 {
    let mut current;
    let mut shift = 0;
    let mut decoded: i64 = 0;
    let mut more;
    loop {
        current = bytes[*index as usize];
        *index += 1;
        decoded |= (base::BitField8::<u32, 0, 7>::decode(current) as i64) << shift;
        more = base::BitField8::<bool, 7, 1>::decode(current);
        shift += base::BitField8::<u8, 0, 7>::kSize;
        if !more {
            break;
        }
    }
    assert!(decoded >= 0);
    ((decoded >> 1) ^ (-(decoded & 1))) as i64
}

fn DecodeEntryToPositionTableEntry(bytes: &[u8], index: &mut i32, entry: &mut PositionTableEntry) {
    let tmp = DecodeInt::<i32>(bytes, index);
    if tmp >= 0 {
        entry.is_statement = true;
        entry.code_offset = tmp as i32;
    } else {
        entry.is_statement = false;
        entry.code_offset = -(tmp + 1) as i32;
    }
    entry.source_position = DecodeInt::<i64>(bytes, index);
}

fn VectorFromByteArrayToVec(byte_array: Tagged<codegen::TrustedByteArray>) -> Vec<u8> {
    Vec::new()
}

#[cfg(debug_assertions)]
fn CheckTableEquals(
    raw_entries: &zone::ZoneVector<PositionTableEntry>,
    encoded: &mut SourcePositionTableIterator,
) {
    let mut raw = raw_entries.begin();
    while !encoded.done() {
        assert!(raw != raw_entries.end());
        assert_eq!(encoded.code_offset(), raw.code_offset);
        assert_eq!(
            encoded.source_position().raw(),
            raw.source_position
        );
        assert_eq!(encoded.is_statement(), raw.is_statement);
        encoded.Advance();
        raw = raw_entries.iter().next().unwrap();
    }
    assert_eq!(raw_entries.iter().next(), None);
}
