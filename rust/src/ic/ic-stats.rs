use lazy_static::lazy_static;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Mutex;
use tracing::{event, Level};

const MAX_IC_INFO: usize = 256;

lazy_static! {
    static ref IC_STATS_INSTANCE: ICStats = ICStats::new();
}

pub struct ICStats {
    ic_infos: Mutex<[ICInfo; MAX_IC_INFO]>,
    pos: AtomicUsize,
    enabled: AtomicBool,
    script_name_map: Mutex<HashMap<usize, Box<str>>>,
    function_name_map: Mutex<HashMap<usize, Box<str>>>,
}

impl ICStats {
    fn new() -> Self {
        ICStats {
            ic_infos: Mutex::new([ICInfo::new(); MAX_IC_INFO]),
            pos: AtomicUsize::new(0),
            enabled: AtomicBool::new(false),
            script_name_map: Mutex::new(HashMap::new()),
            function_name_map: Mutex::new(HashMap::new()),
        }
    }

    pub fn begin(&self) {
        if !tracing::enabled!(Level::TRACE) {
            return;
        }
        self.enabled.store(true, Ordering::Relaxed);
    }

    pub fn end(&self) {
        if !self.enabled.load(Ordering::Relaxed) {
            return;
        }
        let mut pos = self.pos.load(Ordering::Relaxed);
        pos += 1;
        self.pos.store(pos, Ordering::Relaxed);
        if pos == MAX_IC_INFO {
            self.dump();
        }
        self.enabled.store(false, Ordering::Relaxed);
    }

    fn reset(&self) {
        let mut ic_infos = self.ic_infos.lock().unwrap();
        for ic_info in ic_infos.iter_mut() {
            ic_info.reset();
        }
        self.pos.store(0, Ordering::Relaxed);
    }

    fn dump(&self) {
        let mut value = serde_json::json!({
            "data": []
        });

        let pos = self.pos.load(Ordering::Relaxed);
        let ic_infos = self.ic_infos.lock().unwrap();
        let mut data_array = Vec::new();
        for i in 0..pos {
            data_array.push(ic_infos[i].to_json());
        }
        value["data"] = serde_json::json!(data_array);

        event!(
            Level::TRACE,
            target: "v8.ic_stats",
            V8_ICStats = %value.to_string(),
        );

        self.reset();
    }

    // Need to figure out Tagged<Script> and Tagged<Object> to implement
    pub fn get_or_cache_script_name(&self, _script_ptr: usize) -> Option<Box<str>> {
        // let mut script_name_map = self.script_name_map.lock().unwrap();
        // if let Some(name) = script_name_map.get(&script_ptr) {
        //     return name.clone();
        // }

        // Placeholder implementation
        None
    }

    // Need to figure out IsolateForSandbox and Tagged<JSFunction> to implement
    pub fn get_or_cache_function_name(&self, _function_ptr: usize, _is_optimized: bool) -> Option<Box<str>> {
        // let mut function_name_map = self.function_name_map.lock().unwrap();
        // if let Some(name) = function_name_map.get(&function_ptr) {
        //     return name.clone();
        // }

        // Placeholder implementation
        None
    }
}

pub struct ICInfo {
    type_: RefCell<String>,
    function_name: RefCell<Option<Box<str>>>,
    script_offset: usize,
    script_name: RefCell<Option<Box<str>>>,
    line_num: i32,
    column_num: i32,
    is_constructor: bool,
    is_optimized: bool,
    state: RefCell<String>,
    map: usize,
    is_dictionary_map: bool,
    number_of_own_descriptors: usize,
    instance_type: RefCell<String>,
}

impl ICInfo {
    fn new() -> Self {
        ICInfo {
            type_: RefCell::new(String::new()),
            function_name: RefCell::new(None),
            script_offset: 0,
            script_name: RefCell::new(None),
            line_num: -1,
            column_num: -1,
            is_constructor: false,
            is_optimized: false,
            state: RefCell::new(String::new()),
            map: 0,
            is_dictionary_map: false,
            number_of_own_descriptors: 0,
            instance_type: RefCell::new(String::new()),
        }
    }

    fn reset(&self) {
        self.type_.borrow_mut().clear();
        *self.function_name.borrow_mut() = None;
        self.script_offset = 0;
        *self.script_name.borrow_mut() = None;
        self.line_num = -1;
        self.column_num = -1;
        self.is_constructor = false;
        self.is_optimized = false;
        self.state.borrow_mut().clear();
        self.map = 0;
        self.is_dictionary_map = false;
        self.number_of_own_descriptors = 0;
        self.instance_type.borrow_mut().clear();
    }

    fn to_json(&self) -> serde_json::Value {
        let mut json = serde_json::json!({});

        json["type"] = serde_json::json!(self.type_.borrow().clone());

        if let Some(ref function_name) = *self.function_name.borrow() {
            json["functionName"] = serde_json::json!(function_name.to_string());
            if self.is_optimized {
                json["optimized"] = serde_json::json!(self.is_optimized);
            }
        }

        if self.script_offset != 0 {
            json["offset"] = serde_json::json!(self.script_offset);
        }

        if let Some(ref script_name) = *self.script_name.borrow() {
            json["scriptName"] = serde_json::json!(script_name.to_string());
        }

        if self.line_num != -1 {
            json["lineNum"] = serde_json::json!(self.line_num);
        }

        if self.column_num != -1 {
            json["columnNum"] = serde_json::json!(self.column_num);
        }

        if self.is_constructor {
            json["constructor"] = serde_json::json!(self.is_constructor);
        }

        if !self.state.borrow().is_empty() {
            json["state"] = serde_json::json!(self.state.borrow().clone());
        }

        if self.map != 0 {
            json["map"] = serde_json::json!(format!("{}", self.map));
            json["dict"] = serde_json::json!(self.is_dictionary_map);
            json["own"] = serde_json::json!(self.number_of_own_descriptors);
        }

        if !self.instance_type.borrow().is_empty() {
            json["instanceType"] = serde_json::json!(self.instance_type.borrow().clone());
        }

        json
    }
}