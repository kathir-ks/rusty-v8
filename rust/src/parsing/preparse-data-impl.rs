// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod preparse_data_impl {
    use std::cell::Cell;
    use std::rc::Rc;

    use crate::common::assert_scope::*;
    use crate::parsing::preparse_data::*;

    // Wraps a Vec<u8> to have with functions named the same as
    // Tagged<PodArray<u8>>.
    pub struct ZoneVectorWrapper {
        inner: Inner,
    }

    impl ZoneVectorWrapper {
        pub fn new() -> Self {
            ZoneVectorWrapper {
                inner: Inner::new(),
            }
        }

        pub fn with_data(data: Rc<Cell<Vec<u8>>>) -> Self {
            ZoneVectorWrapper {
                inner: Inner::with_data(data),
            }
        }

        pub fn inner(&self) -> &Inner {
            &self.inner
        }
    }

    pub struct Inner {
        data: Option<Rc<Cell<Vec<u8>>>>,
    }

    impl Inner {
        pub fn new() -> Self {
            Inner { data: None }
        }

        pub fn with_data(data: Rc<Cell<Vec<u8>>>) -> Self {
            Inner { data: Some(data) }
        }

        pub fn data_length(&self) -> i32 {
            self.data.as_ref().map_or(0, |d| d.borrow().len() as i32)
        }

        pub fn get(&self, index: usize) -> u8 {
            self.data.as_ref().unwrap().borrow()[index]
        }
    }

    pub trait DataTrait {}
    impl DataTrait for ZoneVectorWrapper {}
    impl DataTrait for PreparseData {}

    pub struct ByteData<D: DataTrait> {
        data: D,
        index: usize,
        stored_quarters: u8,
        stored_byte: u8,
        has_data: bool,
    }

    impl<D: DataTrait> ByteData<D> {
        pub fn new(data: D) -> Self {
            ByteData {
                data,
                index: 0,
                stored_quarters: 0,
                stored_byte: 0,
                has_data: false,
            }
        }

        pub fn set_position(&mut self, position: usize) {
            //  DCHECK_LE(position, data_->data_length()); // TODO implement this check
            self.index = position;
        }

        pub fn remaining_bytes(&self) -> usize {
            assert!(self.has_data);
            //  DCHECK_LE(index_, data_->data_length()); // TODO implement this check
            let data_length = match &self.data {
                _ => { 0 } // TODO: Properly implement getting data length
            };
            data_length - self.index
        }

        pub fn has_remaining_bytes(&self, bytes: usize) -> bool {
            assert!(self.has_data);

            let data_length = match &self.data {
                _ => { 0 } // TODO: Properly implement getting data length
            };

            self.index <= data_length && bytes <= self.remaining_bytes()
        }

        pub fn read_uint32(&mut self) -> i32 {
            assert!(self.has_data);
            assert!(self.has_remaining_bytes(4)); // kUint32Size

            // Check that there indeed is an integer following.
            //  DCHECK_EQ(data_->get(index_++), kUint32Size); // TODO implement this check
            self.index += 1;

            let result = (self.data_->get(self.index) as i32)
                + ((self.data_->get(self.index + 1) as i32) << 8)
                + ((self.data_->get(self.index + 2) as i32) << 16)
                + ((self.data_->get(self.index + 3) as i32) << 24);
            self.index += 4;
            self.stored_quarters = 0;
            result
        }

        pub fn read_varint32(&mut self) -> i32 {
            assert!(self.has_remaining_bytes(1)); //kVarint32MinSize
            //  DCHECK_EQ(data_->get(index_++), kVarint32MinSize); // TODO implement this check
            self.index += 1;

            let mut value: i32 = 0;
            let mut has_another_byte: bool;
            let mut shift: u32 = 0;
            loop {
                let byte: u8 = self.data_->get(self.index);
                self.index += 1;
                value |= (byte & 0x7F) as i32 << shift;
                shift += 7;
                has_another_byte = (byte & 0x80) != 0;
                if !has_another_byte {
                    break;
                }
            }
            //  DCHECK_EQ(data_->get(index_++), kVarint32EndMarker); // TODO implement this check
            self.index += 1;
            self.stored_quarters = 0;
            value
        }

        pub fn read_uint8(&mut self) -> u8 {
            assert!(self.has_data);
            assert!(self.has_remaining_bytes(1)); //kUint8Size

            // Check that there indeed is a byte following.
            //  DCHECK_EQ(data_->get(index_++), kUint8Size); // TODO implement this check
            self.index += 1;
            self.stored_quarters = 0;
            self.data_->get(self.index) // TODO: Get from actual data
        }

        pub fn read_quarter(&mut self) -> u8 {
            assert!(self.has_data);
            if self.stored_quarters == 0 {
                assert!(self.has_remaining_bytes(1)); // kUint8Size
                                                        // Check that there indeed are quarters following.
                                                        //  DCHECK_EQ(data_->get(index_++), kQuarterMarker); // TODO implement this check
                self.index += 1;
                self.stored_byte = self.data_->get(self.index);
                self.index += 1;
                self.stored_quarters = 4;
            }

            // Read the first 2 bits from stored_byte_.
            let result: u8 = (self.stored_byte >> 6) & 3;
            assert!(result <= 3);
            self.stored_quarters -= 1;
            self.stored_byte <<= 2;
            result
        }

        // placeholders
        fn data_(&self) -> &Inner {
            unimplemented!()
        }
    }

    pub trait GetScopeDataTrait<T> {
        fn get_scope_data(&self) -> T;
    }

    pub struct BaseConsumedPreparseData<T> {
        scope_data: Box<ByteData<T>>,
        child_index: i32,
    }

    impl<T: DataTrait> BaseConsumedPreparseData<T> {
        pub fn new(data: T) -> Self {
            BaseConsumedPreparseData {
                scope_data: Box::new(ByteData::new(data)),
                child_index: 0,
            }
        }
        pub fn scope_data(&mut self) -> &mut ByteData<T> {
            &mut self.scope_data
        }
        // TODO: Add implementation for other methods
    }

    impl<T> GetScopeDataTrait<T> for BaseConsumedPreparseData<T> {
        fn get_scope_data(&self) -> T {
            todo!()
        }
    }

    pub struct OnHeapConsumedPreparseData {
        isolate: i32, //LocalIsolate
        data: i32,    //Handle<PreparseData>
    }

    impl OnHeapConsumedPreparseData {
        // TODO: Add implementation for other methods
    }

    pub struct ZonePreparseData {
        byte_data: Vec<u8>,
        children: Vec<Box<ZonePreparseData>>,
    }

    impl ZonePreparseData {
        pub fn new(byte_data: Vec<u8>, child_length: i32) -> Self {
            let mut children = Vec::new();
            for _ in 0..child_length {
                // Initialize with empty boxes, to be filled later.
                children.push(Box::new(ZonePreparseData {
                    byte_data: Vec::new(),
                    children: Vec::new(),
                }));
            }

            ZonePreparseData {
                byte_data,
                children,
            }
        }

        pub fn children_length(&self) -> i32 {
            self.children.len() as i32
        }

        pub fn get_child(&mut self, index: usize) -> &mut ZonePreparseData {
            &mut self.children[index]
        }

        pub fn set_child(&mut self, index: usize, child: ZonePreparseData) {
            self.children[index] = Box::new(child);
        }

        pub fn byte_data(&mut self) -> &mut Vec<u8> {
            &mut self.byte_data
        }

        // TODO: Add implementation for other methods
    }

    pub struct ZoneConsumedPreparseData {
        data: Box<ZonePreparseData>,
        scope_data_wrapper: ZoneVectorWrapper,
    }

    impl ZoneConsumedPreparseData {
        // TODO: Add implementation for other methods
    }
}