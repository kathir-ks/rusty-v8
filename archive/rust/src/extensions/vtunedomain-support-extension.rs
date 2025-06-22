// Copyright 2019 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

mod v8_api {
    pub struct Isolate {}
    pub struct FunctionCallbackInfo {}
    pub struct Value {}
    pub struct String {}
    pub struct FunctionTemplate {}

    impl Isolate {
        pub fn throw_error(&self, _message: &str) {}
        pub fn new_string_from_utf8(&self, _data: &str) -> String {
            String {}
        }
    }

    impl FunctionCallbackInfo {
        pub fn length(&self) -> usize {
            0
        }
        pub fn get_isolate(&self) -> &Isolate {
            &Isolate {}
        }
    }

    impl String {
        pub fn to_string(&self) -> String {
            String {}
        }
    }

    impl FunctionTemplate {
        pub fn new(_isolate: &Isolate, _callback: fn(_: &FunctionCallbackInfo)) -> FunctionTemplate {
            FunctionTemplate {}
        }
    }
}

mod vtune_domain {
    use std::sync::{Arc, Mutex};

    #[derive(Debug, PartialEq)]
    pub enum VTuneError {
        DomainNameMissing,
        TaskNameMissing,
        CreateDomainFailed,
        TaskBeginFailed,
        UnknownParams,
    }

    pub struct VTuneDomain {
        name: String,
    }

    impl VTuneDomain {
        pub fn create_domain(name: &str) -> Result<Arc<Mutex<VTuneDomain>>, VTuneError> {
            Ok(Arc::new(Mutex::new(VTuneDomain {
                name: name.to_string(),
            })))
        }

        pub fn begin_task(&self, _task_name: &str) -> Result<(), VTuneError> {
            // Simulate task start
            Ok(())
        }

        pub fn end_task(&self) {
            // Simulate task end
        }
    }
}

use vtune_domain::{VTuneDomain, VTuneError};

// Error codes
const NO_DOMAIN_NAME: i32 = 1;
const NO_TASK_NAME: i32 = 2;
const CREATE_DOMAIN_FAILED: i32 = 3;
const TASK_BEGIN_FAILED: i32 = 4;
const UNKNOWN_PARAMS: i32 = 5;

lazy_static::lazy_static! {
    static ref FUNCTION_MAP: HashMap<String, fn(&Vec<String>) -> Result<i32, VTuneError>> = {
        let mut m = HashMap::new();
        m.insert("start".to_string(), start_task as fn(&Vec<String>) -> Result<i32, VTuneError>);
        m.insert("end".to_string(), end_task as fn(&Vec<String>) -> Result<i32, VTuneError>);
        m
    };
}

fn split(str: &str, delimiter: char, vparams: &mut Vec<String>) {
    let mut baseindex = 0;
    if str.is_empty() {
        return;
    }
    let mut offindex = str[baseindex..].find(delimiter);

    while let Some(off) = offindex {
        vparams.push(str[baseindex..baseindex + off].to_string());
        baseindex += off + 1;
        offindex = str[baseindex..].find(delimiter);
    }

    vparams.push(str[baseindex..].to_string());
}

fn start_task(vparams: &Vec<String>) -> Result<i32, VTuneError> {
    if vparams.len() < 3 {
        return Err(VTuneError::TaskNameMissing);
    }

    let domain_name = &vparams[1];
    let task_name = &vparams[2];

    let domainptr = VTuneDomain::create_domain(domain_name)?;

    match domainptr.lock().unwrap().begin_task(task_name) {
        Ok(_) => Ok(0),
        Err(_) => Ok(TASK_BEGIN_FAILED),
    }
}

fn end_task(vparams: &Vec<String>) -> Result<i32, VTuneError> {
    if vparams.len() < 2 {
        return Err(VTuneError::DomainNameMissing);
    }

    let domain_name = &vparams[1];

    let _domainptr = VTuneDomain::create_domain(domain_name)?;
    //domainptr.lock().unwrap().end_task();
    Ok(0)
}

fn invoke(params: &str) -> Result<i32, VTuneError> {
    let mut vparams = Vec::new();
    split(params, ' ', &mut vparams);

    let func = FUNCTION_MAP.get(vparams.get(0).unwrap_or(&"".to_string()));

    match func {
        Some(f) => f(&vparams),
        None => Err(VTuneError::UnknownParams),
    }
}

pub struct VTuneDomainSupportExtension {}

impl VTuneDomainSupportExtension {
    pub fn get_native_function_template(
        _isolate: &v8_api::Isolate,
        _str: &v8_api::String,
    ) -> v8_api::FunctionTemplate {
        v8_api::FunctionTemplate::new(_isolate, VTuneDomainSupportExtension::mark)
    }

    // info should take three parameters
    // %0 : string, which is the domain name. Domain is used to tagging trace data
    // for different modules or libraryies in a program
    // %1 : string, which is the task name. Task is a logical unit of work performed
    // by a particular thread statement. Task can nest.
    // %2 : string, "start" / "end". Action to be taken on a task in a particular
    // domain
    pub fn mark(info: &v8_api::FunctionCallbackInfo) {
        if info.length() != 3 {
            info.get_isolate().throw_error(
                "Parameter number should be exactly three, first domain name second task name, third start/end",
            );
            return;
        }

        let isolate = info.get_isolate();
        //v8::String::Utf8Value domainName(isolate, info[0]);
        //v8::String::Utf8Value taskName(isolate, info[1]);
        //v8::String::Utf8Value statName(isolate, info[2]);

        //char* cdomainName = *domainName;
        //char* ctaskName = *taskName;
        //char* cstatName = *statName;
        let cstatName = "start";
        let cdomainName = "domain";
        let ctaskName = "task";

        let params = format!("{} {} {}", cstatName, cdomainName, ctaskName);

        let r = invoke(&params);
        match r {
            Ok(0) => {},
            Ok(error_code) => {
                let error_string = error_code.to_string();
                isolate.throw_error(&error_string);
            },
            Err(err) => {
                let error_string = format!("{:?}", err);
                isolate.throw_error(&error_string);
            }
        }
    }
}