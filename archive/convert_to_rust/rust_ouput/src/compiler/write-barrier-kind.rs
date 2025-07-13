// Converted from V8 C++ source files:
// Header: write-barrier-kind.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

#[allow(dead_code)]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum WriteBarrierKind {
  kNoWriteBarrier,
  kAssertNoWriteBarrier,
  kMapWriteBarrier,
  kPointerWriteBarrier,
  kIndirectPointerWriteBarrier,
  kEphemeronKeyWriteBarrier,
  kFullWriteBarrier,
}

impl WriteBarrierKind {
  
}

impl std::hash::Hash for WriteBarrierKind {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        (*self as u8).hash(state);
    }
}

impl std::fmt::Display for WriteBarrierKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WriteBarrierKind::kNoWriteBarrier => write!(f, "NoWriteBarrier"),
      WriteBarrierKind::kAssertNoWriteBarrier => write!(f, "AssertNoWriteBarrier"),
      WriteBarrierKind::kMapWriteBarrier => write!(f, "MapWriteBarrier"),
      WriteBarrierKind::kPointerWriteBarrier => write!(f, "PointerWriteBarrier"),
      WriteBarrierKind::kIndirectPointerWriteBarrier => write!(f, "IndirectPointerWriteBarrier"),
      WriteBarrierKind::kEphemeronKeyWriteBarrier => write!(f, "EphemeronKeyWriteBarrier"),
      WriteBarrierKind::kFullWriteBarrier => write!(f, "FullWriteBarrier"),
    }
  }
}
