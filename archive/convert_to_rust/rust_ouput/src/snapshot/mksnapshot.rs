// Converted from V8 C++ source files:
// Header: N/A
// Implementation: mksnapshot.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

use std::collections::HashMap;
use std::error::Error;
use std::ffi::CString;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::os::raw::c_char;
use std::path::Path;
use std::ptr;
use v8::StartupData;
use v8::V8;
mod base {
    pub mod platform {
        use std::time::Instant;
        pub struct ElapsedTimer {
            start: Option<Instant>,
        }

        impl ElapsedTimer {
            pub fn new() -> Self {
                ElapsedTimer { start: None }
            }

            pub fn start(&mut self) {
                self.start = Some(Instant::now());
            }

            pub fn stop(&mut self) {}

            pub fn elapsed(&self) -> Duration {
                match self.start {
                    Some(start) => {
                        let elapsed = Instant::now().duration_since(start);
                        Duration {
                            seconds: elapsed.as_secs(),
                            nanos: elapsed.subsec_nanos(),
                        }
                    }
                    None => Duration {
                        seconds: 0,
                        nanos: 0,
                    },
                }
            }
        }

        pub struct Duration {
            seconds: u64,
            nanos: u32,
        }

        impl Duration {
            pub fn in_milliseconds_f(&self) -> f64 {
                (self.seconds as f64 * 1000.0) + (self.nanos as f64 / 1_000_000.0)
            }
        }
        use std::fs::File;

        pub fn fopen(filename: &str, mode: &str) -> Result<File, std::io::Error> {
            let path = Path::new(filename);

            let file = match mode {
                "rb" => File::open(path)?,
                "wb" => OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path)?,
                _ => panic!("Unsupported mode: {}", mode),
            };

            Ok(file)
        }
    }
    pub fn fclose(file: File) {
        drop(file);
    }
}
mod codegen {
    pub mod cpu_features {
        pub fn probe(arg: bool) {}
    }
}
mod common {
    pub mod globals {}
}
mod flags {
    pub mod flags {
        use std::sync::atomic::{AtomicBool, AtomicPtr, AtomicUsize, Ordering};
        use std::sync::Mutex;
        #[derive(Default)]
        pub struct Flags {
            pub predictable: AtomicBool,
            pub startup_src: AtomicPtr<c_char>,
            pub startup_blob: AtomicPtr<c_char>,
            pub embedded_src: AtomicPtr<c_char>,
            pub embedded_variant: AtomicPtr<c_char>,
            pub static_roots_src: AtomicPtr<c_char>,
            pub target_arch: AtomicPtr<c_char>,
            pub target_os: AtomicPtr<c_char>,
            pub native_code_counters: AtomicBool,
            pub use_ic: AtomicBool,
            pub verify_snapshot_checksum: AtomicBool,
        }
        lazy_static::lazy_static! {
            pub static ref FLAGS: Flags = Flags::default();
        }

        impl Flags {
            pub fn set_startup_src(&self, value: *mut c_char) {
                self.startup_src.store(value, Ordering::Relaxed);
            }
            pub fn startup_src(&self) -> *const c_char {
                self.startup_src.load(Ordering::Relaxed) as *const c_char
            }
            pub fn set_startup_blob(&self, value: *mut c_char) {
                self.startup_blob.store(value, Ordering::Relaxed);
            }
            pub fn startup_blob(&self) -> *const c_char {
                self.startup_blob.load(Ordering::Relaxed) as *const c_char
            }

            pub fn set_embedded_src(&self, value: *mut c_char) {
                self.embedded_src.store(value, Ordering::Relaxed);
            }
            pub fn embedded_src(&self) -> *const c_char {
                self.embedded_src.load(Ordering::Relaxed) as *const c_char
            }
            pub fn set_embedded_variant(&self, value: *mut c_char) {
                self.embedded_variant.store(value, Ordering::Relaxed);
            }
            pub fn embedded_variant(&self) -> *const c_char {
                self.embedded_variant.load(Ordering::Relaxed) as *const c_char
            }
            pub fn set_static_roots_src(&self, value: *mut c_char) {
                self.static_roots_src.store(value, Ordering::Relaxed);
            }
            pub fn static_roots_src(&self) -> *const c_char {
                self.static_roots_src.load(Ordering::Relaxed) as *const c_char
            }
            pub fn set_target_arch(&self, value: *mut c_char) {
                self.target_arch.store(value, Ordering::Relaxed);
            }
            pub fn target_arch(&self) -> *const c_char {
                self.target_arch.load(Ordering::Relaxed) as *const c_char
            }
            pub fn set_target_os(&self, value: *mut c_char) {
                self.target_os.store(value, Ordering::Relaxed);
            }
            pub fn target_os(&self) -> *const c_char {
                self.target_os.load(Ordering::Relaxed) as *const c_char
            }

            pub fn set_native_code_counters(&self, value: bool) {
                self.native_code_counters.store(value, Ordering::Relaxed);
            }
            pub fn native_code_counters(&self) -> bool {
                self.native_code_counters.load(Ordering::Relaxed)
            }
            pub fn set_use_ic(&self, value: bool) {
                self.use_ic.store(value, Ordering::Relaxed);
            }
            pub fn use_ic(&self) -> bool {
                self.use_ic.load(Ordering::Relaxed)
            }

            pub fn set_verify_snapshot_checksum(&self, value: bool) {
                self.verify_snapshot_checksum.store(value, Ordering::Relaxed);
            }
            pub fn verify_snapshot_checksum(&self) -> bool {
                self.verify_snapshot_checksum.load(Ordering::Relaxed)
            }

            pub fn set_predictable(&self, value: bool) {
                self.predictable.store(value, Ordering::Relaxed);
            }
            pub fn predictable(&self) -> bool {
                self.predictable.load(Ordering::Relaxed)
            }
        }

        pub struct FlagList {}
        impl FlagList {
            pub struct HelpOptions {
                exit_on_help: bool,
                usage: String,
            }
            impl HelpOptions {
                pub fn new(exit_on_help: bool, usage: String) -> Self {
                    HelpOptions { exit_on_help, usage }
                }
                pub struct KExit;
            }
            pub fn set_flags_from_command_line(
                argc: &mut i32,
                argv: &mut [*mut c_char],
                remove_flags: bool,
                help_options: HelpOptions,
            ) -> i32 {
                let mut i = 1;
                while i < *argc {
                    let arg = unsafe { std::ffi::CStr::from_ptr(argv[i as usize]).to_str().unwrap() };

                    if arg.starts_with("--") {
                        let parts: Vec<&str> = arg.splitn(2, "=").collect();
                        let flag_name = parts[0];

                        match flag_name {
                            "--predictable" => {
                                FLAGS.set_predictable(true);
                            }
                            "--startup-src" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_startup_src(ptr);
                                } else {
                                    eprintln!("Error: --startup-src requires a value");
                                    return 1;
                                }
                            }
                            "--startup-blob" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_startup_blob(ptr);
                                } else {
                                    eprintln!("Error: --startup-blob requires a value");
                                    return 1;
                                }
                            }
                            "--embedded-src" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_embedded_src(ptr);
                                } else {
                                    eprintln!("Error: --embedded-src requires a value");
                                    return 1;
                                }
                            }
                            "--embedded-variant" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_embedded_variant(ptr);
                                } else {
                                    eprintln!("Error: --embedded-variant requires a value");
                                    return 1;
                                }
                            }
                            "--static-roots-src" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_static_roots_src(ptr);
                                } else {
                                    eprintln!("Error: --static-roots-src requires a value");
                                    return 1;
                                }
                            }
                            "--target-arch" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_target_arch(ptr);
                                } else {
                                    eprintln!("Error: --target-arch requires a value");
                                    return 1;
                                }
                            }
                            "--target-os" => {
                                if parts.len() > 1 {
                                    let value = CString::new(parts[1]).unwrap();
                                    let ptr = value.into_raw();
                                    FLAGS.set_target_os(ptr);
                                } else {
                                    eprintln!("Error: --target-os requires a value");
                                    return 1;
                                }
                            }
                            "--native-code-counters" => {
                                FLAGS.set_native_code_counters(true);
                            }
                            "--use-ic" => {
                                FLAGS.set_use_ic(true);
                            }
                            "--verify-snapshot-checksum" => {
                                FLAGS.set_verify_snapshot_checksum(true);
                            }
                            _ => {
                                println!("Unknown flag: {}", arg);
                                return 1;
                            }
                        }

                        if remove_flags {
                            // Shift remaining arguments to overwrite the flag
                            for j in (i + 1)..*argc {
                                argv[(j - 1) as usize] = argv[j as usize];
                            }
                            *argc -= 1;
                            continue; // Don't increment i, as we've shifted the next arg into place
                        }
                    }
                    i += 1;
                }

                0
            }
        }
    }
}
mod snapshot {
    pub mod embedded {
        pub struct EmbeddedFileWriter {
            embedded_file_path: *const i8,
            embedded_variant: *const i8,
            target_arch: *const i8,
            target_os: *const i8,
        }
        impl EmbeddedFileWriter {
            pub fn new() -> EmbeddedFileWriter {
                EmbeddedFileWriter {
                    embedded_file_path: ptr::null(),
                    embedded_variant: ptr::null(),
                    target_arch: ptr::null(),
                    target_os: ptr::null(),
                }
            }

            pub fn set_embedded_file(&mut self, embedded_src: *const i8) {
                self.embedded_file_path = embedded_src;
            }

            pub fn set_embedded_variant(&mut self, embedded_variant: *const i8) {
                self.embedded_variant = embedded_variant;
            }

            pub fn set_target_arch(&mut self, target_arch: *const i8) {
                self.target_arch = target_arch;
            }

            pub fn set_target_os(&mut self, target_os: *const i8) {
                self.target_os = target_os;
            }

            pub fn write_embedded(&mut self, embedded_blob: &EmbeddedData) {}
        }

        pub struct EmbeddedData {}

        impl EmbeddedData {
            pub fn from_blob() -> EmbeddedData {
                EmbeddedData {}
            }
        }
    }
    pub mod static_roots_gen {
        use crate::v8::Isolate;
        pub struct StaticRootsTableGen {}
        impl StaticRootsTableGen {
            pub fn write(i_isolate: *mut Isolate, file_path: *const i8) {}
        }
    }
}

use flags::flags as i;
use std::sync::Mutex;

struct SnapshotFileWriter {
    snapshot_cpp_path: Option<CString>,
    snapshot_blob_path: Option<CString>,
}

impl SnapshotFileWriter {
    fn new() -> Self {
        SnapshotFileWriter {
            snapshot_cpp_path: None,
            snapshot_blob_path: None,
        }
    }

    fn set_snapshot_file(&mut self, snapshot_cpp_file: *const c_char) {
        if !snapshot_cpp_file.is_null() {
            let c_str = unsafe { CString::from_raw(snapshot_cpp_file as *mut c_char) };
            self.snapshot_cpp_path = Some(c_str);
        } else {
            self.snapshot_cpp_path = None;
        }
    }

    fn set_startup_blob_file(&mut self, snapshot_blob_file: *const c_char) {
        if !snapshot_blob_file.is_null() {
            let c_str = unsafe { CString::from_raw(snapshot_blob_file as *mut c_char) };
            self.snapshot_blob_path = Some(c_str);
        } else {
            self.snapshot_blob_path = None;
        }
    }

    fn write_snapshot(&self, blob: v8::StartupData) -> Result<(), Box<dyn Error>> {
        let blob_vector = unsafe {
            Vec::from_raw_parts(
                blob.data as *mut u8,
                blob.raw_size as usize,
                blob.raw_size as usize,
            )
        };

        self.maybe_write_snapshot_file(&blob_vector)?;
        self.maybe_write_startup_blob(&blob_vector)?;

        //Reconstruct the vector so that the memory is deallocated
        std::mem::forget(blob_vector);
        Ok(())
    }

    fn maybe_write_startup_blob(&self, blob: &[u8]) -> Result<(), Box<dyn Error>> {
        if let Some(ref snapshot_blob_path) = self.snapshot_blob_path {
            let file_path = snapshot_blob_path.to_str()?;
            let mut fp = Self::get_file_descriptor_or_die(file_path)?;
            let written = fp.write_all(blob)?;
            if written.is_ok() {
                base::fclose(fp);
                Ok(())
            } else {
                std::fs::remove_file(file_path)?;
                Err("Writing snapshot file failed. Aborting.".into())
            }
        } else {
            Ok(())
        }
    }

    fn maybe_write_snapshot_file(&self, blob: &[u8]) -> Result<(), Box<dyn Error>> {
        if let Some(ref snapshot_cpp_path) = self.snapshot_cpp_path {
            let file_path = snapshot_cpp_path.to_str()?;
            let mut fp = Self::get_file_descriptor_or_die(file_path)?;

            Self::write_snapshot_file_prefix(&mut fp)?;
            Self::write_snapshot_file_data(&mut fp, blob)?;
            Self::write_snapshot_file_suffix(&mut fp)?;

            base::fclose(fp);
        }
        Ok(())
    }

    fn write_snapshot_file_prefix(fp: &mut File) -> Result<(), Box<dyn Error>> {
        writeln!(fp, "// Autogenerated snapshot file. Do not edit.\n")?;
        writeln!(fp, "#include \"src/init/v8.h\"")?;
        writeln!(fp, "#include \"src/base/platform/platform.h\"\n")?;
        writeln!(fp, "#include \"src/flags/flags.h\"")?;
        writeln!(fp, "#include \"src/snapshot/snapshot.h\"\n")?;
        writeln!(fp, "namespace v8 {{")?;
        writeln!(fp, "namespace internal {{\n")?;
        Ok(())
    }

    fn write_snapshot_file_suffix(fp: &mut File) -> Result<(), Box<dyn Error>> {
        writeln!(fp, "const v8::StartupData* Snapshot::DefaultSnapshotBlob() {{")?;
        writeln!(fp, "  return &blob;")?;
        writeln!(fp, "}}\n")?;
        writeln!(
            fp,
            "bool Snapshot::ShouldVerifyChecksum(const v8::StartupData* data) {{"
        )?;
        writeln!(fp, "  return v8_flags.verify_snapshot_checksum;")?;
        writeln!(fp, "}}\n")?;
        writeln!(fp, "}}  // namespace internal")?;
        writeln!(fp, "}}  // namespace v8")?;
        Ok(())
    }

    fn write_snapshot_file_data(fp: &mut File, blob: &[u8]) -> Result<(), Box<dyn Error>> {
        writeln!(
            fp,
            "alignas(kPointerAlignment) static const uint8_t blob_data[] = {{"
        )?;
        Self::write_binary_contents_as_c_array(fp, blob)?;
        writeln!(fp, "}};\n")?;
        writeln!(fp, "static const int blob_size = {};", blob.len())?;
        writeln!(fp, "static const v8::StartupData blob =")?;
        writeln!(fp, "{{ (const char*) blob_data, blob_size }};\n")?;
        Ok(())
    }

    fn write_binary_contents_as_c_array(fp: &mut File, blob: &[u8]) -> Result<(), Box<dyn Error>> {
        for (i, &byte) in blob.iter().enumerate() {
            if (i & 0x1F) == 0x1F {
                writeln!(fp)?;
            }
            if i > 0 {
                write!(fp, ",")?;
            }
            write!(fp, "{}", byte)?;
        }
        writeln!(fp, "\n")?;
        Ok(())
    }

    fn get_file_descriptor_or_die(filename: &str) -> Result<File, Box<dyn Error>> {
        match base::platform::fopen(filename, "wb") {
            Ok(fp) => Ok(fp),
            Err(_) => {
                eprintln!("Unable to open file \"{}\" for writing.", filename);
                std::process::exit(1);
            }
        }
    }
}

fn get_extra_code(filename: *const c_char, description: &str) -> Option<CString> {
    if filename.is_null() {
        return None;
    }

    let filename_cstr = unsafe { CString::from_raw(filename as *mut c_char) };
    let filename_str = filename_cstr.to_str().unwrap();

    if filename_str.is_empty() {
        return None;
    }

    println!("Loading script for {}: {}", description, filename_str);

    match base::platform::fopen(filename_str, "rb") {
        Ok(mut file) => {
            let mut buffer = Vec::new();
            file.read_to_end(&mut buffer).unwrap();
            Some(CString::new(buffer).unwrap())
        }
        Err(err) => {
            eprintln!("Failed to open '{}': {:?}", filename_str, err);
            std::process::exit(1);
        }
    }
}

fn create_snapshot_data_blob(
    snapshot_creator: &mut v8::SnapshotCreator,
    embedded_source: Option<&CString>,
) -> v8::StartupData {
    use base::platform::ElapsedTimer;
    use std::time::Instant;

    let mut timer = ElapsedTimer::new();
    timer.start();

    let embedded_source_str = embedded_source.map(|s| s.as_ptr()).unwrap_or(ptr::null());

    let result = create_snapshot_data_blob_internal(
        v8::FunctionCodeHandling::kClear,
        embedded_source_str,
        snapshot_creator,
    );

    if i::FLAGS.native_code_counters() {
        println!(
            "[Creating snapshot took {:.3} ms]",
            timer.elapsed().in_milliseconds_f()
        );
    }

    timer.stop();
    return result;
}

fn warm_up_snapshot_data_blob(
    cold_snapshot_blob: v8::StartupData,
    warmup_source: Option<&CString>,
) -> v8::StartupData {
    use base::platform::ElapsedTimer;
    use std::time::Instant;

    let mut timer = base::platform::ElapsedTimer::new();
    timer.start();

    let warmup_source_str = warmup_source.map(|s| s.as_ptr()).unwrap_or(ptr::null());

    let result = warm_up_snapshot_data_blob_internal(cold_snapshot_blob, warmup_source_str);

    if i::FLAGS.native_code_counters() {
        println!(
            "Warming up snapshot took {:.3} ms",
            timer.elapsed().in_milliseconds_f()
        );
    }

    timer.stop();
    return result;
}
fn create_snapshot_data_blob_internal(
    function_code_handling: v8::FunctionCodeHandling,
    embedded_source: *const i8,
    snapshot_creator: &mut v8::SnapshotCreator,
) -> v8::StartupData {
    StartupData {
        data: ptr::null(),
        raw_size: 0,
    }
}

fn warm_up_snapshot_data_blob_internal(
    cold_snapshot_blob: v8::StartupData,
    warmup_source: *const i8,
) -> v8::StartupData {
    StartupData {
        data: ptr::null(),
        raw_size: 0,
    }
}

use snapshot::embedded::EmbeddedFileWriter;
use snapshot::embedded::EmbeddedData;
fn write_embedded_file(writer: &mut EmbeddedFileWriter) {
    let embedded_blob = EmbeddedData::from_blob();
    writer.write_embedded(&embedded_blob);
}

lazy_static::lazy_static! {
    static ref COUNTER_MAP: Mutex<HashMap<String, i32>> = Mutex::new(HashMap::new());
}

fn maybe_set_counter_function(isolate: *mut v8::V8) {
    if i::FLAGS.native_code_counters() {
        unsafe {
            V8::set_counter_function(isolate, Some(counter_function));
        }
    }
}

extern "C" fn counter_function(name: *const i8) -> *mut i32 {
    let name_str = unsafe { CStr::from_ptr(name) }.to_str().unwrap().to_string();
    let mut map = COUNTER_MAP.lock().unwrap();
    map.entry(name_str)
        .or_insert(0);
    map.get_mut(&name_str).unwrap() as *mut i32
}

use std::ffi::CStr;
use flags::flags::FLAGS;

fn main() -> Result<(), Box<dyn Error>> {
    base::platform::EnsureConsoleOutput();

    FLAGS.set_predictable(true);

    FLAGS.set_use_ic(false);

    let mut args: Vec<*mut i8> = std::env::args()
        .map(|arg| {
            let c_str = CString::new(arg).unwrap();
            c_str.into_raw()
        })
        .collect();
    let mut argc = args.len() as i32;
    let mut argv = args.as_mut_ptr();

    use flags::flags::FlagList;
    let usage = "Usage: mksnapshot [--startup-src=file] [--startup-blob=file] [--embedded-src=file] [--embedded-variant=label] [--static-roots-src=file] [--target-arch=arch] [--target-os=os] [extras]\n\n";
    let result = FlagList::set_flags_from_command_line(
        &mut argc,
        std::slice::from_raw_parts_mut(argv, argc as usize),
        true,
        FlagList::HelpOptions::new(false, usage.to_string()),
    );
    if result > 0 || (argc > 3) {
        println!("{}", usage);
        return Ok(());
    }

    codegen::cpu_features::probe(true);
    v8::V8::initialize_icu_default_location(args[0]);
    let platform = v8::platform::NewDefaultPlatform();
    v8::V8::initialize_platform(platform);
    v8::V8::initialize();

    let mut snapshot_writer = SnapshotFileWriter::new();
    unsafe {
        snapshot_writer.set_snapshot_file(FLAGS.startup_src());
        snapshot_writer.set_startup_blob_file(FLAGS.startup_blob());
    }

    let mut embedded_writer = snapshot::embedded::EmbeddedFileWriter::new();
    unsafe {
        embedded_writer.set_embedded_file(FLAGS.embedded_src());
        embedded_writer.set_embedded_variant(FLAGS.embedded_variant());
        embedded_writer.set_target_arch(FLAGS.target_arch());
        embedded_writer.set_target_os(FLAGS.target_os());
    }

    let embed_script = unsafe { get_extra_code(if argc >= 2 { args[1] } else { ptr::null() }, "embedding") };
    let warmup_script = unsafe { get_extra_code(if argc >= 3 { args[2] } else { ptr::null() }, "warm up") };

    let mut blob: v8::StartupData;
    {
        let isolate = V8::allocate();

        maybe_set_counter_function(isolate);

        let mut i_isolate = isolate;

        unsafe {
            V8::register_embedded_file_writer(i_isolate, &mut embedded_writer);
        }

        let array_buffer_allocator = v8::ArrayBuffer::Allocator::NewDefaultAllocator();
        let mut create_params = v8::Isolate::CreateParams::new();
        create_params.array_buffer_allocator = array_buffer_allocator;

        let code_range_size_mb = if 0 == 0 {
            1024
        } else {
            std::cmp::min(0 / 1024 / 1024, 1024)
        };
        create_params.constraints.set_code_range_size_in_bytes(code_range_size_mb * 1024 * 1024);

        {
            let mut creator = v8::SnapshotCreator::new(isolate, create_params);
            blob = create_snapshot_data_blob(&mut creator, embed_script.as_ref());
            write_embedded_file(&mut embedded_writer);

            unsafe {
                if !FLAGS.static_roots_src().is_null() {
                    snapshot::static_roots_gen::StaticRootsTableGen::write(
                        i_isolate,
                        FLAGS.static_roots_src(),
                    );
                }
            }
        }

        V8::dispose(isolate);
    }

    if let Some(_) = warmup_script {
        let cold = blob;
        blob = warm_up_snapshot_data_blob(cold, warmup_script.as_ref());
        unsafe {
            drop(Vec::from_raw_parts(cold.data as *mut u8, cold.raw_size, cold.raw_size));
        }
    }

    {
        let mut map = COUNTER_MAP.lock().unwrap();
        map.clear();
    }

    if blob.data.is_null() {
        panic!("blob.data is null");
    }
    snapshot_writer.write_snapshot(blob)?;
    unsafe {
        drop(Vec::from_raw_parts(blob.data as *mut u8, blob.raw_size, blob.raw_size));
    }

    v8::V8::dispose();
    v8::V8::dispose_platform();

    Ok(())
}
