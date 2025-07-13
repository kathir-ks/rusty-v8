// Converted from V8 C++ source files:
// Header: function-syntax-kind.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod function_syntax_kind {
  #[derive(Debug, Copy, Clone, PartialEq, Eq)]
  pub enum FunctionSyntaxKind {
    kAnonymousExpression,
    kNamedExpression,
    kDeclaration,
    kAccessorOrMethod,
    kWrapped,

    kLastFunctionSyntaxKind = Self::kWrapped,
  }

  impl FunctionSyntaxKind {
    pub fn to_string(&self) -> &'static str {
      match self {
        FunctionSyntaxKind::kAnonymousExpression => "AnonymousExpression",
        FunctionSyntaxKind::kNamedExpression => "NamedExpression",
        FunctionSyntaxKind::kDeclaration => "Declaration",
        FunctionSyntaxKind::kAccessorOrMethod => "AccessorOrMethod",
        FunctionSyntaxKind::kWrapped => "Wrapped",
      }
    }
  }

  impl std::fmt::Display for FunctionSyntaxKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
      write!(f, "{}", self.to_string())
    }
  }
}
