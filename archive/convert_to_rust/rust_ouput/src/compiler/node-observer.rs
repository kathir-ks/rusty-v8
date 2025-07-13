// Converted from V8 C++ source files:
// Header: node-observer.h
// Implementation: node-observer.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod node_observer {
    use std::cell::RefCell;
    use std::collections::HashMap;
    use std::rc::Rc;
    use crate::compiler::node::Node;
    use crate::compiler::string_builder_optimizer::Operator;
    use crate::compiler::add_type_assertions_reducer::Type;
    use crate::compiler::code_assembler::ZoneObject;
    use std::sync::atomic::{AtomicBool, Ordering};

    pub struct ObservableNodeState {
        id_: u32,
        op_: *const Operator,
        opcode_: i32, // Cache the opcode here
        type_: Type,
    }

    impl ObservableNodeState {
        pub fn new(node: &Node) -> Self {
            // Assuming Node has methods to access id(), op(), and type()
            // and op() returns a reference or pointer to Operator
            let op = todo!();
            let opcode = todo!();

            ObservableNodeState {
                id_: 0,
                op_: op,
                opcode_: opcode,
                type_: Type::Any,
            }
        }

        pub fn id(&self) -> u32 {
            self.id_
        }

        pub fn op(&self) -> *const Operator {
            self.op_
        }

        pub fn opcode(&self) -> i16 {
            self.opcode_ as i16
        }

        pub fn type_(&self) -> Type {
            self.type_
        }
    }

    impl PartialEq for ObservableNodeState {
        fn eq(&self, other: &Self) -> bool {
            self.id() == other.id() && self.op() == other.op() && self.type_() == other.type_()
        }
    }

    impl Eq for ObservableNodeState {}

    #[derive(Debug, PartialEq)]
    pub enum Observation {
        kContinue,
        kStop,
    }

    pub trait NodeObserverTrait {
        fn on_node_created(&mut self, node: &Node) -> Observation;
        fn on_node_changed(&mut self, reducer_name: &str, node: &Node, old_state: &ObservableNodeState) -> Observation;
        fn set_has_observed_changes(&mut self);
        fn has_observed_changes(&self) -> bool;
    }
    pub struct DefaultNodeObserver {
        has_observed_changes_: AtomicBool,
    }
    impl DefaultNodeObserver {
        pub fn new() -> Self {
            DefaultNodeObserver {
                has_observed_changes_: AtomicBool::new(false),
            }
        }
    }
    impl NodeObserverTrait for DefaultNodeObserver {
        fn on_node_created(&mut self, _node: &Node) -> Observation {
            Observation::kContinue
        }

        fn on_node_changed(&mut self, _reducer_name: &str, _node: &Node, _old_state: &ObservableNodeState) -> Observation {
            Observation::kContinue
        }

        fn set_has_observed_changes(&mut self) {
            self.has_observed_changes_.store(true, Ordering::Relaxed);
        }

        fn has_observed_changes(&self) -> bool {
            self.has_observed_changes_.load(Ordering::Relaxed)
        }
    }
    pub struct NodeObserverWrapper {
        observer: Box<dyn NodeObserverTrait>,
    }

    impl NodeObserverWrapper {
        pub fn new(observer: Box<dyn NodeObserverTrait>) -> Self {
            NodeObserverWrapper { observer }
        }

        pub fn on_node_created(&mut self, node: &Node) -> Observation {
            self.observer.on_node_created(node)
        }

        pub fn on_node_changed(&mut self, reducer_name: &str, node: &Node, old_state: &ObservableNodeState) -> Observation {
            self.observer.on_node_changed(reducer_name, node, old_state)
        }

        pub fn set_has_observed_changes(&mut self) {
            self.observer.set_has_observed_changes();
        }

        pub fn has_observed_changes(&self) -> bool {
            self.observer.has_observed_changes()
        }
    }

    pub struct NodeObservation {
        pub observer: Rc<RefCell<NodeObserverWrapper>>,
        pub state: ObservableNodeState,
    }

    impl NodeObservation {
        pub fn new(node_observer: Rc<RefCell<NodeObserverWrapper>>, node: &Node) -> Self {
            NodeObservation {
                observer: node_observer,
                state: ObservableNodeState::new(node),
            }
        }
    }

    type NodeId = u32;

    pub struct ObserveNodeManager {
        observations_: RefCell<HashMap<NodeId, Rc<RefCell<NodeObservation>>>>,
    }

    impl ObserveNodeManager {
        pub fn new() -> Self {
            ObserveNodeManager {
                observations_: RefCell::new(HashMap::new()),
            }
        }

        pub fn start_observing(&self, node: &Node, observer: Rc<RefCell<NodeObserverWrapper>>) {
            let mut observer_borrow_mut = observer.borrow_mut();
            observer_borrow_mut.set_has_observed_changes();

            let observation_result = observer_borrow_mut.on_node_created(node);

            if observation_result == Observation::kContinue {
                let node_observation = Rc::new(RefCell::new(NodeObservation::new(observer.clone(), node)));
                self.observations_.borrow_mut().insert(0, node_observation); //TODO: fix hardcoded zero
            } else {
                assert_eq!(observation_result, Observation::kStop);
            }
        }

        pub fn on_node_changed(&self, reducer_name: &str, old_node: &Node, new_node: &Node) {
            let mut observations = self.observations_.borrow_mut();
            let observation_rc = observations.get(&0).map(|rc| rc.clone()); //TODO: fix hardcoded zero

            if let Some(observation_rc) = observation_rc {
                let mut observation = observation_rc.borrow_mut();
                let new_state = ObservableNodeState::new(new_node);
                if observation.state == new_state {
                    return;
                }

                let old_state = observation.state;
                observation.state = new_state;

                let result = observation.observer.borrow_mut().on_node_changed(reducer_name, new_node, &old_state);

                if result == Observation::kStop {
                    observations.remove(&0); //TODO: fix hardcoded zero
                } else {
                    assert_eq!(result, Observation::kContinue);
                    if old_node as *const _ != new_node as *const _ {
                        observations.remove(&0); //TODO: fix hardcoded zero
                        //observations.insert(new_node.id(), observation_rc.clone());
                    }
                }
            }
        }
    }

    #[derive(Clone)]
    pub struct ObserveNodeInfo {
        pub observe_node_manager: Option<Rc<ObserveNodeManager>>,
        pub node_observer: Option<Rc<RefCell<NodeObserverWrapper>>>,
    }

    impl ObserveNodeInfo {
        pub fn new() -> Self {
            ObserveNodeInfo {
                observe_node_manager: None,
                node_observer: None,
            }
        }

        pub fn with_manager_and_observer(manager: Rc<ObserveNodeManager>, observer: Rc<RefCell<NodeObserverWrapper>>) -> Self {
            ObserveNodeInfo {
                observe_node_manager: Some(manager),
                node_observer: Some(observer),
            }
        }

        pub fn start_observing(&self, node: &Node) {
            if let Some(ref manager) = self.observe_node_manager {
                if let Some(ref observer) = self.node_observer {
                    manager.start_observing(node, observer.clone());
                }
            }
        }
    }
}
