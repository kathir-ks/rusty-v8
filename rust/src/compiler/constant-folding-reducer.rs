// Copyright 2018 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

mod compiler {
    use std::rc::Rc;
    use std::cell::RefCell;

    pub use crate::compiler::js_graph::JSGraph;
    pub use crate::compiler::js_heap_broker::JSHeapBroker;
    pub use crate::objects::objects::HeapObjectReference;
    pub use crate::compiler::node::{Node, NodeProperties};
    pub use crate::compiler::operator::{Operator, OperatorProperties};
    pub use crate::compiler::type_system::{Type};
    pub use crate::compiler::ir_opcode::IrOpcode;

    pub struct ConstantFoldingReducer<'a> {
        editor: &'a mut dyn Editor,
        jsgraph: &'a JSGraph,
        broker: &'a JSHeapBroker,
    }

    impl<'a> ConstantFoldingReducer<'a> {
        pub fn new(editor: &'a mut dyn Editor, jsgraph: &'a JSGraph, broker: &'a JSHeapBroker) -> Self {
            ConstantFoldingReducer {
                editor,
                jsgraph,
                broker,
            }
        }

        pub fn reduce(&mut self, node: &mut Node) -> Reduction {
            if !NodeProperties::is_constant(node)
                && NodeProperties::is_typed(node)
                && node.op().has_property(OperatorProperties::Eliminatable)
                && node.opcode() != IrOpcode::FinishRegion
                && node.opcode() != IrOpcode::TypeGuard
            {
                if let Some(constant) = try_get_constant(self.jsgraph, node, self.broker) {
                    if NodeProperties::is_typed(&constant) {
                        assert_eq!(node.op().control_output_count(), 0);
                        self.replace_with_value(node, &constant);
                        return Reduction::Replace(constant);
                    }
                }
            }
            Reduction::NoChange
        }

        fn replace_with_value(&mut self, node: &mut Node, value: &Node) {
            self.editor.replace_node(node, value);
        }
    }

    fn try_get_constant(jsgraph: &JSGraph, node: &Node, broker: &JSHeapBroker) -> Option<Node> {
        let typ = NodeProperties::get_type(node);

        if typ.is_none() {
            None
        } else if typ.is_null() {
            Some(jsgraph.null_constant())
        } else if typ.is_undefined() {
            Some(jsgraph.undefined_constant())
        } else if typ.is_minus_zero() {
            Some(jsgraph.minus_zero_constant())
        } else if typ.is_nan() {
            Some(jsgraph.nan_constant())
        } else if typ.is_heap_constant() {
            let heap_constant = typ.as_heap_constant().unwrap();
            Some(jsgraph.constant_no_hole(heap_constant.ref_(), broker))
        } else if typ.is_plain_number() && typ.min() == typ.max() {
            Some(jsgraph.constant_no_hole(typ.min()))
        } else {
            None
        }
    }

    pub trait AdvancedReducer {
        fn reduce(&mut self, node: &mut Node) -> Reduction;
    }

    pub trait Editor {
        fn replace_node(&mut self, old: &mut Node, new: &Node);
    }

    pub enum Reduction {
        Change,
        NoChange,
        Replace(Node),
    }
}

mod objects {
    pub mod objects {
        #[derive(Debug, Clone)]
        pub struct HeapObjectReference {}
        impl HeapObjectReference {
            pub fn new() -> Self {
                HeapObjectReference {}
            }
        }
    }
}

mod compiler {
    pub mod js_graph {
        use super::node::Node;

        #[derive(Debug)]
        pub struct JSGraph {}

        impl JSGraph {
            pub fn new() -> Self {
                JSGraph {}
            }
            pub fn null_constant(&self) -> Node {
                Node::new() // Placeholder
            }
            pub fn undefined_constant(&self) -> Node {
                Node::new() // Placeholder
            }
            pub fn minus_zero_constant(&self) -> Node {
                Node::new() // Placeholder
            }
             pub fn nan_constant(&self) -> Node {
                Node::new() // Placeholder
            }
            pub fn constant_no_hole<T>(&self, _value: T, _broker: &super::js_heap_broker::JSHeapBroker) -> Node {
                Node::new() // Placeholder
            }
             pub fn constant_no_hole_number(&self, _value: f64) -> Node {
                Node::new() // Placeholder
            }
        }
    }

    pub mod js_heap_broker {
        #[derive(Debug)]
        pub struct JSHeapBroker {}
        impl JSHeapBroker {
            pub fn new() -> Self {
                JSHeapBroker {}
            }
        }
    }

    pub mod node {
        use super::type_system::Type;
        use super::operator::Operator;
        #[derive(Debug, PartialEq)]
        pub struct Node {}

        impl Node {
            pub fn new() -> Self {
                Node {}
            }
            pub fn op(&self) -> &Operator {
                &Operator{} // Placeholder
            }
            pub fn opcode(&self) -> super::ir_opcode::IrOpcode {
                super::ir_opcode::IrOpcode::Nop // Placeholder
            }
        }

        pub struct NodeProperties {}
        impl NodeProperties {
            pub fn is_constant(_node: &Node) -> bool {
                false // Placeholder
            }
            pub fn is_typed(_node: &Node) -> bool {
                false // Placeholder
            }
             pub fn get_type(_node: &Node) -> Type {
                Type::None // Placeholder
            }
        }
    }

    pub mod operator {
        #[derive(Debug)]
        pub struct Operator {}
        impl Operator {
            pub fn has_property(&self, _property: OperatorProperties) -> bool {
                false // Placeholder
            }
            pub fn control_output_count(&self) -> usize {
                0 // Placeholder
            }
        }
        #[derive(Debug)]
        pub enum OperatorProperties {
            Eliminatable
        }
    }

    pub mod type_system {
        use super::objects::objects::HeapObjectReference;

        #[derive(Debug, PartialEq)]
        pub struct Type {
        kind: TypeKind,
        }

        #[derive(Debug, PartialEq)]
        enum TypeKind {
            None,
            Null,
            Undefined,
            MinusZero,
            NaN,
            HeapConstant(HeapObjectReference),
            PlainNumber{min: f64, max: f64},
        }

        impl Type {
            pub const None: Self = Type { kind: TypeKind::None };

            pub fn is_none(&self) -> bool {
                self.kind == TypeKind::None
            }

            pub fn is_null(&self) -> bool {
                self.kind == TypeKind::Null
            }

            pub fn is_undefined(&self) -> bool {
                self.kind == TypeKind::Undefined
            }

            pub fn is_minus_zero(&self) -> bool {
                self.kind == TypeKind::MinusZero
            }

            pub fn is_nan(&self) -> bool {
                self.kind == TypeKind::NaN
            }

            pub fn is_heap_constant(&self) -> bool {
                match self.kind {
                    TypeKind::HeapConstant(_) => true,
                    _ => false,
                }
            }

            pub fn as_heap_constant(&self) -> Option<&HeapConstantType> {
                match &self.kind {
                    TypeKind::HeapConstant(ref r) => Some(&HeapConstantType {reference: r}),
                    _ => None,
                }
            }

            pub fn is_plain_number(&self) -> bool {
                match self.kind {
                    TypeKind::PlainNumber{..} => true,
                    _ => false,
                }
            }

            pub fn min(&self) -> f64 {
                match self.kind {
                     TypeKind::PlainNumber{min, ..} => min,
                    _ => 0.0, // Placeholder
                }
            }

             pub fn max(&self) -> f64 {
                match self.kind {
                    TypeKind::PlainNumber{max, ..} => max,
                    _ => 0.0, // Placeholder
                }
            }

            pub fn is_singleton(&self) -> bool {
                self.is_null() || self.is_undefined() || self.is_minus_zero() || self.is_nan() || self.is_heap_constant() || (self.is_plain_number() && self.min() == self.max())
            }
            pub fn equals(&self, _other: &Type) -> bool {
                false // Placeholder
            }
        }

        #[derive(Debug, PartialEq)]
        pub struct HeapConstantType<'a> {
            reference: &'a HeapObjectReference,
        }

        impl<'a> HeapConstantType<'a> {
            pub fn ref_(&self) -> &HeapObjectReference {
                self.reference
            }
        }
    }

    pub mod ir_opcode {
        #[derive(Debug, PartialEq, Clone, Copy)]
        pub enum IrOpcode {
            Nop,
            FinishRegion,
            TypeGuard,
        }
    }
}