// Converted from V8 C++ source files:
// Header: cpp-builder.h
// Implementation: cpp-builder.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod cpp {
    use std::collections::HashSet;
    use std::collections::VecDeque;
    use std::io::Write;
    use std::mem;
    use std::ops::{BitAnd, BitOr, BitXor, Not, Sub};
    use std::string::String;

    use crate::torque::ast::CurrentSourcePosition;
    use crate::torque::types::SourcePosition;

    #[derive(Debug, Clone)]
    pub struct TemplateParameter {
        pub name: String,
        pub type_: String,
    }

    impl TemplateParameter {
        pub fn new(name: String) -> Self {
            TemplateParameter {
                name,
                type_: String::new(),
            }
        }
        pub fn new_with_type(type_: String, name: String) -> Self {
            TemplateParameter { name, type_ }
        }
    }

    #[derive(Debug, Clone)]
    pub struct Class {
        template_parameters_: Vec<TemplateParameter>,
        name_: String,
    }

    impl Class {
        pub fn new(name: String) -> Self {
            Class {
                template_parameters_: Vec::new(),
                name_: name,
            }
        }

        pub fn new_with_template_parameters(
            template_parameters: Vec<TemplateParameter>,
            name: String,
        ) -> Self {
            Class {
                template_parameters_: template_parameters,
                name_: name,
            }
        }

        pub fn get_name(&self) -> String {
            self.name_.clone()
        }

        pub fn get_template_parameters(&self) -> Vec<TemplateParameter> {
            self.template_parameters_.clone()
        }
    }

    macro_rules! define_function_flag_list {
        ($V:ident) => {
            $V(Inline, 0x01)
            $V(V8Inline, 0x03)
            $V(Const, 0x04)
            $V(Constexpr, 0x08)
            $V(Export, 0x10)
            $V(Static, 0x20)
            $V(Override, 0x40)
        };
    }

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum FunctionFlag {
        // This list is populated by the macro above.
        // Each entry should be of the form `name, value`.
        Inline = 0x01,
        V8Inline = 0x03,
        Const = 0x04,
        Constexpr = 0x08,
        Export = 0x10,
        Static = 0x20,
        Override = 0x40,
    }

    #[derive(Debug, Clone)]
    pub struct Function {
        pos_: SourcePosition,
        owning_class_: Option<Class>,
        description_: String,
        name_: String,
        return_type_: String,
        parameters_: Vec<Parameter>,
        flags_: Flags<FunctionFlag>,
    }

    #[derive(Debug, Clone)]
    pub struct Parameter {
        pub type_: String,
        pub name: String,
        pub default_value: String,
    }

    impl Parameter {
        pub fn new(type_: String, name: String, default_value: String) -> Self {
            Parameter {
                type_: type_,
                name: name,
                default_value: default_value,
            }
        }
    }

    impl Function {
        pub fn new(name: String) -> Self {
            Function {
                pos_: CurrentSourcePosition::get(),
                owning_class_: None,
                description_: String::new(),
                name_: name,
                return_type_: String::new(),
                parameters_: Vec::new(),
                flags_: Flags::new(),
            }
        }

        pub fn new_with_class(owning_class: &Class, name: String) -> Self {
            Function {
                pos_: CurrentSourcePosition::get(),
                owning_class_: Some(owning_class.clone()),
                description_: String::new(),
                name_: name,
                return_type_: String::new(),
                parameters_: Vec::new(),
                flags_: Flags::new(),
            }
        }

        pub fn default_getter(return_type: String, owner: &Class, name: String) -> Self {
            let mut getter = Function::new_with_class(owner, name);
            getter.set_return_type(return_type);
            getter.set_inline();
            getter.set_const();
            getter
        }

        pub fn default_setter(
            owner: &Class,
            name: String,
            parameter_type: String,
            parameter_name: String,
        ) -> Self {
            let mut setter = Function::new_with_class(owner, name);
            setter.set_return_type("void".to_string());
            setter.add_parameter(parameter_type, parameter_name, String::new());
            setter.set_inline();
            setter
        }

        pub fn set_flag(&mut self, flag: FunctionFlag, value: bool) {
            if value {
                self.flags_ = self.flags_.union(flag);
            } else {
                self.flags_ = self.flags_.difference(flag);
            }
        }

        pub fn set_flags(&mut self, flags: Flags<FunctionFlag>, value: bool) {
            if value {
                self.flags_ = self.flags_.union_flags(flags);
            } else {
                self.flags_ = self.flags_.difference_flags(flags);
            }
        }

        pub fn has_flag(&self, flag: FunctionFlag) -> bool {
            self.flags_.contains(flag)
        }

        pub fn set_description(&mut self, description: String) {
            self.description_ = description;
        }

        pub fn set_name(&mut self, name: String) {
            self.name_ = name;
        }

        pub fn set_return_type(&mut self, return_type: String) {
            self.return_type_ = return_type;
        }

        pub fn add_parameter(&mut self, type_: String, name: String, default_value: String) {
            self.parameters_.push(Parameter::new(type_, name, default_value));
        }

        pub fn insert_parameter(
            &mut self,
            index: usize,
            type_: String,
            name: String,
            default_value: String,
        ) {
            if index > self.parameters_.len() {
                eprintln!(
                    "index {} out of bounds for parameter vector of length {}",
                    index,
                    self.parameters_.len()
                );
                return;
            }
            self.parameters_.insert(index, Parameter::new(type_, name, default_value));
        }

        pub fn get_parameters(&self) -> Vec<Parameter> {
            self.parameters_.clone()
        }

        pub fn get_parameter_names(&self) -> Vec<String> {
            self.parameters_
                .iter()
                .map(|p| p.name.clone())
                .collect::<Vec<_>>()
        }

        pub const K_AUTOMATIC_INDENTATION: i32 = -1;

        pub fn print_declaration(
            &self,
            stream: &mut dyn Write,
            indentation: i32,
        ) -> std::io::Result<()> {
            let indentation = if indentation == Self::K_AUTOMATIC_INDENTATION {
                if self.owning_class_.is_some() {
                    2
                } else {
                    0
                }
            } else {
                indentation
            };
            self.print_declaration_header(stream, indentation)?;
            writeln!(stream, ";")
        }

        pub fn print_definition(
            &self,
            stream: &mut dyn Write,
            builder: &dyn Fn(&mut dyn Write) -> std::io::Result<()>,
            indentation: i32,
        ) -> std::io::Result<()> {
            self.print_begin_definition(stream, indentation)?;
            builder(stream)?;
            self.print_end_definition(stream, indentation)
        }

        pub fn print_inline_definition(
            &self,
            stream: &mut dyn Write,
            builder: &dyn Fn(&mut dyn Write) -> std::io::Result<()>,
            indentation: i32,
        ) -> std::io::Result<()> {
            self.print_declaration_header(stream, indentation)?;
            writeln!(stream, " {{")?;
            builder(stream)?;
            self.print_end_definition(stream, indentation)
        }

        pub fn print_begin_definition(
            &self,
            stream: &mut dyn Write,
            indentation: i32,
        ) -> std::io::Result<()> {
            writeln!(stream, "{}// {}", " ".repeat(indentation as usize), self.pos_)?;
            let mut scope = String::new();
            if let Some(owning_class) = &self.owning_class_ {
                scope = owning_class.get_name();
                let class_template_parameters = owning_class.get_template_parameters();
                if !class_template_parameters.is_empty() {
                    write!(stream, "{}", " ".repeat(indentation as usize))?;
                    write!(stream, "template<")?;
                    scope += "<";
                    let mut first = true;
                    for p in &class_template_parameters {
                        if !first {
                            write!(stream, ", ")?;
                            scope += ", ";
                        }
                        if p.type_.is_empty() {
                            write!(stream, "class {}", p.name)?;
                        } else {
                            write!(stream, "{} {}", p.type_, p.name)?;
                        }
                        scope += &p.name;
                        first = false;
                    }
                    writeln!(stream, ">")?;
                    scope += ">";
                }
                scope += "::";
            }
            write!(stream, "{} {} {}::{}(", " ".repeat(indentation as usize), self.return_type_, scope, self.name_)?;
            let mut first = true;
            for p in &self.parameters_ {
                if !first {
                    write!(stream, ", ")?;
                }
                write!(stream, "{}", p.type_)?;
                if !p.name.is_empty() {
                    write!(stream, " {}", p.name)?;
                }
                first = false;
            }
            write!(stream, ")")?;
            if self.is_const() {
                write!(stream, " const")?;
            }
            writeln!(stream, " {{")
        }

        pub fn print_end_definition(
            &self,
            stream: &mut dyn Write,
            indentation: i32,
        ) -> std::io::Result<()> {
            writeln!(stream, "{} }}", " ".repeat(indentation as usize))?;
            writeln!(stream)
        }

        fn print_declaration_header(
            &self,
            stream: &mut dyn Write,
            indentation: i32,
        ) -> std::io::Result<()> {
            if !self.description_.is_empty() {
                writeln!(
                    stream,
                    "{}// {}",
                    " ".repeat(indentation as usize),
                    self.description_
                )?;
            }
            writeln!(stream, "{}// {}", " ".repeat(indentation as usize), self.pos_)?;
            write!(stream, "{}", " ".repeat(indentation as usize))?;
            if self.is_export() {
                write!(stream, "V8_EXPORT_PRIVATE ")?;
            }
            if self.is_v8inline() {
                write!(stream, "V8_INLINE ")?;
            } else if self.is_inline() {
                write!(stream, "inline ")?;
            }
            if self.is_static() {
                write!(stream, "static ")?;
            }
            if self.is_constexpr() {
                write!(stream, "constexpr ")?;
            }
            write!(stream, "{} {}(", self.return_type_, self.name_)?;
            let mut first = true;
            for p in &self.parameters_ {
                if !first {
                    write!(stream, ", ")?;
                }
                write!(stream, "{}", p.type_)?;
                if !p.name.is_empty() {
                    write!(stream, " {}", p.name)?;
                }
                if !p.default_value.is_empty() {
                    write!(stream, " = {}", p.default_value)?;
                }
                first = false;
            }
            write!(stream, ")")?;
            if self.is_const() {
                write!(stream, " const")?;
            }
            Ok(())
        }
    }

    macro_rules! generate_accessor {
        ($name:ident, $value:ident) => {
            impl Function {
                pub fn set_$name(&mut self, v: bool) {
                    self.set_flag(FunctionFlag::$name, v);
                }
                pub fn is_$name(&self) -> bool {
                    self.has_flag(FunctionFlag::$name)
                }
            }
        };
    }

    generate_accessor!(Inline, Inline);
    generate_accessor!(V8Inline, V8Inline);
    generate_accessor!(Const, Const);
    generate_accessor!(Constexpr, Constexpr);
    generate_accessor!(Export, Export);
    generate_accessor!(Static, Static);
    generate_accessor!(Override, Override);

    #[derive(Debug, Clone)]
    pub struct File<'a> {
        stream_: &'a mut dyn Write,
        includes_: HashSet<String>,
        namespaces_: VecDeque<String>,
    }

    impl<'a> File<'a> {
        pub fn new(stream: &'a mut dyn Write) -> Self {
            File {
                stream_: stream,
                includes_: HashSet::new(),
                namespaces_: VecDeque::new(),
            }
        }

        pub fn begin_include_guard(&mut self, name: &str) -> std::io::Result<()> {
            writeln!(self.stream_, "#ifndef {}", name)?;
            writeln!(self.stream_, "#define {}\n", name)
        }

        pub fn end_include_guard(&mut self, name: &str) -> std::io::Result<()> {
            writeln!(self.stream_, "#endif // {}\n", name)
        }

        pub fn begin_namespace(&mut self, name: String) -> std::io::Result<()> {
            if name.is_empty() {
                eprintln!("Name must not be empty");
                return Ok(());
            }
            if name.contains(':') {
                eprintln!("Name must not contain ':'");
                return Ok(());
            }
            writeln!(self.stream_, "namespace {} {{", name)?;
            self.namespaces_.push_back(name);
            Ok(())
        }

        pub fn begin_namespace2(&mut self, name0: String, name1: String) -> std::io::Result<()> {
            self.begin_namespace(name0)?;
            self.begin_namespace(name1)
        }

        pub fn end_namespace(&mut self, name: &str) -> std::io::Result<()> {
            if self.namespaces_.is_empty() {
                eprintln!("Namespaces is empty");
                return Ok(());
            }
            if self.namespaces_.back().map_or(false, |top| top != name) {
                eprintln!("Expected namespace {}, but found {}", name, self.namespaces_.back().unwrap());
                return Ok(());
            }
            writeln!(self.stream_, "}} // namespace {}\n", self.namespaces_.back().unwrap())?;
            self.namespaces_.pop_back();
            Ok(())
        }

        pub fn end_namespace2(&mut self, name0: &str, name1: &str) -> std::io::Result<()> {
            self.end_namespace(name1)?;
            self.end_namespace(name0)
        }

        pub fn add_include(&mut self, include: String) {
            self.includes_.insert(include);
        }

        fn s(&mut self) -> &mut dyn Write {
            self.stream_
        }
    }

    pub struct IncludeGuardScope<'a> {
        file_: Option<&'a mut File<'a>>,
        name_: String,
    }

    impl<'a> IncludeGuardScope<'a> {
        pub fn new(file: &'a mut File<'a>, name: String) -> Self {
            file.begin_include_guard(&name).unwrap();
            IncludeGuardScope {
                file_: Some(file),
                name_: name,
            }
        }
    }

    impl<'a> Drop for IncludeGuardScope<'a> {
        fn drop(&mut self) {
            if let Some(file) = self.file_.take() {
                file.end_include_guard(&self.name_).unwrap();
            }
        }
    }

    // Define Flags struct and implementations
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, Default)]
    pub struct Flags<T> {
        bits: u64,
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> Flags<T>
    where
        T: Copy + Clone + Eq + std::hash::Hash + Into<u64>,
    {
        pub fn new() -> Self {
            Flags {
                bits: 0,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn contains(&self, flag: T) -> bool {
            self.bits & flag.into() == flag.into()
        }

        pub fn union(&self, flag: T) -> Self {
            Flags {
                bits: self.bits | flag.into(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn union_flags(&self, other: Flags<T>) -> Self {
            Flags {
                bits: self.bits | other.bits,
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn difference(&self, flag: T) -> Self {
            Flags {
                bits: self.bits & !flag.into(),
                _phantom: std::marker::PhantomData,
            }
        }

        pub fn difference_flags(&self, other: Flags<T>) -> Self {
            Flags {
                bits: self.bits & !other.bits,
                _phantom: std::marker::PhantomData,
            }
        }
    }

    // Implement Into<u64> for FunctionFlag
    impl Into<u64> for FunctionFlag {
        fn into(self) -> u64 {
            self as u64
        }
    }
}
