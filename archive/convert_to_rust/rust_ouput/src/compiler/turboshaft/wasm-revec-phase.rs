// Converted from V8 C++ source files:
// Header: wasm-revec-phase.h
// Implementation: wasm-revec-phase.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(dead_code)]
#![allow(non_snake_case)]
use std::cell::RefCell;
use std::rc::Rc;
use crate::wasm_dead_code_elimination_phase::UnparkedScopeIfNeeded;
use crate::csa_optimize_phase::V8;
use crate::execution::isolate::Isolate;
use crate::compiler::turboshaft::phase::PipelineData;
use crate::compiler::turboshaft::wasm_revec_reducer::WasmRevecReducer;
use crate::compiler::turboshaft::phase::WasmRevecAnalyzer;
use crate::compiler::turboshaft::copying_phase::CopyingPhase;

pub struct Graph {}

pub struct WasmRevecVerifier {
  handler_: Option<Box<dyn Fn(&Graph)>>,
}

impl WasmRevecVerifier {
  pub fn new(handler: Option<Box<dyn Fn(&Graph)>>) -> Self {
    WasmRevecVerifier { handler_: handler }
  }

  pub fn verify(&self, graph: &Graph) {
    if let Some(handler) = &self.handler_ {
      handler(graph);
    }
  }
}

pub struct WasmRevecPhase {}

impl WasmRevecPhase {
  pub const NAME: &'static str = "WasmRevec";

  pub fn run(data: &mut PipelineData, temp_zone: &mut Zone) {
    let mut analyzer = WasmRevecAnalyzer::new(data, temp_zone, &data.graph);

    if analyzer.should_reduce() {
      data.set_wasm_revec_analyzer(Some(analyzer));
      let broker = data.broker.clone();
      let mut scope = UnparkedScopeIfNeeded::new(broker, v8_flags::turboshaft_trace_reduction);
      CopyingPhase::<WasmRevecReducer>::run(data, temp_zone);

      let isolate = Isolate::try_get_current();
      let revec_observer_for_test = isolate.map(|i| i.wasm_revec_verifier_for_test());

      if let Some(observer) = revec_observer_for_test {
        observer.borrow().verify(&data.graph);
      }
      data.clear_wasm_revec_analyzer();
    }
  }
}

mod v8_flags {
  pub static turboshaft_trace_reduction: bool = false;
}

pub struct JSHeapBroker {}

impl JSHeapBroker {
    pub fn new() -> Self {
        JSHeapBroker{}
    }
}

pub struct Zone {}
impl Zone {
    pub fn new() -> Self {
        Zone{}
    }
}

impl PipelineData {
    pub fn new() -> Self {
        PipelineData{
            broker: Rc::new(RefCell::new(JSHeapBroker::new())),
            graph: Graph{},
            wasm_revec_analyzer: None,
        }
    }

    pub fn set_wasm_revec_analyzer(&mut self, analyzer: Option<WasmRevecAnalyzer>) {
        self.wasm_revec_analyzer = analyzer;
    }

    pub fn clear_wasm_revec_analyzer(&mut self) {
        self.wasm_revec_analyzer = None;
    }

    pub fn graph(&self) -> &Graph {
        &self.graph
    }
}

impl Isolate {
    pub fn try_get_current() -> Option<Isolate> {
        Some(Isolate { revec_verifier: Rc::new(RefCell::new(WasmRevecVerifier::new(None))) })
    }

    pub fn wasm_revec_verifier_for_test(&self) -> Rc<RefCell<WasmRevecVerifier>> {
        self.revec_verifier.clone()
    }
}
