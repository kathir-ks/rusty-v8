// Converted from V8 C++ source files:
// Header: template-objects.h
// Implementation: template-objects.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod template_objects {
    use crate::objects::fixed_array::FixedArray;
    use crate::objects::structs::Struct;
    use crate::objects::js_objects::JSArray;
    use crate::objects::shared_function_info::SharedFunctionInfo;
    use crate::objects::script::Script;
    use crate::objects::ephemeron_hash_table::EphemeronHashTable;
    use crate::objects::object::Object;
    use crate::objects::array_list::ArrayList;
    use crate::objects::native_context::NativeContext;
    use crate::objects::smi::Smi;
    use crate::objects::js_receiver::JSReceiver;
    use crate::objects::heap_object::HeapObject;
    use crate::execution::isolate::Isolate;
    use crate::heap::factory::Factory;
    use crate::objects::objects::TemplateLiteralObject;
    use crate::ReadOnlyRoots;
    use std::cell::RefCell;
    use std::rc::Rc;

    pub struct TemplateObjectDescription {
        raw_strings: Rc<RefCell<Option<FixedArray>>>,
        cooked_strings: Rc<RefCell<Option<FixedArray>>>,
        struct_base: Struct,
    }

    impl TemplateObjectDescription {
        pub fn new(isolate: &Isolate) -> TemplateObjectDescription {
            TemplateObjectDescription {
                raw_strings: Rc::new(RefCell::new(None)),
                cooked_strings: Rc::new(RefCell::new(None)),
                struct_base: Struct::new(isolate),
            }
        }

        pub fn raw_strings(&self) -> FixedArray {
            self.raw_strings.borrow().as_ref().unwrap().clone()
        }

        pub fn cooked_strings(&self) -> FixedArray {
            self.cooked_strings.borrow().as_ref().unwrap().clone()
        }

        pub fn set_raw_strings(&self, raw_strings: FixedArray) {
            *self.raw_strings.borrow_mut() = Some(raw_strings);
        }

        pub fn set_cooked_strings(&self, cooked_strings: FixedArray) {
            *self.cooked_strings.borrow_mut() = Some(cooked_strings);
        }

        pub fn get_template_object(
            isolate: &Isolate,
            native_context: &NativeContext,
            description: &TemplateObjectDescription,
            shared_info: &SharedFunctionInfo,
            slot_id: i32,
        ) -> Result<JSArray, String> {
            let function_literal_id = shared_info.function_literal_id();

            let script = match shared_info.script(isolate) {
                Some(script) => script,
                None => return Err("Script not found".to_string()),
            };

            let hash = EphemeronHashTable::todo_shape_hash(ReadOnlyRoots(isolate as *mut Isolate), &script);

            let maybe_cached_templates: Option<ArrayList>;

            if native_context.template_weakmap().is_some() {
                let template_weakmap = native_context.template_weakmap().as_ref().unwrap();

                let cached_templates_lookup = template_weakmap.lookup(isolate, &script, hash);
                if cached_templates_lookup.is_none() {
                    maybe_cached_templates = None;
                } else {
                    let cached_templates_lookup = cached_templates_lookup.unwrap();
                    let cached_templates = match cached_templates_lookup {
                        Object::ArrayList(array_list) => array_list,
                        _ => return Err("cached_templates_lookup is not an ArrayList".to_string()),
                    };
                    maybe_cached_templates = Some(cached_templates.clone());

                    if let Some(cached_templates) = &maybe_cached_templates {
                        for i in 0..cached_templates.length() {
                            if let Object::JSArray(template_object) = cached_templates.get(i).clone() {
                                if Self::cached_template_matches(
                                    isolate,
                                    native_context,
                                    &template_object,
                                    function_literal_id,
                                    slot_id,
                                ) {
                                    return Ok(template_object.clone());
                                }
                            }
                        }
                    }
                }

            } else {
                maybe_cached_templates = None;
            }

            let raw_strings = description.raw_strings();
            let cooked_strings = description.cooked_strings();
            let template_object = Factory::new(isolate).new_js_array_for_template_literal_array(
                &cooked_strings,
                &raw_strings,
                function_literal_id,
                slot_id,
            );

            let cached_templates: Rc<RefCell<ArrayList>>;
            if maybe_cached_templates.is_none() {
                cached_templates = Rc::new(RefCell::new(Factory::new(isolate).new_array_list(1)));
            } else {
                cached_templates = Rc::new(RefCell::new(maybe_cached_templates.unwrap()));
            }

            let mut cached_templates_mut = cached_templates.borrow_mut();
            *cached_templates_mut = ArrayList::add(isolate, &cached_templates_mut, &Object::JSArray(template_object.clone()));

            let mut old_cached_templates: Option<ArrayList> = None;
            if let Some(old_templates) = maybe_cached_templates.as_ref() {
                old_cached_templates = Some(old_templates.clone());
            }

            if old_cached_templates.is_none() || old_cached_templates.unwrap() != *cached_templates_mut {
                let mut template_weakmap: Rc<RefCell<EphemeronHashTable>>;
                if native_context.template_weakmap().is_none() {
                    template_weakmap = Rc::new(RefCell::new(EphemeronHashTable::new(isolate, 1)));
                } else {
                    template_weakmap = Rc::new(RefCell::new(native_context.template_weakmap().clone().unwrap()));
                }

                let mut template_weakmap_mut = template_weakmap.borrow_mut();
                *template_weakmap_mut = EphemeronHashTable::put(
                    isolate,
                    &template_weakmap_mut,
                    &script,
                    &Object::ArrayList(cached_templates_mut.clone()),
                    hash,
                );

                native_context.set_template_weakmap(Some(template_weakmap_mut.clone()));
            }

            return Ok(template_object);
        }

        fn cached_template_matches(
            isolate: &Isolate,
            native_context: &NativeContext,
            entry: &JSArray,
            function_literal_id: i32,
            slot_id: i32,
        ) -> bool {
            if let Some(map) = entry.map(isolate) {
                if native_context.is_js_array_template_literal_object_map(&map) {
                   if let Ok(template_object) = TemplateLiteralObject::try_from(Object::JSArray(entry.clone())) {
                        return template_object.function_literal_id() == function_literal_id &&
                            template_object.slot_id() == slot_id;
                    } else {
                        return false;
                    }
                }
            } else {
                return false;
            }

            let template_literal_function_literal_id_symbol = Factory::new(isolate).template_literal_function_literal_id_symbol();
            if let Some(cached_function_literal_id_object) = JSReceiver::get_data_property(isolate, &Object::JSArray(entry.clone()), &template_literal_function_literal_id_symbol) {
                if let Object::Smi(cached_function_literal_id) = cached_function_literal_id_object {
                    if cached_function_literal_id.value() != function_literal_id {
                        return false;
                    }
                } else {
                   return false;
                }
            } else {
                return false;
            }

            let template_literal_slot_id_symbol = Factory::new(isolate).template_literal_slot_id_symbol();
            if let Some(cached_slot_id_object) = JSReceiver::get_data_property(isolate, &Object::JSArray(entry.clone()), &template_literal_slot_id_symbol) {
                if let Object::Smi(cached_slot_id) = cached_slot_id_object {
                    if cached_slot_id.value() != slot_id {
                        return false;
                    }
                }  else {
                    return false;
                }
            } else {
                return false;
            }

            return true;
        }
    }
}
