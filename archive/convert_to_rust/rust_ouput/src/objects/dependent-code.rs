// Converted from V8 C++ source files:
// Header: dependent-code.h
// Implementation: dependent-code.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod dependent_code {
    use crate::roots::roots::ReadOnlyRoots;
    use crate::v8::internal::LazyDeoptimizeReason;
    use std::marker::PhantomData;
    use crate::objects::fixed_array::WeakArrayList;
    use crate::V8;
    use crate::v8::internal::code::Code;
    use crate::objects::fixed_array::MaybeObject;
    use crate::objects::map::Map;
    use crate::objects::property_cell::PropertyCell;
    use crate::objects::allocation_site::AllocationSite;
    use crate::objects::scope_info::ScopeInfo;
    use crate::objects::context_side_property_cell::ContextSidePropertyCell;
    use crate::objects::code::CodeWrapper;
    use crate::objects::heap_object::HeapObject;

    use crate::objects::tagged::Tagged;
    use crate::objects::smi::Smi;
    use crate::objects::heap_object::DirectHandle;
    use crate::objects::heap_object::MaybeObjectDirectHandle;
    use crate::objects::weak::MakeWeak;

    use std::fmt;

    bitflags::bitflags! {
        #[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
        pub struct DependencyGroups: u32 {
            const kTransitionGroup = 1 << 0;
            const kPrototypeCheckGroup = 1 << 1;
            const kPropertyCellChangedGroup = 1 << 2;
            const kFieldTypeGroup = 1 << 3;
            const kFieldConstGroup = 1 << 4;
            const kFieldRepresentationGroup = 1 << 5;
            const kInitialMapChangedGroup = 1 << 6;
            const kAllocationSiteTenuringChangedGroup = 1 << 7;
            const kAllocationSiteTransitionChangedGroup = 1 << 8;
            const kScriptContextSlotPropertyChangedGroup = 1 << 9;
            const kEmptyContextExtensionGroup = 1 << 10;
        }
    }

    impl DependencyGroups {
        pub fn dependency_group_name(self) -> &'static str {
            match self {
                DependencyGroups::kTransitionGroup => "transition",
                DependencyGroups::kPrototypeCheckGroup => "prototype-check",
                DependencyGroups::kPropertyCellChangedGroup => "property-cell-changed",
                DependencyGroups::kFieldConstGroup => "field-const",
                DependencyGroups::kFieldTypeGroup => "field-type",
                DependencyGroups::kFieldRepresentationGroup => "field-representation",
                DependencyGroups::kInitialMapChangedGroup => "initial-map-changed",
                DependencyGroups::kAllocationSiteTenuringChangedGroup => "allocation-site-tenuring-changed",
                DependencyGroups::kAllocationSiteTransitionChangedGroup => "allocation-site-transition-changed",
                DependencyGroups::kScriptContextSlotPropertyChangedGroup => "script-context-slot-property-changed",
                DependencyGroups::kEmptyContextExtensionGroup => "empty-context-extension",
                _ => "unknown",
            }
        }

        pub fn dependency_group_to_lazy_deopt_reason(self) -> LazyDeoptimizeReason {
            match self {
                DependencyGroups::kTransitionGroup => LazyDeoptimizeReason::kMapDeprecated,
                DependencyGroups::kPrototypeCheckGroup => LazyDeoptimizeReason::kPrototypeChange,
                DependencyGroups::kPropertyCellChangedGroup => LazyDeoptimizeReason::kPropertyCellChange,
                DependencyGroups::kFieldConstGroup => LazyDeoptimizeReason::kFieldTypeConstChange,
                DependencyGroups::kFieldTypeGroup => LazyDeoptimizeReason::kFieldTypeChange,
                DependencyGroups::kFieldRepresentationGroup => LazyDeoptimizeReason::kFieldRepresentationChange,
                DependencyGroups::kInitialMapChangedGroup => LazyDeoptimizeReason::kInitialMapChange,
                DependencyGroups::kAllocationSiteTenuringChangedGroup => LazyDeoptimizeReason::kAllocationSiteTenuringChange,
                DependencyGroups::kAllocationSiteTransitionChangedGroup => LazyDeoptimizeReason::kAllocationSiteTransitionChange,
                DependencyGroups::kScriptContextSlotPropertyChangedGroup => LazyDeoptimizeReason::kScriptContextSlotPropertyChange,
                DependencyGroups::kEmptyContextExtensionGroup => LazyDeoptimizeReason::kEmptyContextExtensionChange,
                _ => panic!("Unknown DependencyGroup"),
            }
        }
    }

    impl fmt::Display for DependencyGroups {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    pub struct DependentCode {
        pub weak_array_list: WeakArrayList,
    }

    impl DependentCode {
        pub const kSlotsPerEntry: i32 = 2;
        pub const kCodeSlotOffset: i32 = 0;
        pub const kGroupsSlotOffset: i32 = 1;
        pub const kEmptyDependentCode: crate::objects::string::RootIndex = crate::objects::string::RootIndex {};

        pub fn dependency_group_name(group: DependencyGroups) -> &'static str {
            group.dependency_group_name()
        }

        pub fn dependency_group_to_lazy_deopt_reason(group: DependencyGroups) -> LazyDeoptimizeReason {
             group.dependency_group_to_lazy_deopt_reason()
        }

        pub fn install_dependency(
            isolate: &mut Isolate,
            code: &mut DirectHandle<Code>,
            object: &mut DirectHandle<HeapObject>,
            groups: DependencyGroups,
        ) {
            if false {
                println!(
                    "Installing dependency of [{:?}] on [{:?}] in groups [{}]",
                    code, object, groups
                );
            }

            let mut old_deps = DependentCode::get_dependent_code(*object);
            let new_deps =
                 DependentCode::insert_weak_code(isolate, &mut old_deps, groups, code);

            if new_deps != old_deps{
                 DependentCode::set_dependent_code(object, &mut new_deps);
            }
        }

        fn insert_weak_code(
            isolate: &mut Isolate,
            entries: &mut Tagged<DependentCode>,
            groups: DependencyGroups,
            code: &mut DirectHandle<Code>,
        ) -> Tagged<DependentCode> {
            if entries.get_length() == entries.get_capacity() {
                entries.iterate_and_compact(
                    isolate,
                    |_code, _groups| false,
                );
            }

            let code_slot = MaybeObjectDirectHandle {
                value: MakeWeak(code.value.wrapper()),
                _phantom: PhantomData,
            };

            let smi_groups = Smi::from_int(groups.bits() as i32);

            let new_entries = WeakArrayList::add_to_end(
                isolate,
                entries,
                code_slot,
                smi_groups,
            );
            Tagged {
                ptr: new_entries.ptr,
            }
        }

        pub fn empty_dependent_code(roots: &ReadOnlyRoots) -> Tagged<DependentCode> {
            let weak_array_list = WeakArrayList {
                ptr: roots.empty_weak_array_list().ptr,
            };
            Tagged {
                ptr: weak_array_list.ptr,
            }
        }

        fn get_dependent_code(object: Tagged<HeapObject>) -> Tagged<DependentCode> {
            if object.is_map(){
                let map = unsafe {object.ptr.cast::<Map>().as_ref().unwrap()};
                return map.dependent_code();
            }else if object.is_property_cell() {
                let property_cell = unsafe {object.ptr.cast::<PropertyCell>().as_ref().unwrap()};
                return property_cell.dependent_code();
            } else if object.is_allocation_site() {
                let allocation_site = unsafe {object.ptr.cast::<AllocationSite>().as_ref().unwrap()};
                return allocation_site.dependent_code();
            } else if object.is_context_side_property_cell(){
                let context_side_property_cell = unsafe {object.ptr.cast::<ContextSidePropertyCell>().as_ref().unwrap()};
                return context_side_property_cell.dependent_code();
            } else if object.is_scope_info(){
                let scope_info = unsafe {object.ptr.cast::<ScopeInfo>().as_ref().unwrap()};
                return scope_info.dependent_code();
            }
            panic!("Unreachable");
        }

        fn set_dependent_code(
            object: &mut DirectHandle<HeapObject>,
            dep: &mut Tagged<DependentCode>,
        ) {
            if object.value.is_map(){
                let map = unsafe {object.value.ptr.cast::<Map>().as_mut().unwrap()};
                map.set_dependent_code(*dep);
            } else if object.value.is_property_cell(){
                let property_cell = unsafe {object.value.ptr.cast::<PropertyCell>().as_mut().unwrap()};
                property_cell.set_dependent_code(*dep);
            } else if object.value.is_allocation_site(){
                let allocation_site = unsafe {object.value.ptr.cast::<AllocationSite>().as_mut().unwrap()};
                allocation_site.set_dependent_code(*dep);
            } else if object.value.is_context_side_property_cell(){
                let context_side_property_cell = unsafe {object.value.ptr.cast::<ContextSidePropertyCell>().as_mut().unwrap()};
                context_side_property_cell.set_dependent_code(*dep);
            } else if object.value.is_scope_info(){
                let scope_info = unsafe {object.value.ptr.cast::<ScopeInfo>().as_mut().unwrap()};
                scope_info.set_dependent_code(*dep);
            } else{
                panic!("Unreachable");
            }
        }

        fn iterate_and_compact<F>(&mut self, isolate: &mut Isolate, mut fn_: F)
        where
            F: FnMut(Tagged<Code>, DependencyGroups) -> bool,
        {
           let len = self.get_length();
            if len == 0 {
                return;
            }

            let mut i = (len - Self::kSlotsPerEntry) as usize;
            while i >= 0 {
                let obj = unsafe { self.weak_array_list.get(i as i32 + Self::kCodeSlotOffset) };
                if obj.is_cleared() {
                    let new_len = self.fill_entry_from_back(i as i32, len);
                    i = (new_len - Self::kSlotsPerEntry) as usize;
                    continue;
                }

                let code_wrapper = unsafe {
                    obj.get_heap_object_assume_weak()
                        .ptr
                        .cast::<CodeWrapper>()
                        .as_ref()
                        .unwrap()
                };
                let groups = unsafe {
                     self.weak_array_list
                        .get(i as i32 + Self::kGroupsSlotOffset)
                        .to_smi()
                        .value() as u32
                };

                let dependency_groups = DependencyGroups::from_bits_truncate(groups);

                if fn_(code_wrapper.code(isolate), dependency_groups) {
                    let new_len = self.fill_entry_from_back(i as i32, len);
                    i = (new_len - Self::kSlotsPerEntry) as usize;
                }

                if i == 0 {
                    break;
                }
                i -= Self::kSlotsPerEntry as usize;
            }

            self.set_length(i as i32);
        }

        fn fill_entry_from_back(&mut self, index: i32, length: i32) -> i32 {
            assert_eq!(index % 2, 0);
            assert_eq!(length % 2, 0);

            let mut i = length - Self::kSlotsPerEntry;
            while i > index {
                let obj = unsafe { self.weak_array_list.get(i + Self::kCodeSlotOffset) };
                if obj.is_cleared() {
                    i -= Self::kSlotsPerEntry;
                    continue;
                }

                unsafe {
                     self.weak_array_list.set(index + Self::kCodeSlotOffset, obj);
                    let groups = self.weak_array_list.get(i + Self::kGroupsSlotOffset);
                     self.weak_array_list.set(index + Self::kGroupsSlotOffset, groups);
                }
                return i;
            }

            index
        }

        fn mark_code_for_deoptimization(
            &mut self,
            isolate: &mut Isolate,
            deopt_groups: DependencyGroups,
        ) -> bool {
            let mut marked_something = false;
            self.iterate_and_compact(
                isolate,
                |code, groups| {
                    if (groups & deopt_groups).is_empty() {
                        return false;
                    }

                    if !code.marked_for_deoptimization() {
                       let groups = groups & deopt_groups;
                        let first_group_bits = groups.bits();
                        let first_group = DependencyGroups::from_bits_truncate(
                            1 << first_group_bits.trailing_zeros(),
                        );
                        code.set_marked_for_deoptimization(
                            isolate,
                            DependentCode::dependency_group_to_lazy_deopt_reason(first_group),
                        );
                        marked_something = true;
                    }

                    true
                },
            );

            marked_something
        }

        fn deoptimize_dependency_groups(
            &mut self,
            isolate: &mut Isolate,
            groups: DependencyGroups,
        ) {
            let marked_something = self.mark_code_for_deoptimization(isolate, groups);
            if marked_something {
                //Deoptimizer::deoptimize_marked_code(isolate);
            }
        }
        const kEmptyWeakArrayList: crate::objects::string::RootIndex = crate::objects::string::RootIndex {};

        fn get_length(&self) -> i32{
            self.weak_array_list.length()
        }

        fn set_length(&mut self, new_length: i32){
            unsafe{self.weak_array_list.set_length(new_length)}
        }

        fn get_capacity(&self) -> i32{
            self.weak_array_list.capacity()
        }
    }

    impl Tagged<HeapObject>{
        fn is_map(&self) -> bool{
            true
        }
        fn is_property_cell(&self) -> bool{
            true
        }
        fn is_allocation_site(&self) -> bool{
            true
        }
        fn is_context_side_property_cell(&self) -> bool{
            true
        }
        fn is_scope_info(&self) -> bool{
            true
        }
    }

    pub struct Isolate{

    }
}
