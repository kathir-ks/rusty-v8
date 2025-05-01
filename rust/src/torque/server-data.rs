// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::option::Option;
use std::vec::Vec;

// These are dummy implementations of the C++ classes.  They need to be properly defined
// and implemented to match the C++ semantics.  In the absence of the actual classes'
// definitions, the Rust code can only provide type placeholders.

/// Placeholder for SourcePosition
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourcePosition {}

/// Placeholder for SourceId
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct SourceId {}

/// Placeholder for LineAndColumn
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct LineAndColumn {}

/// Placeholder for Declarable
pub struct Declarable {}

/// Placeholder for GlobalContext
pub struct GlobalContext {}

/// Placeholder for TypeOracle
pub struct TypeOracle {}

/// Represents a mapping between a token's definition and its location.
pub type DefinitionMapping = (SourcePosition, SourcePosition);

/// A collection of definitions.
pub type Definitions = Vec<DefinitionMapping>;

/// A map of definitions, keyed by SourceId.
pub type DefinitionsMap = HashMap<SourceId, Definitions>;

/// A collection of symbols (declarables).
pub type Symbols = Vec<*mut Declarable>;

/// A map of symbols, keyed by SourceId.
pub type SymbolsMap = HashMap<SourceId, Symbols>;

thread_local! {
    static LANGUAGE_SERVER_DATA: std::cell::RefCell<LanguageServerData> = std::cell::RefCell::new(LanguageServerData::new());
}

/// Holds data for language server requests.
pub struct LanguageServerData {
    definitions_map: DefinitionsMap,
    symbols_map: SymbolsMap,
    global_context: Option<Box<GlobalContext>>,
    type_oracle: Option<Box<TypeOracle>>,
}

impl LanguageServerData {
    /// Creates a new `LanguageServerData` instance.
    pub fn new() -> Self {
        LanguageServerData {
            definitions_map: HashMap::new(),
            symbols_map: HashMap::new(),
            global_context: None,
            type_oracle: None,
        }
    }

    /// Adds a definition to the definitions map.
    pub fn add_definition(token: SourcePosition, definition: SourcePosition) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut data = data.borrow_mut();
            let source_id = todo!(); // Get source_id from token.  Requires implementation on SourcePosition
            data.definitions_map.entry(source_id).or_insert(Vec::new()).push((token, definition));
        });
    }

    /// Finds the definition for a given source and position.
    pub fn find_definition(
        source: SourceId,
        pos: LineAndColumn,
    ) -> Option<SourcePosition> {
        LANGUAGE_SERVER_DATA.with(|data| {
            let data = data.borrow();
            let definitions = data.definitions_map.get(&source)?;
            for (token_pos, definition_pos) in definitions {
              // Requires implementation on SourcePosition and LineAndColumn to compare positions
                todo!();
            }
            None
        })
    }

    /// Sets the global context.
    pub fn set_global_context(global_context: GlobalContext) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.global_context = Some(Box::new(global_context));
            data.prepare_all_declarable_symbols();
        });
    }

    /// Sets the type oracle.
    pub fn set_type_oracle(type_oracle: TypeOracle) {
        LANGUAGE_SERVER_DATA.with(|data| {
            let mut data = data.borrow_mut();
            data.type_oracle = Some(Box::new(type_oracle));
        });
    }

    /// Returns the symbols for a given SourceId.
    pub fn symbols_for_source_id(id: SourceId) -> Symbols {
        LANGUAGE_SERVER_DATA.with(|data| {
            let data = data.borrow();
            match data.symbols_map.get(&id) {
                Some(symbols) => symbols.clone(),
                None => Vec::new(),
            }
        })
    }

    /// Splits declarables by SourceId and filters auto-generated ones.
    fn prepare_all_declarable_symbols(&mut self) {
        // Requires access to the list of declarables in GlobalContext.  This is a placeholder.
        // Also, need to determine how to filter out auto-generated ones.
        todo!();
    }
}