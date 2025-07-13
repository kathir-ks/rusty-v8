// Converted from V8 C++ source files:
// Header: growable-fixed-array-gen.h
// Implementation: growable-fixed-array-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/builtins/growable-fixed-array-gen.h
pub struct GrowableFixedArray {
    state: compiler::CodeAssemblerState,
    var_array_: TVariable<FixedArray>,
    var_length_: TVariable<IntPtrT>,
    var_capacity_: TVariable<IntPtrT>,
    assembler: CodeStubAssembler,
}

impl GrowableFixedArray {
    pub fn new(state: compiler::CodeAssemblerState) -> Self {
        let mut result = GrowableFixedArray {
            state,
            var_array_: TVariable::new(),
            var_length_: TVariable::new(),
            var_capacity_: TVariable::new(),
            assembler: CodeStubAssembler::new(state),
        };
        result.var_array_.bind(result.assembler.EmptyFixedArrayConstant());
        result.var_capacity_.bind(result.assembler.IntPtrConstant(0));
        result.var_length_.bind(result.assembler.IntPtrConstant(0));
        result
    }

    pub fn length(&self) -> TNode<IntPtrT> {
        self.var_length_.value()
    }

    pub fn var_array(&mut self) -> &mut TVariable<FixedArray> {
        &mut self.var_array_
    }
    pub fn var_length(&mut self) -> &mut TVariable<IntPtrT> {
        &mut self.var_length_
    }
    pub fn var_capacity(&mut self) -> &mut TVariable<IntPtrT> {
        &mut self.var_capacity_
    }

    pub fn reserve(&mut self, required_capacity: TNode<IntPtrT>) {
        let mut out = Label::new();

        if self.assembler.IntPtrGreaterThanOrEqual(self.var_capacity_.value(), required_capacity) {
            self.assembler.goto(&mut out);
            return;
        }

        // Gotta grow.
        let mut var_new_capacity = TVariable::new();
        var_new_capacity.bind(self.var_capacity_.value());
        let mut loop_label = Label::new();
        self.assembler.goto(&mut loop_label);

        // First find the new capacity.
        self.assembler.bind(&mut loop_label);
        {
            var_new_capacity.bind(self.new_capacity(var_new_capacity.value()));
            if self.assembler.IntPtrLessThan(var_new_capacity.value(), required_capacity) {
                self.assembler.goto(&mut loop_label);
            }
        }

        // Now grow.
        self.var_capacity_.bind(var_new_capacity.value());
        self.var_array_.bind(self.resize_fixed_array(self.var_length_.value(), self.var_capacity_.value()));
        self.assembler.goto(&mut out);

        self.assembler.bind(&mut out);
    }

    pub fn push(&mut self, value: TNode<Object>) {
        let length = self.var_length_.value();
        let capacity = self.var_capacity_.value();

        let mut grow = Label::new();
        let mut store = Label::new();
        if self.assembler.IntPtrEqual(capacity, length) {
            self.assembler.branch(&mut grow, &mut store);
        } else {
            self.assembler.goto(&mut store);
            self.assembler.block_sequence(vec![&mut grow, &mut store]);
            return;
        }

        self.assembler.bind(&mut grow);
        {
            self.var_capacity_.bind(self.new_capacity(capacity));
            self.var_array_.bind(self.resize_fixed_array(length, self.var_capacity_.value()));

            self.assembler.goto(&mut store);
        }

        self.assembler.bind(&mut store);
        {
            let array = self.var_array_.value();
            self.assembler.unsafe_store_fixed_array_element(array, length, value);

            self.var_length_.bind(self.assembler.IntPtrAdd(length, self.assembler.IntPtrConstant(1)));
        }
    }

    pub fn to_fixed_array(&mut self) -> TNode<FixedArray> {
        self.resize_fixed_array(self.length(), self.length())
    }

    pub fn to_js_array(&mut self, context: TNode<Context>) -> TNode<JSArray> {
        let kind = ElementsKind::PACKED_ELEMENTS;

        let native_context = self.assembler.load_native_context(context);
        let array_map = self.assembler.load_js_array_elements_map(kind, native_context);

        // Shrink to fit if necessary.
        {
            let mut next = Label::new();

            let length = self.var_length_.value();
            let capacity = self.var_capacity_.value();

            if self.assembler.WordEqual(length, capacity) {
                self.assembler.goto(&mut next);
            } else {
                self.var_array_.bind(self.resize_fixed_array(length, length));
                self.var_capacity_.bind(length);
                self.assembler.goto(&mut next);
            }

            self.assembler.bind(&mut next);
        }

        let result_length = self.assembler.smi_tag(self.length());
        let result = self.assembler.allocate_js_array(array_map, self.var_array_.value(), result_length);
        result
    }

    fn new_capacity(&self, current_capacity: TNode<IntPtrT>) -> TNode<IntPtrT> {
        self.assembler.csa_dcheck(
            IntPtrGreaterThanOrEqualAssertion {
                node: current_capacity,
                constant: 0,
            }
        );

        // Growth rate is analog to JSObject::NewElementsCapacity:
        // new_capacity = (current_capacity + (current_capacity >> 1)) + 16.

        let new_capacity = self.assembler.IntPtrAdd(
            self.assembler.IntPtrAdd(current_capacity, self.assembler.WordShr(current_capacity, 1)),
            self.assembler.IntPtrConstant(16),
        );

        new_capacity
    }

    fn resize_fixed_array(&mut self, element_count: TNode<IntPtrT>, new_capacity: TNode<IntPtrT>) -> TNode<FixedArray> {
        self.assembler.csa_dcheck(
            IntPtrGreaterThanOrEqualAssertion {
                node: element_count,
                constant: 0,
            }
        );
        self.assembler.csa_dcheck(
            IntPtrGreaterThanOrEqualAssertion {
                node: new_capacity,
                constant: 0,
            }
        );
        self.assembler.csa_dcheck(
            IntPtrGreaterThanOrEqualAssertion {
                node: new_capacity,
                constant: element_count,
            }
        );

        let from_array = self.var_array_.value();

        let mut flags = CodeStubAssembler::ExtractFixedArrayFlags::kFixedArrays;
        let to_array = self.assembler.extract_fixed_array(
            from_array,
            None,
            Some(element_count),
            Some(new_capacity),
            flags,
        );

        to_array.into()
    }
}

// Mocked structs and enums

pub struct TVariable<T> {
    value: Option<T>,
}

impl<T> TVariable<T> {
    pub fn new() -> Self {
        TVariable { value: None }
    }
    pub fn bind(&mut self, value: T) {
        self.value = Some(value);
    }
    pub fn value(&self) -> T where T: Copy {
        self.value.unwrap()
    }
}

impl<T> Default for TVariable<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Copy)]
pub struct TNode<T> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T> From<TNode<FixedArray>> for TNode<Object> {
    fn from(node: TNode<FixedArray>) -> Self {
        TNode { _phantom: std::marker::PhantomData }
    }
}

impl<T> From<TNode<JSArray>> for TNode<Object> {
    fn from(node: TNode<JSArray>) -> Self {
        TNode { _phantom: std::marker::PhantomData }
    }
}

impl TNode<FixedArray> {
    fn into(self) -> TNode<FixedArray> {
        TNode { _phantom: std::marker::PhantomData }
    }
}

#[derive(Clone, Copy)]
pub struct IntPtrT;

#[derive(Clone, Copy)]
pub struct Object;

#[derive(Clone, Copy)]
pub struct FixedArray;

#[derive(Clone, Copy)]
pub struct Context;

#[derive(Clone, Copy)]
pub struct JSArray;

#[derive(Clone, Copy)]
pub struct NativeContext;

#[derive(Clone, Copy)]
pub struct Map;

#[derive(Clone, Copy)]
pub struct Smi;

pub struct Label {
    name: String,
}

impl Label {
    pub fn new() -> Self {
        Label { name: String::from("default") }
    }
}

pub mod compiler {
    pub struct CodeAssemblerState {}
}

pub mod CodeStubAssemblerHelper {
    use super::*;
    pub trait CodeAssembler {
        fn empty_fixed_array_constant(&self) -> TNode<FixedArray>;
        fn intptr_constant(&self, value: i64) -> TNode<IntPtrT>;
        fn unsafe_store_fixed_array_element(&self, array: TNode<FixedArray>, index: TNode<IntPtrT>, value: TNode<Object>);
        fn smi_tag(&self, value: TNode<IntPtrT>) -> TNode<Smi>;
        fn allocate_js_array(&self, array_map: TNode<Map>, elements: TNode<FixedArray>, length: TNode<Smi>) -> TNode<JSArray>;
        fn load_native_context(&self, context: TNode<Context>) -> TNode<NativeContext>;
        fn load_js_array_elements_map(&self, kind: ElementsKind, native_context: TNode<NativeContext>) -> TNode<Map>;
        fn extract_fixed_array(
            &self,
            from_array: TNode<FixedArray>,
            first_element: Option<TNode<IntPtrT>>,
            element_count: Option<TNode<IntPtrT>>,
            new_capacity: Option<TNode<IntPtrT>>,
            flags: CodeStubAssembler::ExtractFixedArrayFlags,
        ) -> TNode<FixedArray>;
        fn csa_dcheck(&self, assertion: IntPtrGreaterThanOrEqualAssertion);
        fn word_shr(&self, value: TNode<IntPtrT>, shift: i32) -> TNode<IntPtrT>;
        fn branch(&self, true_label: &mut Label, false_label: &mut Label);
        fn goto(&self, label: &mut Label);
        fn block_sequence(&self, labels: Vec<&mut Label>);
    }
}

impl CodeStubAssemblerHelper::CodeAssembler for CodeStubAssembler {
    fn empty_fixed_array_constant(&self) -> TNode<FixedArray> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn intptr_constant(&self, value: i64) -> TNode<IntPtrT> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn unsafe_store_fixed_array_element(&self, array: TNode<FixedArray>, index: TNode<IntPtrT>, value: TNode<Object>) {
    }

    fn smi_tag(&self, value: TNode<IntPtrT>) -> TNode<Smi> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn allocate_js_array(&self, array_map: TNode<Map>, elements: TNode<FixedArray>, length: TNode<Smi>) -> TNode<JSArray> {
        TNode { _phantom: std::marker::PhantomData }
    }

     fn load_native_context(&self, context: TNode<Context>) -> TNode<NativeContext> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn load_js_array_elements_map(&self, kind: ElementsKind, native_context: TNode<NativeContext>) -> TNode<Map> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn extract_fixed_array(
        &self,
        from_array: TNode<FixedArray>,
        first_element: Option<TNode<IntPtrT>>,
        element_count: Option<TNode<IntPtrT>>,
        new_capacity: Option<TNode<IntPtrT>>,
        flags: CodeStubAssembler::ExtractFixedArrayFlags,
    ) -> TNode<FixedArray> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn csa_dcheck(&self, _assertion: IntPtrGreaterThanOrEqualAssertion) {}

    fn word_shr(&self, value: TNode<IntPtrT>, shift: i32) -> TNode<IntPtrT> {
        TNode { _phantom: std::marker::PhantomData }
    }

    fn branch(&self, true_label: &mut Label, false_label: &mut Label) {}
    fn goto(&self, label: &mut Label) {}
    fn block_sequence(&self, labels: Vec<&mut Label>) {}

}

pub struct CodeStubAssembler {
    state: compiler::CodeAssemblerState,
}

impl CodeStubAssembler {
    pub fn new(state: compiler::CodeAssemblerState) -> Self {
        CodeStubAssembler { state }
    }
    fn EmptyFixedArrayConstant(&self) -> TNode<FixedArray> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn IntPtrConstant(&self, _value: i64) -> TNode<IntPtrT> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn IntPtrGreaterThanOrEqual(&self, a: TNode<IntPtrT>, b: TNode<IntPtrT>) -> bool {
        false
    }
    fn IntPtrLessThan(&self, a: TNode<IntPtrT>, b: TNode<IntPtrT>) -> bool {
        false
    }
    fn IntPtrAdd(&self, a: TNode<IntPtrT>, b: TNode<IntPtrT>) -> TNode<IntPtrT> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn WordEqual(&self, a: TNode<IntPtrT>, b: TNode<IntPtrT>) -> bool {
        false
    }

    pub enum ExtractFixedArrayFlag {
        kFixedArrays = 0,
    }

    pub struct ExtractFixedArrayFlags(i32);

    impl std::ops::BitOr for ExtractFixedArrayFlags {
        type Output = Self;

        fn bitor(self, rhs: Self) -> Self {
            ExtractFixedArrayFlags(self.0 | rhs.0)
        }
    }
    fn extract_fixed_array(
        &self,
        from_array: TNode<FixedArray>,
        first_element: Option<TNode<IntPtrT>>,
        element_count: Option<TNode<IntPtrT>>,
        new_capacity: Option<TNode<IntPtrT>>,
        flags: CodeStubAssembler::ExtractFixedArrayFlags,
    ) -> TNode<FixedArray> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn csa_dcheck(&self, assertion: IntPtrGreaterThanOrEqualAssertion) {}

    fn word_shr(&self, value: TNode<IntPtrT>, shift: i32) -> TNode<IntPtrT> {
        TNode { _phantom: std::marker::PhantomData }
    }
    fn unsafe_store_fixed_array_element(&self, array: TNode<FixedArray>, index: TNode<IntPtrT>, value: TNode<Object>){}
    fn smi_tag(&self, value: TNode<IntPtrT>) -> TNode<Smi>{TNode { _phantom: std::marker::PhantomData }}
    fn load_native_context(&self, context: TNode<Context>) -> TNode<NativeContext> {TNode { _phantom: std::marker::PhantomData }}
    fn load_js_array_elements_map(&self, kind: ElementsKind, native_context: TNode<NativeContext>) -> TNode<Map> {TNode { _phantom: std::marker::PhantomData }}
    fn allocate_js_array(&self, array_map: TNode<Map>, elements: TNode<FixedArray>, length: TNode<Smi>) -> TNode<JSArray> {TNode { _phantom: std::marker::PhantomData }}
}

struct IntPtrGreaterThanOrEqualAssertion {
    node: TNode<IntPtrT>,
    constant: i32,
}
