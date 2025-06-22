pub mod uniform_reducer_adapter {
    //use crate::compiler::turboshaft::operations::*;
    //use std::marker::PhantomData;

    // Macro for generating boilerplate code.  This doesn't have an exact
    // Rust equivalent, so we will manually expand the parts that are used.
    // macro_rules! TURBOSHAFT_REDUCER_BOILERPLATE {
    //     () => {
    //         //  Typically includes type aliases, constructors, and other
    //         //  boilerplate needed for a reducer.
    //     }
    // }

    // Macro for generating the operation list.  Since this is just a list
    // of operation names, and the REDUCE macro just generates repetitive code,
    // we can replace it with a simpler list and macro in Rust.
    // macro_rules! TURBOSHAFT_OPERATION_LIST {
    //     ($macro:ident) => {
    //         $macro!(Add);
    //         $macro!(Sub);
    //         // ... other operations
    //     }
    // }

    // Example "Operations" to satisfy template type constraints
    pub struct ConstantOp {}
    pub enum Opcode {
        kAdd,
        kSub,
    }
    pub type OpIndex = usize;

    pub trait Operation {
        const OPCODE: Opcode;
    }

    pub struct AddOp {}
    impl Operation for AddOp {
        const OPCODE: Opcode = Opcode::kAdd;
    }

    pub struct SubOp {}
    impl Operation for SubOp {
        const OPCODE: Opcode = Opcode::kSub;
    }

    // UniformReducerAdapter allows to handle all operations uniformly during a
    // reduction by wiring all ReduceInputGraphXyz and ReduceXyz calls through
    // a single ReduceInputGraphOperation and ReduceOperation, respectively.
    //
    // This is how to use the adapter with your reducer MyReducer, which can then
    // be used in a ReducerStack like any other reducer):
    //
    // template <typename Next>
    // class MyReducer : public UniformReducerAdapter<MyReducer, Next> {
    //  public:
    //   TURBOSHAFT_REDUCER_BOILERPLATE()
    //   using Adapter = UniformReducerAdapter<MyReducer, Next>;
    //
    //   OpIndex ReduceInputGraphConstant(OpIndex ig_index, const ConstantOp& op) {
    //     /* Handle ConstantOps separately */
    //     /* ... */
    //
    //     /* Call Adapter::ReduceInputGraphConstant(index, op) to also run */
    //     /* through the generic handling in ReduceInputGraphOperation */
    //     return Next::ReduceInputGraphConstant(index, op);
    //   }
    //
    //   template <typename Op, typename Continuation>
    //   OpIndex ReduceInputGraphOperation(OpIndex ig_index, const Op& op) {
    //     /* Handle all (other) operations uniformly */
    //     /* ... */
    //
    //     /* Forward to next reducer using the Continuation object */
    //     return Continuation{this}.ReduceInputGraph(ig_index, op);
    //   }
    //
    //   OpIndex ReduceConstant(ConstantOp::Kind kind, ConstantOp::Storage st) {
    //     /* Handle Constants separately */
    //     /* ... */
    //
    //     /* Call Adapter::ReduceConstant(kind, st) to also run through the */
    //     /* generic handling in ReduceOperation */
    //     return Next::ReduceConstant(kind, st);
    //   }
    //
    //   template <Opcode opcode, typename Continuation, typename... Args>
    //   OpIndex ReduceOperation(Args... args) {
    //     /* Handle all (other) operations uniformly */
    //     /* ... */
    //
    //     /* Forward to next reducer using the Continuation object */
    //     return Continuation{this}.Reduce(args...);
    //   }
    //
    //  private:
    //   /* ... */
    // };
    //
    // NOTICE: Inside the ReduceXyz and ReduceInputGraphXyz callbacks of MyReducer,
    // you need to make a choice:
    //
    //   A) Call Next::ReduceXyz (or Next::ReduceInputGraphXyz) to forward to the
    //      next reducer in the stack. Then the uniform ReduceOperation (and
    //      ReduceInputGraphOperation) of the current reducer is not visited for
    //      OperationXyz.
    //   B) Call Adapter::ReduceXyz (or Adapter::ReduceInputGraphXyz) to forward to
    //      the uniform ReduceOperation (and ReduceInputGraphOperation) such that
    //      OperationXyz is also processed by those (in addition to the special
    //      handling in ReduceXyz and ReduceInputGraphXyz).
    //
    // For the above MyReducer, consider this CopyingPhase<R1, MyReducer, R2>.
    // Then the ReduceInputGraph (RIG) and Reduce (R) implementations are visited as
    // follows for Operations OpA and OpB (and all other operations that are not
    // ConstantOp), when all reducers just forward to Next. For ConstantOp, the
    // reduction is equivalent to any "normal" reducer that does not use a
    // UniformReducerAdapter.
    //
    //
    // InputGraph OpA                     OpB     ____________________________
    //             |                       |     |  ___                       |
    //             |                       |     | |   |                      |
    //             v                       v     | |   v                      v
    // R1        RIGOpA                  RIGOpB  | |  ROpA                   ROpB
    //             |     __          __    |     | |   |    ___        ___    |
    //             |    |  |        |  |   |     | |   |   |   |      |   |   |
    //             |    |  v        v  |   |     | |   |   |   v      v   |   |
    // MyReducer   |    | RIGOperation |   |     | |   |   |  ROperation  |   |
    //             v    |      v       |   |     | |   v   |      v       |   v
    // (Adapter) RIGOpA | Continuation | RIGOpB  | |  ROpA | Continuation |  ROpB
    //             |____|  |        |  |___|     | |   |___|  |        |  |___|
    //                     |        |            | |          |        |
    //              _______|        |______      | |    ______|        |______
    //             |                       |     | |   |                      |
    //             |                       |     | |   |                      |
    //             v                       v     | |   v                      v
    // R2        RIGOpA                  RIGOpB  | |  ROpA                   ROpB
    //             |                       |_____| |   |                      |
    //             |_______________________________|   |                      |
    //                                                 v                      v
    // OutputGraph                                    OpA                    OpB
    //
    //
    pub struct UniformReducerAdapter<Reducer, Next> {
        next: Next,
        _reducer: std::marker::PhantomData<Reducer>,
    }

    impl<Reducer, Next> UniformReducerAdapter<Reducer, Next> {
        pub fn new(next: Next) -> Self {
            UniformReducerAdapter {
                next,
                _reducer: std::marker::PhantomData,
            }
        }

        pub fn reduce_operation<Continuation, Args>(
            &self,
            args: Args,
            continuation: Continuation,
        ) -> OpIndex
        where
            Continuation: Fn(Args) -> OpIndex,
        {
            continuation(args)
        }

        pub fn reduce_input_graph_operation<Op, Continuation>(
            &self,
            ig_index: OpIndex,
            operation: &Op,
            continuation: Continuation,
        ) -> OpIndex
        where
            Continuation: Fn(OpIndex, &Op) -> OpIndex,
        {
            continuation(ig_index, operation)
        }
    }

    // This macro would need to be expanded, creating structs and impls for
    // each operation listed in `TURBOSHAFT_OPERATION_LIST`.
    // Since `TURBOSHAFT_OPERATION_LIST` is not defined, we define it here in macro_rules!
    // to avoid the `error: cannot find macro `TURBOSHAFT_OPERATION_LIST` in this scope`

    macro_rules! define_reduce_function {
        ($op:ident) => {
            pub struct ReduceContinuation<'a, Next> {
                this_: &'a Next,
            }

            impl<'a, Next> ReduceContinuation<'a, Next> {
                pub fn new(this_: &'a Next) -> Self {
                    ReduceContinuation { this_ }
                }

                pub fn reduce_input_graph(&self, ig_index: OpIndex, operation: &$op) -> OpIndex {
                    todo!()
                    //self.this_.reduce_input_graph_$op(ig_index, operation)
                }

                // TODO: replace unit tuple with expected types based on opcode
                pub fn reduce(&self, _args: ()) -> OpIndex {
                    todo!()
                    //self.this_.reduce_$op(args)
                }
            }

            impl<Reducer, Next> UniformReducerAdapter<Reducer, Next>
            where
                Next: ReduceInputGraph<Op = $op>,
            {
                pub fn reduce_input_graph_$op(
                    &self,
                    ig_index: OpIndex,
                    operation: &$op,
                ) -> OpIndex {
                    // This cast is not possible in Rust
                    // It would require a way to prove that `Self` implements `Reducer<Next>`

                    self.reduce_input_graph_operation(
                        ig_index,
                        operation,
                        |ig_index, operation| self.next.reduce_input_graph(ig_index, operation),
                    )
                }

                pub fn reduce_$op(&self, _args: ()) -> OpIndex {
                    // This cast is not possible in Rust
                    // It would require a way to prove that `Self` implements `Reducer<Next>`
                    // static_cast<Reducer<Next>*>(this)
                    //     ->template ReduceOperation<Opcode::k##op, Reduce##op##Continuation>(
                    //         args...);
                    todo!()
                }
            }
        };
    }

    pub trait ReduceInputGraph {
        type Op;
        fn reduce_input_graph(&self, ig_index: OpIndex, operation: &Self::Op) -> OpIndex;
    }
    
    define_reduce_function!(AddOp);
    define_reduce_function!(SubOp);
}