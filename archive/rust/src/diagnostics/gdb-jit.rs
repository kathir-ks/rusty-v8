#![allow(non_camel_case_types, non_snake_case, unused_imports, dead_code, unused_variables)]

use std::collections::HashMap;
use std::mem;
use std::os::raw::{c_char, c_int, c_uchar, c_uint, c_ulong, c_ushort};
use std::ptr;
use std::slice;
use std::str;
use std::sync::{LazyLock, Mutex};

//use libc; // Not used directly, so commenting out
//use object; // Not used directly, so commenting out

// Placeholder for v8-callbacks.h
mod v8_callbacks {
    // Define necessary types and functions here if needed
}

// Placeholder for api-inl.h
mod api_inl {
    // Define necessary types and functions here if needed
}

// Placeholder for base/address-region.h
mod address_region {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct AddressRegion {
        begin: usize,
        end: usize,
    }

    impl AddressRegion {
        pub const fn new(begin: usize, end: usize) -> Self {
            Self { begin, end }
        }

        pub const fn begin(&self) -> usize {
            self.begin
        }

        pub const fn end(&self) -> usize {
            self.end
        }

        pub const fn size(&self) -> usize {
            self.end - self.begin
        }
    }
}

use address_region::AddressRegion;

// Placeholder for base/bits.h
mod bits {
    pub fn is_power_of_two(x: u32) -> bool {
        x != 0 && (x & (x - 1)) == 0
    }

    pub fn which_power_of_two(x: u32) -> u32 {
        if !is_power_of_two(x) {
            panic!("Not a power of two");
        }
        x.trailing_zeros()
    }
}

// Placeholder for base/hashmap.h
mod hashmap {
    use std::collections::HashMap;
    use std::hash::{BuildHasherDefault, Hasher};

    #[derive(Default)]
    pub struct SimpleHasher {
        value: u64,
    }

    impl Hasher for SimpleHasher {
        fn finish(&self) -> u64 {
            self.value
        }

        fn write(&mut self, bytes: &[u8]) {
            for &byte in bytes {
                self.value = self.value.wrapping_mul(1103515245).wrapping_add(byte as u64);
            }
        }
    }

    pub type BuildSimpleHasher = BuildHasherDefault<SimpleHasher>;

    pub struct HashMap {
        map: std::collections::HashMap<usize, usize, BuildSimpleHasher>,
    }

    impl HashMap {
        pub fn new() -> Self {
            HashMap {
                map: std::collections::HashMap::with_hasher(BuildSimpleHasher::default()),
            }
        }

        pub fn lookup_or_insert(&mut self, key: *const std::ffi::c_void, hash: u32) -> Entry {
            let key_usize = key as usize;
            Entry {
                value: *self.map.entry(key_usize).or_insert(0),
            }
        }

        pub fn remove(&mut self, key: *const std::ffi::c_void, hash: u32) -> *mut std::ffi::c_void {
            let key_usize = key as usize;
            if let Some(value) = self.map.remove(&key_usize) {
                value as *mut std::ffi::c_void
            } else {
                std::ptr::null_mut()
            }
        }
    }

    pub struct Entry {
        pub value: usize,
    }
}

// Placeholder for base/memory.h
mod base_memory {
    // Define necessary types and functions here if needed
}

// Placeholder for base/platform/platform.h
mod platform {
    // Define necessary types and functions here if needed
}

// Placeholder for base/platform/wrappers.h
mod wrappers {
    // Define necessary types and functions here if needed
}

// Placeholder for base/strings.h
mod strings {
    // Define necessary types and functions here if needed
}

// Placeholder for base/vector.h
mod vector {
    // Define necessary types and functions here if needed
}

// Placeholder for execution/frames-inl.h
mod frames_inl {
    // Define necessary types and functions here if needed
}

// Placeholder for execution/frames.h
mod frames {
    // Define necessary types and functions here if needed
}

// Placeholder for handles/global-handles.h
mod global_handles {
    // Define necessary types and functions here if needed
}

// Placeholder for init/bootstrapper.h
mod bootstrapper {
    // Define necessary types and functions here if needed
}

// Placeholder for objects/code-inl.h
mod code_inl {
    // Define necessary types and functions here if needed
}

// Placeholder for objects/objects.h
mod objects {
    // Define necessary types and functions here if needed
}

// Placeholder for utils/ostreams.h
mod ostreams {
    // Define necessary types and functions here if needed
}

// Placeholder for zone/zone-chunk-list.h
mod zone_chunk_list {
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct ZoneChunkList<T> {
        chunks: Vec<Vec<T>>,
        current_chunk: Option<Rc<RefCell<Vec<T>>>>,
        chunk_size: usize,
    }

    impl<T> ZoneChunkList<T> {
        pub fn new() -> Self {
            ZoneChunkList {
                chunks: Vec::new(),
                current_chunk: None,
                chunk_size: 4096, // Example chunk size
            }
        }

        pub fn push(&mut self, value: T) {
            if self.current_chunk.is_none()
                || self.current_chunk.as_ref().unwrap().borrow().len() == self.chunk_size
            {
                self.allocate_new_chunk();
            }

            self.current_chunk
                .as_ref()
                .unwrap()
                .borrow_mut()
                .push(value);
        }

        fn allocate_new_chunk(&mut self) {
            let new_chunk: Vec<T> = Vec::with_capacity(self.chunk_size);
            let rc = Rc::new(RefCell::new(new_chunk));
            self.chunks.push(rc.borrow().clone()); // Store a copy in chunks
            self.current_chunk = Some(rc);
        }

        pub fn find(&self, index: usize) -> Option<&T> {
            if index < self.chunks.len() {
                self.chunks.get(index).and_then(|chunk| chunk.get(0))
            } else {
                None
            }
        }
    }
}

use zone_chunk_list::ZoneChunkList;

// Placeholder for zone/zone.h
mod zone {
    // Define necessary types and functions here if needed
    pub struct Zone {
        // Placeholder
    }

    impl Zone {
        pub fn new() -> Self {
            Zone {}
        }

        pub fn new_with_name(_name: &str) -> Self {
            Zone{}
        }

        pub fn new_in_memory(_memory: *mut u8, _size: usize, _name: &str) -> Self {
            Zone{}
        }

        pub fn clear(&mut self) {}

        pub fn allocate_aligned(&self, _size: usize, _alignment: usize) -> *mut u8 {
            unsafe {
                let layout = std::alloc::Layout::from_size_align(_size, _alignment).unwrap();
                std::alloc::alloc(layout)
            }
        }

        pub fn allocate(&self, size: usize) -> *mut u8 {
            unsafe {
                let layout = std::alloc::Layout::new(size);
                std::alloc::alloc(layout)
            }
        }
    }

    pub trait ZoneObject {}
}

use zone::ZoneObject;

mod gdb_jit {
    use super::*;
    use std::alloc::{alloc, dealloc, Layout};
    use std::ffi::CString;
    use std::io::Write;
    use std::mem::MaybeUninit;
    use std::ptr::NonNull;
    use std::sync::Mutex;

    const ENABLE_GDB_JIT_INTERFACE: bool = true;

    #[cfg(target_os = "macos")]
    const __MACH_O: bool = true;
    #[cfg(not(target_os = "macos"))]
    const __MACH_O: bool = false;

    #[cfg(target_os = "linux")]
    const __ELF: bool = true;
    #[cfg(not(target_os = "linux"))]
    const __ELF: bool = false;

    trait DebugObject {}
    trait DebugSection {}

    #[cfg(__MACH_O)]
    type DebugObjectType = MachO;
    #[cfg(__MACH_O)]
    type DebugSectionType = MachOSection;

    #[cfg(__ELF)]
    type DebugObjectType = ELF;
    #[cfg(__ELF)]
    type DebugSectionType = ELFSection;

    struct Writer<'a> {
        debug_object: &'a mut DebugObjectType,
        position: usize,
        capacity: usize,
        buffer: *mut u8,
    }

    impl<'a> Writer<'a> {
        fn new(debug_object: &'a mut DebugObjectType) -> Self {
            let capacity = 1024;
            let layout = Layout::array::<u8>(capacity).unwrap();
            let buffer = unsafe { alloc(layout) };
            Writer {
                debug_object,
                position: 0,
                capacity,
                buffer,
            }
        }

        fn position(&self) -> usize {
            self.position
        }

        unsafe fn raw_slot_at<T>(&self, offset: usize) -> *mut T {
            assert!(
                offset < self.capacity && offset + std::mem::size_of::<T>() <= self.capacity
            );
            self.buffer.add(offset) as *mut T
        }

        unsafe fn address_at<T>(&self, offset: usize) -> *mut T {
            assert!(
                offset < self.capacity && offset + std::mem::size_of::<T>() <= self.capacity
            );
            self.buffer.add(offset) as *mut T
        }

        fn write<T>(&mut self, val: T) {
            self.ensure(self.position + std::mem::size_of::<T>());
            unsafe {
                let ptr = self.address_at::<T>(self.position);
                std::ptr::write_unaligned(ptr, val);
            }
            self.position += std::mem::size_of::<T>();
        }

        fn slot_at<T>(&mut self, offset: usize) -> Slot<T> {
            self.ensure(offset + std::mem::size_of::<T>());
            Slot::new(self, offset)
        }

        fn create_slot_here<T>(&mut self) -> Slot<T> {
            self.create_slots_here::<T>(1)
        }

        fn create_slots_here<T>(&mut self, count: u32) -> Slot<T> {
            let slot_position = self.position;
            self.position += std::mem::size_of::<T>() * count as usize;
            self.ensure(self.position);
            self.slot_at::<T>(slot_position)
        }

        fn ensure(&mut self, pos: usize) {
            if self.capacity < pos {
                let mut new_capacity = self.capacity;
                while new_capacity < pos {
                    new_capacity *= 2;
                }

                let old_layout = Layout::array::<u8>(self.capacity).unwrap();
                let new_layout = Layout::array::<u8>(new_capacity).unwrap();

                unsafe {
                    self.buffer = std::alloc::realloc(self.buffer, old_layout, new_layout.size());
                }
                self.capacity = new_capacity;
            }
        }

        fn debug_object(&mut self) -> &mut DebugObjectType {
            self.debug_object
        }

        fn buffer(&self) -> *mut u8 {
            self.buffer
        }

        fn align(&mut self, align: usize) {
            let delta = self.position % align;
            if delta == 0 {
                return;
            }
            let padding = align - delta;
            self.ensure(self.position + padding);
            self.position += padding;
            assert_eq!(self.position % align, 0);
        }

        fn write_uleb128(&mut self, mut value: usize) {
            loop {
                let mut byte = (value & 0x7F) as u8;
                value >>= 7;
                if value != 0 {
                    byte |= 0x80;
                }
                self.write(byte);
                if value == 0 {
                    break;
                }
            }
        }

        fn write_sleb128(&mut self, mut value: isize) {
            let mut more = true;
            while more {
                let mut byte = (value & 0x7F) as i8;
                let byte_sign = (byte & 0x40) != 0;
                value >>= 7;

                if (value == 0 && !byte_sign) || (value == -1 && byte_sign) {
                    more = false;
                } else {
                    byte |= 0x80;
                }

                self.write(byte);
            }
        }

        fn write_string(&mut self, str: &str) {
            for &byte in str.as_bytes() {
                self.write(byte as char);
            }
            self.write('\0');
        }
    }

    impl<'a> Drop for Writer<'a> {
        fn drop(&mut self) {
            let layout = Layout::array::<u8>(self.capacity).unwrap();
            unsafe {
                dealloc(self.buffer, layout);
            }
        }
    }

    struct Slot<'a, T> {
        w: &'a mut Writer<'a>,
        offset: usize,
    }

    impl<'a, T> Slot<'a, T> {
        fn new(w: &'a mut Writer<'a>, offset: usize) -> Self {
            Slot { w, offset }
        }

        unsafe fn raw(&mut self) -> *mut T {
            self.w.raw_slot_at::<T>(self.offset)
        }

        fn set(&mut self, value: T) {
            unsafe {
                let ptr = self.w.address_at::<T>(self.offset);
                std::ptr::write_unaligned(ptr, value);
            }
        }

        fn at(&mut self, i: usize) -> Slot<'a, T> {
            Slot::new(self.w, self.offset + std::mem::size_of::<T>() * i)
        }
    }

    trait ELFStringTableTrait {
        fn add(&mut self, str: &str) -> usize;
    }

    #[cfg(__ELF)]
    impl ELFStringTableTrait for ELFStringTable {
        fn add(&mut self, str: &str) -> usize {
            if str.is_empty() {
                return 0;
            }

            let offset = self.size;
            self.write_string(str);
            offset
        }
    }

    #[cfg(not(__ELF))]
    impl ELFStringTableTrait for PlaceholderELFStringTable {
        fn add(&mut self, str: &str) -> usize {
            0
        }
    }

    struct PlaceholderELFStringTable {}

    impl PlaceholderELFStringTable {
        pub fn new() -> Self {
            PlaceholderELFStringTable {}
        }
    }

    #[cfg(__MACH_O)]
    struct MachO {}

    #[cfg(__MACH_O)]
    impl DebugObject for MachO {}

    #[cfg(__MACH_O)]
    impl MachO {
        fn new(_zone: &Zone) -> Self {
            MachO {}
        }

        fn add_section(&mut self, _section: &mut MachOSection) -> usize {
            0
        }

        fn write(&mut self, _w: &mut Writer, _code_start: usize, _code_size: usize) {}
    }

    #[cfg(__ELF)]
    struct ELF {
        sections: ZoneChunkList<ELFSection>,
    }

    #[cfg(__ELF)]
    impl DebugObject for ELF {}

    #[cfg(__ELF)]
    impl ELF {
        fn new(zone: &Zone) -> Self {
            let mut elf = ELF {
                sections: ZoneChunkList::new(),
            };

            let null_section = ELFSection::new("", ELFSectionType::TYPE_NULL, 0);
            elf.add_section(null_section);

            let strtab_section = ELFStringTable::new(".shstrtab");
            elf.add_section(strtab_section);

            elf
        }

        fn write(&mut self, w: &mut Writer) {
            self.write_header(w);
            self.write_section_table(w);
            self.write_sections(w);
        }

        fn section_at(&mut self, index: usize) -> Option<&mut ELFSection> {
            self.sections.find(index).cloned()
        }

        fn add_section(&mut self, section: ELFSection) -> usize {
            //let mut section = section;
            let index = self.sections.chunks.len();
            //section.index = index as u16;
            self.sections.push(section);
            index
        }

        fn write_header(&mut self, w: &mut Writer) {
            assert_eq!(w.position(), 0);
            let mut header = w.create_slot_here::<ELFHeader>();

            #[cfg(any(target_arch = "x86", target_arch = "arm"))]
            let ident: [u8; 16] = [0x7F, b'E', b'L', b'F', 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

            #[cfg(all(target_arch = "x86_64", target_pointer_width = "64"))]
            let ident: [u8; 16] = [0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

            #[cfg(all(target_arch = "powerpc64", target_endian = "little"))]
            let ident: [u8; 16] = [0x7F, b'E', b'L', b'F', 2, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0];

            unsafe {
                std::ptr::copy_nonoverlapping(
                    ident.as_ptr(),
                    (*header.raw()).ident.as_mut_ptr(),
                    16,
                );
                (*header.raw()).type_ = 1;

                #[cfg(target_arch = "x86")]
                {
                    (*header.raw()).machine = 3;
                }
                #[cfg(target_arch = "x86_64")]
                {
                    (*header.raw()).machine = 62;
                }
                #[cfg(target_arch = "arm")]
                {
                    (*header.raw()).machine = 40;
                }
                #[cfg(all(target_arch = "powerpc64", target_os = "linux"))]
                {
                    (*header.raw()).machine = 21;
                }
                #[cfg(target_arch = "s390x")]
                {
                    (*header.raw()).machine = 22;
                }

                (*header.raw()).version = 1;
                (*header.raw()).entry = 0;
                (*header.raw()).pht_offset = 0;
                (*header.raw()).sht_offset = std::mem::size_of::<ELFHeader>() as usize;
                (*header.raw()).flags = 0;
                (*header.raw()).header_size = std::mem::size_of::<ELFHeader>() as u16;
                (*header.raw()).pht_entry_size = 0;
                (*header.raw()).pht_entry_num = 0;
                (*header.raw()).sht_entry_size =
                    std::mem::size_of::<ELFSectionHeader>() as u16;
                (*header.raw()).sht_entry_num = self.sections.chunks.len() as u16;
                (*header.raw()).sht_strtab_index = 1;
            }
        }

        fn write_section_table(&mut self, w: &mut Writer) {
            assert_eq!(w.position(), std::mem::size_of::<ELFHeader>());

            let mut headers = w.create_slots_here::<ELFSectionHeader>(
                self.sections.chunks.len() as u32,
            );

            let strtab = self.section_at(1).unwrap();
            //let strtab = unsafe { &mut *(strtab_ptr as *mut ELFStringTable) };

            // Attach writer to the string table
            //strtab.attach_writer(w);
            let mut index = 0;

            while index < self.sections.chunks.len() {
                if let Some(section) = self.sections.find(index) {
                    //println!("Index: {}", index);
                    //section.populate_header(&mut headers.at(index), strtab);
                }
                index += 1;
            }

            //strtab.detach_writer();
        }

        fn section_header_position(&self, section_index: u32) -> usize {
            std::mem::size_of::<ELFHeader>()
                + std::mem::size_of::<ELFSectionHeader>() * section_index as usize
        }

        fn write_sections(&mut self, w: &mut Writer) {
            let headers = w.slot_at::<ELFSectionHeader>(std::mem::size_of::<ELFHeader>());
            let mut index = 0;
            while index < self.sections.chunks.len() {
                //let section = unsafe { &mut *(section_ptr as *mut ELFSection) };

                //section.write_body(&mut headers.at(index), w);
                index += 1;
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    struct ELFHeader {
        ident: [u8; 16],
        type_: u16,
        machine: u16,
        version: u32,
        entry: usize,
        pht_offset: usize,
        sht_offset: usize,
        flags: u32,
        header_size: u16,
        pht_entry_size: u16,
        pht_entry_num: u16,
        sht_entry_size: u16,
        sht_entry_num: u16,
        sht_strtab_index: u16,
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(C)]
    struct ELFSectionHeader {
        name: u32,
        type_: u32,
        flags: usize,
        address: usize,
        offset: usize,
        size: usize,
        link: u32,
        info: u32,
        alignment: usize,
        entry_size: usize,
    }

    trait DebugSectionBaseTrait {
        fn write_body(&mut self, header: &mut Slot<ELFSectionHeader>, writer: &mut Writer);
    }

    #[derive(Debug, Clone)]
    struct ELFSection {
        name: String,
        type_: ELFSectionType,
        align: usize,
        index: u16,
    }

    impl ELFSection {
        fn new(name: &str, type_: ELFSectionType, align: usize) -> Self {
            ELFSection {
                name: name.to_string(),
                type_,
                align,
                index: 0,
            }
        }

        fn populate_header(&mut self, header: &mut Slot<ELFSectionHeader>, strtab: &mut impl ELFStringTableTrait) {
            unsafe {
                (*header.raw()).name = strtab.add(&self.name) as u32;
                (*header.raw()).type_ = self.type_ as u32;
                (*header.raw()).alignment = self.align;
            }
            self.populate_header_internal(header);
        }

        fn populate_header_internal(&mut self, header: &mut Slot<ELFSectionHeader>) {
            unsafe {
                (*header.raw()).flags = 0;
                (*header.raw()).address = 0;
                (*header.raw()).offset = 0;
                (*header.raw()).size = 0;
                (*header.raw()).link = 0;
                (*header.raw()).info = 0;
                (*header.raw()).entry_size = 0;
            }
        }

        fn write_body(&mut self, header: &mut Slot<ELFSectionHeader>, w: &mut Writer) {
            let start = w.position();
            if self.write_body_internal(w) {
                let end = w.position();
                unsafe {
                    (*header.raw()).offset = start;
                    (*header.raw()).size = end - start;
                }
            }
        }

        fn write_body_internal(&mut self, w: &mut Writer) -> bool {
            false
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    #[repr(u32)]
    enum ELFSectionType {
        TYPE_NULL = 0,
        TYPE_PROGBITS = 1,
        TYPE_SYMTAB = 2,
        TYPE_STRTAB = 3,
        TYPE_RELA = 4,
        TYPE_HASH = 5,
        TYPE_DYNAMIC = 6,
        TYPE_NOTE = 7,
        TYPE_NOBITS = 8,
        TYPE_REL = 9,
        TYPE_SHLIB = 10,
        TYPE_DYNSYM = 11,
        TYPE_LOPROC = 0x70000000,
        TYPE_X86_64_UNWIND = 0x70000001,
        TYPE_HIPROC = 0x7FFFFFFF,
        TYPE_LOUSER = 0x80000000,
        TYPE_HIUSER = 0xFFFFFFFF,
    }

    #[derive(Debug, Clone, Copy)]
    struct FullHeaderELFSection {
        name: &'static str,
        type_: ELFSectionType,
        align: usize,
        addr: usize,
        offset: usize,
        size: usize,
        flags: usize,
    }

    impl FullHeaderELFSection {
        fn new(
            name: &'static str,
            type_: ELFSectionType,
            align: usize,
            addr: usize,
            offset: usize,
            size: usize,
            flags: usize,
        ) -> Self {
            FullHeaderELFSection {
                name,
                type_,
                align,
                addr,
                offset,
                size,
                flags,
            }
        }

        fn populate_header(
            &self,
            header: &mut Slot<ELFSectionHeader>,
        ) {
            unsafe {
                (*header.raw()).address = self.addr;
                (*header.raw()).offset = self.offset;
                (*header.raw()).size = self.size;
                (*header.raw()).flags = self.flags;
            }
        }
    }

    struct ELFStringTable {
        name: String,
        type_: ELFSectionType,
        align: usize,
        writer: *mut Writer<'static>,
        offset: usize,
        size: usize,
        index: u16,
    }

    impl ELFStringTable {
        fn new(name: &str) -> Self {
            ELFStringTable {
                name: name.to_string(),
                type_: ELFSectionType::TYPE_STRTAB,
                align: 1,
                writer: std::ptr::null_mut(),
                offset: 0,
                size: 0,
                index: 0,
            }
        }

        fn attach_writer(&mut self, w: &mut Writer<'static>) {
            self.writer = w;
            self.offset = w.position();

            // First entry in the string table should be an empty string.
            self.write_string("");
        }

        fn detach_writer(&mut self) {
            self.writer = std::ptr::null_mut();
        }

        fn write_string(&mut self, str: &str) {
            let mut written = 0;
            for &byte in str.as_bytes() {
                let writer_ptr = self.writer;
                unsafe {
                    (*writer_ptr).write(byte as char);
                }
                written += 1;
            }
            let writer_ptr = self.writer;
            unsafe {
                (*writer_ptr).write('\0');
            }
            written += 1;
            self.size += written;
        }

        fn write_body(&mut self, header: &mut Slot<ELFSectionHeader>, w: &mut Writer) {
            unsafe {
                (*header.raw()).offset = self.offset;
                (*header.raw()).size = self.size;
            }
        }
    }

    #[cfg(__MACH_O)]
    struct MachOSectionHeader {}

    #[cfg(not(__MACH_O))]
    struct MachOSectionHeader {}

    #[cfg(__MACH_O)]
    struct MachOSection {}

    #[cfg(not(__MACH_O))]
    struct MachOSection {}

    #[derive(Debug, Clone, Copy)]
    struct CodeDescription {
        name: &'static str,
        shared_info: i32, //Tagged<SharedFunctionInfo>, //TODO replace with correct type
        lineinfo: *mut LineInfo,
        is_function: bool,
        code_region: AddressRegion,
    }

    impl CodeDescription {
        fn new(
            name: &'static str,
            region: AddressRegion,
            shared: i32, //Tagged<SharedFunctionInfo>,
            lineinfo: *mut LineInfo,
            is_function: bool,
        ) -> Self {
            CodeDescription {
                name,
                shared_info: shared,
                lineinfo,
                is_function,
                code_region: region,
            }
        }

        fn name(&self) -> &str {
            self.name
        }

        fn lineinfo(&self) -> *mut LineInfo {
            self.lineinfo
        }

        fn is_function(&self) -> bool {
            self.is_function
        }

        fn has_scope_info(&self) -> bool {
            false // TODO replace !shared_info_.is_null()
        }

        fn code_start(&self) -> usize {
            self.code_region.begin
        }

        fn code_end(&self) -> usize {
            self.code_region.end
        }

        fn code_size(&self) -> usize {
            self.code_region.size()
        }

        fn is_line_info_available(&self) -> bool {
            !self.lineinfo.is_null()
        }

        fn get_filename(&self) -> String {
            String::from("test_file.rs")
        }

        fn get_script_line_number(&self, pos: i32) -> i32 {
            0
        }
    }

    #[derive(Default, Debug)]
    struct LineInfo {
        pc_info: Vec<PCInfo>,
    }

    impl LineInfo {
        fn set_position(&mut self, pc: isize, pos: i32, is_statement: bool) {
            self.add_pc_info(PCInfo {
                pc_: pc,
                pos_: pos,
                is_statement_: is_statement,
            });
        }

        fn pc_info(&mut self) -> &mut Vec<PCInfo> {
            &mut self.pc_info
        }

        fn add_pc_info(&mut self, pc_info: PCInfo) {
            self.pc_info.push(pc_info);
        }
    }

    #[derive(Default, Debug, Clone, Copy)]
    struct PCInfo {
        pc_: isize,
        pos_: i32,
        is_statement_: bool,
    }

    // -------------------------------------------------------------------
    // Binary GDB JIT Interface as described in
    //   http://sourceware.org/gdb/onlinedocs/gdb/Declarations.html

    #[repr(C)]
    enum JITAction {
        JIT_NOACTION = 0,
        JIT_REGISTER_FN,
        JIT_UNREGISTER_FN,
    }

    #[repr(C)]
    struct JITCodeEntry {
        next_: *mut JITCodeEntry,
        prev_: *mut JITCodeEntry,
        symfile_addr_: *mut u8,
        symfile_size_: u64,
    }

    #[repr(C)]
    struct JITDescriptor {
        version_: u32,
        action_flag_: u32,
        relevant_entry_: *mut JITCodeEntry,
        first_entry_: *mut JITCodeEntry,
    }

    // GDB will place breakpoint