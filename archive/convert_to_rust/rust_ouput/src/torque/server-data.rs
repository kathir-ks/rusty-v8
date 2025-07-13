// Converted from V8 C++ source files:
// Header: server-data.h
// Implementation: server-data.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

// src/torque/server-data.h
use std::cell::{RefCell, RefMut};
use std::collections::HashMap;
use std::rc::Rc;

use crate::torque::declarable::Declarable;
use crate::torque::global_context::{GlobalContext, LineAndColumn, SourcePosition};
use crate::torque::source_positions::SourceId;
use crate::torque::type_oracle::TypeOracle;

pub type DefinitionMapping = (SourcePosition, SourcePosition);
pub type Definitions = Vec<DefinitionMapping>;
pub type DefinitionsMap = HashMap<SourceId, Definitions>;
pub type Symbols = Vec<*mut dyn Declarable>;
pub type SymbolsMap = HashMap<SourceId, Symbols>;

thread_local! {
    static LANGUAGE_SERVER_DATA: RefCell<LanguageServerData> = RefCell::new(LanguageServerData::new());
}

pub struct LanguageServerData {
    definitions_map: DefinitionsMap,
    symbols_map: SymbolsMap,
    global_context: Option<Box<GlobalContext>>,
    type_oracle: Option<Box<TypeOracle>>,
}

impl LanguageServerData {
    fn new() -> Self {
        LanguageServerData {
            definitions_map: HashMap::new(),
            symbols_map: HashMap::new(),
            global_context: None,
            type_oracle: None,
        }
    }

    pub fn add_definition(token: SourcePosition, definition: SourcePosition) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut mut_data = data.borrow_mut();
            let source = token.source;
            mut_data.definitions_map
                .entry(source)
                .or_insert_with(Vec::new)
                .push((token, definition));
        });
    }

    pub fn find_definition(source: SourceId, pos: LineAndColumn) -> Option<SourcePosition> {
        LANGUAGE_SERVER_DATA.with(|data| {
            let data = data.borrow();
            if !source.is_valid() {
                return None;
            }

            let iter = data.definitions_map.get(&source);
            if iter.is_none() {
                return None;
            }

            for mapping in iter.unwrap() {
                let current = mapping.0;
                if current.contains(pos) {
                    return Some(mapping.1);
                }
            }

            None
        })
    }

    pub fn set_global_context(global_context: GlobalContext) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut mut_data = data.borrow_mut();
            mut_data.global_context = Some(Box::new(global_context));
            mut_data.prepare_all_declarable_symbols();
        });
    }

    pub fn set_type_oracle(type_oracle: TypeOracle) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut mut_data = data.borrow_mut();
            mut_data.type_oracle = Some(Box::new(type_oracle));
        });
    }

    pub fn symbols_for_source_id(id: SourceId) -> Vec<*mut dyn Declarable> {
        LANGUAGE_SERVER_DATA.with(|data| {
            let data = data.borrow();
            data.symbols_map.get(&id).map(|v| v.clone()).unwrap_or_default()
        })
    }

    fn prepare_all_declarable_symbols(&mut self) {
        if let Some(global_context) = &self.global_context {
            let all_declarables: &Vec<Rc<dyn Declarable>> = &global_context.declarables_;

            for declarable in all_declarables {
                if !declarable.is_user_defined() {
                    continue;
                }

                let source = declarable.position().source;
                self.symbols_map
                    .entry(source)
                    .or_insert_with(Vec::new)
                    .push(Rc::into_raw(declarable.clone()) as *mut dyn Declarable);
            }
        }
    }
    
    fn get() -> RefMut<'static, LanguageServerData> {
        LANGUAGE_SERVER_DATA.with(|data| data.borrow_mut())
    }
}

// src/torque/server-data.cc
impl SourcePosition {
    fn contains(&self, pos: LineAndColumn) -> bool {
        // A basic implementation, refine based on actual SourcePosition logic
        self.line == pos.line && self.column == pos.column
    }
}

impl SourceId {
    fn is_valid(&self) -> bool {
        self.id != 0 // Assuming 0 is an invalid SourceId
    }
}

impl Declarable {
    fn position(&self) -> SourcePosition {
        // Placeholder, implement based on actual Declarable logic
        SourcePosition {
            source: SourceId { id: 1 },
            line: 1,
            column: 1,
        }
    }

    fn is_user_defined(&self) -> bool {
        true
    }
}
