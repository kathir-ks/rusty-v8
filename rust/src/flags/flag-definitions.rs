// Copyright 2012 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

// This file defines all of the flags.  It is separated into different section,
// for Debug, Release, Logging and Profiling, etc.  To add a new flag, find the
// correct section, and use one of the DEFINE_ macros, without a trailing ';'.
//
// This include does not have a guard, because it is a template-style include,
// which can be included multiple times in different modes.  It expects to have
// a mode defined before it's included.  The modes are FLAG_MODE_... below:
//
// PRESUBMIT_INTENTIONALLY_MISSING_INCLUDE_GUARD

macro_rules! define_implication {
    ($whenflag:ident, $thenflag:ident) => {
        define_value_implication!($whenflag, $thenflag, true);
    };
}

// A weak implication will be overwritten by a normal implication or by an
// explicit flag.
macro_rules! define_weak_implication {
    ($whenflag:ident, $thenflag:ident) => {
        define_weak_value_implication!($whenflag, $thenflag, true);
    };
}

macro_rules! define_weak_neg_implication {
    ($whenflag:ident, $thenflag:ident) => {
        define_weak_value_implication!($whenflag, $thenflag, false);
    };
}

macro_rules! define_neg_implication {
    ($whenflag:ident, $thenflag:ident) => {
        define_value_implication!($whenflag, $thenflag, false);
    };
}

macro_rules! define_neg_neg_implication {
    ($whenflag:ident, $thenflag:ident) => {
        define_neg_value_implication!($whenflag, $thenflag, false);
    };
}

#[derive(Debug, Clone, Copy)]
pub enum FlagType {
    Bool,
    MaybeBool,
    Int,
    Uint,
    Uint64,
    Float,
    SizeT,
    String,
}

#[derive(Debug, Clone)]
pub struct FlagValue<T> {
    value: T,
}

impl<T> FlagValue<T> {
    pub fn new(value: T) -> Self {
        FlagValue { value }
    }

    pub fn get(&self) -> &T {
        &self.value
    }

    pub fn set(&mut self, value: T) {
        self.value = value;
    }
}

// With FLAG_MODE_DECLARE we declare the fields in the {FlagValues} struct.
// Read-only flags are static constants instead of fields.
#[cfg(feature = "flag_mode_declare")]
macro_rules! flag_full {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        pub static $nam: FlagValue<$ctype> = FlagValue::new($def);
    };
}

#[cfg(feature = "flag_mode_declare")]
macro_rules! flag_readonly {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        pub const $nam: FlagValue<$ctype> = FlagValue::new($def);
    };
}

// We need to define all of our default values so that the Flag structure can
// access them by pointer.  These are just used internally inside of one .cc,
// for MODE_META, so there is no impact on the flags interface.
#[cfg(feature = "flag_mode_define_defaults")]
macro_rules! flag_full {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        const FLAGDEFAULT_$nam: $ctype = $def;
    };
}

#[cfg(feature = "flag_mode_define_defaults")]
macro_rules! flag_readonly {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        const FLAGDEFAULT_$nam: $ctype = $def;
    };
}

// We want to write entries into our meta data table, for internal parsing and
// printing / etc in the flag parser code.
#[cfg(feature = "flag_mode_meta")]
macro_rules! flag_full {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        (
            FlagType::$ftype,
            stringify!($nam),
            /*&v8_flags.$nam*/ std::ptr::null(), //TODO: replace with address of the field
            &FLAGDEFAULT_$nam,
            $cmt,
            false,
        );
    };
}

// Readonly flags don't pass the value pointer since the struct expects a
// mutable value. That's okay since the value always equals the default.
#[cfg(feature = "flag_mode_meta")]
macro_rules! flag_readonly {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        (
            FlagType::$ftype,
            stringify!($nam),
            std::ptr::null(),
            &FLAGDEFAULT_$nam,
            $cmt,
            false,
        );
    };
}

#[cfg(feature = "flag_mode_meta")]
macro_rules! flag_alias {
    ($ftype:ident, $ctype:ty, $alias:ident, $nam:ident) => {
        (
            FlagType::$ftype,
            stringify!($alias),
            /*&v8_flags.$nam*/ std::ptr::null(), //TODO: replace with address of the field
            &FLAGDEFAULT_$nam,
            concat!("alias for --", stringify!($nam)),
            false,
        );
    };
}

// We produce the code to set flags when it is implied by another flag.
#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {
        let mut changed = false;
        changed |= trigger_implication(
            /*v8_flags.$whenflag*/ true, // TODO: replace with the actual flag value
            stringify!($whenflag),
            /*&v8_flags.$thenflag*/ std::ptr::null_mut(), // TODO: replace with the flag address
            stringify!($thenflag),
            $value,
            false,
        );
    };
}

// A weak implication will be overwritten by a normal implication or by an
// explicit flag.
#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_weak_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {
        let mut changed = false;
        changed |= trigger_implication(
            /*v8_flags.$whenflag*/ true, // TODO: replace with the actual flag value
            stringify!($whenflag),
            /*&v8_flags.$thenflag*/ std::ptr::null_mut(), // TODO: replace with the flag address
            stringify!($thenflag),
            $value,
            true,
        );
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_generic_implication {
    ($whenflag:ident, $statement:expr) => {
        if /*v8_flags.$whenflag*/ true {
            // TODO: replace with actual flag value
            $statement
        }
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_requirement {
    ($statement:expr) => {
        if !$statement {
            panic!("Requirement failed: {}", stringify!($statement));
        }
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_neg_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {
        let mut changed = false;
        changed |= trigger_implication(
            /*!v8_flags.$whenflag*/ true, // TODO: replace with the actual flag value
            concat!("!", stringify!($whenflag)),
            /*&v8_flags.$thenflag*/ std::ptr::null_mut(), // TODO: replace with the flag address
            stringify!($thenflag),
            $value,
            false,
        );
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_neg_value_value_implication {
    ($whenflag:ident, $whenvalue:expr, $thenflag:ident, $thenvalue:expr) => {
        let mut changed = false;
        changed |= trigger_implication(
            /*v8_flags.$whenflag != $whenvalue*/ true, // TODO: replace with the actual flag value
            stringify!($whenflag),
            /*&v8_flags.$thenflag*/ std::ptr::null_mut(), // TODO: replace with the flag address
            stringify!($thenflag),
            $thenvalue,
            false,
        );
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_min_value_implication {
    ($flag:ident, $min_value:expr) => {
        let mut changed = false;
        changed |= trigger_implication(
            /*v8_flags.$flag < $min_value*/ true, // TODO: replace with the actual flag value
            stringify!($flag),
            /*&v8_flags.$flag*/ std::ptr::null_mut(), // TODO: replace with the flag address
            stringify!($flag),
            $min_value,
            false,
        );
    };
}

#[cfg(feature = "flag_mode_define_implications")]
macro_rules! define_disable_flag_implication {
    ($whenflag:ident, $thenflag:ident) => {
        if /*v8_flags.$whenflag && v8_flags.$thenflag*/ true {
            // TODO: replace with actual flag values
            eprintln!(
                "Warning: disabling flag --{} due to conflicting flags",
                stringify!($thenflag)
            );
        }
        define_value_implication!($whenflag, $thenflag, false);
    };
}

// We apply a generic macro to the flags.
#[cfg(feature = "flag_mode_apply")]
macro_rules! flag_full {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        FLAG_MODE_APPLY!($ftype, $ctype, $nam, $def, $cmt);
    };
}

// Dummy defines for modes where it is not relevant.
#[cfg(not(feature = "flag_full"))]
macro_rules! flag_full {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {};
}

#[cfg(not(feature = "flag_readonly"))]
macro_rules! flag_readonly {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {};
}

#[cfg(not(feature = "flag_alias"))]
macro_rules! flag_alias {
    ($ftype:ident, $ctype:ty, $alias:ident, $nam:ident) => {};
}

#[cfg(not(feature = "define_value_implication"))]
macro_rules! define_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {};
}

#[cfg(not(feature = "define_weak_value_implication"))]
macro_rules! define_weak_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {};
}

#[cfg(not(feature = "define_generic_implication"))]
macro_rules! define_generic_implication {
    ($whenflag:ident, $statement:expr) => {};
}

#[cfg(not(feature = "define_neg_value_implication"))]
macro_rules! define_neg_value_implication {
    ($whenflag:ident, $thenflag:ident, $value:expr) => {};
}

#[cfg(not(feature = "define_neg_value_value_implication"))]
macro_rules! define_neg_value_value_implication {
    ($whenflag:ident, $whenvalue:expr, $thenflag:ident, $thenvalue:expr) => {};
}

#[cfg(not(feature = "define_min_value_implication"))]
macro_rules! define_min_value_implication {
    ($flag:ident, $min_value:expr) => {};
}

#[cfg(not(feature = "define_disable_flag_implication"))]
macro_rules! define_disable_flag_implication {
    ($whenflag:ident, $thenflag:ident) => {};
}

#[cfg(not(feature = "define_requirement"))]
macro_rules! define_requirement {
    ($statement:expr) => {};
}

// Dummy trigger implication function, replace with actual implementation
fn trigger_implication<T>(
    when_flag: bool,
    when_flag_name: &str,
    then_flag: *mut T,
    then_flag_name: &str,
    value: T,
    weak: bool,
) -> bool
where
    T: PartialEq + Copy,
{
    println!(
        "Trigger implication: {} -> {} = {} (weak: {})",
        when_flag_name, then_flag_name, stringify!(value), weak
    );
    false // Dummy return
}

// Dummy PrintF macro. Replace with actual implementation
macro_rules! PrintF {
    ($stream:expr, $($arg:tt)*) => {
        eprintln!($($arg)*);
    }
}

// Define DEBUG_BOOL
#[cfg(debug_assertions)]
const DEBUG_BOOL: bool = true;
#[cfg(not(debug_assertions))]
const DEBUG_BOOL: bool = false;

// Define ENABLE_SPARKPLUG_BY_DEFAULT
#[cfg(feature = "v8_enable_sparkplug")]
const ENABLE_SPARKPLUG_BY_DEFAULT: bool = true;
#[cfg(not(feature = "v8_enable_sparkplug"))]
const ENABLE_SPARKPLUG_BY_DEFAULT: bool = false;

// Define ARM_ARCH_DEFAULT - The original C++ uses many preprocessor defines for this.
// This example defaults to "armv6"
const ARM_ARCH_DEFAULT: &str = "armv6";

// Define ENABLE_LOG_COLOUR - depending on V8_OS_WIN
#[cfg(target_os = "windows")]
const ENABLE_LOG_COLOUR: bool = false;
#[cfg(not(target_os = "windows"))]
const ENABLE_LOG_COLOUR: bool = true;

macro_rules! define_bool {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!($nam, bool, $def, $cmt);
    };
}

macro_rules! define_bool_readonly {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag_readonly!(Bool, bool, $nam, $def, $cmt);
    };
}

macro_rules! define_maybe_bool {
    ($nam:ident, $cmt:expr) => {
        flag!(MaybeBool, Option<bool>, $nam, None, $cmt);
    };
}

macro_rules! define_int {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(Int, i32, $nam, $def, $cmt);
    };
}

macro_rules! define_uint {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(Uint, u32, $nam, $def, $cmt);
    };
}

macro_rules! define_uint_readonly {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag_readonly!(Uint, u32, $nam, $def, $cmt);
    };
}

macro_rules! define_uint64 {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(Uint64, u64, $nam, $def, $cmt);
    };
}

macro_rules! define_float {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(Float, f64, $nam, $def, $cmt);
    };
}

macro_rules! define_size_t {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(SizeT, usize, $nam, $def, $cmt);
    };
}

macro_rules! define_string {
    ($nam:ident, $def:expr, $cmt:expr) => {
        flag!(String, &'static str, $nam, $def, $cmt);
    };
}

macro_rules! define_alias_bool {
    ($alias:ident, $nam:ident) => {
        flag_alias!(Bool, bool, $alias, $nam);
    };
}

macro_rules! define_alias_int {
    ($alias:ident, $nam:ident) => {
        flag_alias!(Int, i32, $alias, $nam);
    };
}

macro_rules! define_alias_float {
    ($alias:ident, $nam:ident) => {
        flag_alias!(Float, f64, $alias, $nam);
    };
}

macro_rules! define_alias_size_t {
    ($alias:ident, $nam:ident) => {
        flag_alias!(SizeT, usize, $alias, $nam);
    };
}

macro_rules! define_alias_string {
    ($alias:ident, $nam:ident) => {
        flag_alias!(String, &'static str, $alias, $nam);
    };
}

#[cfg(debug_assertions)]
macro_rules! define_debug_bool {
    ($nam:ident, $def:expr, $cmt:expr) => {
        define_bool!($nam, $def, $cmt);
    };
}

#[cfg(not(debug_assertions))]
macro_rules! define_debug_bool {
    ($nam:ident, $def:expr, $cmt:expr) => {
        define_bool_readonly!($nam, $def, $cmt);
    };
}

//
// Flags in all modes.
//
macro_rules! flag {
    ($ftype:ident, $ctype:ty, $nam:ident, $def:expr, $cmt:expr) => {
        flag_full!($ftype, $ctype, $nam, $def, $cmt);
    };
}

// Experimental features.
// Features that are still considered experimental and which are not ready for
// fuzz testing should be defined using this macro. The feature will then imply
// --experimental, which will indicate to the user that they are running an
// experimental configuration of V8. Experimental features are always disabled
// by default. When these features mature, the flag should first turn into a
// regular feature flag (still disabled by default) and then ideally be staged
// behind (for example) --future before being enabled by default.
define_bool!(
    experimental,
    false,
    "Indicates that V8 is running with experimental features enabled. \
     This flag is typically not set explicitly but instead enabled as \
     an implication of other flags which enable experimental features."
);

macro_rules! define_experimental_feature {
    ($nam:ident, $cmt:expr) => {
        flag!(Bool, bool, $nam, false, concat!($cmt, " (experimental)"));
        define_implication!($nam, experimental);
    };
}

// ATTENTION: This is set to true by default in d8. But for API compatibility,
// it generally defaults to false.
define_bool!(
    abort_on_contradictory_flags,
    false,
    "Disallow flags or implications overriding each other."
);
// This implication is also hard-coded into the flags processing to make sure it
// becomes active before we even process subsequent flags.
define_neg_implication!(fuzzing, abort_on_contradictory_flags);
// As abort_on_contradictory_flags, but it will simply exit with return code 0.
define_bool!(
    exit_on_contradictory_flags,
    false,
    "Exit with return code 0 on contradictory flags."
);
// We rely on abort_on_contradictory_flags to turn on the analysis.
define_weak_implication!(exit_on_contradictory_flags, abort_on_contradictory_flags);
// This is not really a flag, it affects the interpretation of the next flag but
// doesn't become permanently true when specified. This only works for flags
// defined in this file, but not for d8 flags defined in src/d8/d8.cc.
define_bool!(
    allow_overwriting_for_next_flag,
    false,
    "temporary disable flag contradiction to allow overwriting just \
     the next flag"
);

// Flags for language modes and experimental language features.
define_bool!(use_strict, false, "enforce strict mode");

define_bool!(trace_temporal, false, "trace temporal code");

define_bool!(harmony, false, "enable all completed harmony features");
define_bool!(harmony_shipping, true, "enable all shipped harmony features");

define_bool!(js_staging, false, "enable all completed JavaScript features");
define_bool!(js_shipping, true, "enable all shipped JavaScript features");

// Update bootstrapper.cc whenever adding a new feature flag.

// Features that are still work in progress (behind individual flags).
//
// The "harmony" naming is now outdated and will no longer be used for new JS
// features. Use the JAVASCRIPT macros instead.
//
// TODO(v8:14214): Remove --harmony flags once transition is complete.
macro_rules! harmony_inprogress_base {
    ($V:ident) => {
        $V!(harmony_temporal, "Temporal");
        $V!(harmony_shadow_realm, "harmony ShadowRealm");
        $V!(harmony_struct, "harmony structs, shared structs, and shared arrays");
    };
}

macro_rules! javascript_inprogress_features_base {
    ($V:ident) => {
        $V!(js_decorators, "decorators");
        $V!(js_source_phase_imports, "source phase imports");
        $V!(js_base_64, "Uint8Array to/from base64 and hex");
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! harmony_inprogress {
    ($V:ident) => {
        harmony_inprogress_base!($V);
        $V!(harmony_intl_best_fit_matcher, "Intl BestFitMatcher");
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! javascript_inprogress_features {
    ($V:ident) => {
        javascript_inprogress_features_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! harmony_inprogress {
    ($V:ident) => {
        harmony_inprogress_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! javascript_inprogress_features {
    ($V:ident) => {
        javascript_inprogress_features_base!($V);
    };
}

// Features that are complete (but still behind the --harmony flag).
macro_rules! harmony_staged_base {
    ($V:ident) => {};
}

macro_rules! javascript_staged_features_base {
    ($V:ident) => {
        $V!(
            js_explicit_resource_management,
            "explicit resource management"
        );
        $V!(
            js_float16array,
            "Float16Array, Math.f16round, DataView.getFloat16, DataView.setFloat16"
        );
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! harmony_staged {
    ($V:ident) => {
        harmony_staged_base!($V);
        $V!(
            harmony_remove_intl_locale_info_getters,
            "Remove Obsoleted Intl Locale Info getters"
        );
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! javascript_staged_features {
    ($V:ident) => {
        javascript_staged_features_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! harmony_staged {
    ($V:ident) => {
        harmony_staged_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! javascript_staged_features {
    ($V:ident) => {
        javascript_staged_features_base!($V);
    };
}

// Features that are shipping (turned on by default, but internal flag remains).
macro_rules! harmony_shipping_base {
    ($V:ident) => {
        $V!(harmony_iterator_helpers, "JavaScript iterator helpers");
        $V!(harmony_set_methods, "harmony Set Methods");
        $V!(harmony_import_attributes, "harmony import attributes");
    };
}

macro_rules! javascript_shipping_features_base {
    ($V:ident) => {
        $V!(
            js_regexp_duplicate_named_groups,
            "RegExp duplicate named groups"
        );
        $V!(js_regexp_modifiers, "RegExp modifiers");
        $V!(js_promise_try, "Promise.try");
        $V!(js_atomics_pause, "Atomics.pause");
        $V!(js_error_iserror, "Error.isError");
        $V!(js_regexp_escape, "RegExp.escape");
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! harmony_shipping {
    ($V:ident) => {
        harmony_shipping_base!($V);
    };
}

#[cfg(feature = "v8_intl_support")]
macro_rules! javascript_shipping_features {
    ($V:ident) => {
        javascript_shipping_features_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! harmony_shipping {
    ($V:ident) => {
        harmony_shipping_base!($V);
    };
}

#[cfg(not(feature = "v8_intl_support"))]
macro_rules! javascript_shipping_features {
    ($V:ident) => {
        javascript_shipping_features_base!($V);
    };
}

// Once a shipping feature has proved stable in the wild, it will be dropped
// from HARMONY_SHIPPING, all occurrences of the FLAG_ variable are removed,
// and associated tests are moved from the harmony directory to the appropriate
// esN directory.
//
// In-progress features are not code complete and are considered experimental,
// i.e. not ready for fuzz testing.

macro_rules! flag_inprogress_features {
    ($id:ident, $description:expr) => {
        define_bool!(
            $id,
            false,
            concat!("enable ", $description, " (in progress / experimental)")
        );
        define_implication!($id, experimental);
    };
}

harmony_inprogress!(flag_inprogress_features);
javascript_inprogress_features!(flag_inprogress_features);
undefine_macro!(flag_inprogress_features);

macro_rules! flag_staged_features {
    ($id:ident, $description:expr) => {
        define_bool!($id, false, concat!("enable ", $description));
        define_implication!(harmony, $id);
        define_implication!(js_staging, $id);
    };
}

harmony_staged!(flag_staged_features);
javascript_staged_features!(flag_staged_features);
define_implication!(harmony, js_staging);
undefine_macro!(flag_staged_features);

macro_rules! flag_shipping_features {
    ($id:ident, $description:expr) => {
        define_bool!($id, true, concat!("enable ", $description));
        define_neg_neg_implication!(harmony_shipping, $id);
        define_neg_neg_implication!(js_shipping, $id);
    };
}

harmony_shipping!(flag_shipping_features);
javascript_shipping_features!(flag_shipping_features);
define_neg_neg_implication!(harmony_shipping, js_shipping);
undefine_macro!(flag_shipping_features);

define_bool!(
    builtin_subclassing,
    true,
    "subclassing support in built-in methods"
);

// If the following flag is set to `true`, the SharedArrayBuffer constructor is
// enabled per context depending on the callback set via
// `SetSharedArrayBufferConstructorEnabledCallback`. If no callback is set, the
// SharedArrayBuffer constructor is disabled.
define_bool!(
    enable_sharedarraybuffer_per_context,
    false,
    "enable the SharedArrayBuffer constructor per context"
);

#[cfg(feature = "v8_intl_support")]
define_bool!(
    icu_timezone_data,
    true,
    "get information about timezones from ICU"
);

#[cfg(feature = "v8_enable_double_const_store_check")]
const V8_ENABLE_DOUBLE_CONST_STORE_CHECK_BOOL: bool = true;
#[cfg(not(feature = "v8_enable_double_const_store_check"))]
const V8_ENABLE_DOUBLE_CONST_STORE_CHECK_BOOL: bool = false;

#[cfg(feature = "v8_enable_lazy_source_positions")]
const V8_LAZY_SOURCE_POSITIONS_BOOL: bool = true;
#[cfg(not(feature = "v8_enable_lazy_source_positions"))]
const V8_LAZY_SOURCE_POSITIONS_BOOL: bool = false;

define_bool!(
    stress_snapshot,
    false,
    "disables sharing of the read-only heap for testing"
);
// Incremental marking is incompatible with the stress_snapshot mode;
// specifically, serialization may clear bytecode arrays from shared function
// infos which the MarkCompactCollector (running concurrently) may still need.
// See also https://crbug.com/v8/10882.
//
// Note: This is not an issue in production because we don't clear SFI's
// there (that only happens in mksnapshot and in --stress-snapshot mode).
define_neg_implication!(stress_snapshot, incremental_marking);

#[cfg(feature = "v8_lite_mode")]
const V8_LITE_MODE_BOOL: bool = true;
#[cfg(not(feature = "v8_lite_mode"))]
const V8_LITE_MODE_BOOL: bool = false;

define_bool!(
    lite_mode,
    V8_LITE_MODE_BOOL,
    "enables trade-off of performance for memory savings"
);

// Lite mode implies other flags to trade-off performance for memory.
define_implication!(lite_mode, jitless);
define_implication!(lite_mode, optimize_for_size);

#[cfg(feature = "v8_allocation_folding")]
const V8_ALLOCATION_FOLDING_BOOL: bool = true;
#[cfg(not(feature = "v8_allocation_folding"))]
const V8_ALLOCATION_FOLDING_BOOL: bool = false;

define_bool_readonly!(
    enable_allocation_folding,
    V8_ALLOCATION_FOLDING_BOOL,
    "Use allocation folding globally"
);
define_neg_neg_implication!(enable_allocation_folding, turbo_allocation_folding);

#[cfg(feature = "v8_disable_write_barriers")]
const V8_DISABLE_WRITE_BARRIERS_BOOL: bool = true;
#[cfg(not(feature = "v8_disable_write_barriers"))]
const V8_DISABLE_WRITE_BARRIERS_BOOL: bool = false;

define_bool_readonly!(
    disable_write_barriers,
    V8_DISABLE_WRITE_BARRIERS_BOOL,
    "disable write barriers when GC is non-incremental \
     and heap contains single generation."
);

// Disable incremental marking barriers
define_neg_implication!(disable_write_barriers, incremental_marking);
define_neg_implication!(disable_write_barriers, concurrent_marking);
define_neg_implication!(disable_write_barriers, cppheap_incremental_marking);
define_neg_implication!(disable_write_barriers, cppheap_concurrent_marking);

#[cfg(feature = "v8_enable_unconditional_write_barriers")]
const V8_ENABLE_UNCONDITIONAL_WRITE_BARRIERS_BOOL: bool = true;
#[cfg(not(feature = "v8_enable_unconditional_write_barriers"))]
const V8_ENABLE_UNCONDITIONAL_WRITE_BARRIERS_BOOL: bool = false;

define_bool_readonly!(
    enable_unconditional_write_barriers,
    V8_ENABLE_UNCONDITIONAL_WRITE_BARRIERS_BOOL,
    "always use full write barriers"
);

#[cfg(feature = "v8_enable_single_generation")]
const V8_SINGLE_GENERATION_BOOL: bool = true;
#[cfg(not(feature = "v8_enable_single_generation"))]
const V8_SINGLE_GENERATION_BOOL: bool = false;

define_bool_readonly