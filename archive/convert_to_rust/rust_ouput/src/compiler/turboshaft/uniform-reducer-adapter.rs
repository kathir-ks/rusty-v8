// Converted from V8 C++ source files:
// Header: uniform-reducer-adapter.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod turboshaft {
  pub use crate::compiler::turboshaft::operations::*;

  pub trait UniformReducerAdapterTrait<Next> {
    fn reduce_operation<Continuation, Args>(
      &mut self,
      args: Args,
    ) -> Result<OpIndex, String>;

    fn reduce_input_graph_operation<Op, Continuation>(
      &mut self,
      ig_index: OpIndex,
      operation: &Op,
    ) -> Result<OpIndex, String>;
  }

  pub struct ReduceContinuation<'a, Next> {
    this_: &'a mut Next,
  }

  impl<'a, Next> ReduceContinuation<'a, Next> {
    pub fn new(this_: &'a mut Next) -> Self {
      Self { this_ }
    }
  }

  #[macro_export]
  macro_rules! define_reduce_continuation {
    ($op:ident, $next_type:ty) => {
      pub struct Reduce##$op##Continuation<'a, Next> {
        this_: &'a mut Next,
      }

      impl<'a, Next> Reduce##$op##Continuation<'a, Next> {
        pub fn new(this_: &'a mut Next) -> Self {
          Self { this_ }
        }

        pub fn reduce_input_graph(
          &mut self,
          ig_index: OpIndex,
          operation: &op##Op,
        ) -> Result<OpIndex, String> {
          self
            .this_
            .reduce_input_graph_##$op(ig_index, operation)
        }

        pub fn reduce<Args>(
          &mut self,
          args: Args,
        ) -> Result<OpIndex, String> {
          self.this_.reduce_##$op(args)
        }
      }
    };
  }

  #[macro_export]
  macro_rules! implement_reduce_methods {
    ($reducer_type:ty, $next_type:ty) => {
      impl $reducer_type {
        fn reduce_input_graph_operation<Op, Continuation>(
          &mut self,
          ig_index: OpIndex,
          operation: &Op,
        ) -> Result<OpIndex, String> {
          let continuation = Continuation::new(self);
          (self as &mut dyn UniformReducerAdapterTrait<$next_type>)
            .reduce_input_graph_operation::<Op, Continuation>(
              ig_index, operation,
            )
        }

        fn reduce_operation<Continuation, Args>(
          &mut self,
          args: Args,
        ) -> Result<OpIndex, String> {
          let continuation = Continuation::new(self);
          (self as &mut dyn UniformReducerAdapterTrait<$next_type>)
            .reduce_operation::<Continuation, Args>(args)
        }
      }
    };
  }

  #[macro_export]
  macro_rules! turboshaft_operation_list {
    ($macro:ident) => {
      $macro!(Phi);
      $macro!(Constant);
      $macro!(Load);
      $macro!(Store);
    };
  }

  #[macro_export]
  macro_rules! define_reduce_functions {
    ($op:ident) => {
      fn reduce_input_graph_##$op(
        &mut self,
        ig_index: OpIndex,
        operation: &op##Op,
      ) -> Result<OpIndex, String>;

      fn reduce_##$op<Args>(
        &mut self,
        args: Args,
      ) -> Result<OpIndex, String>;
    };
  }

  pub mod internal {
    pub mod compiler {
      pub mod turboshaft {
        use super::super::super::super::turboshaft::operations::*;
        use crate::define_reduce_continuation;
        use crate::define_reduce_functions;
        use crate::implement_reduce_methods;
        use crate::turboshaft_operation_list;

        pub struct UniformReducerAdapter {}

        impl UniformReducerAdapter {
          pub fn new() -> Self {
            Self {}
          }
        }

        impl Default for UniformReducerAdapter {
          fn default() -> Self {
            Self::new()
          }
        }

        pub struct ReducerStack<R1, R2, R3> {
          r1: R1,
          reducer: R2,
          r3: R3,
        }

        impl<R1, R2, R3> ReducerStack<R1, R2, R3> {
          pub fn new(r1: R1, reducer: R2, r3: R3) -> Self {
            Self { r1, reducer, r3 }
          }
        }

        pub trait NextReducer {
          fn reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String>;
          fn reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String>;
          fn reduce_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String>;
          fn reduce_input_graph_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String>;
        }

        pub trait MyReducerTrait: NextReducer {
          fn my_reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String>;

          fn my_reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String>;

          fn my_reduce_input_graph_operation<Op>(
            &mut self,
            ig_index: OpIndex,
            op: &Op,
          ) -> Result<OpIndex, String>;
          fn my_reduce_operation<Args>(
            &mut self,
            args: Args,
          ) -> Result<OpIndex, String>;
        }

        impl<T: NextReducer> MyReducerTrait for T {
          fn my_reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String> {
            self.reduce_constant(kind, storage)
          }

          fn my_reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String> {
            self.reduce_input_graph_constant(ig_index, op)
          }

          fn my_reduce_input_graph_operation<Op>(
            &mut self,
            ig_index: OpIndex,
            op: &Op,
          ) -> Result<OpIndex, String> {
            Err("Default my_reduce_input_graph_operation implementation".into())
          }

          fn my_reduce_operation<Args>(
            &mut self,
            args: Args,
          ) -> Result<OpIndex, String> {
            Err("Default my_reduce_operation implementation".into())
          }
        }

        pub struct MyReducer<Next> {
          next: Next,
        }

        impl<Next> MyReducer<Next> {
          pub fn new(next: Next) -> Self {
            Self { next }
          }
        }

        impl<Next> NextReducer for MyReducer<Next>
        where
          Next: NextReducer,
        {
          fn reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_constant");
            self.next.reduce_constant(kind, storage)
          }
          fn reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_input_graph_constant");
            self.next.reduce_input_graph_constant(ig_index, op)
          }

          fn reduce_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_phi");
            self.next.reduce_phi(ig_index, op)
          }

          fn reduce_input_graph_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_input_graph_phi");
            self.next.reduce_input_graph_phi(ig_index, op)
          }
        }

        impl<Next> UniformReducerAdapterTrait<Next> for MyReducer<Next>
        where
          Next: NextReducer,
        {
          fn reduce_operation<Continuation, Args>(
            &mut self,
            _args: Args,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_operation");
            Err("MyReducer::reduce_operation not implemented".into())
          }

          fn reduce_input_graph_operation<Op, Continuation>(
            &mut self,
            _ig_index: OpIndex,
            _operation: &Op,
          ) -> Result<OpIndex, String> {
            println!("MyReducer::reduce_input_graph_operation");
            Err("MyReducer::reduce_input_graph_operation not implemented".into())
          }
        }

        impl<Reducer, Next> UniformReducerAdapterTrait<Next> for Reducer
        where
          Reducer: TurboshaftReducer + MyReducerTrait,
          Next: NextReducer,
        {
          fn reduce_operation<Continuation, Args>(
            &mut self,
            args: Args,
          ) -> Result<OpIndex, String> {
            (self as &mut dyn MyReducerTrait).my_reduce_operation(args)
          }

          fn reduce_input_graph_operation<Op, Continuation>(
            &mut self,
            ig_index: OpIndex,
            operation: &Op,
          ) -> Result<OpIndex, String> {
            (self as &mut dyn MyReducerTrait)
              .my_reduce_input_graph_operation(ig_index, operation)
          }
        }

        pub trait TurboshaftReducer: NextReducer {
          define_reduce_functions!(Phi);
          define_reduce_functions!(Constant);
          define_reduce_functions!(Load);
          define_reduce_functions!(Store);
        }

        pub struct MyTurboshaftReducer<Next> {
          next: Next,
        }

        impl<Next> MyTurboshaftReducer<Next> {
          pub fn new(next: Next) -> Self {
            Self { next }
          }
        }

        impl<Next> NextReducer for MyTurboshaftReducer<Next>
        where
          Next: NextReducer,
        {
          fn reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::reduce_constant");
            self.next.reduce_constant(kind, storage)
          }
          fn reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::reduce_input_graph_constant");
            self.next.reduce_input_graph_constant(ig_index, op)
          }

          fn reduce_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::reduce_phi");
            self.next.reduce_phi(ig_index, op)
          }

          fn reduce_input_graph_phi(
            &mut self,
            ig_index: OpIndex,
            op: &PhiOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::reduce_input_graph_phi");
            self.next.reduce_input_graph_phi(ig_index, op)
          }
        }

        impl<Next> MyReducerTrait for MyTurboshaftReducer<Next>
        where
          Next: NextReducer,
        {
          fn my_reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::my_reduce_constant");
            self.next.reduce_constant(kind, storage)
          }

          fn my_reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            op: &ConstantOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::my_reduce_input_graph_constant");
            self.next.reduce_input_graph_constant(ig_index, op)
          }
          fn my_reduce_input_graph_operation<Op>(
            &mut self,
            ig_index: OpIndex,
            op: &Op,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::my_reduce_input_graph_operation");
            Err("Default my_reduce_input_graph_operation implementation".into())
          }

          fn my_reduce_operation<Args>(
            &mut self,
            args: Args,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::my_reduce_operation");
            Err("Default my_reduce_operation implementation".into())
          }
        }

        impl<Next> TurboshaftReducer for MyTurboshaftReducer<Next>
        where
          Next: NextReducer,
        {
          fn reduce_input_graph_phi(
            &mut self,
            ig_index: OpIndex,
            operation: &PhiOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceInputGraphPhi");
            self.next.reduce_input_graph_phi(ig_index, operation)
          }

          fn reduce_phi(
            &mut self,
            args: OpIndex,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReducePhi");
            self.next.reduce_phi(0.into(), &PhiOp::default())
          }

          fn reduce_input_graph_constant(
            &mut self,
            ig_index: OpIndex,
            operation: &ConstantOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceInputGraphConstant");
            self.next.reduce_input_graph_constant(ig_index, operation)
          }

          fn reduce_constant(
            &mut self,
            kind: ConstantOpKind,
            storage: ConstantOpStorage,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceConstant");
            self.next.reduce_constant(kind, storage)
          }
          fn reduce_input_graph_load(
            &mut self,
            ig_index: OpIndex,
            operation: &LoadOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceInputGraphLoad");
            Err("MyTurboshaftReducer::ReduceInputGraphLoad not implemented".into())
          }

          fn reduce_load(
            &mut self,
            args: OpIndex,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceLoad");
            Err("MyTurboshaftReducer::ReduceLoad not implemented".into())
          }
          fn reduce_input_graph_store(
            &mut self,
            ig_index: OpIndex,
            operation: &StoreOp,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceInputGraphStore");
            Err("MyTurboshaftReducer::ReduceInputGraphStore not implemented".into())
          }

          fn reduce_store(
            &mut self,
            args: OpIndex,
          ) -> Result<OpIndex, String> {
            println!("MyTurboshaftReducer::ReduceStore");
            Err("MyTurboshaftReducer::ReduceStore not implemented".into())
          }
        }

        define_reduce_continuation!(Phi, Next);
        define_reduce_continuation!(Constant, Next);
        define_reduce_continuation!(Load, Next);
        define_reduce_continuation!(Store, Next);
      }
    }
  }
}
