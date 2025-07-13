// Converted from V8 C++ source files:
// Header: common.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmbedderStackState {
  /**
   * Stack may contain interesting heap pointers.
   */
  kMayContainHeapPointers,
  /**
   * Stack does not contain any interesting heap pointers.
   */
  kNoHeapPointers,
}
