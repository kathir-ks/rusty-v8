// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Indicator for the stack state of the embedder.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum EmbedderStackState {
    /// Stack may contain interesting heap pointers.
    MayContainHeapPointers,
    /// Stack does not contain any interesting heap pointers.
    NoHeapPointers,
}