// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// src/debug/debug-coverage.h (implied module definition)

mod debug_coverage {
    use std::cmp::{max, Ordering};
    use std::collections::HashMap;
    use std::sync::atomic::{AtomicU32, Ordering::Relaxed};

    // Placeholder types; replace with actual V8 types
    type Tagged<T> = *mut T; // Example: raw pointer.  Consider Box, Arc, etc.
    type Object = u8; // Example
    type SharedFunctionInfo = Object; // Example
    type CoverageInfo = Object; // Example
    type Script = Object; // Example
    type FeedbackVector = Object; // Example
    type JSFunction = Object; // Example
    type HeapObject = Object; // Example
    type String = Object; // Example
    type ArrayList = Object; // Example
    type Handle<T> = *mut T;
    // Other types, constants, and macros should be defined similarly

    const kNoSourcePosition: i32 = -1;
    const UINT32_MAX: u32 = u32::MAX;
    const SourceRange_kFunctionLiteralSourcePosition: i32 = 0; //Example
    const kRelaxedStore: Relaxed = Relaxed; //Example

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct CoverageBlock {
        pub start: i32,
        pub end: i32,
        pub count: u32,
    }

    impl CoverageBlock {
        pub fn new(start: i32, end: i32, count: u32) -> Self {
            CoverageBlock { start, end, count }
        }
    }

    #[derive(Debug)]
    pub struct CoverageFunction {
        pub start: i32,
        pub end: i32,
        pub count: u32,
        pub name: Tagged<String>, // Assuming String is a V8 string type
        pub blocks: Vec<CoverageBlock>,
        pub has_block_coverage: bool,
    }

    impl CoverageFunction {
        pub fn new(start: i32, end: i32, count: u32, name: Tagged<String>) -> Self {
            CoverageFunction {
                start,
                end,
                count,
                name,
                blocks: Vec::new(),
                has_block_coverage: false,
            }
        }

        pub fn HasNonEmptySourceRange(&self) -> bool {
            self.start != kNoSourcePosition && self.end != kNoSourcePosition
        }

        pub fn HasBlocks(&self) -> bool {
            !self.blocks.is_empty()
        }
    }

    pub struct Coverage {
        script_data: Vec<ScriptData>,
    }

    impl Coverage {
        pub fn new() -> Self {
            Coverage {
                script_data: Vec::new(),
            }
        }

        pub fn CollectPrecise(isolate: *mut Isolate) -> std::unique_ptr<Coverage> {
            assert!(!unsafe { (*isolate).is_best_effort_code_coverage() });
            let mut result = Coverage::Collect(isolate, unsafe { (*isolate).code_coverage_mode() });
            if unsafe { (*isolate).is_precise_binary_code_coverage() } || unsafe { (*isolate).is_block_binary_code_coverage() } {
                // We do not have to hold onto feedback vectors for invocations we already
                // reported. So we can reset the list.
                unsafe { (*isolate).SetFeedbackVectorsForProfilingTools((*(*isolate).factory()).empty_array_list()) };
            }
            result
        }

        pub fn CollectBestEffort(isolate: *mut Isolate) -> std::unique_ptr<Coverage> {
            Coverage::Collect(isolate, debug::CoverageMode::kBestEffort)
        }

        pub fn Collect(isolate: *mut Isolate, collectionMode: debug::CoverageMode) -> std::unique_ptr<Coverage> {
            // Unsupported if jitless mode is enabled at build-time since related
            // optimizations deactivate invocation count updates.
            //CHECK(!V8_JITLESS_BOOL);

            // Collect call counts for all functions.
            let mut counter_map = SharedToCounterMap::new();
            collect_and_maybe_reset_counts(isolate, &mut counter_map, collectionMode);

            // Iterate shared function infos of every script and build a mapping
            // between source ranges and invocation counts.
            let mut result = std::unique_ptr::new(Coverage::new());

            let mut scripts: Vec<Handle<Script>> = Vec::new();
            let mut script_it = ScriptIterator::new(isolate);
            while let Some(script) = script_it.Next() {
                if script.IsUserJavaScript() {
                    scripts.push(script);
                }
            }

            for script in scripts.iter() {
                // Create and add new script data.
                result.push_back(ScriptData::new(script));
                let functions = &mut result.back().functions;

                let mut sorted: Vec<SharedFunctionInfoAndCount> = Vec::new();

                {
                    // Sort functions by start position, from outer to inner functions.
                    let mut infos = SharedFunctionInfo::ScriptIterator::new(isolate, *script);
                    while let Some(info) = infos.Next() {
                        sorted.push(SharedFunctionInfoAndCount::new(info, counter_map.Get(info)));
                    }
                    sorted.sort();
                }

                // Stack to track nested functions, referring function by index.
                let mut nesting: Vec<usize> = Vec::new();

                // Use sorted list to reconstruct function nesting.
                for v in sorted.iter() {
                    let info = v.info;
                    let start = v.start;
                    let end = v.end;
                    let count = v.count;

                    // Find the correct outer function based on start position.
                    //
                    // This is, in general, not robust when considering two functions with
                    // identical source ranges; then the notion of inner and outer is unclear.
                    // Identical source ranges arise when the source range of top-most entity
                    // (e.g. function) in the script is identical to the whole script, e.g.
                    // <script>function foo() {}<script>. The script has its own shared
                    // function info, which has the same source range as the SFI for `foo`.
                    // Node.js creates an additional wrapper for scripts (again with identical
                    // source range) and those wrappers will have a call count of zero even if
                    // the wrapped script was executed (see v8:9212). We mitigate this issue
                    // by sorting top-level SFIs first among SFIs with the same source range:
                    // This ensures top-level SFIs are processed first. If a top-level SFI has
                    // a non-zero call count, it gets recorded due to `function_is_relevant`
                    // below (e.g. script wrappers), while top-level SFIs with zero call count
                    // do not get reported (this ensures node's extra wrappers do not get
                    // reported). If two SFIs with identical source ranges get reported, we
                    // report them in decreasing order of call count, as in all known cases
                    // this corresponds to the nesting order. In the case of the script tag
                    // example above, we report the zero call count of `foo` last. As it turns
                    // out, embedders started to rely on functions being reported in nesting
                    // order.
                    // TODO(jgruber):  Investigate whether it is possible to remove node's
                    // extra  top-level wrapper script, or change its source range, or ensure
                    // that it follows the invariant that nesting order is descending count
                    // order for SFIs with identical source ranges.
                    while !nesting.is_empty() && functions[nesting.last().unwrap()].end <= start {
                        nesting.pop();
                    }

                    let mut count = count; //Shadow to allow modification
                    if count != 0 {
                        match collectionMode {
                            debug::CoverageMode::kBlockCount | debug::CoverageMode::kPreciseCount => {},
                            debug::CoverageMode::kBlockBinary | debug::CoverageMode::kPreciseBinary => {
                                count = if info.has_reported_binary_coverage() { 0 } else { 1 };
                                info.set_has_reported_binary_coverage(true);
                            }
                            debug::CoverageMode::kBestEffort => {
                                count = 1;
                            }
                        }
                    }

                    let name = SharedFunctionInfo::DebugName(isolate, info);
                    let mut function = CoverageFunction::new(start, end, count, name);

                    if is_block_mode(collectionMode) && info.HasCoverageInfo(isolate) {
                        collect_block_coverage(isolate, &mut function, info, collectionMode);
                    }

                    // Only include a function range if itself or its parent function is
                    // covered, or if it contains non-trivial block coverage.
                    let is_covered = (count != 0);
                    let parent_is_covered =
                        (!nesting.is_empty() && functions[nesting.last().unwrap()].count != 0);
                    let has_block_coverage = !function.blocks.is_empty();
                    let function_is_relevant =
                        (is_covered || parent_is_covered || has_block_coverage);

                    // It must also have a non-empty source range (otherwise it is not
                    // interesting to report).
                    let has_nonempty_source_range = function.HasNonEmptySourceRange();

                    if has_nonempty_source_range && function_is_relevant {
                        nesting.push(functions.len());
                        functions.push(function);
                    }

                    if v8_flags.trace_block_coverage {
                        print_block_coverage(&function, info, has_nonempty_source_range,
                                           function_is_relevant);
                    }
                }

                // Remove entries for scripts that have no coverage.
                if functions.is_empty() { result.pop_back(); }
            }
            result
        }

        pub fn SelectMode(isolate: *mut Isolate, mode: debug::CoverageMode) {
            if mode != unsafe { (*isolate).code_coverage_mode() } {
                // Changing the coverage mode can change the bytecode that would be
                // generated for a function, which can interfere with lazy source positions,
                // so just force source position collection whenever there's such a change.
                unsafe { (*isolate).CollectSourcePositionsForAllBytecodeArrays() };
                // Changing the coverage mode changes the generated bytecode and hence it is
                // not safe to flush bytecode. Set a flag here, so we can disable bytecode
                // flushing.
                unsafe { (*isolate).set_disable_bytecode_flushing(true) };
            }

            match mode {
                debug::CoverageMode::kBestEffort => {
                    // Note that DevTools switches back to best-effort coverage once the
                    // recording is stopped. Since we delete coverage infos at that point, any
                    // following coverage recording (without reloads) will be at function
                    // granularity.
                    unsafe { (*isolate).debug().RemoveAllCoverageInfos() };
                    unsafe { (*isolate).SetFeedbackVectorsForProfilingTools((*(*isolate).factory()).undefined_value()) };
                }
                debug::CoverageMode::kBlockBinary | debug::CoverageMode::kBlockCount |
                debug::CoverageMode::kPreciseBinary | debug::CoverageMode::kPreciseCount => {
                    //HandleScope scope(isolate); //Need to convert to Rust
                    // Remove all optimized function. Optimized and inlined functions do not
                    // increment invocation count.
                    //Deoptimizer::DeoptimizeAll(isolate);

                    let mut funcs_needing_feedback_vector: Vec<Handle<JSFunction>> = Vec::new();
                    {
                        let mut heap_iterator = HeapObjectIterator::new(unsafe { (*isolate).heap() });
                        while let Some(o) = heap_iterator.Next() {
                            if o.IsJSFunction() {
                                let func = o.Cast<JSFunction>();
                                if func.has_closure_feedback_cell_array() {
                                    funcs_needing_feedback_vector.push(func);
                                }
                            } else if is_binary_mode(mode) && o.IsSharedFunctionInfo() {
                                // If collecting binary coverage, reset
                                // SFI::has_reported_binary_coverage to avoid optimizing / inlining
                                // functions before they have reported coverage.
                                let shared = o.Cast<SharedFunctionInfo>();
                                shared.set_has_reported_binary_coverage(false);
                            } else if o.IsFeedbackVector() {
                                // In any case, clear any collected invocation counts.
                                o.Cast<FeedbackVector>().clear_invocation_count(kRelaxedStore);
                            }
                        }
                    }

                    for func in funcs_needing_feedback_vector.iter() {
                        //IsCompiledScope is_compiled_scope(func.shared().is_compiled_scope(isolate));
                        //CHECK(is_compiled_scope.is_compiled());
                        //JSFunction::EnsureFeedbackVector(isolate, func, &is_compiled_scope);
                    }

                    // Root all feedback vectors to avoid early collection.
                    unsafe { (*isolate).MaybeInitializeVectorListFromHeap() };

                    break;
                }
            }
            unsafe { (*isolate).set_code_coverage_mode(mode) };
        }

        fn push_back(&mut self, script_data: ScriptData) {
            self.script_data.push(script_data);
        }

        fn back(&self) -> &ScriptData {
            self.script_data.last().unwrap()
        }

        fn pop_back(&mut self) {
            self.script_data.pop();
        }
    }

    struct ScriptData {
        functions: Vec<CoverageFunction>,
        // Other script-related data
    }

    impl ScriptData {
        fn new(_script: Handle<Script>) -> Self {
            ScriptData {
                functions: Vec::new(),
            }
        }
    }

    // Assuming this is an enum
    mod debug {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum CoverageMode {
            kBestEffort,
            kBlockBinary,
            kBlockCount,
            kPreciseBinary,
            kPreciseCount,
        }
    }

    struct SharedToCounterMap {
        map: HashMap<Tagged<SharedFunctionInfo>, u32>,
    }

    impl SharedToCounterMap {
        fn new() -> Self {
            SharedToCounterMap {
                map: HashMap::new(),
            }
        }

        fn Add(&mut self, key: Tagged<SharedFunctionInfo>, count: u32) {
            let entry = self.map.entry(key).or_insert(0);
            if UINT32_MAX - count < *entry {
                *entry = UINT32_MAX;
            } else {
                *entry += count;
            }
        }

        fn Get(&self, key: Tagged<SharedFunctionInfo>) -> u32 {
            *self.map.get(&key).unwrap_or(&0)
        }
    }

    fn start_position(info: Tagged<SharedFunctionInfo>) -> i32 {
        let mut start = unsafe { (*info).function_token_position() };
        if start == kNoSourcePosition {
            start = unsafe { (*info).StartPosition() };
        }
        start
    }

    fn compare_coverage_block(a: &CoverageBlock, b: &CoverageBlock) -> Ordering {
        assert_ne!(kNoSourcePosition, a.start);
        assert_ne!(kNoSourcePosition, b.start);
        if a.start == b.start {
            if a.end > b.end {
                Ordering::Greater
            } else if a.end < b.end {
                Ordering::Less
            } else {
                Ordering::Equal
            }
        } else if a.start < b.start {
            Ordering::Less
        } else {
            Ordering::Greater
        }
    }

    fn sort_block_data(v: &mut Vec<CoverageBlock>) {
        v.sort_by(compare_coverage_block);
    }

    fn get_sorted_block_data(isolate: *mut Isolate, shared: Tagged<SharedFunctionInfo>) -> Vec<CoverageBlock> {
        assert!(unsafe { (*shared).HasCoverageInfo(isolate) });

        let coverage_info = unsafe { (*Cast::<CoverageInfo>(shared.GetDebugInfo(isolate).coverage_info())).unchecked_cast::<CoverageInfo>() };

        let mut result = Vec::new();
        if unsafe { (*coverage_info).slot_count() } == 0 {
            return result;
        }

        for i in 0..unsafe { (*coverage_info).slot_count() } {
            let start_pos = unsafe { (*coverage_info).slots_start_source_position(i) };
            let until_pos = unsafe { (*coverage_info).slots_end_source_position(i) };
            let count = unsafe { (*coverage_info).slots_block_count(i) };

            assert_ne!(kNoSourcePosition, start_pos);
            result.push(CoverageBlock::new(start_pos, until_pos, count));
        }

        sort_block_data(&mut result);

        result
    }

    struct CoverageBlockIterator<'a> {
        function_: &'a mut CoverageFunction,
        nesting_stack_: Vec<CoverageBlock>,
        ended_: bool,
        delete_current_: bool,
        read_index_: isize,
        write_index_: isize,
    }

    impl<'a> CoverageBlockIterator<'a> {
        fn new(function: &'a mut CoverageFunction) -> Self {
            assert!(function.blocks.windows(2).all(|w| compare_coverage_block(&w[0], &w[1]) != Ordering::Greater));

            CoverageBlockIterator {
                function_: function,
                nesting_stack_: Vec::new(),
                ended_: false,
                delete_current_: false,
                read_index_: -1,
                write_index_: -1,
            }
        }

        fn HasNext(&self) -> bool {
            (self.read_index_ + 1) < self.function_.blocks.len() as isize
        }

        fn Next(&mut self) -> bool {
            if !self.HasNext() {
                if !self.ended_ {
                    self.MaybeWriteCurrent();
                }
                self.ended_ = true;
                return false;
            }

            // If a block has been deleted, subsequent iteration moves trailing blocks
            // to their updated position within the array.
            self.MaybeWriteCurrent();

            if self.read_index_ == -1 {
                // Initialize the nesting stack with the function range.
                self.nesting_stack_.push(CoverageBlock::new(self.function_.start, self.function_.end, self.function_.count));
            } else if !self.delete_current_ {
                self.nesting_stack_.push(self.GetBlock());
            }

            self.delete_current_ = false;
            self.read_index_ += 1;

            assert!(self.IsActive());

            let mut block = self.GetBlock();
            while self.nesting_stack_.len() > 1 &&
                self.nesting_stack_.last().unwrap().end <= block.start {
                self.nesting_stack_.pop();
            }

            //DCHECK_IMPLIES(block.start >= function_->end,
            //               block.end == kNoSourcePosition);
            assert_ne!(block.start, kNoSourcePosition);
            assert!(block.end <= self.GetParent().end);

            true
        }

        fn GetBlock(&mut self) -> CoverageBlock {
            assert!(self.IsActive());
            self.function_.blocks[self.read_index_ as usize]
        }

        fn GetNextBlock(&mut self) -> CoverageBlock {
            assert!(self.IsActive());
            assert!(self.HasNext());
            self.function_.blocks[(self.read_index_ + 1) as usize]
        }

        fn GetPreviousBlock(&mut self) -> CoverageBlock {
            assert!(self.IsActive());
            assert!(self.read_index_ > 0);
            self.function_.blocks[(self.read_index_ - 1) as usize]
        }

        fn GetParent(&mut self) -> CoverageBlock {
            assert!(self.IsActive());
            *self.nesting_stack_.last().unwrap()
        }

        fn HasSiblingOrChild(&mut self) -> bool {
            assert!(self.IsActive());
            self.HasNext() && self.GetNextBlock().start < self.GetParent().end
        }

        fn GetSiblingOrChild(&mut self) -> CoverageBlock {
            assert!(self.HasSiblingOrChild());
            assert!(self.IsActive());
            self.GetNextBlock()
        }

        // A range is considered to be at top level if its parent range is the
        // function range.
        fn IsTopLevel(&self) -> bool {
            self.nesting_stack_.len() == 1
        }

        fn DeleteBlock(&mut self) {
            assert!(!self.delete_current_);
            assert!(self.IsActive());
            self.delete_current_ = true;
        }

        fn MaybeWriteCurrent(&mut self) {
            if self.delete_current_ {
                return;
            }
            if self.read_index_ >= 0 && self.write_index_ != self.read_index_ {
                self.function_.blocks[self.write_index_ as usize] = self.function_.blocks[self.read_index_ as usize];
            }
            self.write_index_ += 1;
        }

        fn Finalize(&mut self) {
            while self.Next() {}
            self.function_.blocks.resize(self.write_index_ as usize);
        }

        fn IsActive(&self) -> bool {
            self.read_index_ >= 0 && !self.ended_
        }
    }

    fn have_same_source_range(lhs: &CoverageBlock, rhs: &CoverageBlock) -> bool {
        lhs.start == rhs.start && lhs.end == rhs.end
    }

    fn merge_duplicate_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() && iter.HasNext() {
            let block = iter.GetBlock();
            let next_block = iter.GetNextBlock();

            if !have_same_source_range(&block, &next_block) {
                continue;
            }

            assert_ne!(kNoSourcePosition, block.end);  // Non-singleton range.
            function.blocks[(iter.read_index_ + 1) as usize].count = max(block.count, next_block.count);
            iter.DeleteBlock();
        }
    }

    // Rewrite position singletons (produced by unconditional control flow
    // like return statements, and by continuation counters) into source
    // ranges that end at the next sibling range or the end of the parent
    // range, whichever comes first.
    fn rewrite_position_singletons_to_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let mut block = iter.GetBlock();
            let parent = iter.GetParent();

            if block.start >= function.end {
                assert_eq!(block.end, kNoSourcePosition);
                iter.DeleteBlock();
            } else if block.end == kNoSourcePosition {
                // The current block ends at the next sibling block (if it exists) or the
                // end of the parent block otherwise.
                if iter.HasSiblingOrChild() {
                    function.blocks[iter.read_index_ as usize].end = iter.GetSiblingOrChild().start;
                } else if iter.IsTopLevel() {
                    // See https://crbug.com/v8/6661. Functions are special-cased because
                    // we never want the closing brace to be uncovered. This is mainly to
                    // avoid a noisy UI.
                    function.blocks[iter.read_index_ as usize].end = parent.end - 1;
                } else {
                    function.blocks[iter.read_index_ as usize].end = parent.end;
                }
            }
        }
    }

    fn merge_consecutive_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let block = iter.GetBlock();

            if iter.HasSiblingOrChild() {
                let sibling = iter.GetSiblingOrChild();
                if sibling.start == block.end && sibling.count == block.count {
                    // Best-effort: this pass may miss mergeable siblings in the presence of
                    // child blocks.
                    function.blocks[(iter.read_index_ + 1) as usize].start = block.start;
                    iter.DeleteBlock();
                }
            }
        }
    }

    fn merge_nested_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let block = iter.GetBlock();
            let parent = iter.GetParent();

            if parent.count == block.count {
                // Transformation may not be valid if sibling blocks exist with a
                // differing count.
                iter.DeleteBlock();
            }
        }
    }

    fn rewrite_function_scope_counter(function: &mut CoverageFunction) {
        // Every function must have at least the top-level function counter.
        assert!(!function.blocks.is_empty());

        let mut iter = CoverageBlockIterator::new(function);
        if iter.Next() {
            assert!(iter.IsTopLevel());

            let block = iter.GetBlock();
            if block.start == SourceRange_kFunctionLiteralSourcePosition &&
                block.end == SourceRange_kFunctionLiteralSourcePosition {
                // If a function-scope block exists, overwrite the function count. It has
                // a more reliable count than what we get from the FeedbackVector (which
                // is imprecise e.g. for generator functions and optimized code).
                function.count = block.count;

                // Then delete it; for compatibility with non-block coverage modes, the
                // function-scope block is expected in CoverageFunction, not as a
                // CoverageBlock.
                iter.DeleteBlock();
            }
        }
    }

    fn filter_aliased_singletons(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        iter.Next();  // Advance once since we reference the previous block later.

        while iter.Next() {
            let previous_block = iter.GetPreviousBlock();
            let block = iter.GetBlock();

            let is_singleton = block.end == kNoSourcePosition;
            let aliases_start = block.start == previous_block.start;

            if is_singleton && aliases_start {
                // The previous block must have a full range since duplicate singletons
                // have already been merged.
                assert_ne!(previous_block.end, kNoSourcePosition);
                // Likewise, the next block must have another start position since
                // singletons are sorted to the end.
                //DCHECK_IMPLIES(iter.HasNext(), iter.GetNextBlock().start != block.start); //Hard to translate with shared access.
                iter.DeleteBlock();
            }
        }
    }

    fn filter_uncovered_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let block = iter.GetBlock();
            let parent = iter.GetParent();
            if block.count == 0 && parent.count == 0 {
                iter.DeleteBlock();
            }
        }
    }

    fn filter_empty_ranges(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let block = iter.GetBlock();
            if block.start == block.end {
                iter.DeleteBlock();
            }
        }
    }

    fn clamp_to_binary(function: &mut CoverageFunction) {
        let mut iter = CoverageBlockIterator::new(function);

        while iter.Next() {
            let mut block = iter.GetBlock();
            if block.count > 0 {
                function.blocks[iter.read_index_ as usize].count = 1;
            }
        }
    }

    fn reset_all_block_counts(isolate: *mut Isolate, shared: Tagged<SharedFunctionInfo>) {
        assert!(unsafe { (*shared).HasCoverageInfo(isolate) });

        let coverage_info = unsafe { (*Cast::<CoverageInfo>(shared.GetDebugInfo(isolate).coverage_info())).unchecked_cast::<CoverageInfo>() };

        for i in 0..unsafe { (*coverage_info).slot_count() } {
            unsafe { (*coverage_info).ResetBlockCount(i) };
        }
    }

    fn is_block_mode(mode: debug::CoverageMode) -> bool {
        match mode {
            debug::CoverageMode::kBlockBinary | debug::CoverageMode::kBlockCount => true,
            _ => false,
        }
    }

    fn is_binary_mode(mode: debug::CoverageMode) -> bool {
        match mode {
            debug::CoverageMode::kBlockBinary | debug::CoverageMode::kPreciseBinary => true,
            _ => false,
        }
    }

    fn collect_block_coverage_internal(isolate: *mut Isolate, function: &mut CoverageFunction,
                                      info: Tagged<SharedFunctionInfo>, mode: debug::CoverageMode) {
        assert!(is_block_mode(mode));

        // Functions with empty source ranges are not interesting to report. This can
        // happen e.g. for internally-generated functions like class constructors.
        if !function.HasNonEmptySourceRange() {
            return;
        }

        function.has_block_coverage = true;
        function.blocks = get_sorted_block_data(isolate, info);

        // If in binary mode, only report counts of 0/1.
        if mode == debug::CoverageMode::kBlockBinary {
            clamp_to_binary(function);
        }

        // To stay compatible with non-block coverage modes, the function-scope count
        // is expected to be in the CoverageFunction, not as part of its blocks.
        // This finds the function-scope counter, overwrites CoverageFunction::count,
        // and removes it from the block list.
        //
        // Important: Must be called before other transformation passes.
        rewrite_function_scope_counter(function);

        // Functions without blocks don't need to be processed further.
        if !function.HasBlocks() {
            return;
        }

        // Remove singleton ranges with the same start position as a full range and
        // throw away their counts.
        // Singleton ranges are only intended to split existing full ranges and should
        // never expand into a full range. Consider 'if (cond) { ... } else { ... }'
        // as a problematic example; if the then-block produces a continuation
        // singleton, it would incorrectly expand into the else range.
        // For more context, see https://crbug.com/v8/8237.
        filter_aliased_singletons(function);

        // Rewrite all singletons (created e.g. by continuations and unconditional
        // control flow) to ranges.
        rewrite_position_singletons_to_ranges(function);

        // Merge nested and consecutive ranges with identical counts.
        // Note that it's necessary to merge duplicate ranges prior to merging nested
        // changes in order to avoid invalid transformations. See crbug.com/827530.
        merge_consecutive_ranges(function);

        sort_block_data(&mut function.blocks);
        merge_duplicate_ranges(function);
        merge_nested_ranges(function);

        merge_consecutive_ranges(function);

        // Filter out ranges with count == 0 unless the immediate parent range has
        // a count != 0.
        filter_uncovered_ranges(function);

        // Filter out ranges of zero length.
        filter_empty_ranges(function);
    }

    fn collect_block_coverage(isolate: *mut Isolate, function: &mut CoverageFunction,
                              info: Tagged<SharedFunctionInfo>, mode: debug::CoverageMode) {
        collect_block_coverage_internal(isolate, function, info, mode);

        // Reset all counters on the DebugInfo to zero.
        reset_all_block_counts(isolate, info);
    }

    fn print_block_coverage(function: &CoverageFunction, info: Tagged<SharedFunctionInfo>,
                            has_nonempty_source_range: bool, function_is_relevant: bool) {
        if v8_flags.trace_block_coverage {
            //let function_name = function.name.ToCString(); //Need to convert to Rust
            println!(
                "Coverage for function='{}', SFI={:p}, has_nonempty_source_range={}, function_is_relevant={}",
                unsafe { (*function.name).ToString() }, info, has_nonempty_source_range, function_is_relevant
            );
            println!("{{start: {}, end: {}, count: {}}}", function.start, function.end, function.count);
            for block in &function.blocks {
                println!("{{start: {}, end: {}, count: {}}}", block.start, block.end, block.count);
            }
        }
    }

    fn collect_and_maybe_reset_counts(isolate: *mut Isolate, counter_map: &mut SharedToCounterMap,
                                      coverage_mode: v8::debug::CoverageMode) {
        let reset_count = coverage_mode != v8::debug::CoverageMode::kBestEffort;

        match unsafe { (*isolate).code_coverage_mode() } {
            v8::debug::CoverageMode::kBlockBinary | v8::debug::CoverageMode::kBlockCount |
            v8::debug::CoverageMode::kPreciseBinary | v8::debug::CoverageMode::kPreciseCount => {
                //