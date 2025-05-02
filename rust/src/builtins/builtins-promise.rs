pub mod promise_builtins {
    // Mimicking the Context structure, assuming it exists elsewhere in the V8 codebase
    // and is not directly translatable without more context.  Using a simple enum
    // as a placeholder. In a real translation, this would need to be a proper
    // representation of the V8 Context.
    pub enum Context {
        MIN_CONTEXT_SLOTS, // Placeholder
    }

    pub struct PromiseBuiltins {}

    impl PromiseBuiltins {
        pub enum PromiseResolvingFunctionContextSlot {
            // The promise which resolve/reject callbacks fulfill.
            kPromiseSlot,

            // Whether the callback was already invoked.
            kAlreadyResolvedSlot,

            // Whether to trigger a debug event or not. Used in catch
            // prediction.
            kDebugEventSlot,
            kPromiseContextLength,
        }

        pub enum PromiseAllResolveElementContextSlots {
            // Remaining elements count
            kPromiseAllResolveElementRemainingSlot,

            // Promise capability from Promise.all
            kPromiseAllResolveElementCapabilitySlot,

            // Values array from Promise.all
            kPromiseAllResolveElementValuesSlot,

            kPromiseAllResolveElementLength
        }

        pub enum PromiseAnyRejectElementContextSlots {
            // Remaining elements count
            kPromiseAnyRejectElementRemainingSlot,

            // Promise capability from Promise.any
            kPromiseAnyRejectElementCapabilitySlot,

            // errors array from Promise.any
            kPromiseAnyRejectElementErrorsSlot,
            kPromiseAnyRejectElementLength
        }

        pub enum FunctionContextSlot {
            kCapabilitySlot,
            kCapabilitiesContextLength,
        }

        // This is used by the Promise.prototype.finally builtin to store
        // onFinally callback and the Promise constructor.
        // TODO(gsathya): For native promises we can create a variant of
        // this without extra space for the constructor to save memory.
        pub enum PromiseFinallyContextSlot {
            kOnFinallySlot,
            kConstructorSlot,

            kPromiseFinallyContextLength,
        }

        // This is used by the ThenFinally and CatchFinally builtins to
        // store the value to return or reason to throw.
        pub enum PromiseValueThunkOrReasonContextSlot {
            kValueSlot,

            kPromiseValueThunkOrReasonContextLength,
        }
    }
}