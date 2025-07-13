// Converted from V8 C++ source files:
// Header: bytecode-source-info.h
// Implementation: bytecode-source-info.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interpreter {
    use std::fmt;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct BytecodeSourceInfo {
        position_type_: PositionType,
        source_position_: i32,
    }

    impl BytecodeSourceInfo {
        pub const K_UNINITIALIZED_POSITION: i32 = -1;

        pub fn new() -> Self {
            BytecodeSourceInfo {
                position_type_: PositionType::kNone,
                source_position_: Self::K_UNINITIALIZED_POSITION,
            }
        }

        pub fn with_position(source_position: i32, is_statement: bool) -> Self {
            assert!(source_position >= 0);
            BytecodeSourceInfo {
                position_type_: if is_statement {
                    PositionType::kStatement
                } else {
                    PositionType::kExpression
                },
                source_position_: source_position,
            }
        }

        pub fn make_statement_position(&mut self, source_position: i32) {
            self.position_type_ = PositionType::kStatement;
            self.source_position_ = source_position;
        }

        pub fn make_expression_position(&mut self, source_position: i32) {
            assert!(!self.is_statement());
            self.position_type_ = PositionType::kExpression;
            self.source_position_ = source_position;
        }

        pub fn force_expression_position(&mut self, source_position: i32) {
            self.position_type_ = PositionType::kExpression;
            self.source_position_ = source_position;
        }

        pub fn source_position(&self) -> i32 {
            assert!(self.is_valid());
            self.source_position_
        }

        pub fn is_statement(&self) -> bool {
            self.position_type_ == PositionType::kStatement
        }

        pub fn is_expression(&self) -> bool {
            self.position_type_ == PositionType::kExpression
        }

        pub fn is_valid(&self) -> bool {
            self.position_type_ != PositionType::kNone
        }

        pub fn set_invalid(&mut self) {
            self.position_type_ = PositionType::kNone;
            self.source_position_ = Self::K_UNINITIALIZED_POSITION;
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum PositionType {
        kNone,
        kExpression,
        kStatement,
    }

    impl fmt::Display for BytecodeSourceInfo {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            if self.is_valid() {
                let description = if self.is_statement() { 'S' } else { 'E' };
                write!(f, "{} {}>", self.source_position(), description)
            } else {
                Ok(())
            }
        }
    }
}
