// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Defines the `FunctionSyntaxKind` enum and its related functions.
pub mod function_syntax_kind {
    use std::fmt;

    /// Represents the different syntax kinds for functions.
    #[derive(Debug, Copy, Clone, PartialEq, Eq)]
    #[repr(u8)]
    pub enum FunctionSyntaxKind {
        AnonymousExpression,
        NamedExpression,
        Declaration,
        AccessorOrMethod,
        Wrapped,
    }

    impl FunctionSyntaxKind {
        /// Converts a `FunctionSyntaxKind` to a string representation.
        pub fn to_string(&self) -> &'static str {
            match self {
                FunctionSyntaxKind::AnonymousExpression => "AnonymousExpression",
                FunctionSyntaxKind::NamedExpression => "NamedExpression",
                FunctionSyntaxKind::Declaration => "Declaration",
                FunctionSyntaxKind::AccessorOrMethod => "AccessorOrMethod",
                FunctionSyntaxKind::Wrapped => "Wrapped",
            }
        }
    }

    impl fmt::Display for FunctionSyntaxKind {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.to_string())
        }
    }
}