// Converted from V8 C++ source files:
// Header: N/A
// Implementation: builtins-conversion-gen.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
struct Context {}
struct Object {}
struct Number {}
struct Numeric {}
struct Smi {}
struct FeedbackVector {}
struct UintPtrT {}
struct TNode<T> {}
struct TVARIABLE<T> {
    value: Option<T>,
}

impl<T> TVARIABLE<T> {
    fn new() -> Self {
        TVARIABLE { value: None }
    }
    fn value(&self) -> &Option<T> {
        &self.value
    }
}
struct CodeStubAssembler {
}

impl CodeStubAssembler {
    fn Return<T>(&self, _value: T) {}
    fn Parameter<T>(&self, _descriptor: i32) -> T {
        Self::create_dummy()
    }
    fn UncheckedParameter<T>(&self, _descriptor: i32) -> T {
        Self::create_dummy()
    }
    fn create_dummy<T>() -> T {
        unsafe { std::mem::zeroed() }
    }
    fn LoadContextFromBaseline(&self) -> Context {
        Context {}
    }
    fn LoadFeedbackVectorFromBaseline(&self) -> FeedbackVector {
        FeedbackVector {}
    }
    fn UpdateFeedback(&self, _value: &Option<Smi>, _feedback_vector: FeedbackVector, _slot: UintPtrT) {}
    fn TrueConstant(&self) -> bool {
        true
    }
    fn FalseConstant(&self) -> bool {
        false
    }
    fn ChangeFloat64ToTagged<T>(&self, _value: f64) -> T {
        Self::create_dummy()
    }
    fn BranchIfToBooleanIsTrue(&self, _value: Object, _return_true: &Label, _return_false: &Label) {}
    fn Typeof(&self, _object: Object) -> String {
        String::from("typeof")
    }
    fn Typeof(&self, _object: Object, _slot: UintPtrT, _feedback_vector: FeedbackVector) -> String {
        String::from("typeof")
    }

}

struct DirectHandle<T> {}
struct Tagged<T> {}

struct Label {}
struct Register {}
struct Managed<T> {}
struct DisplayNamesInternal {}
struct FeedbackSlot {}
struct OpIndex {}
struct Vector<T> {
    data: Vec<T>,
}

impl<T> Vector<T> {
    pub fn of(data: Vec<T>) -> Vector<T> {
        Vector { data }
    }
}

enum BigIntHandling {
    kConvertToNumber,
}

impl CodeStubAssembler {
    fn ToNumber(&self, _context: Context, _input: Object) -> Number {
        Number {}
    }

    fn ToBigInt(&self, _context: Context, _input: Object) -> Object {
        Object {}
    }

    fn ToNumberOrNumeric(&self, _context: Context, _input: Object, var_type_feedback: &mut TVARIABLE<Smi>, _conversion: i32) -> TNode<Numeric> {
        var_type_feedback.value = Some(Smi {});
        TNode {}
    }

    fn PlainPrimitiveToNumber(&self, _input: Object) -> Number {
        Number {}
    }

    fn ToNumber(&self, _context: Context, _input: Object, _big_int_handling: BigIntHandling) -> Number {
        Number {}
    }

    fn ToBigIntConvertNumber(&self, _context: Context, _input: Object) -> Object {
        Object {}
    }

    fn Float64Round(&self, _value: f64) -> f64 {
        _value.round()
    }

    fn Float64Floor(&self, _value: f64) -> f64 {
        _value.floor()
    }

    fn Float64Ceil(&self, _value: f64) -> f64 {
        _value.ceil()
    }

    fn ChangeNumberToFloat64(&self, _value: Number) -> f64 {
        0.0
    }
}

mod Descriptor {
    pub const kContext: i32 = 0;
    pub const kArgument: i32 = 1;
    pub const kSlot: i32 = 2;
    pub const kObject: i32 = 3;
    pub const kValue: i32 = 4;
}

fn foo() {
    let mut var_type_feedback = TVARIABLE::<Smi>::new();
    var_type_feedback.value = Some(Smi {});
    println!("{:?}", var_type_feedback.value());
}

mod internal {
    use super::*;

    macro_rules! TF_BUILTIN {
        ($name:ident, $assembler:ident) => {
            fn $name(_assembler: $assembler) {}
        };
    }

    TF_BUILTIN!(ToNumber, CodeStubAssembler);

    TF_BUILTIN!(ToBigInt, CodeStubAssembler);

    TF_BUILTIN!(ToNumber_Baseline, CodeStubAssembler);

    TF_BUILTIN!(ToNumeric_Baseline, CodeStubAssembler);

    TF_BUILTIN!(PlainPrimitiveToNumber, CodeStubAssembler);

    TF_BUILTIN!(ToNumberConvertBigInt, CodeStubAssembler);

    TF_BUILTIN!(ToBigIntConvertNumber, CodeStubAssembler);

    TF_BUILTIN!(ToBooleanLazyDeoptContinuation, CodeStubAssembler);

    TF_BUILTIN!(MathRoundContinuation, CodeStubAssembler);

    TF_BUILTIN!(MathFloorContinuation, CodeStubAssembler);

    TF_BUILTIN!(MathCeilContinuation, CodeStubAssembler);

    TF_BUILTIN!(Typeof, CodeStubAssembler);

    TF_BUILTIN!(Typeof_Baseline, CodeStubAssembler);
}
} // namespace v8
