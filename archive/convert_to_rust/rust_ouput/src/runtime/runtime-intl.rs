// Converted from V8 C++ source files:
// Header: N/A
// Implementation: runtime-intl.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod internal {
    use std::rc::Rc;

    pub struct V8 {}
    pub struct Isolate {}
    pub struct HandleScope {}
    pub struct String {}
    pub struct Object {}
    pub struct FixedArray {}

    impl String {
        pub fn Flatten(_isolate: &Isolate, s: &String) -> Rc<String> {
            Rc::new(String {})
        }
    }

    impl Isolate {
        pub fn CountUsage(&self, _feature: UseCounterFeature) {}
    }

    pub enum UseCounterFeature {
        kStringToLocaleLowerCase,
    }

    pub struct JSListFormat {}
    impl JSListFormat {
        pub fn FormatList(
            _isolate: &Isolate,
            _list_format: &JSListFormat,
            _list: &FixedArray,
        ) -> Result<String, String> {
            Ok(String {})
        }

        pub fn FormatListToParts(
            _isolate: &Isolate,
            _list_format: &JSListFormat,
            _list: &FixedArray,
        ) -> Result<String, String> {
            Ok(String {})
        }
    }

    pub struct Intl {}
    impl Intl {
        pub fn ConvertToLower(_isolate: &Isolate, s: &String) -> Result<String, String> {
            Ok(String {})
        }

        pub fn ConvertToUpper(_isolate: &Isolate, s: &String) -> Result<String, String> {
            Ok(String {})
        }

        pub fn StringLocaleConvertCase(
            _isolate: &Isolate,
            s: &String,
            _to_lower: bool,
            _locale: &Object,
        ) -> Result<String, String> {
            Ok(String {})
        }
    }

    pub struct DirectHandle<T> {
        value: T,
    }

    impl<T> DirectHandle<T> {
        pub fn new(value: T) -> Self {
            DirectHandle { value }
        }
    }

    macro_rules! RETURN_RESULT_OR_FAILURE {
        ($isolate:expr, $result:expr) => {
            match $result {
                Ok(value) => value,
                Err(err) => {
                    return err;
                }
            }
        };
    }

    pub fn Runtime_FormatList(args: &[&dyn std::any::Any]) -> String {
        let isolate = &Isolate {};
        let list_format = args[0].downcast_ref::<DirectHandle<JSListFormat>>().unwrap();
        let list = args[1].downcast_ref::<DirectHandle<FixedArray>>().unwrap();
        RETURN_RESULT_OR_FAILURE!(
            isolate,
            JSListFormat::FormatList(
                isolate,
                &list_format.value,
                &list.value
            )
        )
    }

    pub fn Runtime_FormatListToParts(args: &[&dyn std::any::Any]) -> String {
        let isolate = &Isolate {};
        let list_format = args[0].downcast_ref::<DirectHandle<JSListFormat>>().unwrap();
        let list = args[1].downcast_ref::<DirectHandle<FixedArray>>().unwrap();
        RETURN_RESULT_OR_FAILURE!(
            isolate,
            JSListFormat::FormatListToParts(
                isolate,
                &list_format.value,
                &list.value
            )
        )
    }

    pub fn Runtime_StringToLowerCaseIntl(args: &[&dyn std::any::Any]) -> String {
        let isolate = &Isolate {};
        let s = args[0].downcast_ref::<String>().unwrap();
        let s = String::Flatten(isolate, s);
        RETURN_RESULT_OR_FAILURE!(isolate, Intl::ConvertToLower(isolate, &s))
    }

    pub fn Runtime_StringToUpperCaseIntl(args: &[&dyn std::any::Any]) -> String {
        let isolate = &Isolate {};
        let s = args[0].downcast_ref::<String>().unwrap();
        let s = String::Flatten(isolate, s);
        RETURN_RESULT_OR_FAILURE!(isolate, Intl::ConvertToUpper(isolate, &s))
    }

    pub fn Runtime_StringToLocaleLowerCase(args: &[&dyn std::any::Any]) -> String {
        let isolate = &Isolate {};
        let s = args[0].downcast_ref::<DirectHandle<String>>().unwrap();
        let locale = args[1].downcast_ref::<DirectHandle<Object>>().unwrap();

        isolate.CountUsage(UseCounterFeature::kStringToLocaleLowerCase);

        RETURN_RESULT_OR_FAILURE!(
            isolate,
            Intl::StringLocaleConvertCase(
                isolate,
                &s.value,
                false,
                &locale.value
            )
        )
    }
}
