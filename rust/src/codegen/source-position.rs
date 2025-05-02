// src/codegen/source_position.rs

use std::fmt;
// use std::error::Error;  // Consider using this for more specific error types
// use std::rc::Rc;  // If shared ownership without mutability is needed
use std::sync::Arc; // If shared ownership with mutability is needed.
use std::borrow::Cow; // use for CString conversion
//use std::collections::HashMap; // Consider using this if InliningPositions is a HashMap

// Placeholder types and functions, replace with actual V8 types and logic
#[derive(Debug, Clone, Copy)]
pub struct SourcePosition {
    inlining_id: i32,
    script_offset: i32,
    external_line: i32,
    external_file_id: i32,
    is_inlined_flag: bool,
    is_external_flag: bool,
}

impl SourcePosition {
    pub fn new() -> Self {
        SourcePosition {
            inlining_id: 0,
            script_offset: 0,
            external_line: 0,
            external_file_id: 0,
            is_inlined_flag: false,
            is_external_flag: false,
        }
    }

    pub fn inlined(inlining_id: i32) -> Self {
        SourcePosition {
            inlining_id,
            script_offset: 0,
            external_line: 0,
            external_file_id: 0,
            is_inlined_flag: true,
            is_external_flag: false,
        }
    }
    pub fn is_inlined(&self) -> bool {
        self.is_inlined_flag
    }
    pub fn inlining_id(&self) -> i32 {
        self.inlining_id
    }
    pub fn script_offset(&self) -> i32 {
        self.script_offset
    }
    pub fn is_external(&self) -> bool {
        self.is_external_flag
    }
    pub fn external_line(&self) -> i32 {
        self.external_line
    }
    pub fn external_file_id(&self) -> i32 {
        self.external_file_id
    }

    fn print_internal<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        if self.is_inlined() {
            write!(out, "<inlined({}):", self.inlining_id())?;
        } else {
            write!(out, "<not inlined:")?;
        }

        if self.is_external() {
            write!(out, "{}, {}>", self.external_line(), self.external_file_id())?;
        } else {
            write!(out, "{}>", self.script_offset())?;
        }
        Ok(())
    }

    pub fn inlining_stack(
        &self,
        isolate: &Isolate,
        cinfo: &OptimizedCompilationInfo,
    ) -> Vec<SourcePositionInfo> {
        let mut pos = *self;
        let mut stack = Vec::new();
        while pos.is_inlined() {
            let inl = &cinfo.inlined_functions[pos.inlining_id() as usize];
            stack.push(SourcePositionInfo::new(isolate, pos, inl.shared_info.clone()));
            pos = inl.position.position;
        }
        stack.push(SourcePositionInfo::new(isolate, pos, cinfo.shared_info.clone()));
        stack
    }

    pub fn inlining_stack_code(
        &self,
        isolate: &Isolate,
        code: &Code,
    ) -> Vec<SourcePositionInfo> {
        let deopt_data = code.deoptimization_data.as_ref().unwrap(); //Cast::<DeoptimizationData>(code.deoptimization_data());
        let mut pos = *self;
        let mut stack = Vec::new();
        while pos.is_inlined() {
            let inl = deopt_data.inlining_positions[pos.inlining_id() as usize];
            let function = deopt_data.get_inlined_function(inl.inlined_function_id).clone();
            stack.push(SourcePositionInfo::new(isolate, pos, function));
            pos = inl.position;
        }
        let function = deopt_data.get_shared_function_info().clone();
        stack.push(SourcePositionInfo::new(isolate, pos, function));
        stack
    }

    pub fn first_info(&self, isolate: &Isolate, code: &Code) -> SourcePositionInfo {
        let deopt_data = code.deoptimization_data.as_ref().unwrap();
        let pos = *self;
        if pos.is_inlined() {
            let inl = deopt_data.inlining_positions[pos.inlining_id() as usize];
            let function = deopt_data.get_inlined_function(inl.inlined_function_id).clone();
            SourcePositionInfo::new(isolate, pos, function)
        } else {
            let function = deopt_data.get_shared_function_info().clone();
            SourcePositionInfo::new(isolate, pos, function)
        }
    }

    pub fn print<W: std::io::Write>(&self, out: &mut W, function: &SharedFunctionInfo) -> std::io::Result<()> {
        let mut pos_info = ScriptPositionInfo::default();
        let mut source_name: Option<String> = None;

        if let Some(script) = &function.script {
            source_name = script.name.clone();

            // Get position info based on script offset
            let script_ref = script.clone();
            script_ref.get_position_info(self.script_offset(), &mut pos_info);
        }

        write!(out, "<")?;
        if let Some(name) = source_name {
            write!(out, "{}", name)?;
        } else {
            write!(out, "unknown")?;
        }
        write!(out, ":{}:{}>", pos_info.line + 1, pos_info.column + 1)?;
        Ok(())
    }

    pub fn print_json<W: std::io::Write>(&self, out: &mut W) -> std::io::Result<()> {
        if self.is_external() {
            write!(out, "{{ \"line\" : {}, ", self.external_line())?;
            write!(out, "  \"fileId\" : {}, ", self.external_file_id())?;
            write!(out, "  \"inliningId\" : {} }}", self.inlining_id())?;
        } else {
            write!(out, "{{ \"scriptOffset\" : {}, ", self.script_offset())?;
            write!(out, "  \"inliningId\" : {} }}", self.inlining_id())?;
        }
        Ok(())
    }

    pub fn print_code<W: std::io::Write>(&self, out: &mut W, code: &Code) -> std::io::Result<()> {
        let deopt_data = code.deoptimization_data.as_ref().unwrap();

        if !self.is_inlined() {
            let function = deopt_data.get_shared_function_info();
            self.print(out, function)?;
        } else {
            let inl = deopt_data.inlining_positions[self.inlining_id() as usize];
            if inl.inlined_function_id == -1 {
                self.print_internal(out)?;
            } else {
                let function = deopt_data.get_inlined_function(inl.inlined_function_id);
                self.print(out, function)?;
            }
            write!(out, " inlined at ")?;
            inl.position.print_code(out, code)?;
        }
        Ok(())
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buffer = Vec::new();
        self.print_internal(&mut buffer).unwrap();
        write!(f, "{}", String::from_utf8_lossy(&buffer))
    }
}

#[derive(Debug, Clone)]
pub struct SourcePositionInfo {
    pub position: SourcePosition,
    pub shared: Arc<SharedFunctionInfo>,
    pub script: Option<Arc<Script>>,
    pub line: i32,
    pub column: i32,
}

impl SourcePositionInfo {
    pub fn new(isolate: &Isolate, pos: SourcePosition, sfi: Arc<SharedFunctionInfo>) -> Self {
        let mut info = SourcePositionInfo {
            position: pos,
            shared: sfi.clone(),
            script: None,
            line: 0,
            column: 0,
        };

        if let Some(script) = &sfi.script {
            info.script = Some(script.clone());
            let mut script_info = ScriptPositionInfo::default();
            if script.get_position_info(pos.script_offset(), &mut script_info) {
                info.line = script_info.line;
                info.column = script_info.column;
            }
        }
        info
    }
}

impl fmt::Display for SourcePositionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<")?;
        if let Some(script) = &self.script {
            if let Some(name) = &script.name {
                write!(f, "{}", name)?;
            } else {
                write!(f, "unknown")?;
            }
        } else {
            write!(f, "unknown")?;
        }
        write!(f, ":{}:{}>", self.line + 1, self.column + 1)
    }
}

//Implement display for a vector of source position info
impl fmt::Display for Vec<SourcePositionInfo> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut first = true;
        for pos in self {
            if !first {
                write!(f, " inlined at ")?;
            }
            write!(f, "{}", pos)?;
            first = false;
        }
        Ok(())
    }
}

// Placeholder types, replace with actual V8 types
pub struct Isolate {}

impl Isolate {
    pub fn new() -> Self {
        Isolate {}
    }
}

#[derive(Debug, Clone)]
pub struct SharedFunctionInfo {
    pub script: Option<Arc<Script>>,
}

impl SharedFunctionInfo {
    pub fn new() -> Self {
        SharedFunctionInfo { script: None }
    }
}

#[derive(Debug, Clone)]
pub struct Script {
    pub name: Option<String>,
}

impl Script {
    pub fn new() -> Self {
        Script { name: None }
    }
    pub fn get_position_info(&self, _offset: i32, pos_info: &mut ScriptPositionInfo) -> bool {
        pos_info.line = 10;
        pos_info.column = 5;
        true
    }
}

#[derive(Debug, Clone, Default)]
pub struct ScriptPositionInfo {
    pub line: i32,
    pub column: i32,
}

#[derive(Debug, Clone)]
pub struct OptimizedCompilationInfo {
    pub inlined_functions: Vec<InlinedFunctionInfo>,
    pub shared_info: Arc<SharedFunctionInfo>,
}

impl OptimizedCompilationInfo {
    pub fn new() -> Self {
        OptimizedCompilationInfo {
            inlined_functions: Vec::new(),
            shared_info: Arc::new(SharedFunctionInfo::new()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct InlinedFunctionInfo {
    pub position: InliningPosition,
    pub shared_info: Arc<SharedFunctionInfo>,
}

#[derive(Debug, Clone, Copy)]
pub struct InliningPosition {
    pub position: SourcePosition,
    pub inlined_function_id: i32,
}

#[derive(Debug, Clone)]
pub struct Code {
    pub deoptimization_data: Option<Box<DeoptimizationData>>,
}

impl Code {
    pub fn new() -> Self {
        Code {
            deoptimization_data: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DeoptimizationData {
    pub inlining_positions: Vec<InliningPosition>,
    pub shared_function_info: Arc<SharedFunctionInfo>,
    pub inlined_functions: Vec<Arc<SharedFunctionInfo>>,
}

impl DeoptimizationData {
    pub fn new() -> Self {
        DeoptimizationData {
            inlining_positions: Vec::new(),
            shared_function_info: Arc::new(SharedFunctionInfo::new()),
            inlined_functions: Vec::new(),
        }
    }

    pub fn get_inlined_function(&self, index: i32) -> Arc<SharedFunctionInfo> {
        self.inlined_functions[index as usize].clone()
    }

    pub fn get_shared_function_info(&self) -> Arc<SharedFunctionInfo> {
        self.shared_function_info.clone()
    }
}

pub mod test {
    use super::*;

    #[test]
    fn test_source_position_info_display() {
        let isolate = Isolate::new();
        let mut script = Script::new();
        script.name = Some("test.js".to_string());
        let shared_info = Arc::new(SharedFunctionInfo { script: Some(Arc::new(script)) });
        let pos = SourcePosition::new();
        let info = SourcePositionInfo::new(&isolate, pos, shared_info);

        assert_eq!(format!("{}", info), "<test.js:11:6>");
    }

    #[test]
    fn test_inlining_stack() {
        let isolate = Isolate::new();
        let mut cinfo = OptimizedCompilationInfo::new();

        let mut shared_info_1 = SharedFunctionInfo::new();
        let mut script_1 = Script::new();
        script_1.name = Some("inline1.js".to_string());
        shared_info_1.script = Some(Arc::new(script_1));

        let mut shared_info_2 = SharedFunctionInfo::new();
        let mut script_2 = Script::new();
        script_2.name = Some("inline2.js".to_string());
        shared_info_2.script = Some(Arc::new(script_2));

        let mut shared_info_3 = SharedFunctionInfo::new();
        let mut script_3 = Script::new();
        script_3.name = Some("top.js".to_string());
        shared_info_3.script = Some(Arc::new(script_3));

        cinfo.shared_info = Arc::new(shared_info_3);

        let pos1 = SourcePosition::inlined(0);
        let pos2 = SourcePosition::inlined(1);

        cinfo.inlined_functions.push(InlinedFunctionInfo {
            position: InliningPosition {
                position: pos2,
                inlined_function_id: 0,
            },
            shared_info: Arc::new(shared_info_1),
        });

        cinfo.inlined_functions.push(InlinedFunctionInfo {
            position: InliningPosition {
                position: SourcePosition::new(),
                inlined_function_id: 0,
            },
            shared_info: Arc::new(shared_info_2),
        });

        let stack = pos1.inlining_stack(&isolate, &cinfo);
        assert_eq!(stack.len(), 3);
        assert_eq!(format!("{}", stack[0]), "<inline1.js:11:6>");
        assert_eq!(format!("{}", stack[1]), "<inline2.js:11:6>");
        assert_eq!(format!("{}", stack[2]), "<top.js:11:6>");
    }
}