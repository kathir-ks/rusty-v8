// Converted from V8 C++ source files:
// Header: type-parser.h
// Implementation: type-parser.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
    pub use std::cmp::Ordering;
    use std::str::FromStr;

    #[derive(Debug, Clone, PartialEq)]
    pub enum Type {
        Word32(Word32Type),
        Word64(Word64Type),
        Float32(Float32Type),
        Float64(Float64Type),
        Other, // Add a catch-all variant
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Word32Type {
        range: Option<(u32, u32)>,
        set: Option<Vec<u32>>,
    }

    impl Word32Type {
        pub fn any() -> Type {
            Type::Word32(Word32Type {
                range: None,
                set: None,
            })
        }

        pub fn range(from: u32, to: u32, _zone: &Zone) -> Result<Type, TypeParserError> {
            if from > to {
                return Err(TypeParserError::InvalidRange);
            }
            Ok(Type::Word32(Word32Type {
                range: Some((from, to)),
                set: None,
            }))
        }

        pub fn set(elements: Vec<u32>, _zone: &Zone) -> Result<Type, TypeParserError> {
            if elements.is_empty() {
                return Err(TypeParserError::EmptySet);
            }
            if elements.len() > Self::k_max_set_size() {
                return Err(TypeParserError::SetTooLarge);
            }
            let mut sorted_elements = elements.clone();
            sorted_elements.sort();
            sorted_elements.dedup();
            Ok(Type::Word32(Word32Type {
                range: None,
                set: Some(sorted_elements),
            }))
        }
        const fn k_max_set_size() -> usize {
            32
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Word64Type {
        range: Option<(u64, u64)>,
        set: Option<Vec<u64>>,
    }

    impl Word64Type {
        pub fn any() -> Type {
            Type::Word64(Word64Type {
                range: None,
                set: None,
            })
        }

        pub fn range(from: u64, to: u64, _zone: &Zone) -> Result<Type, TypeParserError> {
             if from > to {
                return Err(TypeParserError::InvalidRange);
            }
            Ok(Type::Word64(Word64Type {
                range: Some((from, to)),
                set: None,
            }))
        }

        pub fn set(elements: Vec<u64>, _zone: &Zone) -> Result<Type, TypeParserError> {
            if elements.is_empty() {
                return Err(TypeParserError::EmptySet);
            }
            if elements.len() > Self::k_max_set_size() {
                return Err(TypeParserError::SetTooLarge);
            }
            let mut sorted_elements = elements.clone();
            sorted_elements.sort();
            sorted_elements.dedup();
            Ok(Type::Word64(Word64Type {
                range: None,
                set: Some(sorted_elements),
            }))
        }

        const fn k_max_set_size() -> usize {
            32
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Float32Type {
        set: Option<Vec<f32>>,
    }

    impl Float32Type {
        pub fn any() -> Type {
            Type::Float32(Float32Type {
                set: None,
            })
        }

        pub fn set(elements: Vec<f32>, _zone: &Zone) -> Result<Type, TypeParserError> {
            if elements.is_empty() {
                return Err(TypeParserError::EmptySet);
            }
            if elements.len() > Self::k_max_set_size() {
                return Err(TypeParserError::SetTooLarge);
            }
            let mut sorted_elements = elements.clone();
            sorted_elements.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            sorted_elements.dedup();
            Ok(Type::Float32(Float32Type {
                set: Some(sorted_elements),
            }))
        }

        const fn k_max_set_size() -> usize {
            32
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct Float64Type {
        set: Option<Vec<f64>>,
    }

    impl Float64Type {
        pub fn any() -> Type {
             Type::Float64(Float64Type {
                set: None,
            })
        }

        pub fn set(elements: Vec<f64>, _zone: &Zone) -> Result<Type, TypeParserError> {
            if elements.is_empty() {
                return Err(TypeParserError::EmptySet);
            }
            if elements.len() > Self::k_max_set_size() {
                return Err(TypeParserError::SetTooLarge);
            }
            let mut sorted_elements = elements.clone();
            sorted_elements.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
            sorted_elements.dedup();
            Ok(Type::Float64(Float64Type {
                set: Some(sorted_elements),
            }))
        }

        const fn k_max_set_size() -> usize {
            32
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum TypeParserError {
        InvalidRange,
        EmptySet,
        SetTooLarge,
        ParseError,
        UnexpectedToken,
        InvalidFloat,
    }

    pub struct TypeParser<'a> {
        str_: &'a str,
        zone_: *mut Zone,
        pos_: usize,
    }

    impl<'a> TypeParser<'a> {
        pub fn new(str_: &'a str, zone_: *mut Zone) -> Self {
            TypeParser {
                str_,
                zone_,
                pos_: 0,
            }
        }

        pub fn parse(&mut self) -> Result<Option<Type>, TypeParserError> {
            let type_ = self.parse_type()?;

            // Skip trailing whitespace.
            while self.pos_ < self.str_.len() && self.str_.chars().nth(self.pos_).unwrap() == ' ' {
                self.pos_ += 1;
            }

            if self.pos_ < self.str_.len() {
                return Ok(None);
            }

            Ok(type_)
        }

        fn parse_type(&mut self) -> Result<Option<Type>, TypeParserError> {
            if self.consume_if("Word32") {
                if self.is_next("{") {
                    return self.parse_set::<Word32Type>();
                }
                if self.is_next("[") {
                    return self.parse_range::<Word32Type>();
                }
                return Ok(Some(Word32Type::any()));
            } else if self.consume_if("Word64") {
                if self.is_next("{") {
                    return self.parse_set::<Word64Type>();
                }
                if self.is_next("[") {
                    return self.parse_range::<Word64Type>();
                }
                return Ok(Some(Word64Type::any()));
            } else if self.consume_if("Float32") {
                if self.is_next("{") {
                    return self.parse_set::<Float32Type>();
                }
                if self.is_next("[") {
                   return Err(TypeParserError::ParseError);
                }
                return Ok(Some(Float32Type::any()));
            } else if self.consume_if("Float64") {
                if self.is_next("{") {
                    return self.parse_set::<Float64Type>();
                }
                if self.is_next("[") {
                   return Err(TypeParserError::ParseError);
                }
                return Ok(Some(Float64Type::any()));
            } else {
                return Ok(None);
            }
        }

        fn parse_range<T>(&mut self) -> Result<Option<Type>, TypeParserError>
        where
            T: 'static,
        {
            if !self.consume_if("[") {
                return Ok(None);
            }

            let from = self.read_value::<T>()?;
            if from.is_none() {
                return Ok(None);
            }

            if !self.consume_if(",") {
                return Ok(None);
            }

            let to = self.read_value::<T>()?;
            if to.is_none() {
                return Ok(None);
            }

            if !self.consume_if("]") {
                return Ok(None);
            }

            match from.unwrap() {
                Value::U32(from_val) => match to.unwrap() {
                    Value::U32(to_val) => {
                        if from_val > to_val {
                            return Err(TypeParserError::InvalidRange);
                        }
                        Word32Type::range(from_val, to_val, unsafe { &*self.zone_ }).map(Some)
                    }
                    _ => Err(TypeParserError::ParseError),
                },
                Value::U64(from_val) => match to.unwrap() {
                    Value::U64(to_val) => {
                        if from_val > to_val {
                            return Err(TypeParserError::InvalidRange);
                        }
                        Word64Type::range(from_val, to_val, unsafe { &*self.zone_ }).map(Some)
                    }
                    _ => Err(TypeParserError::ParseError),
                },
                _ => Err(TypeParserError::ParseError),
            }
        }

        fn parse_set<T>(&mut self) -> Result<Option<Type>, TypeParserError>
        where
            T: 'static,
        {
            if !self.consume_if("{") {
                return Ok(None);
            }

            let elements = self.parse_set_elements::<T>()?;
            if elements.is_none() {
                return Ok(None);
            }

            if !self.consume_if("}") {
                return Ok(None);
            }

            match elements.unwrap() {
                SetElements::U32(elements) => {
                    if elements.is_empty() {
                        return Err(TypeParserError::EmptySet);
                    }

                    if elements.len() > Word32Type::k_max_set_size() {
                        return Err(TypeParserError::SetTooLarge);
                    }

                    Word32Type::set(elements, unsafe { &*self.zone_ }).map(Some)
                }
                SetElements::U64(elements) => {
                    if elements.is_empty() {
                        return Err(TypeParserError::EmptySet);
                    }

                    if elements.len() > Word64Type::k_max_set_size() {
                        return Err(TypeParserError::SetTooLarge);
                    }

                    Word64Type::set(elements, unsafe { &*self.zone_ }).map(Some)
                }
                SetElements::F32(elements) => {
                    if elements.is_empty() {
                        return Err(TypeParserError::EmptySet);
                    }

                    if elements.len() > Float32Type::k_max_set_size() {
                        return Err(TypeParserError::SetTooLarge);
                    }

                    Float32Type::set(elements, unsafe { &*self.zone_ }).map(Some)
                }
                SetElements::F64(elements) => {
                    if elements.is_empty() {
                        return Err(TypeParserError::EmptySet);
                    }

                    if elements.len() > Float64Type::k_max_set_size() {
                        return Err(TypeParserError::SetTooLarge);
                    }

                    Float64Type::set(elements, unsafe { &*self.zone_ }).map(Some)
                }
            }
        }

        fn parse_set_elements<T>(&mut self) -> Result<Option<SetElements>, TypeParserError>
        where
            T: 'static,
        {
            if self.is_next("}") {
                return Ok(Some(SetElements::empty()));
            }

            let mut elements_u32: Vec<u32> = Vec::new();
            let mut elements_u64: Vec<u64> = Vec::new();
            let mut elements_f32: Vec<f32> = Vec::new();
            let mut elements_f64: Vec<f64> = Vec::new();

            loop {
                let element_opt = self.read_value::<T>()?;
                if element_opt.is_none() {
                    return Ok(None);
                }

                match element_opt.unwrap() {
                    Value::U32(val) => elements_u32.push(val),
                    Value::U64(val) => elements_u64.push(val),
                    Value::F32(val) => elements_f32.push(val),
                    Value::F64(val) => elements_f64.push(val),
                }

                if self.is_next("}") {
                    break;
                }

                if !self.consume_if(",") {
                    return Ok(None);
                }
            }

            if !elements_u32.is_empty() {
                elements_u32.sort();
                elements_u32.dedup();
                return Ok(Some(SetElements::U32(elements_u32)));
            } else if !elements_u64.is_empty() {
                elements_u64.sort();
                elements_u64.dedup();
                return Ok(Some(SetElements::U64(elements_u64)));
            } else if !elements_f32.is_empty() {
                elements_f32.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                elements_f32.dedup();
                return Ok(Some(SetElements::F32(elements_f32)));
            } else if !elements_f64.is_empty() {
                elements_f64.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Ordering::Equal));
                elements_f64.dedup();
                return Ok(Some(SetElements::F64(elements_f64)));
            } else {
                return Ok(Some(SetElements::empty()));
            }
        }

        fn consume_if(&mut self, prefix: &str) -> bool {
            if self.is_next(prefix) {
                self.pos_ += prefix.len();
                return true;
            }
            return false;
        }

        fn is_next(&mut self, prefix: &str) -> bool {
            // Skip leading whitespace.
            while self.pos_ < self.str_.len() && self.str_.chars().nth(self.pos_).unwrap() == ' ' {
                self.pos_ += 1;
            }
            if self.pos_ >= self.str_.len() {
                return false;
            }
            let remaining_length = self.str_.len() - self.pos_;
            if prefix.len() > remaining_length {
                return false;
            }
            self.str_[self.pos_..].starts_with(prefix)
        }

        fn read_value<T>(&mut self) -> Result<Option<Value>, TypeParserError>
        where
            T: 'static,
        {
            // Skip leading whitespace
            while self.pos_ < self.str_.len() && self.str_.chars().nth(self.pos_).unwrap() == ' ' {
                self.pos_ += 1;
            }

            let s = &self.str_[self.pos_..];
            if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Word32Type>() {
                match s.split_whitespace().next() {
                    Some(token) => {
                        if let Ok(result) = token.parse::<u32>() {
                            self.pos_ += token.len();
                            return Ok(Some(Value::U32(result)));
                        } else {
                            return Err(TypeParserError::ParseError);
                        }
                    }
                    None => return Err(TypeParserError::ParseError),
                }
            } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Word64Type>() {
                match s.split_whitespace().next() {
                    Some(token) => {
                        if let Ok(result) = token.parse::<u64>() {
                            self.pos_ += token.len();
                            return Ok(Some(Value::U64(result)));
                        } else {
                            return Err(TypeParserError::ParseError);
                        }
                    }
                    None => return Err(TypeParserError::ParseError),
                }
            } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Float32Type>() {
                match s.split_whitespace().next() {
                    Some(token) => {
                        if let Ok(result) = token.parse::<f32>() {
                            self.pos_ += token.len();
                            return Ok(Some(Value::F32(result)));
                        } else {
                            return Err(TypeParserError::InvalidFloat);
                        }
                    }
                    None => return Err(TypeParserError::ParseError),
                }
            } else if std::any::TypeId::of::<T>() == std::any::TypeId::of::<Float64Type>() {
                match s.split_whitespace().next() {
                    Some(token) => {
                        if let Ok(result) = token.parse::<f64>() {
                            self.pos_ += token.len();
                            return Ok(Some(Value::F64(result)));
                        } else {
                            return Err(TypeParserError::InvalidFloat);
                        }
                    }
                    None => return Err(TypeParserError::ParseError),
                }
            }
            Err(TypeParserError::ParseError)
        }
    }

    #[derive(Debug, Clone, PartialEq)]
    enum Value {
        U32(u32),
        U64(u64),
        F32(f32),
        F64(f64),
    }

    #[derive(Debug, Clone, PartialEq)]
    enum SetElements {
        U32(Vec<u32>),
        U64(Vec<u64>),
        F32(Vec<f32>),
        F64(Vec<f64>),
    }

    impl SetElements {
        fn empty() -> Self {
            SetElements::U32(Vec::new()) // Placeholder, since we can't have truly empty enums
        }
    }

    // Dummy Zone struct
    pub struct Zone {}

    impl Zone {
        pub fn new() -> Zone {
            Zone {}
        }
    }
}
