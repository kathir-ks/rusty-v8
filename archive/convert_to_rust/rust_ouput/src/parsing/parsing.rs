// Converted from V8 C++ source files:
// Header: parsing.h
// Implementation: parsing.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod parsing {
    use std::rc::Rc;

    pub enum ReportStatisticsMode {
        kYes,
        kNo,
    }

    pub struct ParseInfo {}
    pub struct Script {}
    pub struct Isolate {}
    pub struct SharedFunctionInfo {}
    pub struct ScopeInfo {}
    pub struct Utf16CharacterStream {}
    pub struct Parser {
        parsing_on_main_thread_: bool,
    }
    pub struct Handle<T> {
        value: Rc<T>,
    }
    pub struct DirectHandle<T> {
        value: Rc<T>,
    }
    pub struct MaybeDirectHandle<T> {
        value: Option<Rc<T>>,
    }
    pub struct String {}

    impl<T> Handle<T> {
        pub fn new(value: Rc<T>) -> Self {
            Handle { value }
        }
    }
    impl<T> DirectHandle<T> {
        pub fn new(value: Rc<T>) -> Self {
            DirectHandle { value }
        }
    }

    impl ParseInfo {
        pub fn flags(&self) -> ParseInfoFlags {
            ParseInfoFlags {}
        }
        pub fn literal(&self) -> *const i32 {
            std::ptr::null()
        }

        pub fn set_character_stream(&mut self, _stream: std::unique_ptr<Utf16CharacterStream>) {}
    }

    impl Script {
        pub fn source(&self) -> Handle<String> {
            Handle {
                value: Rc::new(String {}),
            }
        }
    }

    impl SharedFunctionInfo {
        pub fn script(&self) -> Rc<Script> {
            Rc::new(Script {})
        }
        pub fn StartPosition(&self) -> u32 {
            0
        }
        pub fn EndPosition(&self) -> u32 {
            10
        }
        pub fn HasOuterScopeInfo(&self) -> bool {
            false
        }
        pub fn GetOuterScopeInfo(&self) -> Rc<ScopeInfo> {
            Rc::new(ScopeInfo {})
        }
    }

    pub struct ParseInfoFlags {
    }
    impl ParseInfoFlags {
        pub fn is_toplevel(&self) -> bool {
            true
        }
    }

    impl Parser {
        pub fn new(_isolate: &Isolate, _info: &ParseInfo) -> Self {
            Parser {
                parsing_on_main_thread_: true,
            }
        }

        pub fn UpdateStatistics(&self, _isolate: &Isolate, _script: DirectHandle<Script>) {}

        pub fn ParseProgram(&mut self, _isolate: &Isolate, _script: DirectHandle<Script>, _info: &ParseInfo, _maybe_outer_scope_info: MaybeDirectHandle<ScopeInfo>) {}
        pub fn ParseFunction(&mut self, _isolate: &Isolate, _info: &ParseInfo, _shared_info: DirectHandle<SharedFunctionInfo>) {}
    }

    pub mod ScannerStream {
        use super::*;
        pub fn For(_isolate: &Isolate, _source: Handle<String>) -> std::unique_ptr<Utf16CharacterStream> {
            std::unique_ptr::<Utf16CharacterStream>::default()
        }

        pub fn For(_isolate: &Isolate, _source: Handle<String>, _start_pos: u32, _end_pos: u32) -> std::unique_ptr<Utf16CharacterStream> {
            std::unique_ptr::<Utf16CharacterStream>::default()
        }
    }

    pub mod VMState {
        pub struct PARSER {}
        pub struct VMState<'a, T> {
            _isolate: &'a super::Isolate,
            _state: std::marker::PhantomData<T>,
        }

        impl<'a, T> VMState<'a, T> {
            pub fn new(_isolate: &'a super::Isolate) -> Self {
                VMState {
                    _isolate,
                    _state: std::marker::PhantomData,
                }
            }
        }
    }

    pub fn ParseProgram(
        info: &mut ParseInfo,
        script: DirectHandle<Script>,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        ParseProgramInner(info, script, MaybeDirectHandle { value: None }, isolate, mode)
    }

    pub fn ParseProgram(
        info: &mut ParseInfo,
        script: DirectHandle<Script>,
        outer_scope: MaybeDirectHandle<ScopeInfo>,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        ParseProgramInner(info, script, outer_scope, isolate, mode)
    }

    fn ParseProgramInner(
        info: &mut ParseInfo,
        script: DirectHandle<Script>,
        maybe_outer_scope_info: MaybeDirectHandle<ScopeInfo>,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        assert!(info.flags().is_toplevel());
        assert!(info.literal().is_null());

        let _state = VMState::VMState::<VMState::PARSER>::new(isolate);

        let source: Handle<String> = script.value.source();
        let stream = ScannerStream::For(isolate, source);
        info.set_character_stream(stream);

        let mut parser = Parser::new(isolate, info);
        parser.ParseProgram(isolate, script, info, maybe_outer_scope_info);

        MaybeReportStatistics(info, script, isolate, &mut parser, mode);

        info.literal().is_null() == false
    }

    pub fn ParseFunction(
        info: &mut ParseInfo,
        shared_info: DirectHandle<SharedFunctionInfo>,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        assert!(!info.flags().is_toplevel());
        assert!(!shared_info.value.script().is_null());
        assert!(info.literal().is_null());

        let _state = VMState::VMState::<VMState::PARSER>::new(isolate);

        let script = DirectHandle::new(shared_info.value.script());
        let source: Handle<String> = script.value.source();
        let start_pos = shared_info.value.StartPosition();
        let end_pos = shared_info.value.EndPosition();

        if end_pos > source.value.length() as u32 {
            isolate.PushStackTraceAndDie(std::ptr::null_mut(), std::ptr::null_mut());
        }

        let stream = ScannerStream::For(isolate, source, start_pos, end_pos);
        info.set_character_stream(stream);

        let mut parser = Parser::new(isolate, info);
        parser.ParseFunction(isolate, info, shared_info);

        MaybeReportStatistics(info, script, isolate, &mut parser, mode);

        info.literal().is_null() == false
    }

    pub fn ParseAny(
        info: &mut ParseInfo,
        shared_info: DirectHandle<SharedFunctionInfo>,
        isolate: &Isolate,
        mode: ReportStatisticsMode,
    ) -> bool {
        assert!(!shared_info.value.script().is_null());
        if info.flags().is_toplevel() {
            let maybe_outer_scope_info: MaybeDirectHandle<ScopeInfo> =
                if shared_info.value.HasOuterScopeInfo() {
                    MaybeDirectHandle {
                        value: Some(DirectHandle::new(shared_info.value.GetOuterScopeInfo()).value),
                    }
                } else {
                    MaybeDirectHandle { value: None }
                };

            ParseProgram(
                info,
                DirectHandle::new(shared_info.value.script()),
                maybe_outer_scope_info,
                isolate,
                mode,
            )
        } else {
            ParseFunction(info, shared_info, isolate, mode)
        }
    }

    impl String {
        pub fn length(&self) -> usize {
            10
        }
    }
    impl Isolate {
        pub fn main_thread_local_isolate(&self) -> &Isolate {
            self
        }
        pub fn PushStackTraceAndDie(&self, _arg1: *mut std::ffi::c_void, _arg2: *mut std::ffi::c_void) {}
    }

    fn MaybeReportStatistics(info: &mut ParseInfo, script: DirectHandle<Script>, isolate: &Isolate, parser: &mut Parser, mode: ReportStatisticsMode) {
        match mode {
            ReportStatisticsMode::kYes => {
                parser.UpdateStatistics(isolate, script);
            }
            ReportStatisticsMode::kNo => {}
        }
    }

    impl<T> MaybeDirectHandle<T> {
        pub const fn kNullMaybeHandle() -> Self {
            MaybeDirectHandle { value: None }
        }
    }
}
