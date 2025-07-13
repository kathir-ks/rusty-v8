// Converted from V8 C++ source files:
// Header: code-pointer-table.h
// Implementation: code-pointer-table.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    pub mod atomicops;
    pub mod memory;
}

pub mod common {
    pub mod globals;
}

pub mod sandbox {
    pub mod code_entrypoint_tag;
    pub mod external_entity_table;
}

#[cfg(v8_compress_pointers)]
pub mod internal {
    use crate::base::atomicops::Atomic;
    use crate::common::globals::kCodePointerTableEntrySize;
    use crate::sandbox::code_entrypoint_tag::CodeEntrypointTag;
    use crate::sandbox::code_entrypoint_tag::CodeEntrypointTag::kFreeCodePointerTableEntryTag;
    use crate::sandbox::external_entity_table::ExternalEntityTable;
    use std::sync::atomic::Ordering;
    use std::marker::PhantomData;

    pub struct Isolate;
    pub struct Counters;
    pub struct Address {}

    #[derive(Debug)]
    pub struct CodePointerTableEntry {
        entrypoint_: Atomic<usize>,
        code_: Atomic<usize>,
    }

    impl CodePointerTableEntry {
        pub const IsWriteProtected: bool = true;
        const kFreeEntryTag: usize = kFreeCodePointerTableEntryTag as usize;
        const kMarkingBit: usize = 1;

        #[inline]
        pub fn make_code_pointer_entry(&self, code: usize, entrypoint: usize, tag: CodeEntrypointTag, mark_as_alive: bool) {
            self.entrypoint_.store(entrypoint as usize, Ordering::Relaxed);
            let mut code_address = code as usize;
            if mark_as_alive {
                code_address |= Self::kMarkingBit;
            }
            self.code_.store(code_address, Ordering::Relaxed);
        }

        #[inline]
        pub fn make_freelist_entry(&self, next_entry_index: u32) {
            self.code_.store((next_entry_index as usize) | Self::kFreeEntryTag, Ordering::Relaxed);
        }

        #[inline]
        pub fn get_entrypoint(&self, tag: CodeEntrypointTag) -> usize {
            self.entrypoint_.load(Ordering::Relaxed) as usize
        }

        #[inline]
        pub fn set_entrypoint(&self, value: usize, tag: CodeEntrypointTag) {
            self.entrypoint_.store(value as usize, Ordering::Relaxed);
        }

        #[inline]
        pub fn get_code_object(&self) -> usize {
            self.code_.load(Ordering::Relaxed)
        }

        #[inline]
        pub fn set_code_object(&self, value: usize) {
            self.code_.store(value, Ordering::Relaxed);
        }

        #[inline]
        pub fn is_freelist_entry(&self) -> bool {
            (self.code_.load(Ordering::Relaxed) & !Self::kMarkingBit) as usize == Self::kFreeEntryTag
        }

        #[inline]
        pub fn get_next_freelist_entry_index(&self) -> u32 {
            (self.code_.load(Ordering::Relaxed) & !Self::kMarkingBit) as u32
        }

        #[inline]
        pub fn mark(&self) {
            let current_code = self.code_.load(Ordering::Relaxed);
            self.code_.store(current_code | Self::kMarkingBit, Ordering::Relaxed);
        }

        #[inline]
        pub fn unmark(&self) {
            let current_code = self.code_.load(Ordering::Relaxed);
            self.code_.store(current_code & !Self::kMarkingBit, Ordering::Relaxed);
        }

        #[inline]
        pub fn is_marked(&self) -> bool {
            (self.code_.load(Ordering::Relaxed) & Self::kMarkingBit) != 0
        }
    }

    pub type CodePointerHandle = u32;

    const K_MAX_CODE_POINTERS: usize = 2048;
    const K_CODE_POINTER_TABLE_RESERVATION_SIZE: usize = K_MAX_CODE_POINTERS;

    pub struct CodePointerTable {
        base: ExternalEntityTable<CodePointerTableEntry, K_CODE_POINTER_TABLE_RESERVATION_SIZE>,
    }

    impl CodePointerTable {
        pub const kMaxCodePointers: usize = K_MAX_CODE_POINTERS;
        pub const kSupportsCompaction: bool = false;

        pub fn new() -> Self {
            CodePointerTable {
                base: ExternalEntityTable::new(),
            }
        }

        #[inline]
        pub fn get_entrypoint(&self, handle: CodePointerHandle, tag: CodeEntrypointTag) -> usize {
            let index = self.handle_to_index(handle);
            let entry = &self.base.entries()[index as usize];
            entry.get_entrypoint(tag)
        }

        #[inline]
        pub fn get_code_object(&self, handle: CodePointerHandle) -> usize {
            let index = self.handle_to_index(handle);
            let entry = &self.base.entries()[index as usize];
            entry.get_code_object()
        }

        #[inline]
        pub fn set_entrypoint(&self, handle: CodePointerHandle, value: usize, tag: CodeEntrypointTag) {
            let index = self.handle_to_index(handle);
            let entry = &mut self.base.entries()[index as usize];
            entry.set_entrypoint(value, tag);
        }

        #[inline]
        pub fn set_code_object(&self, handle: CodePointerHandle, value: usize) {
            let index = self.handle_to_index(handle);
            let entry = &mut self.base.entries()[index as usize];
            entry.set_code_object(value);
        }

        #[inline]
        pub fn allocate_and_initialize_entry(&mut self, space: &mut Space, code: usize, entrypoint: usize, tag: CodeEntrypointTag) -> CodePointerHandle {
            let index = self.base.allocate_entry(space).unwrap();
            let entry = &mut self.base.entries()[index as usize];
            entry.make_code_pointer_entry(code, entrypoint, tag, true);
            self.index_to_handle(index)
        }

        #[inline]
        pub fn mark(&self, space: &mut Space, handle: CodePointerHandle) {
            let index = self.handle_to_index(handle);
            let entry = &mut self.base.entries()[index as usize];
            entry.mark();
        }

        pub fn sweep(&mut self, space: &mut Space, counters: &mut Counters) -> u32 {
            let num_live_entries = self.generic_sweep(space);
            //counters.code_pointers_count().AddSample(num_live_entries);
            num_live_entries
        }

        fn generic_sweep(&mut self, space: &mut Space) -> u32 {
            let mut num_live_entries: u32 = 0;
            for i in 0..K_MAX_CODE_POINTERS {
                if self.base.is_valid(i as u32) {
                    let entry = &mut self.base.entries()[i as usize];
                    if entry.is_marked() {
                        entry.unmark();
                        num_live_entries += 1;
                    } else {
                        self.base.free_entry(space, i as u32).unwrap();
                    }
                }
            }
            num_live_entries
        }

        pub fn iterate_active_entries_in<Callback>(&self, space: &mut Space, mut callback: Callback)
            where
                Callback: FnMut(CodePointerHandle, usize),
        {
            for i in 0..K_MAX_CODE_POINTERS {
                if self.base.is_valid(i as u32) {
                    let handle = self.index_to_handle(i as u32);
                    let code_object = self.get_code_object(handle);
                    callback(handle, code_object);
                }
            }
        }

        pub fn base_address(&self) -> usize {
            self.base.base()
        }

        #[inline]
        fn handle_to_index(&self, handle: CodePointerHandle) -> u32 {
            handle
        }

        #[inline]
        fn index_to_handle(&self, index: u32) -> CodePointerHandle {
            index
        }
    }

    pub struct Space {
        entries: [bool; K_MAX_CODE_POINTERS],
    }

    impl Space {
        pub fn new() -> Self {
            Space {
                entries: [false; K_MAX_CODE_POINTERS],
            }
        }

        pub fn allocate(&mut self) -> Option<u32> {
            for i in 0..K_MAX_CODE_POINTERS {
                if !self.entries[i] {
                    self.entries[i] = true;
                    return Some(i as u32);
                }
            }
            None
        }

        pub fn free(&mut self, index: u32) -> Result<(), String> {
            if index as usize >= K_MAX_CODE_POINTERS {
                return Err("Index out of bounds".to_string());
            }
            self.entries[index as usize] = false;
            Ok(())
        }

        pub fn is_valid(&self, index: u32) -> bool {
            if index as usize >= K_MAX_CODE_POINTERS {
                return false;
            }
            self.entries[index as usize]
        }
    }
}

