#![allow(non_snake_case)]
// Copyright 2017 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// TODO: Feature flag equivalent to #ifndef V8_INTL_SUPPORT
// #[cfg(not(feature = "intl"))]
// compile_error!("Internationalization is expected to be enabled.");

use std::f64::NAN;
use std::rc::Rc;
//use icu; // Placeholder, adapt crate usage.
//use icu_locid; // Placeholder, adapt crate usage.

// Placeholder for builtins-utils-inl.h functionality
mod builtins_utils {
    // Implement necessary utility functions here
    // Example:
    // pub fn to_this_string(_isolate: &Isolate, string: String, method_name: &str) -> Result<String, Error> { ... }
}

// Placeholder for builtins.h functionality
mod builtins {
    // Define Builtin enum or constants here
    pub const K_NUMBER_FORMAT_INTERNAL_FORMAT_NUMBER: i32 = 0;
    pub const K_DATE_TIME_FORMAT_INTERNAL_FORMAT: i32 = 1;
    pub const K_COLLATOR_INTERNAL_COMPARE: i32 = 2;
    pub const K_V8_BREAK_ITERATOR_INTERNAL_ADOPT_TEXT: i32 = 3;
    pub const K_V8_BREAK_ITERATOR_INTERNAL_FIRST: i32 = 4;
    pub const K_V8_BREAK_ITERATOR_INTERNAL_NEXT: i32 = 5;
    pub const K_V8_BREAK_ITERATOR_INTERNAL_CURRENT: i32 = 6;
    pub const K_V8_BREAK_ITERATOR_INTERNAL_BREAK_TYPE: i32 = 7;
}

// Placeholder for date/date.h functionality
mod date {
    // Implement necessary date functions here
}

// Placeholder for logging/counters.h functionality
mod logging {
    pub struct Counters {}
    impl Counters {
        pub fn count_usage(&self, _feature: UsageCounterFeature) {}
    }

    #[derive(Debug, Copy, Clone)]
    pub enum UsageCounterFeature {
        KStringNormalize,
        KStringLocaleCompare,
        KNumberFormat,
        KDateTimeFormat,
        KLocale,
        KListFormat,
        KRelativeTimeFormat,
        KPluralRules,
        KCollator,
        KSegmenter,
        KDisplayNames,
        KDurationFormat,
        KLocaleInfoFunctions,
        KLocaleInfoObsoletedGetters,
    }
}

// Placeholder for objects/elements.h functionality
mod elements {
    // Implement necessary elements functions here
}

// Placeholder for objects/intl-objects.h functionality
mod intl_objects {
    // Implement necessary intl objects functions here
}

// Placeholder for objects/js-array-inl.h functionality
mod js_array {
    // Implement necessary js array functions here
}

// Placeholder for objects/js-break-iterator-inl.h functionality
mod js_break_iterator {
    // Implement necessary js break iterator functions here
}

// Placeholder for objects/js-collator-inl.h functionality
mod js_collator {
    // Implement necessary js collator functions here
}

// Placeholder for objects/js-date-time-format-inl.h functionality
mod js_date_time_format {
    // Implement necessary js date time format functions here
}

// Placeholder for objects/js-display-names-inl.h functionality
mod js_display_names {
    // Implement necessary js display names functions here
}

// Placeholder for objects/js-duration-format-inl.h functionality
mod js_duration_format {
    // Implement necessary js duration format functions here
}

// Placeholder for objects/js-list-format-inl.h functionality
mod js_list_format {
    // Implement necessary js list format functions here
}

// Placeholder for objects/js-locale-inl.h functionality
mod js_locale {
    // Implement necessary js locale functions here
}

// Placeholder for objects/js-number-format-inl.h functionality
mod js_number_format {
    // Implement necessary js number format functions here
}

// Placeholder for objects/js-plural-rules-inl.h functionality
mod js_plural_rules {
    // Implement necessary js plural rules functions here
}

// Placeholder for objects/js-relative-time-format-inl.h functionality
mod js_relative_time_format {
    // Implement necessary js relative time format functions here
}

// Placeholder for objects/js-segment-iterator-inl.h functionality
mod js_segment_iterator {
    // Implement necessary js segment iterator functions here
}

// Placeholder for objects/js-segmenter-inl.h functionality
mod js_segmenter {
    // Implement necessary js segmenter functions here
}

// Placeholder for objects/js-segments-inl.h functionality
mod js_segments {
    // Implement necessary js segments functions here
}

// Placeholder for objects/objects-inl.h functionality
mod objects {
    // Implement necessary objects functions here
}

// Placeholder for objects/option-utils.h functionality
mod option_utils {
    // Implement necessary option utils functions here
}

// Placeholder for objects/property-descriptor.h functionality
mod property_descriptor {
    // Implement necessary property descriptor functions here
}

// Placeholder for objects/smi.h functionality
mod smi {
    // Implement necessary smi functions here
}

// Placeholder for unicode/brkiter.h functionality
mod brkiter {
    // Implement necessary break iterator functions here
}

mod intl {
    // Implement necessary intl functions here

    // Example:
    // pub fn convert_to_upper(_isolate: &Isolate, string: String) -> Result<String, Error> { ... }
}

// Dummy types and traits to replace V8's Handle/Tagged pointers and isolate
// These will need to be replaced with concrete Rust implementations
// based on the actual V8 API usage.
pub struct Isolate {
    counters: logging::Counters,
}

impl Isolate {
    pub fn new() -> Self {
        Isolate {
            counters: logging::Counters {},
        }
    }
    pub fn count_usage(&mut self, feature: logging::UsageCounterFeature) {
        self.counters.count_usage(feature);
    }
}

// Define a trait for V8 objects that can be cast to a specific type.
trait V8Cast<T> {
    fn to(self) -> T;
}

pub struct HandleScope<'a> {
    _isolate: &'a Isolate,
}

impl<'a> HandleScope<'a> {
    pub fn new(_isolate: &'a Isolate) -> Self {
        HandleScope { _isolate }
    }
}

pub struct BuiltinArguments {}

impl BuiltinArguments {
    pub fn at_or_undefined(&self, _isolate: &Isolate, _index: usize) -> Object {
        Object {} // Return a dummy undefined object
    }

    pub fn new_target(&self) -> Object {
        Object {} // Return a dummy new_target object
    }

    pub fn target(&self) -> Rc<JSFunction> {
        Rc::new(JSFunction {})
    }

    pub fn receiver(&self) -> Rc<JSAny> {
        Rc::new(JSAny {})
    }

    pub fn at(&self, _index: usize) -> Rc<Object> {
        Rc::new(Object {})
    }

    pub fn length(&self) -> usize {
        0
    }
}

pub struct Object {}

impl Object {
    pub fn boolean_value(_object: &Object, _isolate: &Isolate) -> bool {
        false
    }

    pub fn number_value(_object: &Object) -> f64 {
        0.0
    }

    pub fn integer_value(_isolate: &Isolate, _index: Object) -> Result<i64, Error> {
        Ok(0)
    }
}

pub struct String {}

impl String {
    pub fn flatten(_isolate: &Isolate, _string: String) -> String {
        String {}
    }
}

pub struct JSReceiver {}

impl JSReceiver {
    pub fn define_own_property(
        _isolate: &Isolate,
        _receiver: Rc<JSReceiver>,
        _name: Object,
        _desc: &PropertyDescriptor,
        _throw: Just,
    ) -> Result<bool, Error> {
        Ok(true)
    }
}

#[derive(Clone, Copy)]
pub struct Just(bool);

pub const K_THROW_ON_ERROR: Just = Just(true);

pub struct PropertyDescriptor {}

impl PropertyDescriptor {
    pub fn set_value(&mut self, _value: Rc<Object>) {}
    pub fn set_writable(&mut self, _writable: bool) {}
    pub fn set_enumerable(&mut self, _enumerable: bool) {}
    pub fn set_configurable(&mut self, _configurable: bool) {}
}

pub struct JSFunction {}

impl JSFunction {
    pub fn get_derived_map(_isolate: &Isolate, _target: Rc<JSFunction>, _new_target: Rc<JSReceiver>) -> Result<Map, Error> {
        Ok(Map {})
    }
}

pub struct Map {}

pub struct NativeContext {}

pub struct Context {}

impl Context {
    pub fn get(&self, _index: i32) -> Rc<Object> {
        Rc::new(Object {})
    }

    pub fn set(&self, _index: i32, _value: Object) {}
}

pub struct SharedFunctionInfo {}

pub struct Factory {}

impl Factory {
    pub fn new_string_from_ascii_checked(&self, _str: &str) -> Object {
        Object {}
    }
    pub fn new_builtin_context(&self, _native_context: Rc<NativeContext>, _i: i32) -> Rc<Context> {
        Rc::new(Context {})
    }
    pub fn new_shared_function_info_for_builtin(
        &self,
        _str: Object,
        _builtin: i32,
        _len: i32,
        _adapt: Adapt,
    ) -> Rc<SharedFunctionInfo> {
        Rc::new(SharedFunctionInfo {})
    }
    pub fn nan_value(&self) -> Rc<Object> {
        Rc::new(Object {})
    }
    pub fn intl_fallback_symbol(&self) -> Object {
        Object {}
    }
    pub fn startRange_string(&self) -> Object {
        Object {}
    }
    pub fn endRange_string(&self) -> Object {
        Object {}
    }
    pub fn empty_string(&self) -> Object {
        Object {}
    }
}

pub struct ReadOnlyRoots {}

impl ReadOnlyRoots {
    pub fn exception(&self) -> Object {
        Object {}
    }
    pub fn undefined_value(&self) -> Object {
        Object {}
    }
}

pub struct JSNumberFormat {}

pub struct JSDateTimeFormat {}

pub struct JSListFormat {}

pub struct JSLocale {}

pub struct JSRelativeTimeFormat {}

pub struct JSPluralRules {}

pub struct JSCollator {}

pub struct JSSegmentIterator {}

pub struct JSSegmenter {}

pub struct JSSegments {}

pub struct JSV8BreakIterator {}

pub struct JSDisplayNames {}

pub struct JSDurationFormat {}

pub struct Error {}

pub enum Adapt {
    KAdapt,
}

pub struct NewTypeError {}

impl NewTypeError {
    pub fn new(_message_template: MessageTemplate) -> Self {
        NewTypeError {}
    }
}

pub struct NewRangeError {}

impl NewRangeError {
    pub fn new(_message_template: MessageTemplate, _arg: Object) -> Self {
        NewRangeError {}
    }
}

pub enum MessageTemplate {
    KIncompatibleMethodReceiver,
    KConstructorNotFunction,
    KLocaleNotEmpty,
    KInvalidTimeValue,
    KInvalid,
}

#[derive(Debug, PartialEq)]
pub enum Type {
    String,
    Object,
}

pub struct JSObject {}

pub struct JSAny {}

impl JSAny {
    pub fn to_string(_isolate: &Isolate) -> Result<String, Error> {
        Ok(String {})
    }
}

// Builtins Implementation
macro_rules! builtin {
    ($name:ident, $body:block) => {
        pub fn $name(_args: BuiltinArguments, _isolate: &mut Isolate) -> Object {
            $body
        }
    };
}

builtin!(StringPrototypeToUpperCaseIntl, {
    let _scope = HandleScope::new(_isolate);
    //TO_THIS_STRING(string, "String.prototype.toUpperCase"); // TODO: Implement TO_THIS_STRING
    //string = String::Flatten(isolate, string); // TODO: Implement Flatten
    //RETURN_RESULT_OR_FAILURE(isolate, Intl::ConvertToUpper(isolate, string)); // TODO: Implement ConvertToUpper and error handling
    Object {}
});

builtin!(StringPrototypeNormalizeIntl, {
    let _handle_scope = HandleScope::new(_isolate);
    _isolate.count_usage(logging::UsageCounterFeature::KStringNormalize);
    //TO_THIS_STRING(string, "String.prototype.normalize"); // TODO: Implement TO_THIS_STRING

    //DirectHandle<Object> form_input = args.atOrUndefined(isolate, 1);
    let _form_input = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate,Intl::Normalize(isolate, string, form_input));
    Object {}
});

builtin!(StringPrototypeLocaleCompareIntl, {
    let _handle_scope = HandleScope::new(_isolate);

    _isolate.count_usage(logging::UsageCounterFeature::KStringLocaleCompare);
    let _k_method = "String.prototype.localeCompare";

    //TO_THIS_STRING(str1, kMethod); // TODO: Implement TO_THIS_STRING
    //DirectHandle<String> str2;
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, str2, Object::ToString(isolate, args.atOrUndefined(isolate, 1)));
    //std::optional<int> result = Intl::StringLocaleCompare(isolate, str1, str2, args.atOrUndefined(isolate, 2),args.atOrUndefined(isolate, 3), kMethod);
    //if (!result.has_value()) {
    //    DCHECK(isolate->has_exception());
    //    return ReadOnlyRoots(isolate).exception();
    //}
    //return Smi::FromInt(result.value());
    Object {}
});

builtin!(V8BreakIteratorSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate,Intl::SupportedLocalesOf(isolate, "Intl.v8BreakIterator.supportedLocalesOf",JSV8BreakIterator::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(NumberFormatSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate,Intl::SupportedLocalesOf(isolate, "Intl.NumberFormat.supportedLocalesOf",JSNumberFormat::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(NumberFormatPrototypeFormatToParts, {
    let _method_name = "Intl.NumberFormat.prototype.formatToParts";
    let _handle_scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSNumberFormat, number_format, method_name);

    let _x = if _args.length() >= 2 {
        _args.at(1)
    } else {
        _isolate.factory().nan_value()
    };

    //RETURN_RESULT_OR_FAILURE(isolate, JSNumberFormat::FormatToParts(isolate, number_format, x));
    Object {}
});

builtin!(DateTimeFormatPrototypeResolvedOptions, {
    let _method_name = "Intl.DateTimeFormat.prototype.resolvedOptions";
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSReceiver, format_holder, method_name);

    //DirectHandle<JSDateTimeFormat> date_time_format;
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, date_time_format,JSDateTimeFormat::UnwrapDateTimeFormat(isolate, format_holder));

    //RETURN_RESULT_OR_FAILURE(isolate, JSDateTimeFormat::ResolvedOptions(isolate, date_time_format));
    Object {}
});

builtin!(DateTimeFormatSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate,Intl::SupportedLocalesOf(isolate, "Intl.DateTimeFormat.supportedLocalesOf",JSDateTimeFormat::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(DateTimeFormatPrototypeFormatToParts, {
    let _method_name = "Intl.DateTimeFormat.prototype.formatToParts";
    let _handle_scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSObject, date_format_holder, method_name);
    let _factory = _isolate.factory();

    //if (!IsJSDateTimeFormat(*date_format_holder)) {
    //    THROW_NEW_ERROR_RETURN_FAILURE(isolate, NewTypeError(MessageTemplate::kIncompatibleMethodReceiver,factory->NewStringFromAsciiChecked(method_name),date_format_holder));
    //}
    //auto dtf = Cast<JSDateTimeFormat>(date_format_holder);

    let _x = _args.at_or_undefined(_isolate, 1);
    //RETURN_RESULT_OR_FAILURE(isolate, JSDateTimeFormat::FormatToParts(isolate, dtf, x, false, method_name));
    Object {}
});

// TODO: Implement DateTimeFormatRange
//template <class T,
//          MaybeDirectHandle<T> (*F)(Isolate*, DirectHandle<JSDateTimeFormat>,
//                                    DirectHandle<Object>, DirectHandle<Object>,
//                                    const char* const)>
//V8_WARN_UNUSED_RESULT Tagged<Object> DateTimeFormatRange(
//    BuiltinArguments args, Isolate* isolate, const char* const method_name) {
//  // 1. Let dtf be this value.
//  // 2. Perform ? RequireInternalSlot(dtf, [[InitializedDateTimeFormat]]).
//  CHECK_RECEIVER(JSDateTimeFormat, dtf, method_name);
//
//  // 3. If startDate is undefined or endDate is undefined, throw a TypeError
//  // exception.
//  DirectHandle<Object> start_date = args.atOrUndefined(isolate, 1);
//  DirectHandle<Object> end_date = args.atOrUndefined(isolate, 2);
//  if (IsUndefined(*start_date, isolate) || IsUndefined(*end_date, isolate)) {
//    THROW_NEW_ERROR_RETURN_FAILURE(
//        isolate, NewTypeError(MessageTemplate::kInvalidTimeValue));
//  }
//
//  // 4. Return ? FormatDateTimeRange(dtf, startDate, endDate)
//  // OR
//  // 4. Return ? FormatDateTimeRangeToParts(dtf, startDate, endDate).
//  RETURN_RESULT_OR_FAILURE(isolate,
//                           F(isolate, dtf, start_date, end_date, method_name));
//}

builtin!(DateTimeFormatPrototypeFormatRange, {
    let _method_name = "Intl.DateTimeFormat.prototype.formatRange";
    let _handle_scope = HandleScope::new(_isolate);
    //return DateTimeFormatRange<String, JSDateTimeFormat::FormatRange>(args, isolate, method_name);
    Object {}
});

builtin!(DateTimeFormatPrototypeFormatRangeToParts, {
    let _method_name = "Intl.DateTimeFormat.prototype.formatRangeToParts";
    let _handle_scope = HandleScope::new(_isolate);
    //return DateTimeFormatRange<JSArray, JSDateTimeFormat::FormatRangeToParts>(args, isolate, method_name);
    Object {}
});

fn create_bound_function(
    _isolate: &Isolate,
    _object: Rc<Object>,
    _builtin: i32,
    _len: i32,
) -> Rc<JSFunction> {
    //DirectHandle<NativeContext> native_context(isolate->context()->native_context(), isolate);
    //DirectHandle<Context> context = isolate->factory()->NewBuiltinContext(native_context,static_cast<int>(Intl::BoundFunctionContextSlot::kLength));

    //context->set(static_cast<int>(Intl::BoundFunctionContextSlot::kBoundFunction),*object);

    //DirectHandle<SharedFunctionInfo> info =isolate->factory()->NewSharedFunctionInfoForBuiltin(isolate->factory()->empty_string(), builtin, len, kAdapt);

    //return Factory::JSFunctionBuilder{isolate, info, context}.set_map(isolate->strict_function_without_prototype_map()).Build();
    Rc::new(JSFunction {})
}

// TODO: Implement LegacyFormatConstructor
//template <class T>
//Tagged<Object> LegacyFormatConstructor(BuiltinArguments args, Isolate* isolate,
//                                       v8::Isolate::UseCounterFeature feature,
//                                       DirectHandle<JSAny> constructor,
//                                       const char* method_name) {

// TODO: Implement DisallowCallConstructor
//template <class T>
//Tagged<Object> DisallowCallConstructor(BuiltinArguments args, Isolate* isolate,
//                                       v8::Isolate::UseCounterFeature feature,
//                                       const char* method_name) {

// TODO: Implement CallOrConstructConstructor
//template <class T>
//Tagged<Object> CallOrConstructConstructor(BuiltinArguments args,
//                                          Isolate* isolate,
//                                          const char* method_name) {

builtin!(DisplayNamesConstructor, {
    let _scope = HandleScope::new(_isolate);

    //return DisallowCallConstructor<JSDisplayNames>(args, isolate, v8::Isolate::UseCounterFeature::kDisplayNames,"Intl.DisplayNames");
    Object {}
});

builtin!(DisplayNamesPrototypeResolvedOptions, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSDisplayNames, holder,"Intl.DisplayNames.prototype.resolvedOptions");
    //return *JSDisplayNames::ResolvedOptions(isolate, holder);
    Object {}
});

builtin!(DisplayNamesSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate, Intl::SupportedLocalesOf(isolate, "Intl.DisplayNames.supportedLocalesOf",JSDisplayNames::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(DisplayNamesPrototypeOf, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSDisplayNames, holder, "Intl.DisplayNames.prototype.of");
    let _code_obj = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate,JSDisplayNames::Of(isolate, holder, code_obj));
    Object {}
});

builtin!(DurationFormatConstructor, {
    let _scope = HandleScope::new(_isolate);

    //return DisallowCallConstructor<JSDurationFormat>(args, isolate, v8::Isolate::UseCounterFeature::kDurationFormat,"Intl.DurationFormat");
    Object {}
});

builtin!(DurationFormatPrototypeResolvedOptions, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSDurationFormat, holder,"Intl.DurationFormat.prototype.resolvedOptions");
    //return *JSDurationFormat::ResolvedOptions(isolate, holder);
    Object {}
});

builtin!(DurationFormatSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate, Intl::SupportedLocalesOf(isolate, "Intl.DurationFormat.supportedLocalesOf",JSDurationFormat::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(DurationFormatPrototypeFormat, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSDurationFormat, holder,"Intl.DurationFormat.prototype.format");
    let _value = _args.at_or_undefined(_isolate, 1);
    //RETURN_RESULT_OR_FAILURE(isolate,JSDurationFormat::Format(isolate, holder, value));
    Object {}
});

builtin!(DurationFormatPrototypeFormatToParts, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSDurationFormat, holder,"Intl.DurationFormat.prototype.formatToParts");
    let _value = _args.at_or_undefined(_isolate, 1);
    //RETURN_RESULT_OR_FAILURE(isolate,JSDurationFormat::FormatToParts(isolate, holder, value));
    Object {}
});

builtin!(NumberFormatConstructor, {
    let _scope = HandleScope::new(_isolate);

    //return LegacyFormatConstructor<JSNumberFormat>(args, isolate, v8::Isolate::UseCounterFeature::kNumberFormat,isolate->intl_number_format_function(), "Intl.NumberFormat");
    Object {}
});

builtin!(NumberFormatPrototypeResolvedOptions, {
    let _method_name = "Intl.NumberFormat.prototype.resolvedOptions";
    let _scope = HandleScope::new(_isolate);

    //CHECK_RECEIVER(JSReceiver, number_format_holder, method_name);

    //DirectHandle<JSNumberFormat> number_format;
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, number_format,JSNumberFormat::UnwrapNumberFormat(isolate, number_format_holder));

    //return *JSNumberFormat::ResolvedOptions(isolate, number_format);
    Object {}
});

builtin!(NumberFormatPrototypeFormatNumber, {
    let _method_name = "get Intl.NumberFormat.prototype.format";
    let _scope = HandleScope::new(_isolate);

    //CHECK_RECEIVER(JSReceiver, receiver, method_name);
    //DirectHandle<JSNumberFormat> number_format;
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, number_format,JSNumberFormat::UnwrapNumberFormat(isolate, receiver));

    //DirectHandle<Object> bound_format(number_format->bound_format(), isolate);
    //if (!IsUndefined(*bound_format, isolate)) {
    //    DCHECK(IsJSFunction(*bound_format));
    //    return *bound_format;
    //}

    //DirectHandle<JSFunction> new_bound_format_function = CreateBoundFunction(isolate, number_format, Builtin::kNumberFormatInternalFormatNumber, 1);
    //number_format->set_bound_format(*new_bound_format_function);

    //return *new_bound_format_function;
    Object {}
});

builtin!(NumberFormatInternalFormatNumber, {
    let _scope = HandleScope::new(_isolate);

    //DirectHandle<Context> context(isolate->context(), isolate);

    //DirectHandle<JSNumberFormat> number_format(Cast<JSNumberFormat>(context->get(static_cast<int>(Intl::BoundFunctionContextSlot::kBoundFunction))),isolate);

    let _value = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate, JSNumberFormat::NumberFormatFunction(isolate, number_format, value));
    Object {}
});

// TODO: Implement NumberFormatRange
//template <class T,
//          MaybeDirectHandle<T> (*F)(Isolate*, DirectHandle<JSNumberFormat>,
//                                    Handle<Object>, Handle<Object>)>
//V8_WARN_UNUSED_RESULT Tagged<Object> NumberFormatRange(
//    BuiltinArguments args, Isolate* isolate, const char* const method_name) {

builtin!(NumberFormatPrototypeFormatRange, {
    let _method_name = "Intl.NumberFormat.prototype.formatRange";
    let _handle_scope = HandleScope::new(_isolate);
    //return NumberFormatRange<String, JSNumberFormat::FormatNumericRange>(args, isolate, method_name);
    Object {}
});

builtin!(NumberFormatPrototypeFormatRangeToParts, {
    let _method_name = "Intl.NumberFormat.prototype.formatRangeToParts";
    let _handle_scope = HandleScope::new(_isolate);
    //return NumberFormatRange<JSArray, JSNumberFormat::FormatNumericRangeToParts>(args, isolate, method_name);
    Object {}
});

builtin!(DateTimeFormatConstructor, {
    let _scope = HandleScope::new(_isolate);

    //return LegacyFormatConstructor<JSDateTimeFormat>(args, isolate, v8::Isolate::UseCounterFeature::kDateTimeFormat,isolate->intl_date_time_format_function(), "Intl.DateTimeFormat");
    Object {}
});

builtin!(DateTimeFormatPrototypeFormat, {
    let _method_name = "get Intl.DateTimeFormat.prototype.format";
    let _scope = HandleScope::new(_isolate);

    //CHECK_RECEIVER(JSReceiver, receiver, method_name);

    //DirectHandle<JSDateTimeFormat> format;
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, format,JSDateTimeFormat::UnwrapDateTimeFormat(isolate, receiver));

    //DirectHandle<Object> bound_format =DirectHandle<Object>(format->bound_format(), isolate);

    //if (!IsUndefined(*bound_format, isolate)) {
    //    DCHECK(IsJSFunction(*bound_format));
    //    return *bound_format;
    //}

    //DirectHandle<JSFunction> new_bound_format_function = CreateBoundFunction(isolate, format, Builtin::kDateTimeFormatInternalFormat, 1);
    //format->set_bound_format(*new_bound_format_function);

    //return *new_bound_format_function;
    Object {}
});

builtin!(DateTimeFormatInternalFormat, {
    let _scope = HandleScope::new(_isolate);
    //DirectHandle<Context> context(isolate->context(), isolate);

    //DirectHandle<JSDateTimeFormat> date_format_holder(Cast<JSDateTimeFormat>(context->get(static_cast<int>(Intl::BoundFunctionContextSlot::kBoundFunction))),isolate);

    let _date = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate, JSDateTimeFormat::DateTimeFormat(isolate, date_format_holder, date,"DateTime Format Functions"));
    Object {}
});

builtin!(IntlGetCanonicalLocales, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate,Intl::GetCanonicalLocales(isolate, locales));
    Object {}
});

builtin!(IntlSupportedValuesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);

    //RETURN_RESULT_OR_FAILURE(isolate, Intl::SupportedValuesOf(isolate, locales));
    Object {}
});

builtin!(ListFormatConstructor, {
    let _scope = HandleScope::new(_isolate);

    //return DisallowCallConstructor<JSListFormat>(args, isolate, v8::Isolate::UseCounterFeature::kListFormat,"Intl.ListFormat");
    Object {}
});

builtin!(ListFormatPrototypeResolvedOptions, {
    let _scope = HandleScope::new(_isolate);
    //CHECK_RECEIVER(JSListFormat, format_holder,"Intl.ListFormat.prototype.resolvedOptions");
    //return *JSListFormat::ResolvedOptions(isolate, format_holder);
    Object {}
});

builtin!(ListFormatSupportedLocalesOf, {
    let _scope = HandleScope::new(_isolate);
    let _locales = _args.at_or_undefined(_isolate, 1);
    let _options = _args.at_or_undefined(_isolate, 2);

    //RETURN_RESULT_OR_FAILURE(isolate, Intl::SupportedLocalesOf(isolate, "Intl.ListFormat.supportedLocalesOf",JSListFormat::GetAvailableLocales(), locales, options));
    Object {}
});

builtin!(LocaleConstructor, {
    let mut _scope = HandleScope::new(_isolate);

    _isolate.count_usage(logging::UsageCounterFeature::KLocale);

    let _method_name = "Intl.Locale";
    //if (IsUndefined(*args.new_target(), isolate)) {  // [[Call]]
    //    THROW_NEW_ERROR_RETURN_FAILURE(isolate, NewTypeError(MessageTemplate::kConstructorNotFunction,isolate->factory()->NewStringFromAsciiChecked(method_name)));
    //}
    //// [[Construct]]
    //DirectHandle<JSFunction> target = args.target();
    //DirectHandle<JSReceiver> new_target = Cast<JSReceiver>(args.new_target());

    //DirectHandle<Object> tag = args.atOrUndefined(isolate, 1);
    //DirectHandle<Object> options = args.atOrUndefined(isolate, 2);

    //DirectHandle<Map> map;
    //// 6. Let locale be ? OrdinaryCreateFromConstructor(NewTarget,
    //// %LocalePrototype%, internalSlotsList).
    //ASSIGN_RETURN_FAILURE_ON_EXCEPTION(isolate, map, JSFunction::GetDerivedMap(isolate, target, new_target));

    //// 7. If Type(tag) is not String or Object, throw a TypeError exception.
    //if (!IsString(*tag) && !IsJSReceiver(*tag)) {
    //  THROW_NEW_ERROR_RETURN_FAILURE(isolate, NewTypeError(MessageTemplate::kLocaleNotEmpty));
    