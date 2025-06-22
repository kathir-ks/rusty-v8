// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Write barrier kinds supported by compiler.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum WriteBarrierKind {
  NoWriteBarrier,
  AssertNoWriteBarrier,
  MapWriteBarrier,
  PointerWriteBarrier,
  IndirectPointerWriteBarrier,
  EphemeronKeyWriteBarrier,
  FullWriteBarrier,
}

impl WriteBarrierKind {
  /// Converts the `WriteBarrierKind` to its underlying `u8` representation.
  pub fn to_u8(self) -> u8 {
    self as u8
  }
}

impl From<WriteBarrierKind> for usize {
  fn from(kind: WriteBarrierKind) -> Self {
    kind as u8 as usize
  }
}

impl std::fmt::Display for WriteBarrierKind {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      WriteBarrierKind::NoWriteBarrier => write!(f, "NoWriteBarrier"),
      WriteBarrierKind::AssertNoWriteBarrier => write!(f, "AssertNoWriteBarrier"),
      WriteBarrierKind::MapWriteBarrier => write!(f, "MapWriteBarrier"),
      WriteBarrierKind::PointerWriteBarrier => write!(f, "PointerWriteBarrier"),
      WriteBarrierKind::IndirectPointerWriteBarrier => write!(f, "IndirectPointerWriteBarrier"),
      WriteBarrierKind::EphemeronKeyWriteBarrier => write!(f, "EphemeronKeyWriteBarrier"),
      WriteBarrierKind::FullWriteBarrier => write!(f, "FullWriteBarrier"),
    }
  }
}