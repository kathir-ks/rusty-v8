// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

/// Represents a template parameter for a C++ class.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TemplateParameter {
    pub name: String,
    pub type_: String,
}

impl TemplateParameter {
    /// Creates a new `TemplateParameter` with only a name.
    pub fn new(name: String) -> Self {
        TemplateParameter {
            name,
            type_: String::new(),
        }
    }

    /// Creates a new `TemplateParameter` with a type and a name.
    pub fn with_type(type_: String, name: String) -> Self {
        TemplateParameter { name, type_ }
    }
}

/// Represents a C++ class.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Class {
    template_parameters: Vec<TemplateParameter>,
    name: String,
}

impl Class {
    /// Creates a new `Class` with the given name.
    pub fn new(name: String) -> Self {
        Class {
            template_parameters: Vec::new(),
            name,
        }
    }

    /// Creates a new `Class` with the given template parameters and name.
    pub fn with_template_parameters(
        template_parameters: Vec<TemplateParameter>,
        name: String,
    ) -> Self {
        Class {
            template_parameters,
            name,
        }
    }

    /// Returns the name of the class.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Returns the template parameters of the class.
    pub fn get_template_parameters(&self) -> &Vec<TemplateParameter> {
        &self.template_parameters
    }
}

macro_rules! function_flag_list {
    ($V:ident) => {
        $V!(Inline, 0x01);
        $V!(V8Inline, 0x03);
        $V!(Const, 0x04);
        $V!(Constexpr, 0x08);
        $V!(Export, 0x10);
        $V!(Static, 0x20);
        $V!(Override, 0x40);
    };
}

bitflags::bitflags! {
    /// Represents the flags for a C++ function.
    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub struct FunctionFlag: u32 {
        #[allow(non_upper_case_globals)]
        const Inline = 0x01;
        #[allow(non_upper_case_globals)]
        const V8Inline = 0x03;
        #[allow(non_upper_case_globals)]
        const Const = 0x04;
        #[allow(non_upper_case_globals)]
        const Constexpr = 0x08;
        #[allow(non_upper_case_globals)]
        const Export = 0x10;
        #[allow(non_upper_case_globals)]
        const Static = 0x20;
        #[allow(non_upper_case_globals)]
        const Override = 0x40;
    }
}

/// Represents a parameter for a C++ function.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Parameter {
    pub type_: String,
    pub name: String,
    pub default_value: String,
}

impl Parameter {
    /// Creates a new `Parameter`.
    pub fn new(type_: String, name: String, default_value: String) -> Self {
        Parameter {
            type_,
            name,
            default_value,
        }
    }

    /// Creates a new `Parameter` without a default value.
    pub fn simple(type_: String, name: String) -> Self {
        Parameter {
            type_,
            name,
            default_value: String::new(),
        }
    }
}

/// Represents a C++ function.
#[derive(Debug)]
pub struct Function<'a> {
    // pos_: SourcePosition, // TODO: Add SourcePosition type
    owning_class: Option<&'a Class>,
    description: String,
    name: String,
    return_type: String,
    parameters: Vec<Parameter>,
    flags: FunctionFlag,
}

impl<'a> Function<'a> {
    /// Creates a new `Function`.
    pub fn new(name: String) -> Self {
        Function {
            // pos_: CurrentSourcePosition::Get(), // TODO: Implement
            owning_class: None,
            description: String::new(),
            name,
            return_type: String::new(),
            parameters: Vec::new(),
            flags: FunctionFlag::empty(),
        }
    }

    /// Creates a new `Function` belonging to a class.
    pub fn in_class(owning_class: &'a Class, name: String) -> Self {
        Function {
            // pos_: CurrentSourcePosition::Get(), // TODO: Implement
            owning_class: Some(owning_class),
            description: String::new(),
            name,
            return_type: String::new(),
            parameters: Vec::new(),
            flags: FunctionFlag::empty(),
        }
    }

    /// Creates a default getter function.
    pub fn default_getter(return_type: String, owner: &'a Class, name: String) -> Self {
        let mut getter = Function::in_class(owner, name);
        getter.set_return_type(return_type);
        getter.set_inline(true);
        getter.set_const(true);
        getter
    }

    /// Creates a default setter function.
    pub fn default_setter(
        owner: &'a Class,
        name: String,
        parameter_type: String,
        parameter_name: String,
    ) -> Self {
        let mut setter = Function::in_class(owner, name);
        setter.set_return_type("void".to_string());
        setter.add_parameter(parameter_type, parameter_name, String::new());
        setter.set_inline(true);
        setter
    }

    /// Sets a function flag.
    pub fn set_flag(&mut self, flag: FunctionFlag, value: bool) {
        if value {
            self.flags |= flag;
        } else {
            self.flags.remove(flag);
        }
    }

    /// Sets multiple function flags.
    pub fn set_flags(&mut self, flags: FunctionFlag, value: bool) {
        if value {
            self.flags |= flags;
        } else {
            self.flags.remove(flags);
        }
    }

    /// Checks if a function flag is set.
    pub fn has_flag(&self, flag: FunctionFlag) -> bool {
        self.flags.contains(flag)
    }

    /// Sets the inline flag.
    pub fn set_inline(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Inline, v);
    }

    /// Checks if the inline flag is set.
    pub fn is_inline(&self) -> bool {
        self.has_flag(FunctionFlag::Inline)
    }

    /// Sets the V8Inline flag.
    pub fn set_v8inline(&mut self, v: bool) {
        self.set_flag(FunctionFlag::V8Inline, v);
    }

    /// Checks if the V8Inline flag is set.
    pub fn is_v8inline(&self) -> bool {
        self.has_flag(FunctionFlag::V8Inline)
    }

    /// Sets the const flag.
    pub fn set_const(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Const, v);
    }

    /// Checks if the const flag is set.
    pub fn is_const(&self) -> bool {
        self.has_flag(FunctionFlag::Const)
    }

    /// Sets the constexpr flag.
    pub fn set_constexpr(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Constexpr, v);
    }

    /// Checks if the constexpr flag is set.
    pub fn is_constexpr(&self) -> bool {
        self.has_flag(FunctionFlag::Constexpr)
    }

    /// Sets the export flag.
    pub fn set_export(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Export, v);
    }

    /// Checks if the export flag is set.
    pub fn is_export(&self) -> bool {
        self.has_flag(FunctionFlag::Export)
    }

    /// Sets the static flag.
    pub fn set_static(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Static, v);
    }

    /// Checks if the static flag is set.
    pub fn is_static(&self) -> bool {
        self.has_flag(FunctionFlag::Static)
    }

    /// Sets the override flag.
    pub fn set_override(&mut self, v: bool) {
        self.set_flag(FunctionFlag::Override, v);
    }

    /// Checks if the override flag is set.
    pub fn is_override(&self) -> bool {
        self.has_flag(FunctionFlag::Override)
    }

    /// Sets the description of the function.
    pub fn set_description(&mut self, description: String) {
        self.description = description;
    }

    /// Sets the name of the function.
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    /// Sets the return type of the function.
    pub fn set_return_type(&mut self, return_type: String) {
        self.return_type = return_type;
    }

    /// Adds a parameter to the function.
    pub fn add_parameter(&mut self, type_: String, name: String, default_value: String) {
        self.parameters.push(Parameter::new(type_, name, default_value));
    }

    /// Inserts a parameter at a specific index in the function's parameter list.
    pub fn insert_parameter(
        &mut self,
        index: usize,
        type_: String,
        name: String,
        default_value: String,
    ) {
        if index <= self.parameters.len() {
            self.parameters.insert(index, Parameter::new(type_, name, default_value));
        }
    }

    /// Gets the parameters of the function.
    pub fn get_parameters(&self) -> &Vec<Parameter> {
        &self.parameters
    }

    /// Gets the names of the parameters of the function.
    pub fn get_parameter_names(&self) -> Vec<String> {
        self.parameters.iter().map(|p| p.name.clone()).collect()
    }

    pub const AUTOMATIC_INDENTATION: i32 = -1;

    // TODO: Implement PrintDeclaration, PrintDefinition, PrintInlineDefinition, PrintBeginDefinition, PrintEndDefinition
    // pub fn print_declaration(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
    // pub fn print_definition(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     builder: &dyn Fn(&mut dyn std::io::Write),
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
    // pub fn print_inline_definition(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     builder: &dyn Fn(&mut dyn std::io::Write),
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
    // pub fn print_begin_definition(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
    // pub fn print_end_definition(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
    // pub fn print_declaration_header(
    //     &self,
    //     stream: &mut dyn std::io::Write,
    //     indentation: i32,
    // ) {
    //     todo!()
    // }
}

/// Represents a C++ file.
pub struct File<'a> {
    stream: &'a mut dyn std::io::Write,
    includes: std::collections::HashSet<String>,
    namespaces: Vec<String>,
}

impl<'a> File<'a> {
    /// Creates a new `File`.
    pub fn new(stream: &'a mut dyn std::io::Write) -> Self {
        File {
            stream,
            includes: std::collections::HashSet::new(),
            namespaces: Vec::new(),
        }
    }

    /// Begins an include guard.
    pub fn begin_include_guard(&mut self, name: &str) {
        writeln!(self.stream, "#ifndef {}", name).unwrap();
        writeln!(self.stream, "#define {}", name).unwrap();
        writeln!(self.stream).unwrap();
    }

    /// Ends an include guard.
    pub fn end_include_guard(&mut self, name: &str) {
        writeln!(self.stream).unwrap();
        writeln!(self.stream, "#endif  // {}", name).unwrap();
    }

    /// Begins a namespace.
    pub fn begin_namespace(&mut self, name: String) {
        writeln!(self.stream, "namespace {} {{", name).unwrap();
        self.namespaces.push(name);
    }

    /// Begins two nested namespaces.
    pub fn begin_namespace2(&mut self, name0: String, name1: String) {
        self.begin_namespace(name0);
        self.begin_namespace(name1);
    }

    /// Ends a namespace.
    pub fn end_namespace(&mut self, name: &str) {
        if self.namespaces.last().map(|n| n.as_str()) == Some(name) {
            writeln!(self.stream, "}}  // namespace {}", name).unwrap();
            self.namespaces.pop();
        }
    }

    /// Ends two nested namespaces.
    pub fn end_namespace2(&mut self, name0: String, name1: String) {
        self.end_namespace(&name1);
        self.end_namespace(&name0);
    }

    /// Adds an include directive.
    pub fn add_include(&mut self, include: String) {
        self.includes.insert(include);
    }

    // The `<<` operator is replaced by the `write!` macro or similar.
    // This function serves as a basic example for writing values to the stream.
    pub fn write<T: std::fmt::Display>(&mut self, value: T) -> &mut Self {
        write!(self.stream, "{}", value).unwrap();
        self
    }
}

/// A scope guard for include guards.
pub struct IncludeGuardScope<'a> {
    file: Option<&'a mut File<'a>>,
    name: String,
}

impl<'a> IncludeGuardScope<'a> {
    /// Creates a new `IncludeGuardScope`.
    pub fn new(file: &'a mut File<'a>, name: String) -> Self {
        file.begin_include_guard(&name);
        IncludeGuardScope {
            file: Some(file),
            name,
        }
    }
}

impl<'a> Drop for IncludeGuardScope<'a> {
    fn drop(&mut self) {
        if let Some(file) = self.file.take() {
            file.end_include_guard(&self.name);
        }
    }
}