// Copyright 2006-2008 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::collections::HashMap;
use std::convert::TryInto;
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{Read, Seek, SeekFrom, Write};
use std::os::raw::{c_char, c_int};
use std::path::Path;
use std::ptr;
use std::slice;
use std::sync::Mutex;

//use libc::{remove, printf}; //use libc crate instead of std::os::raw
//use v8_rs as v8; // Assuming a v8-rs crate exists, or a custom binding
//use v8::base;

// Placeholder for v8 related types and functions
mod v8 {
    pub struct StartupData {
        pub data: *const i8,
        pub raw_size: i32,
    }

    impl StartupData {
        pub fn new(data: *const i8, raw_size: i32) -> Self {
            StartupData { data, raw_size }
        }
    }

    pub mod platform {
        pub struct Platform;
        pub fn NewDefaultPlatform() -> Platform {
            Platform
        }
    }

    pub fn InitializeICUDefaultLocation(_argv0: *const i8) {}
    pub fn InitializePlatform(_platform: &platform::Platform) {}
    pub fn Initialize() {}
    pub fn Dispose() {}
    pub fn DisposePlatform() {}

    pub struct Isolate {
        counter_function: Option<Box<dyn Fn(*const i8) -> *mut i32>>,
    }
    impl Isolate {
        pub fn Allocate() -> *mut Isolate {
            Box::into_raw(Box::new(Isolate {
                counter_function: None,
            }))
        }
        pub fn SetCounterFunction(&mut self, f: Option<Box<dyn Fn(*const i8) -> *mut i32>>) {
            self.counter_function = f;
        }
        pub fn Dispose(isolate: *mut Isolate) {
            unsafe {
                drop(Box::from_raw(isolate));
            }
        }
    }

    pub mod SnapshotCreator {
        use super::StartupData;

        pub struct FunctionCodeHandling;
        impl FunctionCodeHandling {
            pub const kClear: i32 = 0;
        }

        pub struct SnapshotCreator {
            isolate: *mut super::Isolate,
            create_params: super::CreateParams,
        }

        impl SnapshotCreator {
            pub fn new(isolate: *mut super::Isolate, create_params: super::CreateParams) -> Self {
                SnapshotCreator {
                    isolate,
                    create_params,
                }
            }
        }
    }

    pub mod ArrayBuffer {
        pub struct Allocator;
        impl Allocator {
            pub fn NewDefaultAllocator() -> Allocator {
                Allocator
            }
        }
    }

    pub struct CreateParams {
        array_buffer_allocator: *mut ArrayBuffer::Allocator,
        constraints: Constraints,
    }

    impl CreateParams {
        pub fn new(array_buffer_allocator: *mut ArrayBuffer::Allocator) -> Self {
            CreateParams {
                array_buffer_allocator,
                constraints: Constraints::default(),
            }
        }
    }

    #[derive(Default)]
    pub struct Constraints {
        code_range_size_in_bytes: usize,
    }

    impl Constraints {
        pub fn set_code_range_size_in_bytes(&mut self, size: usize) {
            self.code_range_size_in_bytes = size;
        }
    }
}

mod base {
    pub struct Vector<T> {
        data: Vec<T>,
    }

    impl<T> Vector<T> {
        pub fn new(data: Vec<T>) -> Self {
            Vector { data }
        }

        pub fn begin(&self) -> *const T {
            self.data.as_ptr()
        }

        pub fn length(&self) -> usize {
            self.data.len()
        }
    }

    impl Vector<u8> {
        pub fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
            unsafe {
                let slice = std::slice::from_raw_parts(ptr, len);
                Vector {
                    data: slice.to_vec(),
                }
            }
        }
    }

    pub mod OS {
        use std::ffi::CString;
        use std::fs::OpenOptions;
        use std::io;

        pub fn FOpen(filename: &str, mode: &str) -> Result<File, io::Error> {
            let c_filename = CString::new(filename).unwrap();
            let mut open_options = OpenOptions::new();

            match mode {
                "rb" => {
                    open_options.read(true);
                }
                "wb" => {
                    open_options.write(true).create(true).truncate(true);
                }
                _ => {
                    return Err(io::Error::new(
                        io::ErrorKind::InvalidInput,
                        "Invalid mode".to_string(),
                    ));
                }
            }

            open_options.open(filename)
        }
    }

    pub fn Fclose(file: Result<File, std::io::Error>) {
        if let Ok(_f) = file {
            // File is closed when it goes out of scope
        }
    }
}

mod codegen {
    pub mod cpu_features {
        pub fn Probe(_flag: bool) {}
    }
}

mod common {
    pub mod globals {
        // Placeholder for globals
    }
}

mod flags {
    pub mod flags {
        use std::sync::Mutex;

        lazy_static::lazy_static! {
            pub static ref V8_FLAGS: Mutex<V8Flags> = Mutex::new(V8Flags::default());
        }

        #[derive(Default, Debug)]
        pub struct V8Flags {
            pub predictable: bool,
            pub use_ic: bool,
            pub profile_deserialization: bool,
            pub startup_src: String,
            pub startup_blob: String,
            pub embedded_src: String,
            pub embedded_variant: String,
            pub static_roots_src: String,
            pub target_arch: String,
            pub target_os: String,
            pub native_code_counters: bool,
            pub verify_snapshot_checksum: bool,
        }

        pub struct FlagList;

        impl FlagList {
            pub struct HelpOptions {
                exit_on_help: bool,
                usage_string: String,
            }

            impl HelpOptions {
                pub fn new(exit_on_help: bool, usage_string: String) -> Self {
                    HelpOptions {
                        exit_on_help,
                        usage_string,
                    }
                }
            }

            pub fn SetFlagsFromCommandLine(
                argc: &mut i32,
                argv: &mut [*mut i8],
                remove_flags: bool,
                help_options: HelpOptions,
            ) -> i32 {
                let mut i = 1;
                let mut result = 0;
                let mut v8_flags = V8_FLAGS.lock().unwrap();

                while i < *argc {
                    let arg = unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };

                    match arg {
                        "--startup-src" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.startup_src = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--startup-blob" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.startup_blob = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--embedded-src" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.embedded_src = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--embedded-variant" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.embedded_variant = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--static-roots-src" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.static_roots_src = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--target-arch" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.target_arch = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--target-os" => {
                            i += 1;
                            if i < *argc {
                                let value =
                                    unsafe { CStr::from_ptr(argv[i as usize]).to_str().unwrap() };
                                v8_flags.target_os = value.to_string();
                            } else {
                                result = 1;
                                break;
                            }
                        }
                        "--native-code-counters" => {
                            v8_flags.native_code_counters = true;
                        }
                        _ => {}
                    }

                    i += 1;
                }

                if result > 0 {
                    println!("{}", help_options.usage_string);
                }

                result
            }
        }

        impl FlagList {
            #[derive(Debug)]
            pub struct Error {}
        }
    }
}

mod snapshot {
    pub mod embedded {
        pub mod embedded_file_writer {
            use super::super::EmbeddedData;
            use crate::flags::flags::V8_FLAGS;
            use std::fs::File;
            use std::io::{self, Write};
            use std::path::Path;

            pub struct EmbeddedFileWriter {
                embedded_file_path: String,
                embedded_variant: String,
                target_arch: String,
                target_os: String,
            }

            impl EmbeddedFileWriter {
                pub fn new() -> Self {
                    EmbeddedFileWriter {
                        embedded_file_path: String::new(),
                        embedded_variant: String::new(),
                        target_arch: String::new(),
                        target_os: String::new(),
                    }
                }

                pub fn set_embedded_file(&mut self, embedded_file: String) {
                    self.embedded_file_path = embedded_file;
                }

                pub fn set_embedded_variant(&mut self, embedded_variant: String) {
                    self.embedded_variant = embedded_variant;
                }

                pub fn set_target_arch(&mut self, target_arch: String) {
                    self.target_arch = target_arch;
                }

                pub fn set_target_os(&mut self, target_os: String) {
                    self.target_os = target_os;
                }

                pub fn WriteEmbedded(&self, embedded_blob: &EmbeddedData) -> Result<(), io::Error> {
                    if self.embedded_file_path.is_empty() {
                        return Ok(());
                    }

                    let path = Path::new(&self.embedded_file_path);
                    let mut file = File::create(&path)?;

                    writeln!(file, "// Autogenerated embedded blob. Do not edit.")?;
                    writeln!(file)?;
                    writeln!(file, "#include \"src/snapshot/embedded/embedded-data.h\"")?;
                    writeln!(file)?;
                    writeln!(file, "namespace v8 {{")?;
                    writeln!(file, "namespace internal {{")?;
                    writeln!(file)?;
                    writeln!(file, "EmbeddedData EmbeddedData::FromBlob() {{")?;
                    writeln!(file, "  return {};", embedded_blob.data)?; // Assuming EmbeddedData has a field 'data' which can be printed
                    writeln!(file, "}}")?;
                    writeln!(file)?;
                    writeln!(file, "}}  // namespace internal")?;
                    writeln!(file, "}}  // namespace v8")?;

                    Ok(())
                }
            }
        }
    }

    pub mod snapshot {
        use super::super::v8::StartupData;
        use std::ptr;

        pub struct Snapshot;

        impl Snapshot {
            pub fn DefaultSnapshotBlob() -> *const StartupData {
                ptr::null() //return null pointer, since actual implementation not available
            }

            pub fn ShouldVerifyChecksum(_data: *const StartupData) -> bool {
                false //return false since v8 flags not implemented
            }
        }
    }

    pub mod static_roots_gen {
        // Placeholder for static roots generation
        pub struct StaticRootsTableGen;

        impl StaticRootsTableGen {
            pub fn write(_i_isolate: usize, _static_roots_src: String) {}
        }
    }
}

struct SnapshotFileWriter {
    snapshot_cpp_path: String,
    snapshot_blob_path: String,
}

impl SnapshotFileWriter {
    fn new() -> Self {
        SnapshotFileWriter {
            snapshot_cpp_path: String::new(),
            snapshot_blob_path: String::new(),
        }
    }

    fn set_snapshot_file(&mut self, snapshot_cpp_file: String) {
        self.snapshot_cpp_path = snapshot_cpp_file;
    }

    fn set_startup_blob_file(&mut self, snapshot_blob_file: String) {
        self.snapshot_blob_path = snapshot_blob_file;
    }

    fn write_snapshot(&self, blob: v8::StartupData) -> Result<(), std::io::Error> {
        let blob_vector = unsafe {
            std::slice::from_raw_parts(blob.data as *const u8, blob.raw_size as usize).to_vec()
        };
        self.maybe_write_snapshot_file(&blob_vector)?;
        self.maybe_write_startup_blob(&blob_vector)?;
        Ok(())
    }

    fn maybe_write_startup_blob(&self, blob: &Vec<u8>) -> Result<(), std::io::Error> {
        if self.snapshot_blob_path.is_empty() {
            return Ok(());
        }

        let file = Self::get_file_descriptor_or_die(&self.snapshot_blob_path)?;
        let mut file = std::io::BufWriter::new(file); //for efficient writing
        let written = file.write_all(blob)?;
        file.flush()?; //flush the buffer to ensure all data is written

        if written.is_err() {
            eprintln!("Writing snapshot file failed.. Aborting.");
            if std::fs::remove_file(&self.snapshot_blob_path).is_ok() {};
            std::process::exit(1);
        }

        Ok(())
    }

    fn maybe_write_snapshot_file(&self, blob: &Vec<u8>) -> Result<(), std::io::Error> {
        if self.snapshot_cpp_path.is_empty() {
            return Ok(());
        }

        let file = Self::get_file_descriptor_or_die(&self.snapshot_cpp_path)?;
        let mut file = std::io::BufWriter::new(file);

        Self::write_snapshot_file_prefix(&mut file)?;
        Self::write_snapshot_file_data(&mut file, blob)?;
        Self::write_snapshot_file_suffix(&mut file)?;
        file.flush()?;

        Ok(())
    }

    fn write_snapshot_file_prefix<W: std::io::Write>(file: &mut W) -> Result<(), std::io::Error> {
        writeln!(file, "// Autogenerated snapshot file. Do not edit.")?;
        writeln!(file)?;
        writeln!(file, "#include \"src/init/v8.h\"")?;
        writeln!(file, "#include \"src/base/platform/platform.h\"")?;
        writeln!(file)?;
        writeln!(file, "#include \"src/flags/flags.h\"")?;
        writeln!(file, "#include \"src/snapshot/snapshot.h\"")?;
        writeln!(file)?;
        writeln!(file, "namespace v8 {{")?;
        writeln!(file, "namespace internal {{")?;
        writeln!(file)?;
        Ok(())
    }

    fn write_snapshot_file_suffix<W: std::io::Write>(file: &mut W) -> Result<(), std::io::Error> {
        writeln!(file, "const v8::StartupData* Snapshot::DefaultSnapshotBlob() {{")?;
        writeln!(file, "  return &blob;")?;
        writeln!(file, "}}")?;
        writeln!(file)?;
        writeln!(
            file,
            "bool Snapshot::ShouldVerifyChecksum(const v8::StartupData* data) {{"
        )?;
        writeln!(
            file,
            "  return crate::flags::flags::V8_FLAGS.lock().unwrap().verify_snapshot_checksum;"
        )?;
        writeln!(file, "}}")?;
        writeln!(file, "}}  // namespace internal")?;
        writeln!(file, "}}  // namespace v8")?;
        Ok(())
    }

    fn write_snapshot_file_data<W: std::io::Write>(
        file: &mut W,
        blob: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        writeln!(
            file,
            "alignas(kPointerAlignment) static const uint8_t blob_data[] = {{"
        )?;
        Self::write_binary_contents_as_c_array(file, blob)?;
        writeln!(file, "}};")?;
        writeln!(file, "static const int blob_size = {};", blob.len())?;
        writeln!(file, "static const v8::StartupData blob =")?;
        writeln!(file, "{{ (const char*) blob_data, blob_size }};")?;
        Ok(())
    }

    fn write_binary_contents_as_c_array<W: std::io::Write>(
        file: &mut W,
        blob: &Vec<u8>,
    ) -> Result<(), std::io::Error> {
        for (i, &byte) in blob.iter().enumerate() {
            if (i & 0x1F) == 0x1F {
                writeln!(file)?;
            }
            if i > 0 {
                write!(file, ",")?;
            }
            write!(file, "{}", byte)?;
        }
        writeln!(file)?;
        Ok(())
    }

    fn get_file_descriptor_or_die(filename: &str) -> Result<File, std::io::Error> {
        match base::OS::FOpen(filename, "wb") {
            Ok(fp) => Ok(fp),
            Err(e) => {
                eprintln!("Unable to open file \"{}\" for writing: {}", filename, e);
                std::process::exit(1);
            }
        }
    }
}

fn get_extra_code(filename: &str, description: &str) -> Option<Vec<u8>> {
    if filename.is_empty() {
        return None;
    }
    println!("Loading script for {}: {}", description, filename);
    let file = base::OS::FOpen(filename, "rb");
    let mut file = match file {
        Ok(file) => file,
        Err(e) => {
            eprintln!("Failed to open '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    let size = file.seek(SeekFrom::End(0)).unwrap();
    file.seek(SeekFrom::Start(0)).unwrap();

    let mut chars: Vec<u8> = Vec::with_capacity(size as usize);
    unsafe {
        chars.set_len(size as usize);
    }

    if let Err(e) = file.read_exact(&mut chars) {
        eprintln!("Failed to read '{}': {}", filename, e);
        std::process::exit(1);
    }

    Some(chars)
}

fn create_snapshot_data_blob(
    snapshot_creator: &mut v8::SnapshotCreator::SnapshotCreator,
    embedded_source: Option<&[u8]>,
) -> v8::StartupData {
    use std::time::Instant;
    let timer = Instant::now();

    let result = create_snapshot_data_blob_internal(
        v8::SnapshotCreator::FunctionCodeHandling::kClear,
        embedded_source,
        snapshot_creator,
    );

    let elapsed = timer.elapsed();
    let profile_deserialization =
        crate::flags::flags::V8_FLAGS.lock().unwrap().profile_deserialization;

    if profile_deserialization {
        println!("[Creating snapshot took {:.3} ms]", elapsed.as_secs_f64() * 1000.0);
    }

    result
}

fn warm_up_snapshot_data_blob(
    cold_snapshot_blob: v8::StartupData,
    warmup_source: Option<&[u8]>,
) -> v8::StartupData {
    use std::time::Instant;
    let timer = Instant::now();

    let result = warm_up_snapshot_data_blob_internal(cold_snapshot_blob, warmup_source);

    let elapsed = timer.elapsed();
    let profile_deserialization =
        crate::flags::flags::V8_FLAGS.lock().unwrap().profile_deserialization;

    if profile_deserialization {
        println!("Warming up snapshot took {:.3} ms", elapsed.as_secs_f64() * 1000.0);
    }

    result
}

// Placeholder functions for internal implementations that are not directly translatable.
fn create_snapshot_data_blob_internal(
    _function_code_handling: i32,
    _embedded_source: Option<&[u8]>,
    _snapshot_creator: &mut v8::SnapshotCreator::SnapshotCreator,
) -> v8::StartupData {
    // This function would contain the V8-specific logic to create the snapshot data blob.
    // Replace with actual implementation if available.
    println!("create_snapshot_data_blob_internal called (placeholder)");
    v8::StartupData::new(ptr::null(), 0)
}

fn warm_up_snapshot_data_blob_internal(
    _cold_snapshot_blob: v8::StartupData,
    _warmup_source: Option<&[u8]>,
) -> v8::StartupData {
    // This function would contain the V8-specific logic to warm up the snapshot data blob.
    // Replace with actual implementation if available.
    println!("warm_up_snapshot_data_blob_internal called (placeholder)");
    v8::StartupData::new(ptr::null(), 0)
}

// Placeholder for EmbeddedData and its method FromBlob.
mod i {
    pub struct EmbeddedData {
        pub data: String,
    }

    impl EmbeddedData {
        pub fn FromBlob() -> Self {
            EmbeddedData {
                data: "/* Embedded data here */".to_string(),
            }
        }
    }

    pub struct EmbeddedFileWriter {
        embedded_file_path: String,
        embedded_variant: String,
        target_arch: String,
        target_os: String,
    }

    impl EmbeddedFileWriter {
        pub fn new() -> Self {
            EmbeddedFileWriter {
                embedded_file_path: String::new(),
                embedded_variant: String::new(),
                target_arch: String::new(),
                target_os: String::new(),
            }
        }

        pub fn SetEmbeddedFile(&mut self, embedded_file: String) {
            self.embedded_file_path = embedded_file;
        }
        pub fn SetEmbeddedVariant(&mut self, embedded_variant: String) {
            self.embedded_variant = embedded_variant;
        }
        pub fn SetTargetArch(&mut self, target_arch: String) {
            self.target_arch = target_arch;
        }
        pub fn SetTargetOs(&mut self, target_os: String) {
            self.target_os = target_os;
        }

        pub fn WriteEmbedded(&self, _embedded_blob: &EmbeddedData) {}
    }

    pub struct Isolate {
        embedded_writer: *mut EmbeddedFileWriter,
    }

    impl Isolate {
        pub fn RegisterEmbeddedFileWriter(&mut self, writer: *mut EmbeddedFileWriter) {
            self.embedded_writer = writer;
        }
    }
}

lazy_static::lazy_static! {
    static ref COUNTER_MAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

fn maybe_set_counter_function(isolate: *mut v8::Isolate) {
    // If --native-code-counters is on then we enable all counters to make
    // sure we generate code to increment them from the snapshot.
    //
    // Note: For the sake of the mksnapshot, the counter function must only
    // return distinct addresses for each counter s.t. the serializer can properly
    // distinguish between them. In theory it should be okay to just return an
    // incremented int value each time this function is called, but we play it
    // safe and return a real distinct memory location tied to every counter name.
    let v8_flags = crate::flags::flags::V8_FLAGS.lock().unwrap();
    if v8_flags.native_code_counters {
        unsafe {
            let isolate_mut = &mut *isolate;
            isolate_mut.SetCounterFunction(Some(Box::new(|name: *const i8| -> *mut i32 {
                let name_str = CStr::from_ptr(name).to_str().unwrap().to_string();
                let mut counter_map = COUNTER_MAP.lock().unwrap();
                if !counter_map.contains_key(&name_str) {
                    counter_map.insert(name_str.clone(), 0);
                }
                let ptr = counter_map.get_mut(&name_str).unwrap() as *mut i32;
                ptr
            })));
        }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Make mksnapshot runs predictable to create reproducible snapshots.
    let mut v8_flags = crate::flags::flags::V8_FLAGS.lock().unwrap();
    v8_flags.predictable = true;

    // Disable ICs globally in mksnapshot to avoid problems with Code handlers.
    // See https://crbug.com/345280736.
    // TODO(jgruber): Re-enable once a better fix is available.
    v8_flags.use_ic = false;
    drop(v8_flags);

    let mut args: Vec<*mut i8> = std::env::args()
        .map(|arg| {
            let c_string = CString::new(arg).unwrap();
            c_string.into_raw()
        })
        .collect();
    let mut argc = args.len() as i32;

    let mut argv: [*mut i8; 64] = [ptr::null_mut(); 64]; // Assuming a maximum of 64 arguments
    for (i, arg) in args.iter().enumerate() {
        if i < 64 {
            argv[i] = *arg;
        }
    }

    // Print the usage if an error occurs when parsing the command line
    // flags or if the help flag is set.
    let usage = "Usage: mksnapshot [--startup-src=file] [--startup-blob=file] [--embedded-src=file] [--embedded-variant=label] [--static-roots-src=file] [--target-arch=arch] [--target-os=os] [extras]\n\n";
    let help_options =
        flags::flags::FlagList::HelpOptions::new(true, usage.to_string());

    let mut result =
        flags::flags::FlagList::SetFlagsFromCommandLine(&mut argc, &mut argv, true, help_options);
    if result > 0 || (argc > 3) {
        println!("{}", usage);
        std::process::exit(result);
    }

    codegen::cpu_features::Probe(true);
    let first_arg = if argc > 0 {
        unsafe { CStr::from_ptr(argv[0]).to_str().unwrap() }
    } else {
        ""
    };
    let first_arg_cstr = CString::new(first_arg).unwrap();

    v8::InitializeICUDefaultLocation(first_arg_cstr.as_ptr());
    let platform = v8::platform::NewDefaultPlatform();
    v8::InitializePlatform(&platform);
    v8::Initialize();

    let v8_flags = crate::flags::flags::V8_FLAGS.lock().unwrap();
    let snapshot_cpp_path = v8_flags.startup_src.clone();
    let snapshot_blob_path = v8_flags.startup_blob.clone();
    let embedded_src = v8_flags.embedded_src.clone();
    let embedded_variant = v8_flags.embedded_variant.clone();
    let static_roots_src = v8_flags.static_roots_src.clone();
    let target_arch = v8_flags.target_arch.clone();
    let target_os = v8_flags.target_os.clone();
    drop(v8_flags);

    let mut snapshot_writer = SnapshotFileWriter::new();
    snapshot_writer.set_snapshot_file(snapshot_cpp_path);
    snapshot_writer.set_startup_blob_file(snapshot_blob_path);

    let mut embedded_writer = snapshot::embedded::embedded_file_writer::EmbeddedFileWriter::new();
    embedded_writer.set_embedded_file(embedded_src);
    embedded_writer.set_embedded_variant(embedded_variant);
    embedded_writer.set_target_arch(target_arch);
    embedded_writer.set_target_os(target_os);

    let embed_script = get_extra_code(
        if argc >= 2 {
            unsafe { CStr::from_ptr(argv[1]).to_str().unwrap() }
        } else {
            ""
        },
        "embedding",
    );
    let warmup_script = get_extra_code(
        if argc >= 3 {
            unsafe { CStr::from_ptr(argv[2]).to_str().unwrap() }
        } else {
            ""
        },
        "warm up",
    );

    let mut blob = v8::StartupData::new(ptr::null(), 0);
    {
        let isolate = v8::Isolate::Allocate();

        maybe_set_counter_function(isolate);

        // The isolate contains data from builtin compilation that needs
        // to be written out if builtins are embedded.
        unsafe {
            let i_isolate = isolate as *mut i::Isolate;
            let embedded_writer_ptr = &mut embedded_writer as *mut snapshot::embedded::embedded_file_writer::EmbeddedFileWriter;
            (*i_isolate).RegisterEmbeddedFileWriter(embedded_writer_ptr);
        }

        let array_buffer_allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
        let mut create_params = v8::CreateParams::new(&mut array_buffer_allocator);

        // Set code range such that relative jumps for builtins to
        // builtin calls in the snapshot are possible.
        let code_range_size_mb = if 0 == 0 {
            512
        } else {
            std::cmp::min(0 / 1024 / 1024, 512)
        };
        create_params
            .constraints
            .set_code_range_size_in_bytes(code_range_size_mb * 1024 * 1024);

        {
            let mut creator = v8::SnapshotCreator::SnapshotCreator::new(isolate, create_params);

            blob = create_snapshot_data_blob(&mut creator, embed_script.as_ref().map(|v| v.as_slice()));

            unsafe {
                let i_isolate = isolate as *mut i::Isolate;
                (*(*i_isolate).embedded_writer).WriteEmbedded(&i::EmbeddedData::FromBlob());
            }

            if !static_roots_src.is_empty() {
                unsafe {
                    let i_isolate = isolate as *mut i::Isolate;
                    snapshot::static_roots_gen::StaticRootsTableGen::write(0, static_roots_src);
                }
            }
        }
        v8::Isolate::Dispose(isolate);
    }

    if let Some(warmup_script_data) = warmup_script {
        let cold = blob;
        blob = warm_up_snapshot_data_blob(cold, Some(&warmup_script_data));
        // Drop the old blob data.
    }

    // Reset the