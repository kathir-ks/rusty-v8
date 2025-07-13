// Converted from V8 C++ source files:
// Header: utils.h
// Implementation: utils.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod torque {
    pub use std::string::String;
    use std::{
        algorithm,
        ffi::OsStr,
        fs::File,
        io::{Read, Write},
        optional::Option,
        os::unix::prelude::OsStrExt,
        path::Path,
        queue::Queue,
        streambuf::Streambuf,
        sync::Mutex,
    };

    use std::{
        collections::HashSet,
        fmt,
        io::{self, ErrorKind},
    };

    use crate::base::hashing;

    use super::SnapshotSpace;

    pub mod ast {
        #[derive(Debug, Clone, PartialEq, Eq, Hash)]
        pub struct Namespace {
            pub name: String,
        }
    }

    pub mod constants {
        pub trait Flag {
            fn as_u32(&self) -> u32;
        }

        pub struct Flags<T: Flag> {
            flags: u32,
        }

        impl<T: Flag> Flags<T> {
            pub fn new() -> Self {
                Flags { flags: 0 }
            }

            pub fn set(&mut self, flag: T) {
                self.flags |= flag.as_u32();
            }

            pub fn clear(&mut self, flag: T) {
                self.flags &= !flag.as_u32();
            }

            pub fn has(&self, flag: T) -> bool {
                self.flags & flag.as_u32() != 0
            }
        }
    }

    pub mod declarable {
        use super::SourcePosition;
        use crate::torque::Scope;
        #[derive(Clone, Copy)]
        pub struct SpecializationRequester {
            pub name: &'static str,
            pub position: Option<SourcePosition>,
            pub scope: *mut Scope,
        }
        impl SpecializationRequester {
            pub fn IsNone(&self) -> bool {
                self.name.is_empty()
            }
        }
    }

    pub mod cfg {
        use std::cell::RefCell;
        use std::rc::Rc;

        use super::Type;

        pub struct Block {}
        impl Block {
            pub fn id(&self) -> usize {
                0
            }
        }
        pub struct StackRange {
            begin_: BottomOffset,
            end_: BottomOffset,
        }

        impl StackRange {
            pub fn new(begin: BottomOffset, end: BottomOffset) -> Self {
                assert!(begin <= end);
                StackRange { begin_: begin, end_: end }
            }
            pub fn begin(&self) -> BottomOffset {
                self.begin_
            }

            pub fn end(&self) -> BottomOffset {
                self.end_
            }
        }
        #[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
        pub struct BottomOffset {
            pub offset: usize,
        }

        impl BottomOffset {
            pub fn from(offset: usize) -> Self {
                BottomOffset { offset }
            }
        }
        impl std::ops::Add<usize> for BottomOffset {
            type Output = Self;

            fn add(self, x: usize) -> Self {
                BottomOffset {
                    offset: self.offset + x,
                }
            }
        }
        impl std::ops::Sub<usize> for BottomOffset {
            type Output = Self;

            fn sub(self, x: usize) -> Self {
                assert!(x <= self.offset);
                BottomOffset {
                    offset: self.offset - x,
                }
            }
        }
        pub struct Stack<T> {
            elements_: Vec<T>,
        }

        impl<T> Stack<T> {
            pub fn new(input_types: &[*const Type]) -> Self {
                let mut elements_ = Vec::new();
                for &ty in input_types {
                    elements_.push(unsafe { &*ty }.clone());
                }
                Stack { elements_ }
            }
            pub fn Size(&self) -> usize {
                self.elements_.len()
            }
        }
    }

    pub mod types {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        pub enum ResidueClass {
            ZeroModOne, // 0 mod 1, i.e. all integers
            Concrete(usize),
        }
        impl ResidueClass {
            pub fn set<T>(&mut self, value: T) {}
        }
        impl Default for ResidueClass {
            fn default() -> Self {
                ResidueClass::ZeroModOne
            }
        }
    }

    use cfg::{BottomOffset, Stack};
    use std::fmt::Write;
    use types::ResidueClass;
    use utils::CurrentScope;
    use utils::CurrentSourcePosition;

    pub mod utils {
        use std::cell::RefCell;
        use std::rc::Rc;
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
        pub struct SourcePosition {
            pub line: usize,
            pub column: usize,
            pub file_name: Option<&'static str>,
        }
        impl SourcePosition {
            pub fn invalid() -> Self {
                SourcePosition {
                    line: 0,
                    column: 0,
                    file_name: None,
                }
            }
        }
        pub struct CurrentSourcePosition {}
        impl CurrentSourcePosition {
            pub fn HasScope() -> bool {
                false
            }

            pub fn Get() -> SourcePosition {
                SourcePosition::default()
            }
        }
        pub struct CurrentScope {}
        impl CurrentScope {
            pub fn HasScope() -> bool {
                false
            }
            pub fn Get() -> *mut Scope {
                std::ptr::null_mut()
            }
        }
    }

    pub struct V8_EXPORT_PRIVATE {}

    pub fn StringLiteralUnquote(s: &str) -> String {
        assert!(
            (s.starts_with('"') && s.ends_with('"'))
                || (s.starts_with('\'') && s.ends_with('\''))
        );
        let mut result = String::new();
        let mut i = 1;
        while i < s.len() - 1 {
            if s.chars().nth(i).unwrap() == '\\' {
                i += 1;
                match s.chars().nth(i).unwrap() {
                    'n' => result.push('\n'),
                    'r' => result.push('\r'),
                    't' => result.push('\t'),
                    '\'' | '"' | '\\' => result.push(s.chars().nth(i).unwrap()),
                    _ => unreachable!(),
                }
            } else {
                result.push(s.chars().nth(i).unwrap());
            }
            i += 1;
        }
        result
    }

    pub fn StringLiteralQuote(s: &str) -> String {
        let mut result = String::from("\"");
        for c in s.chars() {
            match c {
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                '"' | '\\' => {
                    result.push('\\');
                    result.push(c);
                }
                _ => result.push(c),
            }
        }
        result.push('"');
        result
    }

    #[derive(Debug, Clone)]
    pub struct TorqueMessage {
        pub message: String,
        pub position: Option<SourcePosition>,
        pub kind: TorqueMessageKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TorqueMessageKind {
        kError,
        kLint,
    }

    #[macro_export]
    macro_rules! DECLARE_CONTEXTUAL_VARIABLE {
        ($name:ident, $type:ty) => {
            thread_local! {
                static $name: RefCell<Option<$type>> = RefCell::new(None);
            }

            impl $name {
                pub fn get() -> $type {
                    $name.with(|f| {
                        f.borrow().clone().expect(concat!(stringify!($name), " not initialized"))
                    })
                }

                pub fn set(value: $type) {
                    $name.with(|f| {
                        *f.borrow_mut() = Some(value);
                    });
                }

                pub fn clear() {
                    $name.with(|f| {
                        *f.borrow_mut() = None;
                    });
                }
            }
        };
    }
    DECLARE_CONTEXTUAL_VARIABLE!(TorqueMessages, Vec<TorqueMessage>);

    pub fn ToString<T: fmt::Display>(args: T) -> String {
        format!("{}", args)
    }

    pub struct MessageBuilder {
        message_: TorqueMessage,
        extra_messages_: Vec<TorqueMessage>,
    }

    impl MessageBuilder {
        pub fn new(message: String, kind: TorqueMessageKind) -> Self {
            let position = if CurrentSourcePosition::HasScope() {
                Some(CurrentSourcePosition::Get())
            } else {
                None
            };
            let message_ = TorqueMessage {
                message,
                position,
                kind,
            };
            let mut extra_messages_ = Vec::new();
            if CurrentScope::HasScope() {
                let scope = unsafe { &*CurrentScope::Get() };
                let mut current_scope = Some(scope);
                while let Some(scope) = current_scope {
                    let requester =
                        unsafe { std::mem::transmute::<*const Scope, &Scope>(scope) }
                            .GetSpecializationRequester();
                    if !requester.IsNone() {
                        extra_messages_.push(TorqueMessage {
                            message: format!(
                                "Note: in specialization {} requested here",
                                requester.name
                            ),
                            position: requester.position,
                            kind,
                        });
                        current_scope = unsafe {
                            if !requester.scope.is_null() {
                                Some(&*requester.scope)
                            } else {
                                None
                            }
                        };
                    } else {
                        current_scope = unsafe {
                            if !scope.ParentScope().is_null() {
                                Some(&*scope.ParentScope())
                            } else {
                                None
                            }
                        };
                    }
                }
            }

            MessageBuilder {
                message_: message_,
                extra_messages_: extra_messages_,
            }
        }

        pub fn Position(mut self, position: SourcePosition) -> Self {
            self.message_.position = Some(position);
            self
        }

        pub fn Throw(&self) -> ! {
            panic!("TorqueAbortCompilation: {}", self.message_.message);
        }

        fn Report(&self) {
            TorqueMessages::with(|messages| {
                let mut messages = messages.borrow_mut();
                messages.as_mut().unwrap().push(self.message_.clone());
                for message in &self.extra_messages_ {
                    messages.as_mut().unwrap().push(message.clone());
                }
            });
        }
    }

    impl Drop for MessageBuilder {
        fn drop(&mut self) {
            self.Report();
        }
    }

    #[derive(Debug)]
    pub struct TorqueAbortCompilation {}

    impl fmt::Display for TorqueAbortCompilation {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Torque Abort Compilation")
        }
    }

    impl std::error::Error for TorqueAbortCompilation {}

    pub fn Message<T: fmt::Display>(kind: TorqueMessageKind, args: T) -> MessageBuilder {
        MessageBuilder::new(ToString(args), kind)
    }

    pub fn Error<T: fmt::Display>(args: T) -> MessageBuilder {
        Message(TorqueMessageKind::kError, args)
    }

    pub fn Lint<T: fmt::Display>(args: T) -> MessageBuilder {
        Message(TorqueMessageKind::kLint, args)
    }

    fn ContainsUnderscore(s: &str) -> bool {
        !s.is_empty() && s.contains("_")
    }

    fn ContainsUpperCase(s: &str) -> bool {
        !s.is_empty() && s.chars().any(|c| c.is_uppercase())
    }

    fn IsKeywordLikeName(s: &str) -> bool {
        const KEYWORD_LIKE_CONSTANTS: [&str; 6] =
            ["True", "False", "TheHole", "PromiseHole", "Null", "Undefined"];
        KEYWORD_LIKE_CONSTANTS.contains(&s)
    }

    fn IsMachineType(s: &str) -> bool {
        const MACHINE_TYPES: [&str; 20] = [
            "void",
            "never",
            "int8",
            "uint8",
            "int16",
            "uint16",
            "int31",
            "uint31",
            "int32",
            "uint32",
            "int64",
            "uint64",
            "intptr",
            "uintptr",
            "float16rawbits",
            "float32",
            "float64",
            "float64orundefinedorhole",
            "bool",
            "string",
        ];
        MACHINE_TYPES.contains(&s.to_lowercase().as_str())
    }

    pub fn IsLowerCamelCase(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let start = if s.starts_with('_') { 1 } else { 0 };
        s[start..]
            .chars()
            .next()
            .map_or(false, |c| c.is_lowercase() && !ContainsUnderscore(&s[start..]))
    }

    pub fn IsUpperCamelCase(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let start = if s.starts_with('_') { 1 } else { 0 };
        s[start..].chars().next().map_or(false, |c| c.is_uppercase())
    }

    pub fn IsSnakeCase(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        !ContainsUpperCase(s)
    }

    pub fn IsValidNamespaceConstName(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if IsKeywordLikeName(s) {
            return true;
        }
        s.starts_with('k') && IsUpperCamelCase(&s[1..])
    }

    pub fn IsValidTypeName(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        if IsMachineType(s) {
            return true;
        }
        IsUpperCamelCase(s)
    }

    pub fn ReportError<T: fmt::Display>(args: T) -> ! {
        Error(args).Throw();
    }

    pub fn CapifyStringWithUnderscores(camellified_string: &str) -> String {
        let js_position = camellified_string.find("JS");
        let mut result = String::new();
        let mut previous_was_lower_or_digit = false;
        for (index, current) in camellified_string.chars().enumerate() {
            if (previous_was_lower_or_digit && current.is_uppercase())
                || (js_position.is_some() && index == js_position.unwrap() + 2)
            {
                result.push('_');
            }
            if current == '.' || current == '-' {
                result.push('_');
                previous_was_lower_or_digit = false;
                continue;
            }
            result.push(current.to_uppercase().next().unwrap());
            previous_was_lower_or_digit = current.is_lowercase() || current.is_digit(10);
        }
        result
    }

    pub fn CamelifyString(underscore_string: &str) -> String {
        let mut result = String::new();
        let mut word_beginning = true;
        for current in underscore_string.chars() {
            if current == '_' || current == '-' {
                word_beginning = true;
                continue;
            }
            if word_beginning {
                result.push(current.to_uppercase().next().unwrap());
            } else {
                result.push(current);
            }
            word_beginning = false;
        }
        result
    }

    pub fn SnakeifyString(camel_string: &str) -> String {
        let mut result = String::new();
        let mut previous_was_lower = false;
        for current in camel_string.chars() {
            if previous_was_lower && current.is_uppercase() {
                result.push('_');
            }
            result.push(current.to_lowercase().next().unwrap());
            previous_was_lower = current.is_lowercase();
        }
        result
    }

    pub fn DashifyString(underscore_string: &str) -> String {
        underscore_string.replace('_', "-")
    }

    pub fn UnderlinifyPath(path: String) -> String {
        let mut result = path;
        result.replace_range(.., &result.replace('-', "_"));
        result.replace_range(.., &result.replace('/', "_"));
        result.replace_range(.., &result.replace('\\', "_"));
        result.replace_range(.., &result.replace('.', "_"));
        result.chars().map(|c| c.to_uppercase().next().unwrap()).collect()
    }

    pub fn StartsWithSingleUnderscore(str: &str) -> bool {
        str.len() >= 2 && str.starts_with('_') && str.chars().nth(1) != Some('_')
    }

    pub fn ReplaceFileContentsIfDifferent(file_path: &str, contents: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        let old_contents = match File::open(path) {
            Ok(mut file) => {
                let mut s = String::new();
                file.read_to_string(&mut s)?;
                s
            }
            Err(e) => {
                if e.kind() == ErrorKind::NotFound {
                    String::new()
                } else {
                    return Err(e);
                }
            }
        };

        if old_contents != contents {
            let mut new_contents_stream = File::create(path)?;
            new_contents_stream.write_all(contents.as_bytes())?;
        }
        Ok(())
    }

    pub struct Deduplicator<T> {
        storage_: HashSet<T, hashing::DefaultHasher<T>>,
    }

    impl<T: Eq + std::hash::Hash + Clone> Deduplicator<T> {
        pub fn new() -> Self {
            Deduplicator {
                storage_: HashSet::new(),
            }
        }
        pub fn Add(&mut self, x: T) -> &T {
            if self.storage_.contains(&x) {
                self.storage_.get(&x).unwrap()
            } else {
                self.storage_.insert(x.clone());
                self.storage_.get(&x).unwrap()
            }
        }
    }
    pub fn DereferenceIfPointer<T>(x: T) -> T {
        x
    }
    pub struct ListPrintAdaptor<'a, T, L> {
        pub list: &'a Vec<T>,
        pub separator: &'a str,
        pub transformer: L,
    }

    impl<'a, T, L> fmt::Display for ListPrintAdaptor<'a, T, L>
    where
        L: Fn(&T) -> String,
    {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let mut first = true;
            for e in self.list {
                if first {
                    first = false;
                } else {
                    write!(f, "{}", self.separator)?;
                }
                write!(f, "{}", (self.transformer)(e))?;
            }
            Ok(())
        }
    }

    pub fn PrintList<T>(list: &Vec<T>, separator: &str) -> ListPrintAdaptor<T, fn(&T) -> String>
    where
        T: fmt::Display,
    {
        let id = |el: &T| -> String { format!("{}", el) };
        ListPrintAdaptor {
            list: list,
            separator: separator,
            transformer: id,
        }
    }

    pub fn PrintListWithTransform<T, L>(
        list: &Vec<T>,
        separator: &str,
        transformer: L,
    ) -> ListPrintAdaptor<T, L>
    where
        L: Fn(&T) -> String,
    {
        ListPrintAdaptor {
            list: list,
            separator: separator,
            transformer: transformer,
        }
    }

    pub fn PrintCommaSeparatedList<T, C>(os: &mut String, list: &Vec<T>, transform: C)
    where
        C: Fn(&T) -> String,
    {
        os.push_str(&PrintListWithTransform(list, ", ", transform).to_string());
    }

    pub fn PrintCommaSeparatedListSimple<T>(os: &mut String, list: &Vec<T>)
    where
        T: fmt::Display,
    {
        os.push_str(&PrintList(list, ", ").to_string());
    }

    impl BottomOffset {
        pub fn new(offset: usize) -> Self {
            BottomOffset { offset }
        }
    }

    impl fmt::Display for BottomOffset {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "BottomOffset{{{}}}", self.offset)
        }
    }

    impl StackRange {
        pub fn Size(&self) -> usize {
            self.end_.offset - self.begin_.offset
        }
    }

    impl fmt::Display for StackRange {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "StackRange{{{}, {}}}", self.begin_, self.end_)
        }
    }

    impl<T> Stack<T> {
        pub fn Peek(&self, from_bottom: BottomOffset) -> &T {
            &self.elements_[from_bottom.offset]
        }

        pub fn Poke(&mut self, from_bottom: BottomOffset, x: T) {
            self.elements_[from_bottom.offset] = x;
        }

        pub fn Push(&mut self, x: T) {
            self.elements_.push(x);
        }

        pub fn TopRange(&self, slot_count: usize) -> cfg::StackRange {
            assert!(self.Size() >= slot_count);
            cfg::StackRange {
                begin_: self.AboveTop() - slot_count,
                end_: self.AboveTop(),
            }
        }

        pub fn PushMany(&mut self, v: &Vec<T>) -> cfg::StackRange {
            for x in v {
                self.Push(x.clone());
            }
            self.TopRange(v.len())
        }

        pub fn Top(&self) -> &T {
            self.Peek(self.AboveTop() - 1)
        }

        pub fn Pop(&mut self) -> T {
            self.elements_.pop().unwrap()
        }

        pub fn PopMany(&mut self, count: usize) -> Vec<T> {
            assert!(self.elements_.len() >= count);
            let mut result = Vec::with_capacity(count);
            for _ in 0..count {
                result.push(self.elements_.pop().unwrap());
            }
            result.reverse();
            result
        }

        pub fn AboveTop(&self) -> BottomOffset {
            BottomOffset {
                offset: self.elements_.len(),
            }
        }

        pub fn DeleteRange(&mut self, range: cfg::StackRange) {
            if range.Size() == 0 {
                return;
            }
            for i in range.end_.offset..self.AboveTop().offset {
                self.elements_[i - range.Size()] = self.elements_[i].clone();
            }
            self.elements_.resize(self.elements_.len() - range.Size());
        }
    }

    impl<T: PartialEq> PartialEq for Stack<T> {
        fn eq(&self, other: &Self) -> bool {
            self.elements_ == other.elements_
        }
    }

    impl<T: fmt::Display> fmt::Display for Stack<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Stack{{")?;
            let mut s = String::new();
            PrintCommaSeparatedListSimple(&mut s, &self.elements_);
            write!(f, "{}}}", s)
        }
    }

    pub fn CheckNotNull<T>(x: *mut T) -> *mut T {
        assert!(!x.is_null());
        x
    }

    pub const kBaseNamespaceName: &str = "base";
    pub const kTestNamespaceName: &str = "test";

    pub fn EraseIf<C, F, T>(container: &mut C, f: F)
    where
        C: std::ops::DerefMut<Target = Vec<T>>,
        F: Fn(&T) -> bool,
    {
        container.retain(|x| !f(x));
    }

    pub struct NullStreambuf {}

    impl NullStreambuf {
        pub fn new() -> Self {
            NullStreambuf {}
        }
    }

    impl Write for NullStreambuf {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            Ok(buf.len())
        }

        fn flush(&mut self) -> io::Result<()> {
            Ok(())
        }
    }

    pub struct NullOStream {
        buffer_: NullStreambuf,
    }

    impl NullOStream {
        pub fn new() -> Self {
            NullOStream {
                buffer_: NullStreambuf::new(),
            }
        }
    }

    impl Write for NullOStream {
        fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
            self.buffer_.write(buf)
        }

        fn flush(&mut self) -> io::Result<()> {
            self.buffer_.flush()
        }
    }

    pub fn StringStartsWith(s: &str, prefix: &str) -> bool {
        if s.len() < prefix.len() {
            return false;
        }
        s.starts_with(prefix)
    }

    pub fn StringEndsWith(s: &str, suffix: &str) -> bool {
        if s.len() < suffix.len() {
            return false;
        }
        s.ends_with(suffix)
    }

    pub struct IfDefScope<'a> {
        os_: &'a mut dyn Write,
        d_: String,
    }

    impl<'a> IfDefScope<'a> {
        pub fn new(os: &'a mut dyn Write, d: String) -> Self {
            writeln!(os, "#ifdef {}", d).unwrap();
            IfDefScope { os_: os, d_: d }
        }
    }

    impl<'a> Drop for IfDefScope<'a> {
        fn drop(&mut self) {
            writeln!(self.os_, "#endif  // {}", self.d_).unwrap();
        }
    }

    pub struct NamespaceScope<'a> {
        os_: &'a mut dyn Write,
        d_: Vec<String>,
    }

    impl<'a> NamespaceScope<'a> {
        pub fn new(os: &'a mut dyn Write, namespaces: std::vec::Vec<String>) -> Self {
            for s in &namespaces {
                writeln!(os, "namespace {} {{", s).unwrap();
            }
            NamespaceScope { os_: os, d_: namespaces }
        }
    }

    impl<'a> Drop for NamespaceScope<'a> {
        fn drop(&mut self) {
            for i in self.d_.iter().rev() {
                writeln!(self.os_, "}}  // namespace {}", i).unwrap();
            }
        }
    }

    pub struct IncludeGuardScope<'a> {
        os_: &'a mut dyn Write,
        d_: String,
    }

    impl<'a> IncludeGuardScope<'a> {
        pub fn new(os: &'a mut dyn Write, file_name: String) -> Self {
            let d = format!(
                "V8_GEN_TORQUE_GENERATED_{}_",
                CapifyStringWithUnderscores(&file_name)
            );
            writeln!(os, "#ifndef {}", d).unwrap();
            writeln!(os, "#define {}\n", d).unwrap();
            IncludeGuardScope { os_: os, d_: d }
        }
    }

    impl<'a> Drop for IncludeGuardScope<'a> {
        fn drop(&mut self) {
            writeln!(self.os_, "#endif  // {}", self.d_).unwrap();
        }
    }

    pub struct IncludeObjectMacrosScope<'a> {
        os_: &'a mut dyn Write,
    }

    impl<'a> IncludeObjectMacrosScope<'a> {
        pub fn new(os: &'a mut dyn Write) -> Self {
            writeln!(
                os,
                "\n// Has to be the last include (doesn't have include guards):\n\
                 #include \"src/objects/object-macros.h\"\n"
            )
            .unwrap();
            IncludeObjectMacrosScope { os_: os }
        }
    }

    impl<'a> Drop for IncludeObjectMacrosScope<'a> {
        fn drop(&mut self) {
            writeln!(self.os_, "\n#include \"src/objects/object-macros-undef.h\"\n").unwrap();
        }
    }

    impl ResidueClass {
        pub fn new(value: usize, modulus_log_2: usize) -> Self {
            let kMaxModulusLog2 = 8 * std::mem::size_of::<usize>();
            let modulus_log_2_ = std::cmp::min(modulus_log_2, kMaxModulusLog2);
            let value_ = if modulus_log_2_ < kMaxModulusLog2 {
                value % (1 << modulus_log_2_)
            } else {
                value
            };
            ResidueClass::Concrete(value_)
        }

        pub fn Unknown() -> Self {
            ResidueClass::Concrete(0)
        }

        pub fn SingleValue(&self) -> Option<usize> {
            match self {
                ResidueClass::Concrete(value) => Some(*value),
                _ => None,
            }
        }
    }

    impl std::ops::Add for ResidueClass {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            match (self, other) {
                (ResidueClass::Concrete(a), ResidueClass::Concrete(b)) => {
                    ResidueClass::Concrete(a + b)
                }
            }
        }
    }

    impl std::ops::Mul for ResidueClass {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            match (self, other) {
                (ResidueClass::Concrete(a), ResidueClass::Concrete(b)) => {
                    ResidueClass::Concrete(a * b)
                }
            }
        }
    }

    impl fmt::Display for ResidueClass {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            match self {
                ResidueClass::Concrete(value) => write!(f, "{}", value),
            }
        }
    }

    impl std::ops::AddAssign for ResidueClass {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }

    impl std::ops::MulAssign for ResidueClass {
        fn mul_assign(&mut self, other: Self) {
            *self = *self * other;
        }
    }

    impl ResidueClass {
        pub fn AlignmentLog2(&self) -> usize {
            match self {
                ResidueClass::Concrete(value) => {
                    if *value == 0 {
                        8 * std::mem::size_of::<usize>()
                    } else {
                        value.trailing_zeros() as usize
                    }
                }
            }
        }

        pub fn Alignment(&self) -> usize {
            1 << self.AlignmentLog2()
        }
    }

    pub struct Worklist<T> {
        queue_: std::collections::VecDeque<T>,
        contained_: HashSet<T>,
    }

    impl<T: Eq + std::hash::Hash + Clone> Worklist<T> {
        pub fn new() -> Self {
            Worklist {
                queue_: std::collections::VecDeque::new(),
                contained_: HashSet::new(),
            }
        }
        pub fn IsEmpty(&self) -> bool {
            assert_eq!(self.queue_.len(), self.contained_.len());
            self.queue_.is_empty()
        }

        pub fn Enqueue(&mut self, value: T) -> bool {
            if self.contained_.contains(&value) {
                return false;
            }
            self.queue_.push_back(value.clone());
            self.contained_.insert(value);
            assert_eq!(self.queue_.len(), self.contained_.len());
            true
        }


