// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod codegen;
mod objects;

use crate::codegen::reloc_info::{RelocInfo, RelocIterator};
use crate::objects::instruction_stream::InstructionStream;
use crate::objects::smi::Smi;

/// Represents different root types for the garbage collector.
#[allow(non_camel_case_types)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Root {
    kRootList,
    kBuiltins,
    kEmptyStringTable,
    kAllocationSiteTable,
    kWeakCells,
    kStringTable,
    kNumberStringCache,
    kAccessorInfoCache,
    kScriptList,
    kCodeCache,
    kHeapRoots,
    kExternalReferenceTable,
    kEmbedderData,
    kFeedbackMetadata,
    kFeedbackVectors,
    kFeedbackCells,
    kTypeFeedbackVectors,
    kTypeFeedbackCells,
    kSharedFunctionInfoList,
    kNumberOfRoots, // Sentinel value, not a real root.
}

impl Root {
    /// Returns a string representation of the root's name.
    pub fn root_name(&self) -> &'static str {
        match self {
            Root::kRootList => "kRootList",
            Root::kBuiltins => "kBuiltins",
            Root::kEmptyStringTable => "kEmptyStringTable",
            Root::kAllocationSiteTable => "kAllocationSiteTable",
            Root::kWeakCells => "kWeakCells",
            Root::kStringTable => "kStringTable",
            Root::kNumberStringCache => "kNumberStringCache",
            Root::kAccessorInfoCache => "kAccessorInfoCache",
            Root::kScriptList => "kScriptList",
            Root::kCodeCache => "kCodeCache",
            Root::kHeapRoots => "kHeapRoots",
            Root::kExternalReferenceTable => "kExternalReferenceTable",
            Root::kEmbedderData => "kEmbedderData",
            Root::kFeedbackMetadata => "kFeedbackMetadata",
            Root::kFeedbackVectors => "kFeedbackVectors",
            Root::kFeedbackCells => "kFeedbackCells",
            Root::kTypeFeedbackVectors => "kTypeFeedbackVectors",
            Root::kTypeFeedbackCells => "kTypeFeedbackCells",
            Root::kSharedFunctionInfoList => "kSharedFunctionInfoList",
            Root::kNumberOfRoots => unreachable!(), // Sentinel value
        }
    }
}

/// A trait for visiting objects.
pub trait Visitor {
    fn visit_smi(&mut self, smi: Smi);
    // Add other visit methods for different object types as needed.
}

/// A trait for visiting root objects.
pub trait RootVisitor {
    /// Visits a root object.
    fn visit_root(&mut self, root: Root);
}

/// A struct for visiting objects, implementing the Visitor trait.
pub struct ObjectVisitor {}

impl ObjectVisitor {
    pub fn new() -> Self {
        ObjectVisitor {}
    }

    /// Visits reloc information in an InstructionStream.
    pub fn visit_reloc_info(&mut self, host: &InstructionStream, it: &mut RelocIterator) {
        // RelocInfo iteration is only valid for fully-initialized InstructionStream
        // objects. Callers must ensure this.
        assert!(host.is_fully_initialized());
        while !it.done() {
            let rinfo = it.rinfo();
            rinfo.visit(host, self);
            it.next();
        }
    }
}

impl Visitor for ObjectVisitor {
    fn visit_smi(&mut self, _smi: Smi) {
        // Implementation for visiting a Smi
        // Add your logic here
    }
}