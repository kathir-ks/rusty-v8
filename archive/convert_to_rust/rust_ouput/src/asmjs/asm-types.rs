// Converted from V8 C++ source files:
// Header: asm-types.h
// Implementation: asm-types.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod asm_types {
    use std::string::String;

    pub struct Heap {}

    pub struct Zone {}

    impl Zone {
        pub fn new() -> Zone {
            Zone {}
        }

        pub fn New<T>(&self) -> Box<T>
        where
            T: Default,
        {
            Box::new(T::default())
        }
    }

    pub struct ZoneVector<T> {
        data: Vec<T>,
    }

    impl<T> ZoneVector<T> {
        pub fn new() -> ZoneVector<T> {
            ZoneVector { data: Vec::new() }
        }

        pub fn push_back(&mut self, value: T) {
            self.data.push(value);
        }

        pub fn len(&self) -> usize {
            self.data.len()
        }

        pub fn get(&self, index: usize) -> Option<&T> {
            self.data.get(index)
        }
    }

    pub struct NON_EXPORTED_BASE {}

    pub struct V8_EXPORT_PRIVATE {}

    #[derive(Debug, PartialEq, Eq, Clone, Copy)]
    pub struct AsmValueType {
        bits: u32,
    }

    impl AsmValueType {
        pub const kAsmHeap: u32 = (1 << 1) | (0);
        pub const kAsmFloatishDoubleQ: u32 = (1 << 2) | (0);
        pub const kAsmFloatQDoubleQ: u32 = (1 << 3) | (0);
        pub const kAsmVoid: u32 = (1 << 4) | (0);
        pub const kAsmExtern: u32 = (1 << 5) | (0);
        pub const kAsmDoubleQ: u32 = (1 << 6) | (AsmValueType::kAsmFloatishDoubleQ | AsmValueType::kAsmFloatQDoubleQ);
        pub const kAsmDouble: u32 = (1 << 7) | (AsmValueType::kAsmDoubleQ | AsmValueType::kAsmExtern);
        pub const kAsmIntish: u32 = (1 << 8) | (0);
        pub const kAsmInt: u32 = (1 << 9) | (AsmValueType::kAsmIntish);
        pub const kAsmSigned: u32 = (1 << 10) | (AsmValueType::kAsmInt | AsmValueType::kAsmExtern);
        pub const kAsmUnsigned: u32 = (1 << 11) | (AsmValueType::kAsmInt);
        pub const kAsmFixNum: u32 = (1 << 12) | (AsmValueType::kAsmSigned | AsmValueType::kAsmUnsigned);
        pub const kAsmFloatish: u32 = (1 << 13) | (AsmValueType::kAsmFloatishDoubleQ);
        pub const kAsmFloatQ: u32 = (1 << 14) | (AsmValueType::kAsmFloatQDoubleQ | AsmValueType::kAsmFloatish);
        pub const kAsmFloat: u32 = (1 << 15) | (AsmValueType::kAsmFloatQ);
        pub const kAsmUint8Array: u32 = (1 << 16) | (AsmValueType::kAsmHeap);
        pub const kAsmInt8Array: u32 = (1 << 17) | (AsmValueType::kAsmHeap);
        pub const kAsmUint16Array: u32 = (1 << 18) | (AsmValueType::kAsmHeap);
        pub const kAsmInt16Array: u32 = (1 << 19) | (AsmValueType::kAsmHeap);
        pub const kAsmUint32Array: u32 = (1 << 20) | (AsmValueType::kAsmHeap);
        pub const kAsmInt32Array: u32 = (1 << 21) | (AsmValueType::kAsmHeap);
        pub const kAsmFloat32Array: u32 = (1 << 22) | (AsmValueType::kAsmHeap);
        pub const kAsmFloat64Array: u32 = (1 << 23) | (AsmValueType::kAsmHeap);
        pub const kAsmNone: u32 = (1 << 31) | (0);
        pub const kAsmUnknown: u32 = 0;
        pub const kAsmValueTypeTag: u32 = 1;

        fn new(bits: u32) -> Self {
            AsmValueType { bits }
        }

        pub fn bitset(&self) -> u32 {
            self.bits
        }
    }

    #[derive(Debug, PartialEq, Eq)]
    pub enum AsmTypeEnum {
        Value(AsmValueType),
        Callable(Box<dyn AsmCallableType>),
    }

    #[derive(Debug, PartialEq, Eq)]
    pub struct AsmType {
        pub kind: AsmTypeEnum,
    }

    impl AsmType {
        pub fn Heap() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmHeap)),
            }
        }

        pub fn FloatishDoubleQ() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloatishDoubleQ)),
            }
        }

        pub fn FloatQDoubleQ() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloatQDoubleQ)),
            }
        }

        pub fn Void() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmVoid)),
            }
        }

        pub fn Extern() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmExtern)),
            }
        }

        pub fn DoubleQ() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmDoubleQ)),
            }
        }

        pub fn Double() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmDouble)),
            }
        }

        pub fn Intish() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmIntish)),
            }
        }

        pub fn Int() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmInt)),
            }
        }

        pub fn Signed() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmSigned)),
            }
        }

        pub fn Unsigned() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmUnsigned)),
            }
        }

        pub fn FixNum() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFixNum)),
            }
        }

        pub fn Floatish() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloatish)),
            }
        }

        pub fn FloatQ() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloatQ)),
            }
        }

        pub fn Float() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloat)),
            }
        }

        pub fn Uint8Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmUint8Array)),
            }
        }

        pub fn Int8Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmInt8Array)),
            }
        }

        pub fn Uint16Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmUint16Array)),
            }
        }

        pub fn Int16Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmInt16Array)),
            }
        }

        pub fn Uint32Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmUint32Array)),
            }
        }

        pub fn Int32Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmInt32Array)),
            }
        }

        pub fn Float32Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloat32Array)),
            }
        }

        pub fn Float64Array() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmFloat64Array)),
            }
        }

        pub fn None() -> Self {
            AsmType {
                kind: AsmTypeEnum::Value(AsmValueType::new(AsmValueType::kAsmNone)),
            }
        }

        pub fn Function(zone: &Zone, ret: AsmType) -> Self {
            let f = zone.New::<AsmFunctionType>();
            f.return_type_ = Some(Box::new(ret));
            AsmType {
                kind: AsmTypeEnum::Callable(f),
            }
        }

        pub fn OverloadedFunction(zone: &Zone) -> Self {
            let f = zone.New::<AsmOverloadedFunctionType>();
            AsmType {
                kind: AsmTypeEnum::Callable(f),
            }
        }

        pub fn FroundType(zone: &Zone) -> Self {
            let fround = zone.New::<AsmFroundType>();
            AsmType {
                kind: AsmTypeEnum::Callable(fround),
            }
        }

        pub fn MinMaxType(zone: &Zone, dest: AsmType, src: AsmType) -> Self {
            let min_max = zone.New::<AsmMinMaxType>();
            min_max.return_type_ = Some(Box::new(dest));
            min_max.arg_ = Some(Box::new(src));
            AsmType {
                kind: AsmTypeEnum::Callable(min_max),
            }
        }

        pub fn AsValueType(&self) -> Option<&AsmValueType> {
            match &self.kind {
                AsmTypeEnum::Value(value) => Some(value),
                _ => None,
            }
        }

        pub fn AsCallableType(&self) -> Option<&dyn AsmCallableType> {
            match &self.kind {
                AsmTypeEnum::Callable(callable) => Some(callable.as_ref()),
                _ => None,
            }
        }

        pub fn AsFunctionType(&self) -> Option<&AsmFunctionType> {
            match &self.kind {
                AsmTypeEnum::Callable(callable) => callable.AsFunctionType(),
                _ => None,
            }
        }

        pub fn AsOverloadedFunctionType(&self) -> Option<&AsmOverloadedFunctionType> {
            match &self.kind {
                AsmTypeEnum::Callable(callable) => callable.AsOverloadedFunctionType(),
                _ => None,
            }
        }

        pub fn Name(&self) -> String {
            match &self.kind {
                AsmTypeEnum::Value(avt) => {
                    match avt.bitset() {
                        AsmValueType::kAsmHeap => "[]".to_string(),
                        AsmValueType::kAsmFloatishDoubleQ => "floatish|double?".to_string(),
                        AsmValueType::kAsmFloatQDoubleQ => "float?|double?".to_string(),
                        AsmValueType::kAsmVoid => "void".to_string(),
                        AsmValueType::kAsmExtern => "extern".to_string(),
                        AsmValueType::kAsmDoubleQ => "double?".to_string(),
                        AsmValueType::kAsmDouble => "double".to_string(),
                        AsmValueType::kAsmIntish => "intish".to_string(),
                        AsmValueType::kAsmInt => "int".to_string(),
                        AsmValueType::kAsmSigned => "signed".to_string(),
                        AsmValueType::kAsmUnsigned => "unsigned".to_string(),
                        AsmValueType::kAsmFixNum => "fixnum".to_string(),
                        AsmValueType::kAsmFloatish => "floatish".to_string(),
                        AsmValueType::kAsmFloatQ => "float?".to_string(),
                        AsmValueType::kAsmFloat => "float".to_string(),
                        AsmValueType::kAsmUint8Array => "Uint8Array".to_string(),
                        AsmValueType::kAsmInt8Array => "Int8Array".to_string(),
                        AsmValueType::kAsmUint16Array => "Uint16Array".to_string(),
                        AsmValueType::kAsmInt16Array => "Int16Array".to_string(),
                        AsmValueType::kAsmUint32Array => "Uint32Array".to_string(),
                        AsmValueType::kAsmInt32Array => "Int32Array".to_string(),
                        AsmValueType::kAsmFloat32Array => "Float32Array".to_string(),
                        AsmValueType::kAsmFloat64Array => "Float64Array".to_string(),
                        AsmValueType::kAsmNone => "<none>".to_string(),
                        _ => panic!("Unexpected bitset value"),
                    }
                }
                AsmTypeEnum::Callable(callable) => callable.Name(),
            }
        }

        pub fn IsExactly(x: &AsmType, y: &AsmType) -> bool {
            match (&x.kind, &y.kind) {
                (AsmTypeEnum::Value(x_avt), AsmTypeEnum::Value(y_avt)) => x_avt == y_avt,
                (AsmTypeEnum::Callable(_), AsmTypeEnum::Callable(_)) => std::ptr::eq(x, y),
                _ => false,
            }
        }

        pub fn IsA(&self, that: &AsmType) -> bool {
            match (&self.kind, &that.kind) {
                (AsmTypeEnum::Value(avt), AsmTypeEnum::Value(tavt)) => {
                    (avt.bitset() & tavt.bitset()) == tavt.bitset()
                }
                (AsmTypeEnum::Callable(this_callable), AsmTypeEnum::Callable(that_callable)) => {
                    this_callable.IsA(that)
                }
                _ => false,
            }
        }

        pub const kNotHeapType: i32 = -1;

        pub fn ElementSizeInBytes(&self) -> i32 {
            if let Some(value) = self.AsValueType() {
                match value.bitset() {
                    AsmValueType::kAsmInt8Array | AsmValueType::kAsmUint8Array => 1,
                    AsmValueType::kAsmInt16Array | AsmValueType::kAsmUint16Array => 2,
                    AsmValueType::kAsmInt32Array | AsmValueType::kAsmUint32Array | AsmValueType::kAsmFloat32Array => 4,
                    AsmValueType::kAsmFloat64Array => 8,
                    _ => AsmType::kNotHeapType,
                }
            } else {
                AsmType::kNotHeapType
            }
        }

        pub fn LoadType(&self) -> AsmType {
            if let Some(value) = self.AsValueType() {
                match value.bitset() {
                    AsmValueType::kAsmInt8Array | AsmValueType::kAsmUint8Array |
                    AsmValueType::kAsmInt16Array | AsmValueType::kAsmUint16Array |
                    AsmValueType::kAsmInt32Array | AsmValueType::kAsmUint32Array => AsmType::Intish(),
                    AsmValueType::kAsmFloat32Array => AsmType::FloatQ(),
                    AsmValueType::kAsmFloat64Array => AsmType::DoubleQ(),
                    _ => AsmType::None(),
                }
            } else {
                AsmType::None()
            }
        }

        pub fn StoreType(&self) -> AsmType {
            if let Some(value) = self.AsValueType() {
                match value.bitset() {
                    AsmValueType::kAsmInt8Array | AsmValueType::kAsmUint8Array |
                    AsmValueType::kAsmInt16Array | AsmValueType::kAsmUint16Array |
                    AsmValueType::kAsmInt32Array | AsmValueType::kAsmUint32Array => AsmType::Intish(),
                    AsmValueType::kAsmFloat32Array => AsmType::FloatishDoubleQ(),
                    AsmValueType::kAsmFloat64Array => AsmType::FloatQDoubleQ(),
                    _ => AsmType::None(),
                }
            } else {
                AsmType::None()
            }
        }
    }

    pub trait AsmCallableType {
        fn Name(&self) -> String;
        fn CanBeInvokedWith(&self, return_type: &AsmType, args: &ZoneVector<AsmType>) -> bool;
        fn AsFunctionType(&self) -> Option<&AsmFunctionType> {
            None
        }
        fn AsOverloadedFunctionType(&self) -> Option<&AsmOverloadedFunctionType> {
            None
        }
        fn IsA(&self, other: &AsmType) -> bool;
    }

    #[derive(Default)]
    pub struct AsmFunctionType {
        pub return_type_: Option<Box<AsmType>>,
        pub args_: ZoneVector<AsmType>,
    }

    impl AsmFunctionType {
        pub fn AddArgument(&mut self, type_: AsmType) {
            self.args_.push_back(type_);
        }

        pub fn Arguments(&self) -> &ZoneVector<AsmType> {
            &self.args_
        }

        pub fn ReturnType(&self) -> Option<&AsmType> {
            self.return_type_.as_ref().map(|x| x.as_ref())
        }
    }

    impl AsmCallableType for AsmFunctionType {
        fn Name(&self) -> String {
            let mut ret = "(".to_string();
            for ii in 0..self.args_.len() {
                if let Some(arg) = self.args_.get(ii) {
                    ret += &arg.Name();
                    if ii != self.args_.len() - 1 {
                        ret += ", ";
                    }
                }
            }
            ret += ") -> ";
            if let Some(return_type) = &self.return_type_ {
                ret += &return_type.Name();
            }
            ret
        }

        fn CanBeInvokedWith(&self, return_type: &AsmType, args: &ZoneVector<AsmType>) -> bool {
            match &self.return_type_ {
                Some(expected_return_type) => {
                    if !AsmType::IsExactly(expected_return_type.as_ref(), return_type) {
                        return false;
                    }
                }
                None => return false,
            }

            if self.args_.len() != args.len() {
                return false;
            }

            for ii in 0..self.args_.len() {
                if let (Some(arg), Some(expected_arg)) = (args.get(ii), self.args_.get(ii)) {
                    if !arg.IsA(expected_arg) {
                        return false;
                    }
                } else {
                    return false;
                }
            }

            true
        }

        fn AsFunctionType(&self) -> Option<&AsmFunctionType> {
            Some(self)
        }

        fn IsA(&self, other: &AsmType) -> bool {
            if let Some(that) = other.AsFunctionType() {
                match &self.return_type_ {
                    Some(return_type_) => {
                        if !AsmType::IsExactly(return_type_.as_ref(), &that.return_type_.as_ref().unwrap()) {
                            return false;
                        }
                    }
                    None => return false,
                }

                if self.args_.len() != that.args_.len() {
                    return false;
                }

                for ii in 0..self.args_.len() {
                    if let (Some(arg), Some(that_arg)) = (self.args_.get(ii), that.args_.get(ii)) {
                        if !AsmType::IsExactly(arg, that_arg) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }

                return true;
            }
            false
        }
    }

    #[derive(Default)]
    pub struct AsmOverloadedFunctionType {
        overloads_: ZoneVector<AsmType>,
    }

    impl AsmOverloadedFunctionType {
        pub fn AddOverload(&mut self, overload: AsmType) {
            if overload.AsCallableType().is_none() {
                panic!("Overload must be a callable type");
            }
            self.overloads_.push_back(overload);
        }
    }

    impl AsmCallableType for AsmOverloadedFunctionType {
        fn Name(&self) -> String {
            let mut ret = String::new();
            for ii in 0..self.overloads_.len() {
                if ii != 0 {
                    ret += " /\\ ";
                }
                if let Some(overload) = self.overloads_.get(ii) {
                    ret += &overload.Name();
                }
            }
            ret
        }

        fn CanBeInvokedWith(&self, return_type: &AsmType, args: &ZoneVector<AsmType>) -> bool {
            for ii in 0..self.overloads_.len() {
                if let Some(overload) = self.overloads_.get(ii) {
                    if let Some(callable) = overload.AsCallableType() {
                        if callable.CanBeInvokedWith(return_type, args) {
                            return true;
                        }
                    }
                }
            }
            false
        }

        fn AsOverloadedFunctionType(&self) -> Option<&AsmOverloadedFunctionType> {
            Some(self)
        }

        fn IsA(&self, other: &AsmType) -> bool {
            if let Some(that) = other.AsOverloadedFunctionType() {
                std::ptr::eq(self, that)
            } else {
                false
            }
        }
    }

    #[derive(Default)]
    pub struct AsmFroundType {}

    impl AsmCallableType for AsmFroundType {
        fn Name(&self) -> String {
            "fround".to_string()
        }

        fn CanBeInvokedWith(&self, _return_type: &AsmType, args: &ZoneVector<AsmType>) -> bool {
            if args.len() != 1 {
                return false;
            }

            if let Some(arg) = args.get(0) {
                if !arg.IsA(&AsmType::Floatish()) && !arg.IsA(&AsmType::DoubleQ()) &&
                    !arg.IsA(&AsmType::Signed()) && !arg.IsA(&AsmType::Unsigned()) {
                    return false;
                }
                return true;
            }

            false
        }

        fn IsA(&self, other: &AsmType) -> bool {
             if let AsmType { kind: AsmTypeEnum::Callable(other_callable) } = other {
                other_callable.as_ref().Name() == "fround"
            } else {
                false
            }
        }
    }

    #[derive(Default)]
    pub struct AsmMinMaxType {
        pub return_type_: Option<Box<AsmType>>,
        pub arg_: Option<Box<AsmType>>,
    }

    impl AsmCallableType for AsmMinMaxType {
        fn Name(&self) -> String {
            if let (Some(arg_), Some(return_type_)) = (&self.arg_, &self.return_type_) {
                format!("({}, ...) -> {}", arg_.Name(), return_type_.Name())
            } else {
                "unknown".to_string()
            }
        }

        fn CanBeInvokedWith(&self, return_type: &AsmType, args: &ZoneVector<AsmType>) -> bool {
            if let Some(return_type_) = &self.return_type_ {
                if !AsmType::IsExactly(return_type_.as_ref(), return_type) {
                    return false;
                }
            } else {
                return false;
            }

            if args.len() < 2 {
                return false;
            }

            if let Some(arg_) = &self.arg_ {
                for ii in 0..args.len() {
                    if let Some(arg) = args.get(ii) {
                        if !arg.IsA(arg_.as_ref()) {
                            return false;
                        }
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }

            true
        }

        fn IsA(&self, other: &AsmType) -> bool {
            if let AsmType { kind: AsmTypeEnum::Callable(other_callable) } = other {
                other_callable.as_ref().Name() == self.Name()
            } else {
                false
            }
        }
    }
}
