// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod utils {
    use std::cmp;
    use std::collections::{HashSet, VecDeque};
    use std::fmt;
    use std::fs;
    use std::io::{self, Read, Write};
    use std::num::ParseIntError;
    use std::ops::{Add, AddAssign, Deref, Mul, MulAssign, Sub};
    use std::path::Path;
    use std::string::FromUtf8Error;

    use url::ParseError;

    pub fn string_literal_unquote(s: &str) -> String {
        s.replace("\\\"", "\"").replace("\\\\", "\\")
    }

    pub fn string_literal_quote(s: &str) -> String {
        s.replace("\\", "\\\\").replace("\"", "\\\"")
    }

    // Decodes "file://" URIs into file paths which can then be used
    // with the standard stream API.
    pub fn file_uri_decode(s: &str) -> Option<String> {
        if s.starts_with("file://") {
            // Handle "file:///path/to/file"
            let path = &s[7..];
            if cfg!(windows) {
                // Handle "file:///C:/path/to/file"
                if path.starts_with('/') && path[1..2].contains(":") {
                    Some(path[1..].to_string())
                } else {
                    Some(path.to_string())
                }
            } else {
                Some(path.to_string())
            }
        } else {
            None
        }
    }

    #[derive(Debug, Clone)]
    pub struct TorqueMessage {
        pub message: String,
        pub position: Option<SourcePosition>,
        pub kind: TorqueMessageKind,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum TorqueMessageKind {
        Error,
        Lint,
    }

    thread_local! {
        static TORQUE_MESSAGES: std::cell::RefCell<Vec<TorqueMessage>> = std::cell::RefCell::new(Vec::new());
    }

    pub fn with_torque_messages<F, R>(f: F) -> R
    where
        F: FnOnce(&mut Vec<TorqueMessage>) -> R,
    {
        TORQUE_MESSAGES.with(|messages| f(&mut messages.borrow_mut()))
    }

    pub fn to_string<T: fmt::Display>(args: T) -> String {
        format!("{}", args)
    }

    pub struct MessageBuilder {
        message_: TorqueMessage,
        extra_messages_: Vec<TorqueMessage>,
    }

    impl MessageBuilder {
        pub fn new(message: String, kind: TorqueMessageKind) -> Self {
            MessageBuilder {
                message_: TorqueMessage {
                    message,
                    position: None,
                    kind,
                },
                extra_messages_: Vec::new(),
            }
        }

        pub fn position(mut self, position: SourcePosition) -> Self {
            self.message_.position = Some(position);
            self
        }

        #[allow(unreachable_code)]
        #[allow(clippy::diverging_sub_expression)]
        pub fn throw(&self) -> ! {
            self.report();
            panic!("TorqueAbortCompilation");
        }

        fn report(&self) {
            with_torque_messages(|messages| messages.push(self.message_.clone()));
        }
    }

    impl Drop for MessageBuilder {
        fn drop(&mut self) {
            self.report();
        }
    }

    // Used for throwing exceptions. Retrieve TorqueMessage from the contextual
    // for specific error information.
    #[derive(Debug)]
    pub struct TorqueAbortCompilation;

    pub fn message<T: fmt::Display>(kind: TorqueMessageKind, args: T) -> MessageBuilder {
        MessageBuilder::new(to_string(args), kind)
    }

    pub fn error<T: fmt::Display>(args: T) -> MessageBuilder {
        message(TorqueMessageKind::Error, args)
    }

    pub fn lint<T: fmt::Display>(args: T) -> MessageBuilder {
        message(TorqueMessageKind::Lint, args)
    }

    pub fn is_lower_camel_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        first_char.is_lowercase()
            && chars.all(|c| c.is_alphanumeric())
            && s.chars().any(|c| c.is_alphabetic())
    }

    pub fn is_upper_camel_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        let mut chars = s.chars();
        let first_char = chars.next().unwrap();
        first_char.is_uppercase()
            && chars.all(|c| c.is_alphanumeric())
            && s.chars().any(|c| c.is_alphabetic())
    }

    pub fn is_snake_case(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars()
            .all(|c| c.is_lowercase() || c == '_' || c.is_numeric())
            && s.contains('_')
            && !s.starts_with('_')
            && !s.ends_with('_')
            && !s.contains("__")
    }

    pub fn is_valid_namespace_const_name(s: &str) -> bool {
        if s.is_empty() {
            return false;
        }
        s.chars().all(|c| c.is_uppercase() || c == '_' || c.is_numeric())
            && !s.starts_with('_')
            && !s.ends_with('_')
            && !s.contains("__")
    }

    pub fn is_valid_type_name(s: &str) -> bool {
        is_upper_camel_case(s)
    }

    #[allow(unreachable_code)]
    #[allow(clippy::diverging_sub_expression)]
    pub fn report_error<T: fmt::Display>(args: T) -> ! {
        error(args).throw();
    }

    pub fn capify_string_with_underscores(camellified_string: &str) -> String {
        let mut result = String::new();
        for (i, c) in camellified_string.chars().enumerate() {
            if i > 0 && c.is_uppercase() {
                result.push('_');
            }
            result.push(c.to_ascii_uppercase());
        }
        result
    }

    pub fn camelify_string(underscore_string: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = false;
        for c in underscore_string.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        result
    }

    pub fn snakeify_string(camel_string: &str) -> String {
        let mut result = String::new();
        for (i, c) in camel_string.chars().enumerate() {
            if i > 0 && c.is_uppercase() {
                result.push('_');
            }
            result.push(c.to_ascii_lowercase());
        }
        result
    }

    pub fn dashify_string(underscore_string: &str) -> String {
        underscore_string.replace('_', "-")
    }

    pub fn underlinify_path(path: String) -> String {
        path.replace('/', "_")
    }

    pub fn starts_with_single_underscore(s: &str) -> bool {
        s.starts_with('_') && !s[1..].starts_with('_')
    }

    pub fn replace_file_contents_if_different(file_path: &str, contents: &str) -> io::Result<()> {
        let path = Path::new(file_path);
        if path.exists() {
            let mut current_contents = String::new();
            fs::File::open(path)?.read_to_string(&mut current_contents)?;
            if current_contents == contents {
                return Ok(());
            }
        }
        fs::write(path, contents)
    }

    pub struct Deduplicator<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        storage_: HashSet<T>,
    }

    impl<T> Deduplicator<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        pub fn new() -> Self {
            Deduplicator {
                storage_: HashSet::new(),
            }
        }

        pub fn add(&mut self, x: T) -> &T {
            if self.storage_.contains(&x) {
                self.storage_.get(&x).unwrap()
            } else {
                self.storage_.insert(x.clone());
                self.storage_.get(&x).unwrap()
            }
        }
    }

    pub fn dereference_if_pointer<T>(x: &T) -> &T {
        x
    }

    pub struct ListPrintAdaptor<'a, T, L>
    where
        L: Fn(&T) -> &dyn fmt::Display,
    {
        list: &'a [T],
        separator: String,
        transformer: L,
    }

    impl<'a, T, L> fmt::Display for ListPrintAdaptor<'a, T, L>
    where
        L: Fn(&T) -> &dyn fmt::Display,
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

    pub fn print_list<T>(list: &[T], separator: &str) -> ListPrintAdaptor<'_, T, impl Fn(&T) -> &dyn fmt::Display>
    where
        T: fmt::Display,
    {
        let id = |el: &T| el as &dyn fmt::Display;
        ListPrintAdaptor {
            list,
            separator: separator.to_string(),
            transformer: id,
        }
    }

    pub fn print_list_with_transformer<T, L>(
        list: &[T],
        separator: &str,
        transformer: L,
    ) -> ListPrintAdaptor<'_, T, L>
    where
        L: Fn(&T) -> &dyn fmt::Display,
    {
        ListPrintAdaptor {
            list,
            separator: separator.to_string(),
            transformer,
        }
    }

    pub fn print_comma_separated_list<T, C>(os: &mut String, list: &[T], transform: C)
    where
        C: Fn(&T) -> &dyn fmt::Display,
    {
        use std::fmt::Write;
        write!(os, "{}", print_list_with_transformer(list, ", ", transform)).unwrap();
    }

    pub fn print_comma_separated_list_simple<T>(os: &mut String, list: &[T])
    where
        T: fmt::Display,
    {
        use std::fmt::Write;
        write!(os, "{}", print_list(list, ", ")).unwrap();
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
    pub struct BottomOffset {
        pub offset: usize,
    }

    impl BottomOffset {
        pub fn new(offset: usize) -> Self {
            BottomOffset { offset }
        }
    }

    impl Add<usize> for BottomOffset {
        type Output = Self;

        fn add(self, x: usize) -> Self {
            BottomOffset { offset: self.offset + x }
        }
    }

    impl Sub<usize> for BottomOffset {
        type Output = Self;

        fn sub(self, x: usize) -> Self {
            assert!(x <= self.offset);
            BottomOffset { offset: self.offset - x }
        }
    }

    impl fmt::Display for BottomOffset {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "BottomOffset{{{}}}", self.offset)
        }
    }

    // An iterator-style range of stack slots.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct StackRange {
        begin_: BottomOffset,
        end_: BottomOffset,
    }

    impl StackRange {
        pub fn new(begin: BottomOffset, end: BottomOffset) -> Self {
            assert!(begin <= end);
            StackRange { begin_: begin, end_: end }
        }

        pub fn extend(&mut self, adjacent: StackRange) {
            assert_eq!(self.end_, adjacent.begin_);
            self.end_ = adjacent.end_;
        }

        pub fn size(&self) -> usize {
            self.end_.offset - self.begin_.offset
        }

        pub fn begin(&self) -> BottomOffset {
            self.begin_
        }

        pub fn end(&self) -> BottomOffset {
            self.end_
        }
    }

    impl fmt::Display for StackRange {
        fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
            write!(out, "StackRange{{{}, {}}}", self.begin(), self.end())
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct Stack<T> {
        elements_: Vec<T>,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Stack { elements_: Vec::new() }
        }
        pub fn from_vec(v: Vec<T>) -> Self {
            Stack { elements_: v }
        }
        pub fn size(&self) -> usize {
            self.elements_.len()
        }
        pub fn peek(&self, from_bottom: BottomOffset) -> &T {
            &self.elements_[from_bottom.offset]
        }
        pub fn poke(&mut self, from_bottom: BottomOffset, x: T) {
            self.elements_[from_bottom.offset] = x;
        }
        pub fn push(&mut self, x: T) {
            self.elements_.push(x);
        }
        pub fn top_range(&self, slot_count: usize) -> StackRange {
            assert!(self.size() >= slot_count);
            StackRange::new(self.above_top() - slot_count, self.above_top())
        }
        pub fn push_many(&mut self, v: &[T]) -> StackRange {
            for x in v {
                self.push(x.clone());
            }
            self.top_range(v.len())
        }
        pub fn top(&self) -> &T {
            self.peek(self.above_top() - 1)
        }
        pub fn pop(&mut self) -> T {
            self.elements_.pop().unwrap()
        }
        pub fn pop_many(&mut self, count: usize) -> Vec<T> {
            assert!(self.elements_.len() >= count);
            let mut result = self.elements_.split_off(self.elements_.len() - count);
            result.reverse();
            result
        }
        // The invalid offset above the top element. This is useful for StackRange.
        pub fn above_top(&self) -> BottomOffset {
            BottomOffset {
                offset: self.size(),
            }
        }
        // Delete the slots in {range}, moving higher slots to fill the gap.
        pub fn delete_range(&mut self, range: StackRange) {
            assert!(range.end() <= self.above_top());
            if range.size() == 0 {
                return;
            }
            for i in range.end().offset..self.above_top().offset {
                self.elements_.swap(i, i - range.size());
            }
            self.elements_.truncate(self.elements_.len() - range.size());
        }
        pub fn begin(&mut self) -> *mut T {
            self.elements_.as_mut_ptr()
        }
        pub fn end(&mut self) -> *mut T {
            unsafe { self.begin().add(self.elements_.len()) }
        }
        pub fn begin_const(&self) -> *const T {
            self.elements_.as_ptr()
        }
        pub fn end_const(&self) -> *const T {
            unsafe { self.begin_const().add(self.elements_.len()) }
        }
    }

    impl<T: fmt::Display> fmt::Display for Stack<T> {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            let mut s = String::new();
            print_comma_separated_list_simple(&mut s, &self.elements_);
            write!(f, "Stack{{{}}}", s)
        }
    }

    pub fn check_not_null<T>(x: *mut T) -> *mut T {
        assert!(!x.is_null());
        x
    }

    pub const K_BASE_NAMESPACE_NAME: &str = "base";
    pub const K_TEST_NAMESPACE_NAME: &str = "test";

    // Erase elements of a container that has a constant-time erase function, like
    // std::set or std::list. Calling this on std::vector would have quadratic
    // complexity.
    pub fn erase_if<Container, F, T>(container: &mut Container, mut f: F)
    where
        Container: Extend<T> + IntoIterator<Item = T> + FromIterator<T>,
        F: FnMut(&T) -> bool,
        T: Clone,
    {
        let mut new_container = Vec::new();
        for item in container.clone().into_iter() {
            if !f(&item) {
                new_container.push(item.clone());
            }
        }
        *container = new_container.into_iter().collect();
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

    pub fn string_starts_with(s: &str, prefix: &str) -> bool {
        s.starts_with(prefix)
    }

    pub fn string_ends_with(s: &str, suffix: &str) -> bool {
        s.ends_with(suffix)
    }

    //V8_NODISCARD
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

    //V8_NODISCARD
    pub struct NamespaceScope<'a> {
        os_: &'a mut dyn Write,
        d_: Vec<String>,
    }

    impl<'a> NamespaceScope<'a> {
        pub fn new(os: &'a mut dyn Write, namespaces: &[String]) -> Self {
            for ns in namespaces {
                writeln!(os, "namespace {} {{", ns).unwrap();
            }
            NamespaceScope {
                os_: os,
                d_: namespaces.to_vec(),
            }
        }
    }

    impl<'a> Drop for NamespaceScope<'a> {
        fn drop(&mut self) {
            for ns in self.d_.iter().rev() {
                writeln!(self.os_, "}}  // namespace {}", ns).unwrap();
            }
        }
    }

    //V8_NODISCARD
    pub struct IncludeGuardScope<'a> {
        os_: &'a mut dyn Write,
        d_: String,
    }

    impl<'a> IncludeGuardScope<'a> {
        pub fn new(os: &'a mut dyn Write, file_name: String) -> Self {
            let guard_name = file_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_").to_uppercase() + "_H_";
            writeln!(os, "#ifndef {}", guard_name).unwrap();
            writeln!(os, "#define {}", guard_name).unwrap();
            IncludeGuardScope { os_: os, d_: guard_name }
        }
    }

    impl<'a> Drop for IncludeGuardScope<'a> {
        fn drop(&mut self) {
            writeln!(self.os_, "#endif  // {}", self.d_).unwrap();
        }
    }

    //V8_NODISCARD
    pub struct IncludeObjectMacrosScope<'a> {
        os_: &'a mut dyn Write,
    }

    impl<'a> IncludeObjectMacrosScope<'a> {
        pub fn new(os: &'a mut dyn Write) -> Self {
            writeln!(os, "#include \"src/objects/object-macros.h\"").unwrap();
            IncludeObjectMacrosScope { os_: os }
        }
    }

    impl<'a> Drop for IncludeObjectMacrosScope<'a> {
        fn drop(&mut self) {
            writeln!(self.os_, "#include \"src/objects/object-macros-undef.h\"").unwrap();
        }
    }

    // A value of ResidueClass is a congruence class of integers modulo a power
    // of 2.
    // In contrast to common modulo arithmetic, we also allow addition and
    // multiplication of congruence classes with different modulus. In this case, we
    // do an abstract-interpretation style approximation to produce an as small as
    // possible congruence class. ResidueClass is used to represent partial
    // knowledge about offsets and sizes to validate alignment constraints.
    // ResidueClass(x,m) = {y \in Z | x == y mod 2^m} = {x+k2^m | k \in Z} where Z
    // is the set of all integers.
    // Notation: 2^x is 2 to the power of x.
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct ResidueClass {
        // The value is the representative of the congruence class. It's always
        // smaller than 2^modulus_log_2_.
        value_: usize,
        // Base 2 logarithm of the modulus.
        modulus_log_2_: usize,
    }

    impl ResidueClass {
        pub const K_MAX_MODULUS_LOG2: usize = 8 * std::mem::size_of::<usize>();

        pub fn new(value: usize, modulus_log_2: usize) -> Self {
            let modulus_log_2 = cmp::min(modulus_log_2, Self::K_MAX_MODULUS_LOG2);
            let value_ = if modulus_log_2 < Self::K_MAX_MODULUS_LOG2 {
                value % (1usize << modulus_log_2)
            } else {
                value
            };
            ResidueClass {
                value_: value_,
                modulus_log_2_: modulus_log_2,
            }
        }

        // 0 modulo 1, in other words, the class of all integers.
        pub fn unknown() -> Self {
            ResidueClass::new(0, 0)
        }

        // If the modulus corresponds to the size of usize, it represents a concrete
        // value.
        pub fn single_value(&self) -> Option<usize> {
            if self.modulus_log_2_ == Self::K_MAX_MODULUS_LOG2 {
                Some(self.value_)
            } else {
                None
            }
        }

        // 2^AlignmentLog2() is the larget power of 2 that divides all elements of the
        // congruence class.
        pub fn alignment_log2(&self) -> usize {
            let concrete = self.single_value();
            if concrete.is_some() {
                return concrete.unwrap().trailing_zeros() as usize;
            }
            let mut alignment_log2 = 0;
            let mut v = self.value_;
            while v & 1 == 0 && alignment_log2 < self.modulus_log_2_ {
                alignment_log2 += 1;
                v >>= 1;
            }
            alignment_log2
        }
        pub fn alignment(&self) -> usize {
            assert!(self.alignment_log2() < Self::K_MAX_MODULUS_LOG2);
            1usize << self.alignment_log2()
        }
    }

    impl Add for ResidueClass {
        type Output = Self;

        fn add(self, other: Self) -> Self {
            ResidueClass::new(
                self.value_ + other.value_,
                cmp::min(self.modulus_log_2_, other.modulus_log_2_),
            )
        }
    }

    impl Mul for ResidueClass {
        type Output = Self;

        fn mul(self, other: Self) -> Self {
            ResidueClass::new(
                self.value_ * other.value_,
                cmp::min(
                    self.modulus_log_2_ + other.alignment_log2(),
                    other.modulus_log_2_ + self.alignment_log2(),
                ),
            )
        }
    }

    impl AddAssign for ResidueClass {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }

    impl MulAssign for ResidueClass {
        fn mul_assign(&mut self, other: Self) {
            *self = *self * other;
        }
    }

    impl fmt::Display for ResidueClass {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(
                f,
                "ResidueClass{{value_:{}, modulus_log_2_: {}}}",
                self.value_, self.modulus_log_2_
            )
        }
    }

    #[derive(Debug)]
    pub struct Worklist<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        queue_: VecDeque<T>,
        contained_: HashSet<T>,
    }

    impl<T> Worklist<T>
    where
        T: Eq + std::hash::Hash + Clone,
    {
        pub fn new() -> Self {
            Worklist {
                queue_: VecDeque::new(),
                contained_: HashSet::new(),
            }
        }

        pub fn is_empty(&self) -> bool {
            assert_eq!(self.queue_.len(), self.contained_.len());
            self.queue_.is_empty()
        }

        pub fn enqueue(&mut self, value: T) -> bool {
            if self.contained_.contains(&value) {
                return false;
            }
            self.queue_.push_back(value.clone());
            self.contained_.insert(value);
            assert_eq!(self.queue_.len(), self.contained_.len());
            true
        }

        pub fn dequeue(&mut self) -> T {
            assert!(!self.is_empty());
            let value = self.queue_.pop_front().unwrap();
            self.contained_.remove(&value);
            assert_eq!(self.queue_.len(), self.contained_.len());
            value
        }
    }

    pub fn transform_vector<T, U, F>(v: &[U], f: F) -> Vec<T>
    where
        F: Fn(&U) -> T,
        U: Clone,
    {
        v.iter().map(|x| f(x)).collect()
    }

    pub fn transform_vector_simple<T, U>(v: &[U]) -> Vec<T>
    where
        T: From<U>,
        U: Clone,
    {
        v.iter().map(|x| T::from(x.clone())).collect()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    pub struct SourcePosition {
        pub file: String,
        pub line: usize,
        pub column: usize,
    }
}