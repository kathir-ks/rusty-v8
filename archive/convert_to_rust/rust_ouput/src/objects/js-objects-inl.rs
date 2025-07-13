// Converted from V8 C++ source files:
// Header: js-objects-inl.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod js_objects_generated_tq_inl {
}
pub mod object_macros {
}
use std::convert::TryInto;
use std::io::Write;
use std::mem::size_of;
use std::ops::Range;
pub struct JSReceiver {
    ptr: usize,
}
pub struct JSObject {
    ptr: usize,
}
pub struct JSObjectWithEmbedderSlots {
    ptr: usize,
}
pub struct JSAPIObjectWithEmbedderSlots {
    ptr: usize,
}
pub struct JSCustomElementsObject {
    ptr: usize,
}
pub struct JSSpecialObject {
    ptr: usize,
}
pub struct JSAsyncFromSyncIterator {
    ptr: usize,
}
pub struct JSDate {
    ptr: usize,
}
pub struct JSGlobalObject {
    ptr: usize,
}
pub struct JSGlobalProxy {
    ptr: usize,
}
pub struct JSIteratorResult {
    ptr: usize,
}
pub struct JSMessageObject {
    ptr: usize,
}
pub struct JSPrimitiveWrapper {
    ptr: usize,
}
pub struct JSStringIterator {
    ptr: usize,
}
pub struct JSValidIteratorWrapper {
    ptr: usize,
}
impl JSReceiver {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSObjectWithEmbedderSlots {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSAPIObjectWithEmbedderSlots {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSCustomElementsObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSSpecialObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSAsyncFromSyncIterator {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSDate {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSGlobalObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSGlobalProxy {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSIteratorResult {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSMessageObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSPrimitiveWrapper {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSStringIterator {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
impl JSValidIteratorWrapper {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
const kElementsOffset: usize = 0;
impl JSObject {
    pub fn elements(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> Tagged<FixedArrayBase> {
        TaggedField::<FixedArrayBase, kElementsOffset>::Relaxed_Load(cage_base, *self)
    }
    pub fn set_elements(&mut self, value: Tagged<FixedArrayBase>, mode: WriteBarrierMode) {
        TaggedField::<FixedArrayBase, kElementsOffset>::Relaxed_Store(*self, value);
    }
}
impl JSReceiver {
    pub fn GetProperty(
        isolate: *mut isolate,
        receiver: DirectHandle<JSReceiver>,
        name: DirectHandle<Name>,
    ) -> Result<MaybeHandle<Object>, String> {
        let mut it = LookupIterator { dummy: 0 };
        if !it.IsFound() {
            return Err("Property not found".to_string());
        }
        let result = Object::GetProperty(&mut it);
        Ok(result)
    }
    pub fn GetElement(
        isolate: *mut isolate,
        receiver: DirectHandle<JSReceiver>,
        index: u32,
    ) -> Result<MaybeHandle<Object>, String> {
        let mut it = LookupIterator { dummy: 0 };
        if !it.IsFound() {
            return Err("Element not found".to_string());
        }
        let result = Object::GetProperty(&mut it);
        Ok(result)
    }
    pub fn GetDataProperty(
        isolate: *mut isolate,
        object: DirectHandle<JSReceiver>,
        name: DirectHandle<Name>,
    ) -> Result<Handle<Object>, String> {
        let mut it = LookupIterator { dummy: 0 };
        if !it.IsFound() {
            return Err("Data property not found".to_string());
        }
        let result = Object::GetDataProperty(&mut it);
        Ok(result)
    }
    pub fn GetPrototype(
        isolate: *mut isolate,
        receiver: DirectHandle<JSReceiver>,
    ) -> MaybeDirectHandle<JSPrototype> {
        let mut iter = PrototypeIterator { dummy: 0 };
        loop {
            if !iter.AdvanceFollowingProxies() {
                return MaybeDirectHandle::<JSPrototype>::default();
            }
            if iter.IsAtEnd() {
                break;
            }
        }
        PrototypeIterator::GetCurrent(iter)
    }
    pub fn GetProperty_str(
        isolate: *mut isolate,
        receiver: DirectHandle<JSReceiver>,
        name: &str,
    ) -> Result<MaybeHandle<Object>, String> {
        let str = isolate.factory().InternalizeUtf8String(name);
        JSReceiver::GetProperty(isolate, receiver, str)
    }
    pub fn OwnPropertyKeys(
        isolate: *mut isolate,
        object: DirectHandle<JSReceiver>,
    ) -> Result<MaybeDirectHandle<FixedArray>, String> {
        let result = KeyAccumulator::GetKeys(
            isolate,
            object,
            KeyCollectionMode::kOwnOnly,
            ALL_PROPERTIES,
            GetKeysConversion::kConvertToString,
        );
        Ok(result)
    }
}
impl JSObject {
    pub fn PrototypeHasNoElements(isolate: *mut isolate, object: Tagged<JSObject>) -> bool {
        let no_gc = DisallowGarbageCollection {};
        let mut prototype = object.map().prototype();
        let roots = ReadOnlyRoots { dummy: 0 };
        let null = roots.null_value();
        let empty_fixed_array = roots.empty_fixed_array();
        let empty_slow_element_dictionary = roots.empty_slow_element_dictionary();
        while prototype != null {
            let map = prototype.map();
            if IsCustomElementsReceiverMap(map) {
                return false;
            }
            let elements = object.elements();
            if elements != empty_fixed_array && elements != empty_slow_element_dictionary {
                return false;
            }
            prototype = map.prototype();
        }
        true
    }
}
const kPropertiesOrHashOffset: usize = 0;
impl JSReceiver {
    pub fn raw_properties_or_hash(&self, cage_base: PtrComprCageBase, tag: RelaxedLoadTag) -> Tagged<Object> {
        match tag {
            RelaxedLoadTag::kRelaxedLoad => {
                TaggedField::<Object, kPropertiesOrHashOffset>::Relaxed_Load(cage_base, *self)
            }
        }
    }
    pub fn set_raw_properties_or_hash(&mut self, value: Tagged<Object>, mode: WriteBarrierMode) {
    }
}
impl JSObject {
    pub fn EnsureCanContainHeapObjectElements(object: DirectHandle<JSObject>) {
        JSObject::ValidateElements(*object);
        let elements_kind = object.map().elements_kind();
        if !IsObjectElementsKind(elements_kind) {
            if IsHoleyElementsKind(elements_kind) {
                TransitionElementsKind(object, HOLEY_ELEMENTS);
            } else {
                TransitionElementsKind(object, PACKED_ELEMENTS);
            }
        }
    }
    pub fn EnsureCanContainElements<TSlot>(
        object: DirectHandle<JSObject>,
        objects: TSlot,
        count: u32,
        mode: EnsureElementsMode,
    ) where
        TSlot: Sized,
    {
        let current_kind = object.GetElementsKind();
        let mut target_kind = current_kind;
        {
            let no_gc = DisallowGarbageCollection {};
            assert!(mode != EnsureElementsMode::ALLOW_COPIED_DOUBLE_ELEMENTS);
            let is_holey = IsHoleyElementsKind(current_kind);
            if current_kind == HOLEY_ELEMENTS {
                return;
            }
            let the_hole = GetReadOnlyRoots().the_hole_value();
            for i in 0..count {
                let current = Object::default();
                if current == the_hole {
                    target_kind = GetHoleyElementsKind(target_kind);
                } else if !IsSmi(current) {
                    if mode == EnsureElementsMode::ALLOW_CONVERTED_DOUBLE_ELEMENTS && IsNumber(current) {
                        if IsSmiElementsKind(target_kind) {
                            if is_holey {
                                target_kind = HOLEY_DOUBLE_ELEMENTS;
                            } else {
                                target_kind = PACKED_DOUBLE_ELEMENTS;
                            }
                        }
                    } else if is_holey {
                        target_kind = HOLEY_ELEMENTS;
                        break;
                    } else {
                        target_kind = PACKED_ELEMENTS;
                    }
                }
            }
        }
        if target_kind != current_kind {
            TransitionElementsKind(object, target_kind);
        }
    }
    pub fn EnsureCanContainElements_handle(
        object: DirectHandle<JSObject>,
        elements: DirectHandle<FixedArrayBase>,
        length: u32,
        mode: EnsureElementsMode,
    ) {
        let roots = GetReadOnlyRoots();
        if elements.map() != roots.fixed_double_array_map() {
            assert!(
                elements.map() == roots.fixed_array_map()
                    || elements.map() == roots.fixed_cow_array_map()
            );
            let objects = FixedArray::RawFieldOfFirstElement();
            JSObject::EnsureCanContainElements(object, objects, length, mode);
            return;
        }
        if object.GetElementsKind() == HOLEY_SMI_ELEMENTS {
            TransitionElementsKind(object, HOLEY_DOUBLE_ELEMENTS);
        } else if object.GetElementsKind() == PACKED_SMI_ELEMENTS {
            let double_array = FixedDoubleArray::default();
            for i in 0..length {
                if double_array.is_the_hole(i) {
                    TransitionElementsKind(object, HOLEY_DOUBLE_ELEMENTS);
                    return;
                }
            }
            TransitionElementsKind(object, PACKED_DOUBLE_ELEMENTS);
        }
    }
    pub fn SetMapAndElements(
        object: DirectHandle<JSObject>,
        new_map: DirectHandle<Map>,
        value: DirectHandle<FixedArrayBase>,
    ) {
        let isolate = object.GetIsolate();
        JSObject::MigrateToMap(isolate, object, new_map);
        object.set_elements(*value, WriteBarrierMode::UPDATE_WRITE_BARRIER);
    }
    pub fn initialize_elements(&mut self) {
        let elements = self.map().GetInitialElements();
        self.set_elements(elements, WriteBarrierMode::SKIP_WRITE_BARRIER);
    }
}
impl JSObject {
    pub fn GetIndexedInterceptor(&self, cage_base: PtrComprCageBase) -> Tagged<InterceptorInfo> {
        self.map().GetIndexedInterceptor(cage_base)
    }
    pub fn GetNamedInterceptor(&self, cage_base: PtrComprCageBase) -> Tagged<InterceptorInfo> {
        self.map().GetNamedInterceptor(cage_base)
    }
}
impl JSObject {
    pub fn GetHeaderSize(map: Tagged<Map>) -> i32 {
        let instance_type = map.instance_type();
        if instance_type == InstanceType::JS_OBJECT_TYPE {
            JSObject::kHeaderSize as i32
        } else {
            JSObject::GetHeaderSize(instance_type, map.has_prototype_slot()) as i32
        }
    }
    pub fn GetEmbedderFieldsStartOffset(map: Tagged<Map>) -> i32 {
        JSObject::GetHeaderSize(map) as i32
    }
    pub fn GetEmbedderFieldsStartOffset(&self) -> i32 {
        JSObject::GetEmbedderFieldsStartOffset(self.map()) as i32
    }
    pub fn MayHaveEmbedderFields(map: Tagged<Map>) -> bool {
        let instance_type = map.instance_type();
        InstanceTypeChecker::IsJSObjectWithEmbedderSlots(instance_type)
            || InstanceTypeChecker::IsJSAPIObjectWithEmbedderSlots(instance_type)
            || InstanceTypeChecker::IsJSSpecialObject(instance_type)
    }
    pub fn MayHaveEmbedderFields(&self) -> bool {
        JSObject::MayHaveEmbedderFields(self.map())
    }
    pub fn GetEmbedderFieldCount(map: Tagged<Map>) -> i32 {
        let instance_size = map.instance_size() as i32;
        if instance_size == -1 {
            return 0;
        }
        (instance_size - JSObject::GetEmbedderFieldsStartOffset(map))
            / (size_of::<Tagged<Object>>() as i32)
    }
    pub fn GetEmbedderFieldCount(&self) -> i32 {
        JSObject::GetEmbedderFieldCount(self.map())
    }
    pub fn GetEmbedderFieldOffset(&self, index: i32) -> i32 {
        assert!(index as u32 <= self.GetEmbedderFieldCount() as u32);
        self.GetEmbedderFieldsStartOffset() + (size_of::<Tagged<Object>>() as i32 * index)
    }
    pub fn GetEmbedderField(&self, index: i32) -> Tagged<Object> {
        let slot = EmbedderDataSlot { dummy: 0 };
        slot.load_tagged()
    }
    pub fn SetEmbedderField(&mut self, index: i32, value: Tagged<Object>) {
        let slot = EmbedderDataSlot { dummy: 0 };
        EmbedderDataSlot::store_tagged(*self, index, value);
    }
    pub fn SetEmbedderField_smi(&mut self, index: i32, value: Tagged<Smi>) {
        let slot = EmbedderDataSlot { dummy: 0 };
        EmbedderDataSlot::store_smi(*self, index, value);
    }
    pub fn IsDroppableApiObject(map: Tagged<Map>) -> bool {
        let instance_type = map.instance_type();
        InstanceTypeChecker::IsJSApiObject(instance_type) || instance_type == InstanceType::JS_SPECIAL_API_OBJECT_TYPE
    }
    pub fn IsDroppableApiObject(&self) -> bool {
        JSObject::IsDroppableApiObject(self.map())
    }
    pub fn RawFastPropertyAt(&self, index: FieldIndex) -> Tagged<JSAny> {
        let cage_base = PtrComprCageBase {};
        self.RawFastPropertyAt_cage(cage_base, index)
    }
    pub fn RawFastPropertyAt_cage(&self, cage_base: PtrComprCageBase, index: FieldIndex) -> Tagged<JSAny> {
        if index.is_inobject() {
            TaggedField::<JSAny, 0>::Relaxed_Load(cage_base, *self, index.offset())
        } else {
            self.property_array(cage_base).get(cage_base, index.outobject_array_index())
        }
    }
    pub fn RawFastPropertyAt_seqcst(
        &self,
        index: FieldIndex,
        tag: SeqCstAccessTag,
    ) -> Tagged<JSAny> {
        let cage_base = PtrComprCageBase {};
        self.RawFastPropertyAt_cage_seqcst(cage_base, index, tag)
    }
    pub fn RawFastPropertyAt_cage_seqcst(
        &self,
        cage_base: PtrComprCageBase,
        index: FieldIndex,
        tag: SeqCstAccessTag,
    ) -> Tagged<JSAny> {
        if index.is_inobject() {
            TaggedField::<JSAny, 0>::SeqCst_Load(cage_base, *self, index.offset())
        } else {
            self.property_array(cage_base).get(cage_base, index.outobject_array_index(), tag)
        }
    }
    pub fn RawInobjectPropertyAt(
        &self,
        cage_base: PtrComprCageBase,
        original_map: Tagged<Map>,
        index: FieldIndex,
    ) -> Option<Tagged<Object>> {
        assert!(index.is_inobject());
        let maybe_tagged_object = TaggedField::<Object, 0>::Acquire_Load(cage_base, *self, index.offset());
        if original_map != self.map() {
            return None;
        }
        Some(maybe_tagged_object)
    }
    pub fn RawFastInobjectPropertyAtPut(&mut self, index: FieldIndex, value: Tagged<Object>, mode: WriteBarrierMode) {
        assert!(index.is_inobject());
        let offset = index.offset();
        //RELAXED_WRITE_FIELD(*this, offset, value);
        match mode {
            WriteBarrierMode::UPDATE_WRITE_BARRIER => {
                //CONDITIONAL_WRITE_BARRIER(*this, offset, value, mode);
            }
            WriteBarrierMode::SKIP_WRITE_BARRIER => {}
        }
    }
    pub fn RawFastInobjectPropertyAtPut_seqcst(
        &mut self,
        index: FieldIndex,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) {
        assert!(index.is_inobject());
        assert!(IsShared(value));
        //SEQ_CST_WRITE_FIELD(*this, index.offset(), value);
        //CONDITIONAL_WRITE_BARRIER(*this, index.offset(), value, WriteBarrierMode::UPDATE_WRITE_BARRIER);
    }
    pub fn FastPropertyAtPut(&mut self, index: FieldIndex, value: Tagged<Object>, mode: WriteBarrierMode) {
        if index.is_inobject() {
            self.RawFastInobjectPropertyAtPut(index, value, mode);
        } else {
            assert_eq!(WriteBarrierMode::UPDATE_WRITE_BARRIER, mode);
            self.property_array().set(index.outobject_array_index(), value);
        }
    }
    pub fn FastPropertyAtPut_seqcst(
        &mut self,
        index: FieldIndex,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) {
        if index.is_inobject() {
            self.RawFastInobjectPropertyAtPut_seqcst(index, value, tag);
        } else {
            self.property_array().set(index.outobject_array_index(), value, tag);
        }
    }
    pub fn WriteToField(
        &mut self,
        descriptor: InternalIndex,
        details: PropertyDetails,
        value: Tagged<Object>,
    ) {
        assert_eq!(PropertyLocation::kField, details.location());
        assert_eq!(PropertyKind::kData, details.kind());
        let no_gc = DisallowGarbageCollection {};
        let index = FieldIndex::ForDetails(self.map(), details);
        if details.representation().IsDouble() {
            let bits: u64;
            if IsSmi(value) {
                bits = (Smi::ToInt(value) as f64).to_bits();
            } else if IsUninitialized(value) {
                bits = kHoleNanInt64;
            } else {
                assert!(IsHeapNumber(value));
                let heap_number = HeapNumber {};
                bits = heap_number.value_as_bits();
            }
            let box_ = self.RawFastPropertyAt(index);
            //box_->set_value_as_bits(bits);
        } else {
            self.FastPropertyAtPut(index, value, WriteBarrierMode::UPDATE_WRITE_BARRIER);
        }
    }
    pub fn RawFastInobjectPropertyAtSwap(
        &mut self,
        index: FieldIndex,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) -> Tagged<Object> {
        assert!(index.is_inobject());
        assert!(IsShared(value));
        let offset = index.offset();
        let old_value = Object {};
        //CONDITIONAL_WRITE_BARRIER(*this, offset, value, WriteBarrierMode::UPDATE_WRITE_BARRIER);
        old_value
    }
    pub fn RawFastPropertyAtSwap(
        &mut self,
        index: FieldIndex,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) -> Tagged<Object> {
        if index.is_inobject() {
            self.RawFastInobjectPropertyAtSwap(index, value, tag)
        } else {
            self.property_array().Swap(index.outobject_array_index(), value, tag)
        }
    }
    pub fn RawFastInobjectPropertyAtCompareAndSwap(
        &mut self,
        index: FieldIndex,
        expected: Tagged<Object>,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) -> Tagged<Object> {
        assert!(index.is_inobject());
        assert!(IsShared(value));
        let previous_value = Object {};
        if previous_value == expected {
            //CONDITIONAL_WRITE_BARRIER(*this, index.offset(), value, WriteBarrierMode::UPDATE_WRITE_BARRIER);
        }
        previous_value
    }
    pub fn RawFastPropertyAtCompareAndSwapInternal(
        &mut self,
        index: FieldIndex,
        expected: Tagged<Object>,
        value: Tagged<Object>,
        tag: SeqCstAccessTag,
    ) -> Tagged<Object> {
        if index.is_inobject() {
            self.RawFastInobjectPropertyAtCompareAndSwap(index, expected, value, tag)
        } else {
            self.property_array().CompareAndSwap(
                index.outobject_array_index(),
                expected,
                value,
                tag,
            )
        }
    }
    pub fn GetInObjectPropertyOffset(&self, index: i32) -> i32 {
        self.map().GetInObjectPropertyOffset(index)
    }
    pub fn InObjectPropertyAt(&self, index: i32) -> Tagged<Object> {
        let offset = self.GetInObjectPropertyOffset(index);
        TaggedField::<Object, 0>::load(*self, offset)
    }
    pub fn InObjectPropertyAtPut(
        &mut self,
        index: i32,
        value: Tagged<Object>,
        mode: WriteBarrierMode,
    ) -> Tagged<Object> {
        let offset = self.GetInObjectPropertyOffset(index);
        //WRITE_FIELD(*this, offset, value);
        match mode {
            WriteBarrierMode::UPDATE_WRITE_BARRIER => {
                //CONDITIONAL_WRITE_BARRIER(*this, offset, value, mode);
            }
            WriteBarrierMode::SKIP_WRITE_BARRIER => {}
        }
        value
    }
    pub fn InitializeBody(
        &mut self,
        map: Tagged<Map>,
        start_offset: i32,
        is_slack_tracking_in_progress: bool,
        filler_map: MapWord,
        undefined_filler: Tagged<Object>,
    ) {
        let size = map.instance_size() as i32;
        let mut offset = start_offset;
        if JSObject::MayHaveEmbedderFields(map) {
            let embedder_field_start = JSObject::GetEmbedderFieldsStartOffset(map) as i32;
            let embedder_field_count = JSObject::GetEmbedderFieldCount(map) as i32;
            while offset < embedder_field_start {
                //WRITE_FIELD(*this, offset, undefined_filler);
                offset += size_of::<Tagged<Object>>() as i32;
            }
            assert_eq!(offset, embedder_field_start);
            for i in 0..embedder_field_count {
                let slot = EmbedderDataSlot { dummy: 0 };
                EmbedderDataSlot(*self, i).Initialize(undefined_filler);
                offset += size_of::<EmbedderDataSlot>() as i32;
            }
        } else {
            assert_eq!(0, JSObject::GetEmbedderFieldCount(map));
        }
        assert!(offset <= size);
        if is_slack_tracking_in_progress {
            let end_of_pre_allocated_offset =
                size - (map.UnusedPropertyFields() * size_of::<Tagged<Object>>() as u32) as i32;
            assert!(size_of::<Tagged<Map>>() as i32 <= end_of_pre_allocated_offset);
            assert!(offset <= end_of_pre_allocated_offset);
            while offset < end_of_pre_allocated_offset {
                //WRITE_FIELD(*this, offset, undefined_filler);
                offset += size_of::<Tagged<Object>>() as i32;
            }
            while offset < size {
                let fm = Tagged::<Object>::default();
                //WRITE_FIELD(*this, offset, fm);
                offset += size_of::<Tagged<Object>>() as i32;
            }
        } else {
            while offset < size {
                //WRITE_FIELD(*this, offset, undefined_filler);
                offset += size_of::<Tagged<Object>>() as i32;
            }
        }
    }
    pub fn DefineOwnPropertyIgnoreAttributes<T, HandleType>(
        it: *mut LookupIterator,
        value: HandleType,
        attributes: PropertyAttributes,
        handling: AccessorInfoHandling,
        semantics: EnforceDefineSemantics,
    ) -> Result<HandleType, String> {
        Ok(value)
    }
}
struct JSExternalObject {
    ptr: usize,
}
impl JSExternalObject {
    pub fn new(ptr: usize) -> Self {
        Self { ptr }
    }
}
const kValueOffset: usize = 0;
const kExternalObjectValueTag: usize = 0;
impl JSExternalObject {
    pub fn value(&self) -> *mut void {
        std::ptr::null_mut()
    }
}
struct JSApiWrapper {
    object_: Tagged<JSObject>,
}
impl JSApiWrapper {
    pub fn new(object: Tagged<JSObject>) -> Self {
        Self { object_: object }
    }
    pub fn GetCppHeapWrappable<const lower_bound: usize, const upper_bound: usize>(
        &self,
        isolate: IsolateForPointerCompression,
    ) -> *mut void {
        let address = self.object_.ReadCppHeapPointerField::<lower_bound, upper_bound>(
            kCppHeapWrappableOffset,
            isolate,
        );
        address as *mut void
    }
    pub fn GetCppHeapWrappable_tagrange(
        &self,
        isolate: IsolateForPointerCompression,
        tag_range: CppHeapPointerTagRange,
    ) -> *mut void {
        let address = self.object_.ReadCppHeapPointerField(
            kCppHeapWrappableOffset,
            isolate,
            tag_range,
        );
        address as *mut void
    }
    pub fn SetCppHeapWrappable<const tag: usize>(
        &mut self,
        isolate: IsolateForPointerCompression,
        instance: *mut void,
    ) {
        self.object_.WriteLazilyInitializedCppHeapPointerField::<tag>(
            JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffset,
            isolate,
            instance as usize,
        );
    }
    pub fn SetCppHeapWrappable_tag(
        &mut self,
        isolate: IsolateForPointerCompression,
        instance: *mut void,
        tag: CppHeapPointerTag,
    ) {
        self.object_.WriteLazilyInitializedCppHeapPointerField(
            JSAPIObjectWithEmbedderSlots::kCppHeapWrappableOffset,
            isolate,
            instance as usize,
            tag,
        );
    }
}
impl JSMessageObject {
    pub fn DidEnsureSourcePositionsAvailable(&self) -> bool {
        self.shared_info() == Smi::zero()
    }
    pub fn EnsureSourcePositionsAvailable(
        isolate: *mut isolate,
        message: DirectHandle<JSMessageObject>,
    ) {
        if message.DidEnsureSourcePositionsAvailable() {
            assert!(message.script().has_line_ends());
        } else {
            JSMessageObject::InitializeSourcePositions(isolate, message);
        }
    }
    pub fn GetStartPosition(&self) -> i32 {
        assert!(-1 <= self.start_position());
        self.start_position()
    }
    pub fn GetEndPosition(&self) -> i32 {
        assert!(-1 <= self.end_position());
        self.end_position()
    }
    pub fn type_(&self) -> MessageTemplate {
        MessageTemplate::default()
    }
    pub fn set_type(&mut self, value: MessageTemplate) {
        self.set_raw_type(value as i32);
    }
    pub fn shared_info(&self) -> Tagged<Object> {
        Tagged::<Object>::default()
    }
    pub fn set_shared_info(&mut self, value: Tagged<Object>) {}
    pub fn bytecode_offset(&self) -> Tagged<Smi> {
        Tagged::<Smi>::default()
    }
    pub fn set_bytecode_offset(&mut self, value: Tagged<Smi>) {}
    pub fn start_position(&self) -> i32 {
        0
    }
    pub fn set_start_position(&mut self, value: i32) {}
    pub fn end_position(&self) -> i32 {
        0
    }
    pub fn set_end_position(&mut self, value: i32) {}
    pub fn error_level(&self) -> i32 {
        0
    }
    pub fn set_error_level(&mut self, value: i32) {}
    pub fn raw_type(&self) -> i32 {
        0
    }
    pub fn set_raw_type(&mut self, value: i32) {}
}
impl JSObject {
    pub fn GetElementsKind(&self, cage_base: PtrComprCageBase) -> ElementsKind {
        let kind = self.map().elements_kind();
        kind
    }
    pub fn GetElementsAccessor(&self, cage_base: PtrComprCageBase) -> *mut ElementsAccessor {
        ElementsAccessor::ForKind(self.GetElementsKind(cage_base))
    }
    pub fn HasObjectElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsObjectElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasSmiElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsSmiElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasSmiOrObjectElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsSmiOrObjectElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasDoubleElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsDoubleElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasHoleyElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsHoleyElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasFastElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsFastElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasFastPackedElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsFastPackedElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasDictionaryElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsDictionaryElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasPackedElements(&self, cage_base: PtrComprCageBase) -> bool {
        self.GetElementsKind(cage_base) == ElementsKind::PACKED_ELEMENTS
    }
    pub fn HasAnyNonextensibleElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsAnyNonextensibleElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasSealedElements(&self, cage_base: PtrComprCageBase) -> bool {
        IsSealedElementsKind(self.GetElementsKind(cage_base))
    }
    pub fn HasSharedArrayElements(&self, cage_base: PtrCompr
