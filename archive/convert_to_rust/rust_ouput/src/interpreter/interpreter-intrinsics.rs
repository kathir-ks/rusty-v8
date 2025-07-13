// Converted from V8 C++ source files:
// Header: interpreter-intrinsics.h
// Implementation: interpreter-intrinsics.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod interpreter_intrinsics {
    use crate::runtime::runtime::FunctionId;

    pub const kMaxUInt8: u32 = 255;

    pub struct IntrinsicsHelper {}

    impl IntrinsicsHelper {
        #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
        pub enum IntrinsicId {
            kAsyncFunctionAwait,
            kAsyncFunctionEnter,
            kAsyncFunctionReject,
            kAsyncFunctionResolve,
            kAsyncGeneratorAwait,
            kAsyncGeneratorReject,
            kAsyncGeneratorResolve,
            kAsyncGeneratorYieldWithAwait,
            kCreateJSGeneratorObject,
            kGeneratorGetResumeMode,
            kGeneratorClose,
            kGetImportMetaObject,
            kCopyDataProperties,
            kCopyDataPropertiesWithExcludedPropertiesOnStack,
            kCreateIterResultObject,
            kCreateAsyncFromSyncIterator,
            kIdCount,
        }

        pub fn is_supported(function_id: FunctionId) -> bool {
            match function_id {
                FunctionId::kInlineAsyncFunctionAwait
                | FunctionId::kInlineAsyncFunctionEnter
                | FunctionId::kInlineAsyncFunctionReject
                | FunctionId::kInlineAsyncFunctionResolve
                | FunctionId::kInlineAsyncGeneratorAwait
                | FunctionId::kInlineAsyncGeneratorReject
                | FunctionId::kInlineAsyncGeneratorResolve
                | FunctionId::kInlineAsyncGeneratorYieldWithAwait
                | FunctionId::kInlineCreateJSGeneratorObject
                | FunctionId::kInlineGeneratorGetResumeMode
                | FunctionId::kInlineGeneratorClose
                | FunctionId::kInlineGetImportMetaObject
                | FunctionId::kInlineCopyDataProperties
                | FunctionId::kInlineCopyDataPropertiesWithExcludedPropertiesOnStack
                | FunctionId::kInlineCreateIterResultObject
                | FunctionId::kInlineCreateAsyncFromSyncIterator => true,
                _ => false,
            }
        }

        pub fn from_runtime_id(function_id: FunctionId) -> IntrinsicId {
            match function_id {
                FunctionId::kInlineAsyncFunctionAwait => IntrinsicId::kAsyncFunctionAwait,
                FunctionId::kInlineAsyncFunctionEnter => IntrinsicId::kAsyncFunctionEnter,
                FunctionId::kInlineAsyncFunctionReject => IntrinsicId::kAsyncFunctionReject,
                FunctionId::kInlineAsyncFunctionResolve => IntrinsicId::kAsyncFunctionResolve,
                FunctionId::kInlineAsyncGeneratorAwait => IntrinsicId::kAsyncGeneratorAwait,
                FunctionId::kInlineAsyncGeneratorReject => IntrinsicId::kAsyncGeneratorReject,
                FunctionId::kInlineAsyncGeneratorResolve => IntrinsicId::kAsyncGeneratorResolve,
                FunctionId::kInlineAsyncGeneratorYieldWithAwait => {
                    IntrinsicId::kAsyncGeneratorYieldWithAwait
                }
                FunctionId::kInlineCreateJSGeneratorObject => IntrinsicId::kCreateJSGeneratorObject,
                FunctionId::kInlineGeneratorGetResumeMode => IntrinsicId::kGeneratorGetResumeMode,
                FunctionId::kInlineGeneratorClose => IntrinsicId::kGeneratorClose,
                FunctionId::kInlineGetImportMetaObject => IntrinsicId::kGetImportMetaObject,
                FunctionId::kInlineCopyDataProperties => IntrinsicId::kCopyDataProperties,
                FunctionId::kInlineCopyDataPropertiesWithExcludedPropertiesOnStack => {
                    IntrinsicId::kCopyDataPropertiesWithExcludedPropertiesOnStack
                }
                FunctionId::kInlineCreateIterResultObject => IntrinsicId::kCreateIterResultObject,
                FunctionId::kInlineCreateAsyncFromSyncIterator => {
                    IntrinsicId::kCreateAsyncFromSyncIterator
                }
                _ => panic!("UNREACHABLE"),
            }
        }

        pub fn to_runtime_id(intrinsic_id: IntrinsicId) -> FunctionId {
            match intrinsic_id {
                IntrinsicId::kAsyncFunctionAwait => FunctionId::kInlineAsyncFunctionAwait,
                IntrinsicId::kAsyncFunctionEnter => FunctionId::kInlineAsyncFunctionEnter,
                IntrinsicId::kAsyncFunctionReject => FunctionId::kInlineAsyncFunctionReject,
                IntrinsicId::kAsyncFunctionResolve => FunctionId::kInlineAsyncFunctionResolve,
                IntrinsicId::kAsyncGeneratorAwait => FunctionId::kInlineAsyncGeneratorAwait,
                IntrinsicId::kAsyncGeneratorReject => FunctionId::kInlineAsyncGeneratorReject,
                IntrinsicId::kAsyncGeneratorResolve => FunctionId::kInlineAsyncGeneratorResolve,
                IntrinsicId::kAsyncGeneratorYieldWithAwait => {
                    FunctionId::kInlineAsyncGeneratorYieldWithAwait
                }
                IntrinsicId::kCreateJSGeneratorObject => FunctionId::kInlineCreateJSGeneratorObject,
                IntrinsicId::kGeneratorGetResumeMode => FunctionId::kInlineGeneratorGetResumeMode,
                IntrinsicId::kGeneratorClose => FunctionId::kInlineGeneratorClose,
                IntrinsicId::kGetImportMetaObject => FunctionId::kInlineGetImportMetaObject,
                IntrinsicId::kCopyDataProperties => FunctionId::kInlineCopyDataProperties,
                IntrinsicId::kCopyDataPropertiesWithExcludedPropertiesOnStack => {
                    FunctionId::kInlineCopyDataPropertiesWithExcludedPropertiesOnStack
                }
                IntrinsicId::kCreateIterResultObject => FunctionId::kInlineCreateIterResultObject,
                IntrinsicId::kCreateAsyncFromSyncIterator => {
                    FunctionId::kInlineCreateAsyncFromSyncIterator
                }
                _ => panic!("UNREACHABLE"),
            }
        }
    }
}
