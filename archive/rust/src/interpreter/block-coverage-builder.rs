pub mod interpreter {
    use std::usize;

    pub mod bytecode_array_builder;
    pub mod ast;
    use self::bytecode_array_builder::BytecodeArrayBuilder;
    use self::ast::ast_source_ranges::{AstNodeSourceRanges, SourceRange, SourceRangeKind, SourceRangeMap, NaryOperation, NaryOperationSourceRanges, ConditionalChain, ConditionalChainSourceRanges};

    pub struct BlockCoverageBuilder<'a> {
        slots: Vec<SourceRange>,
        builder: &'a mut BytecodeArrayBuilder,
        source_range_map: &'a mut SourceRangeMap,
    }

    impl<'a> BlockCoverageBuilder<'a> {
        pub const K_NO_COVERAGE_ARRAY_SLOT: i32 = -1;

        pub fn new(builder: &'a mut BytecodeArrayBuilder,
                   source_range_map: &'a mut SourceRangeMap) -> Self {
            BlockCoverageBuilder {
                slots: Vec::new(),
                builder,
                source_range_map,
            }
        }

        pub fn allocate_block_coverage_slot(&mut self, node: &dyn std::any::Any, kind: SourceRangeKind) -> i32 {
           let ranges = self.source_range_map.find(node);

            match ranges {
                None => BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT,
                Some(any) => {
                   if let Some(ranges) = any.downcast_ref::<AstNodeSourceRanges>() {
                        let range = ranges.get_range(kind);
                        if range.is_empty() {
                            BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                        } else {
                            let slot = self.slots.len() as i32;
                            self.slots.push(range);
                            slot
                        }
                   } else {
                       BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                   }
                }
            }
        }

        pub fn allocate_nary_block_coverage_slot(&mut self, node: &NaryOperation, index: usize) -> i32 {
             let ranges = self.source_range_map.find(node);

            match ranges {
                None => BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT,
                Some(any) => {
                   if let Some(ranges) = any.downcast_ref::<NaryOperationSourceRanges>() {
                        let range = ranges.get_range_at_index(index);
                        if range.is_empty() {
                            BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                        } else {
                            let slot = self.slots.len() as i32;
                            self.slots.push(range);
                            slot
                        }
                   } else {
                       BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                   }
                }
            }
        }

        pub fn allocate_conditional_chain_block_coverage_slot(&mut self, node: &ConditionalChain, kind: SourceRangeKind, index: usize) -> i32 {
            let ranges = self.source_range_map.find(node);

            match ranges {
                None => BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT,
                Some(any) => {
                    if let Some(ranges) = any.downcast_ref::<ConditionalChainSourceRanges>() {
                         let range = ranges.get_range_at_index(kind, index);
                         if range.is_empty() {
                             BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                         } else {
                             let slot = self.slots.len() as i32;
                             self.slots.push(range);
                             slot
                         }
                    } else {
                        BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT
                    }
                 }
            }
        }

        pub fn increment_block_counter(&mut self, coverage_array_slot: i32) {
            if coverage_array_slot == BlockCoverageBuilder::K_NO_COVERAGE_ARRAY_SLOT {
                return;
            }
            self.builder.inc_block_counter(coverage_array_slot);
        }

        pub fn increment_block_counter_node(&mut self, node: &dyn std::any::Any, kind: SourceRangeKind) {
            let slot = self.allocate_block_coverage_slot(node, kind);
            self.increment_block_counter(slot);
        }

        pub fn slots(&self) -> &Vec<SourceRange> {
            &self.slots
        }
    }
}