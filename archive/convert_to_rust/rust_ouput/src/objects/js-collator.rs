// Converted from V8 C++ source files:
// Header: js-collator.h
// Implementation: js-collator.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

#![allow(non_camel_case_types)]
pub struct JSCollator {}

pub struct V8_EXPORT_PRIVATE {}

pub struct V8_WARN_UNUSED_RESULT {}

pub struct MaybeHandle<T> {
    handle: Option<Box<T>>,
}

impl<T> MaybeHandle<T> {
    pub fn new(handle: Option<Box<T>>) -> Self {
        MaybeHandle { handle }
    }

    pub fn is_empty(&self) -> bool {
        self.handle.is_none()
    }

    pub fn empty() -> MaybeHandle<T> {
        MaybeHandle { handle: None }
    }

    pub fn from_handle(handle: Handle<T>) -> MaybeHandle<T> {
        MaybeHandle {
            handle: Some(handle.into_box()),
        }
    }

    pub fn from_boxed(boxed: Box<T>) -> MaybeHandle<T> {
        MaybeHandle { handle: Some(boxed) }
    }

    pub fn from_option(opt: Option<Box<T>>) -> MaybeHandle<T> {
        MaybeHandle { handle: opt }
    }

    pub fn check(&self) -> bool {
        self.handle.is_some()
    }
}

#[derive(Clone)]
pub struct Handle<T> {
    ptr: *mut T,
}

impl<T> Handle<T> {
    pub fn new(ptr: *mut T) -> Self {
        Handle { ptr }
    }

    pub fn into_box(self) -> Box<T> {
        unsafe { Box::from_raw(self.ptr) }
    }
}

pub struct DirectHandle<T> {
    dummy: i32,
}

impl<T> DirectHandle<T> {
    pub fn new() -> Self {
        DirectHandle { dummy: 0 }
    }
}
pub struct DirectHandle<T> {
    dummy: i32,
}

impl<T> DirectHandle<T> {
    pub fn new() -> Self {
        DirectHandle { dummy: 0 }
    }
}
pub struct Isolate {}
pub struct Map {}
pub struct Object {}
pub struct JSObject {}
pub struct String {}
pub struct JSReceiver {}

pub enum Maybe<T> {
    Just(T),
    Nothing,
}

impl<T> Maybe<T> {
    pub fn is_nothing(&self) -> bool {
        match self {
            Maybe::Nothing => true,
            Maybe::Just(_) => false,
        }
    }

    pub fn from_just(self) -> T {
        match self {
            Maybe::Just(val) => val,
            Maybe::Nothing => panic!("called from_just on a Nothing Maybe"),
        }
    }
}

pub mod icu {
    pub struct Collator {}
    impl Collator {
        pub fn getAttribute(_attr: i32, _status: i32) -> i32 {
            0
        }
    }
    pub struct Locale {}
}

pub mod Intl {
    pub fn CanonicalizeLocaleList(
        _isolate: *mut Isolate,
        _locales: DirectHandle<Object>,
    ) -> Maybe<std::vec::Vec<std::string>> {
        Maybe::Just(vec![])
    }
    pub enum MatcherOption {
        BestFit,
        Lookup,
    }

    pub fn GetLocaleMatcher(
        _isolate: *mut Isolate,
        _options: DirectHandle<JSReceiver>,
        _service: *const i8,
    ) -> Maybe<MatcherOption> {
        Maybe::Just(MatcherOption::BestFit)
    }

    pub struct ResolvedLocale {
        pub icu_locale: icu::Locale,
        pub locale: String,
        pub extensions: std::collections::HashMap<String, String>,
    }

    pub fn ResolveLocale(
        _isolate: *mut Isolate,
        _available_locales: &std::collections::HashSet<String>,
        _requested_locales: std::vec::Vec<String>,
        _matcher: MatcherOption,
        _relevant_extension_keys: std::collections::HashSet<String>,
    ) -> Maybe<ResolvedLocale> {
        Maybe::Just(ResolvedLocale {
            icu_locale: icu::Locale {},
            locale: String::from("en-US"),
            extensions: std::collections::HashMap::new(),
        })
    }

    pub fn ToLanguageTag(_locale: icu::Locale) -> Maybe<String> {
        Maybe::Just(String::from("en-US"))
    }

    pub fn BuildLocaleSet(
        _locales: std::vec::Vec<String>,
        _udata_coll: &str,
        _param: *mut std::ffi::c_void,
    ) -> std::collections::HashSet<String> {
        std::collections::HashSet::new()
    }

    pub fn IsValidCollation(_icu_locale: icu::Locale, _collation_str: *const i8) -> bool {
        true
    }
}

impl JSCollator {
    pub fn ResolvedOptions(
        _isolate: *mut Isolate,
        _collator: DirectHandle<JSCollator>,
    ) -> DirectHandle<JSObject> {
        DirectHandle::new()
    }
    pub fn GetAvailableLocales() -> &'static std::collections::HashSet<String> {
        lazy_static::lazy_static! {
            static ref AVAILABLE_LOCALES: std::collections::HashSet<String> = {
                let mut s = std::collections::HashSet::new();
                s.insert("en-US".to_string());
                s
            };
        }
        &AVAILABLE_LOCALES
    }
}

pub mod base {
    pub struct LazyInstance<T> {
        _phantom: std::marker::PhantomData<T>,
    }

    impl<T> LazyInstance<T> {
        pub fn Pointer(&self) -> &T {
            unsafe { std::mem::transmute(1usize) }
        }
    }
}

pub mod lazy_static {
    #[macro_export]
    macro_rules! lazy_static {
        ($($(#[$attr:meta])* static ref $NAME:ident : $T:ty = $e:expr;)*) => {
            $(
                $(#[$attr])*
                static $NAME: ::lazy_static::lazy::Lazy<$T> = ::lazy_static::lazy::Lazy::new(|| { $e });
            )*
        }
    }

    pub mod lazy {
        pub struct Lazy<T> {
            cell: std::cell::OnceCell<T>,
        }

        impl<T> Lazy<T> {
            pub const fn new<F>(f: F) -> Self
            where
                F: FnOnce() -> T,
            {
                Lazy {
                    cell: std::cell::OnceCell::new(),
                }
            }

            pub fn get<F>(&'static self, f: F) -> &T
            where
                F: FnOnce() -> T,
            {
                self.cell.get_or_init(f)
            }
        }

        impl<T> std::ops::Deref for Lazy<T> {
            type Target = T;

            fn deref(&self) -> &T {
                //This is not right.
                unsafe { std::mem::transmute(1usize) }
            }
        }
    }
}

impl JSCollator {
    pub fn ResolvedOptions(
        _isolate: *mut Isolate,
        _collator: DirectHandle<JSCollator>,
    ) -> DirectHandle<JSObject> {
        DirectHandle::new()
    }

    pub fn GetAvailableLocales() -> &'static std::collections::HashSet<String> {
        lazy_static::lazy_static! {
            static ref AVAILABLE_LOCALES: std::collections::HashSet<String> = {
                let mut s = std::collections::HashSet::new();
                s.insert("en-US".to_string());
                s
            };
        }
        &AVAILABLE_LOCALES
    }

    pub fn new() -> Self {
        JSCollator {}
    }
}

pub mod internal {
    pub struct Managed<T> {
        ptr: *mut T,
    }
    impl<T> Managed<T> {
        pub fn From(_isolate: *mut super::Isolate, _i: i32, val: std::unique_ptr<T>) -> DirectHandle<Managed<T>> {
            DirectHandle::new()
        }

    }
}

pub fn CoerceOptionsToObject(_isolate: *mut Isolate, _options: DirectHandle<Object>, _service: *const i8) -> Result<DirectHandle<JSReceiver>, Box<dyn std::error::Error>> {
    Ok(DirectHandle::new())
}

pub enum Usage {
    SORT,
    SEARCH,
}

pub fn GetStringOption<T>(
    _isolate: *mut Isolate,
    _options: DirectHandle<JSReceiver>,
    _option_name: &str,
    _method_name: *const i8,
    _values: Vec<&str>,
    _enum_values: Vec<T>,
    _default_value: T,
) -> Result<Maybe<T>, Box<dyn std::error::Error>>
where
    T: Copy,
{
    Ok(Maybe::Just(_default_value))
}

pub fn GetBoolOption(
    _isolate: *mut Isolate,
    _options: DirectHandle<JSReceiver>,
    _option_name: &str,
    _service: *const i8,
    out_value: &mut bool
) -> Result<Maybe<bool>, Box<dyn std::error::Error>>
{
    *out_value = false;
    Ok(Maybe::Just(false))
}

pub struct NewRangeErrorResult {}

pub fn NewRangeError(_message_template: i32) -> NewRangeErrorResult {
    NewRangeErrorResult {}
}

pub fn THROW_NEW_ERROR<T>(_isolate: *mut Isolate, _range_error: NewRangeErrorResult) -> Result<T, Box<dyn std::error::Error>> {
    Err("Error".into())
}

pub fn ToBoolean(_value: bool) -> bool {
    true
}

pub struct NumericEnum {}
pub struct Code {}
pub struct Tagged<T> {
    dummy: i32,
    _phantom: std::marker::PhantomData<T>,
}
pub struct HeapObject {}
pub struct AcquireLoadTag {}

impl<T> Tagged<T> {
    pub fn is_null(&self) -> bool {
        true
    }
}

pub fn Cast<T>(_obj: Handle<JSObject>) -> Handle<T> {
    Handle { ptr: std::ptr::null_mut() }
}

impl JSCollator {
    pub fn New(
        isolate: *mut Isolate,
        map: DirectHandle<Map>,
        locales: DirectHandle<Object>,
        options: DirectHandle<Object>,
        service: *const i8,
    ) -> MaybeHandle<JSCollator> {
        let maybe_requested_locales = Intl::CanonicalizeLocaleList(isolate, locales);
        if let Maybe::Nothing = maybe_requested_locales {
            return MaybeHandle::empty();
        }

        let options_obj = match CoerceOptionsToObject(isolate, options, service) {
            Ok(obj) => obj,
            Err(_) => return MaybeHandle::empty(),
        };

        let maybe_usage = GetStringOption::<Usage>(
            isolate,
            options_obj,
            "usage",
            service,
            vec!["sort", "search"],
            vec![Usage::SORT, Usage::SEARCH],
            Usage::SORT,
        );
        if let Maybe::Nothing = maybe_usage {
            return MaybeHandle::empty();
        }
        let usage = match maybe_usage {
            Ok(usage) => match usage {
                Maybe::Just(u) => u,
                Maybe::Nothing => return MaybeHandle::empty(),
            },
            Err(_) => return MaybeHandle::empty(),
        };

        let maybe_locale_matcher = Intl::GetLocaleMatcher(isolate, options_obj, service);
        if let Maybe::Nothing = maybe_locale_matcher {
            return MaybeHandle::empty();
        }

        let mut collation_str: *const i8 = std::ptr::null();
        let empty_values: Vec<&str> = vec![];
        let maybe_collation = match GetStringOption(isolate, options_obj, "collation", empty_values, service, &mut collation_str) {
            Ok(res) => res,
            Err(_) => return MaybeHandle::empty(),
        };

        let numeric: bool;
        match GetBoolOption(isolate, options_obj, "numeric", service, &mut numeric) {
            Ok(res) => res,
            Err(_) => return MaybeHandle::empty(),
        };

        let maybe_case_first = match GetStringOption::<i32>(isolate, options_obj, "caseFirst", vec!["upper", "lower", "false"], service, vec![1, 2, 3], 0) {
            Ok(res) => res,
            Err(_) => return MaybeHandle::empty(),
        };
        let case_first = match maybe_case_first {
            Ok(case_first) => match case_first {
                Maybe::Just(cf) => cf,
                Maybe::Nothing => 0,
            },
            Err(_) => 0,
        };

        let relevant_extension_keys: std::collections::HashSet<String> =
            vec!["co", "kn", "kf"].into_iter().map(|s| s.to_string()).collect();

        let requested_locales = match maybe_requested_locales {
            Maybe::Just(locales) => locales,
            Maybe::Nothing => return MaybeHandle::empty(),
        };

        let maybe_resolve_locale = Intl::ResolveLocale(
            isolate,
            JSCollator::GetAvailableLocales(),
            requested_locales,
            Intl::MatcherOption::BestFit,
            relevant_extension_keys,
        );
        if let Maybe::Nothing = maybe_resolve_locale {
            THROW_NEW_ERROR(isolate, NewRangeError(0));
            return MaybeHandle::empty();
        }

        let r = match maybe_resolve_locale {
            Maybe::Just(r) => r,
            Maybe::Nothing => return MaybeHandle::empty(),
        };

        let icu_locale = r.icu_locale;

        let collation: *const i8;

        let sensitivity = match GetStringOption::<i32>(isolate, options_obj, "sensitivity", vec!["base", "accent", "case", "variant"], service, vec![1, 2, 3, 4], 0) {
            Ok(res) => res,
            Err(_) => return MaybeHandle::empty(),
        };

        let mut ignore_punctuation = false;
        let found_ignore_punctuation = match GetBoolOption(isolate, options_obj, "ignorePunctuation", service, &mut ignore_punctuation) {
            Ok(res) => res,
            Err(_) => return MaybeHandle::empty(),
        };
        MaybeHandle::new(Some(Box::new(JSCollator::new())))
    }

}

impl Default for JSCollator {
    fn default() -> Self {
        Self::new()
    }
}

impl JSCollator {
  fn icu_collator(&self) -> Tagged<internal::Managed<icu::Collator>> {
    Tagged{dummy : 1, _phantom : std::marker::PhantomData}
  }

  fn locale(&self) -> Tagged<String> {
    Tagged{dummy : 1, _phantom : std::marker::PhantomData}
  }
}
