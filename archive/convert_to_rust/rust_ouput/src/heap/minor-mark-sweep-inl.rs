// Converted from V8 C++ source files:
// Header: minor-mark-sweep-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod minor_mark_sweep_inl {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
    use std::{marker::PhantomData, optional::Option};

    use crate::heap::minor_mark_sweep::*;
    use crate::heap::mutable_page_metadata::*;
    use crate::heap::remembered_set_inl::*;
    use crate::heap::young_generation_marking_visitor_inl::*;
    use crate::objects::heap_object::*;
    use crate::objects::map::*;
    use crate::objects::string::*;
    use crate::roots::static_roots::*;
    use crate::heap::scavenger_inl::SlotCallbackResult;
    use crate::heap::scavenger_inl::SlotCallbackResult::*;
    use crate::objects::slots::FullMaybeObjectSlot;
    use crate::heap::marking_visitor::MaybeObjectSlot;
    use crate::heap::Address;
    use crate::heap::V8;

    impl YoungGenerationRootMarkingVisitor {
        pub fn visit_root_pointer(&mut self, root: Root, description: &str, p: FullObjectSlot) {
            self.visit_pointers_impl(root, p, unsafe { p.offset(1) });
        }

        pub fn visit_root_pointers(
            &mut self,
            root: Root,
            description: &str,
            start: FullObjectSlot,
            end: FullObjectSlot,
        ) {
            self.visit_pointers_impl(root, start, end);
        }

        fn visit_pointers_impl<TSlot>(&mut self, root: Root, start: TSlot, end: TSlot) {
            match root {
                Root::kStackRoots => {
                    let mut slot = start;
                    while slot < end {
                        self.main_marking_visitor_.visit_object_via_slot::<{
                            YoungGenerationMainMarkingVisitor::ObjectVisitationMode::kPushToWorklist
                                as i32
                        }, {
                            YoungGenerationMainMarkingVisitor::SlotTreatmentMode::kReadOnly as i32
                        }, _>(
                            slot,
                        );
                        unsafe {slot = slot.offset(1)};
                    }
                }
                _ => {
                    let mut slot = start;
                    while slot < end {
                        self.main_marking_visitor_.visit_object_via_slot::<{
                            YoungGenerationMainMarkingVisitor::ObjectVisitationMode::kPushToWorklist
                                as i32
                        }, {
                            YoungGenerationMainMarkingVisitor::SlotTreatmentMode::kReadWrite as i32
                        }, _>(
                            slot,
                        );
                        unsafe {slot = slot.offset(1)};
                    }
                }
            }
        }
    }

    impl YoungGenerationRememberedSetsMarkingWorklist {
        pub fn process_next_item<Visitor>(
            &mut self,
            visitor: &mut Visitor,
            index: &mut Option<usize>,
        ) -> bool {
            if self
                .remaining_remembered_sets_marking_items_
                .load(Ordering::Relaxed)
                == 0
            {
                return false;
            }
            loop {
                if let Some(idx) = index {
                    if *idx < self.remembered_sets_marking_items_.len() {
                        let work_item = &mut self.remembered_sets_marking_items_[*idx];
                        if work_item.try_acquire() {
                            self.remaining_remembered_sets_marking_items_
                                .fetch_sub(1, Ordering::Relaxed);
                            work_item.process(visitor);
                            *index = idx + 1;
                            return true;
                        }
                    }
                }
                *index = self.remembered_sets_marking_index_generator_.get_next();
                if index.is_none() {
                    return false;
                }
            }
        }
    }

    impl YoungGenerationRememberedSetsMarkingWorklist {
        impl MarkingItem {
            fn process<Visitor>(&mut self, visitor: &mut Visitor) {
                match self.slots_type_ {
                    SlotsType::kRegularSlots => self.mark_untyped_pointers(visitor),
                    _ => self.mark_typed_pointers(visitor),
                }
            }

            fn mark_untyped_pointers<Visitor>(&mut self, visitor: &mut Visitor) {
                //TRACE_EVENT0(TRACE_DISABLED_BY_DEFAULT("v8.gc"),"MarkingItem::MarkUntypedPointers");
                let callback = |slot: MaybeObjectSlot| -> SlotCallbackResult {
                    self.check_and_mark_object(visitor, slot)
                };

                if let Some(slot_set) = &mut self.slot_set_ {
                    let slot_count = RememberedSet::<OLD_TO_NEW>::iterate::<AccessMode::NON_ATOMIC>(
                        slot_set,
                        self.chunk_,
                        callback,
                        SlotSet::FREE_EMPTY_BUCKETS,
                    );

                    if slot_count == 0 {
                        SlotSet::delete(slot_set);
                        self.slot_set_ = None;
                    }
                }

                if let Some(background_slot_set) = &mut self.background_slot_set_ {
                    let slot_count = RememberedSet::<OLD_TO_NEW_BACKGROUND>::iterate::<
                        AccessMode::NON_ATOMIC,
                    >(
                        background_slot_set,
                        self.chunk_,
                        callback,
                        SlotSet::FREE_EMPTY_BUCKETS,
                    );

                    if slot_count == 0 {
                        SlotSet::delete(background_slot_set);
                        self.background_slot_set_ = None;
                    }
                }
            }

            fn mark_typed_pointers<Visitor>(&mut self, visitor: &mut Visitor) {
                //TRACE_EVENT0(TRACE_DISABLED_BY_DEFAULT("v8.gc"),
                //"MarkingItem::MarkTypedPointers");
                assert!(self.background_slot_set_.is_none());
                assert!(self.typed_slot_set_.is_some());

                let slot_count = RememberedSet::<OLD_TO_NEW>::iterate_typed(
                    self.typed_slot_set_.as_mut().unwrap(),
                    |slot_type: SlotType, slot_address: Address| -> SlotCallbackResult {
                        let object = UpdateTypedSlotHelper::get_target_object(
                            self.heap(),
                            slot_type,
                            slot_address,
                        );
                        let slot = FullMaybeObjectSlot { object };
                        self.check_and_mark_object(visitor, slot)
                    },
                );

                if slot_count == 0 {
                    drop(self.typed_slot_set_.take());
                }
            }

            fn check_and_mark_object<Visitor, TSlot>(
                &mut self,
                visitor: &mut Visitor,
                slot: TSlot,
            ) -> SlotCallbackResult {
                if visitor.visit_object_via_slot_in_remembered_set(slot) {
                    KEEP_SLOT
                } else {
                    REMOVE_SLOT
                }
            }
        }
    }
}
