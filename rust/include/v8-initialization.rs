// Copyright 2021 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

pub mod initialization {
    use std::ffi::{CStr, CString};
    use std::os::raw::{c_char, c_int, c_size_t, c_uchar, c_void};
    // use std::ptr::NonNull;

    /// EntropySource is used as a callback function when v8 needs a source
    /// of entropy.
    pub type EntropySource = extern "C" fn(buffer: *mut c_uchar, length: c_size_t) -> bool;

    /// ReturnAddressLocationResolver is used as a callback function when v8 is
    /// resolving the location of a return address on the stack. Profilers that
    /// change the return address on the stack can use this to resolve the stack
    /// location to wherever the profiler stashed the original return address.
    ///
    /// # Arguments
    ///
    /// * `return_addr_location` - A location on stack where a machine
    ///   return address resides.
    ///
    /// # Returns
    ///
    /// Either return_addr_location, or else a pointer to the profiler's
    /// copy of the original return address.
    ///
    /// # Note
    ///
    /// The resolver function must not cause garbage collection.
    pub type ReturnAddressLocationResolver = extern "C" fn(return_addr_location: usize) -> usize;

    pub type DcheckErrorCallback = extern "C" fn(file: *const c_char, line: c_int, message: *const c_char);

    pub type V8FatalErrorCallback = extern "C" fn(file: *const c_char, line: c_int, message: *const c_char);

    #[repr(C)]
    pub struct StartupData {
        data: *const c_char,
        raw_size: usize,
    }

    pub type OOMErrorCallback = extern "C" fn();

    #[repr(C)]
    pub struct SharedMemoryStatistics {
        pub code_and_metadata_size: usize,
        pub readonly_size: usize,
        pub read_write_size: usize,
    }

    // Callback type for handling unhandled exceptions in V8-generated code on Windows.
    #[cfg(target_os = "windows")]
    pub type UnhandledExceptionCallback = extern "C" fn(exception_info: *mut c_void) -> i32;

    extern "C" {
        /// Hand startup data to V8, in case the embedder has chosen to build
        /// V8 with external startup data.
        ///
        /// # Note
        ///
        /// *   By default the startup data is linked into the V8 library, in which
        /// *   case this function is not meaningful.
        /// *   If this needs to be called, it needs to be called before V8
        /// *   tries to make use of its built-ins.
        /// *   To avoid unnecessary copies of data, V8 will point directly into the
        /// *   given data blob, so pretty please keep it around until V8 exit.
        /// *   Compression of the startup blob might be useful, but needs to
        /// *   handled entirely on the embedders' side.
        /// *   The call will abort if the data is invalid.
        pub fn V8_SetSnapshotDataBlob(startup_blob: *mut StartupData);

        /// Set the callback to invoke in case of Dcheck failures.
        pub fn V8_SetDcheckErrorHandler(that: DcheckErrorCallback);

        /// Set the callback to invoke in the case of CHECK failures or fatal
        /// errors. This is distinct from Isolate::SetFatalErrorHandler, which
        /// is invoked in response to API usage failures.
        pub fn V8_SetFatalErrorHandler(that: V8FatalErrorCallback);

        /// Sets V8 flags from a string.
        pub fn V8_SetFlagsFromString(str: *const c_char);
        pub fn V8_SetFlagsFromString_length(str: *const c_char, length: c_size_t);

        /// Sets V8 flags from the command line.
        pub fn V8_SetFlagsFromCommandLine(argc: *mut c_int, argv: *mut *mut c_char, remove_flags: bool);

        /// Get the version string.
        pub fn V8_GetVersion() -> *const c_char;

        /// Initializes V8 with a specific build configuration.
        fn V8_Initialize(build_config: i32) -> bool;

        /// Allows the host application to provide a callback which can be used
        /// as a source of entropy for random number generators.
        pub fn V8_SetEntropySource(source: EntropySource);

        /// Allows the host application to provide a callback that allows v8 to
        /// cooperate with a profiler that rewrites return addresses on stack.
        pub fn V8_SetReturnAddressLocationResolver(return_address_resolver: ReturnAddressLocationResolver);

        /// Releases any resources used by v8 and stops any utility threads
        /// that may be running.  Note that disposing v8 is permanent, it
        /// cannot be reinitialized.
        ///
        /// It should generally not be necessary to dispose v8 before exiting
        /// a process, this should happen automatically.  It is only necessary
        /// to use if the process needs the resources taken up by v8.
        pub fn V8_Dispose() -> bool;

        /// Initialize the ICU library bundled with V8. The embedder should only
        /// invoke this method when using the bundled ICU. Returns true on success.
        ///
        /// If V8 was compiled with the ICU data in an external file, the location
        /// of the data file has to be provided.
        pub fn V8_InitializeICU(icu_data_file: *const c_char) -> bool;

        /// Initialize the ICU library bundled with V8. The embedder should only
        /// invoke this method when using the bundled ICU. If V8 was compiled with
        /// the ICU data in an external file and when the default location of that
        /// file should be used, a path to the executable must be provided.
        /// Returns true on success.
        ///
        /// The default is a file called icudtl.dat side-by-side with the executable.
        ///
        /// Optionally, the location of the data file can be provided to override the
        /// default.
        pub fn V8_InitializeICUDefaultLocation(exec_path: *const c_char, icu_data_file: *const c_char) -> bool;

        /// Initialize the external startup data. The embedder only needs to
        /// invoke this method when external startup data was enabled in a build.
        ///
        /// If V8 was compiled with the startup data in an external file, then
        /// V8 needs to be given those external files during startup. There are
        /// three ways to do this:
        /// - InitializeExternalStartupData(const char*)
        ///   This will look in the given directory for the file "snapshot_blob.bin".
        /// - InitializeExternalStartupDataFromFile(const char*)
        ///   As above, but will directly use the given file name.
        /// - Call SetSnapshotDataBlob.
        ///   This will read the blobs from the given data structure and will
        ///   not perform any file IO.
        pub fn V8_InitializeExternalStartupData(directory_path: *const c_char);
        pub fn V8_InitializeExternalStartupDataFromFile(snapshot_blob: *const c_char);

        /// Sets the v8::Platform to use. This should be invoked before V8 is
        /// initialized.
        pub fn V8_InitializePlatform(platform: *mut c_void);

        /// Clears all references to the v8::Platform. This should be invoked after
        /// V8 was disposed.
        pub fn V8_DisposePlatform();

        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        {
            /// Returns true if the sandbox is configured securely.
            ///
            /// If V8 cannot create a regular sandbox during initialization, for example
            /// because not enough virtual address space can be reserved, it will instead
            /// create a fallback sandbox that still allows it to function normally but
            /// does not have the same security properties as a regular sandbox. This API
            /// can be used to determine if such a fallback sandbox is being used, in
            /// which case it will return false.
            pub fn V8_IsSandboxConfiguredSecurely() -> bool;

            /// Provides access to the virtual address subspace backing the sandbox.
            ///
            /// This can be used to allocate pages inside the sandbox, for example to
            /// obtain virtual memory for ArrayBuffer backing stores, which must be
            /// located inside the sandbox.
            ///
            /// It should be assumed that an attacker can corrupt data inside the sandbox,
            /// and so in particular the contents of pages allocagted in this virtual
            /// address space, arbitrarily and concurrently. Due to this, it is
            /// recommended to to only place pure data buffers in them.
            pub fn V8_GetSandboxAddressSpace() -> *mut c_void;

            /// Returns the size of the sandbox in bytes.
            ///
            /// This represents the size of the address space that V8 can directly address
            /// and in which it allocates its objects.
            pub fn V8_GetSandboxSizeInBytes() -> c_size_t;

            /// Returns the size of the address space reservation backing the sandbox.
            ///
            /// This may be larger than the sandbox (i.e. |GetSandboxSizeInBytes()|) due
            /// to surrounding guard regions, or may be smaller than the sandbox in case a
            /// fallback sandbox is being used, which will use a smaller virtual address
            /// space reservation. In the latter case this will also be different from
            /// |GetSandboxAddressSpace()->size()| as that will cover a larger part of the
            /// address space than what has actually been reserved.
            pub fn V8_GetSandboxReservationSizeInBytes() -> c_size_t;
        }

        /// Activate trap-based bounds checking for WebAssembly.
        ///
        /// # Arguments
        ///
        /// * `use_v8_signal_handler` - Whether V8 should install its own signal
        ///   handler or rely on the embedder's.
        pub fn V8_EnableWebAssemblyTrapHandler(use_v8_signal_handler: bool) -> bool;

        #[cfg(target_os = "windows")]
        {
            /// On Win64, by default V8 does not emit unwinding data for jitted code,
            /// which means the OS cannot walk the stack frames and the system Structured
            /// Exception Handling (SEH) cannot unwind through V8-generated code:
            /// https://code.google.com/p/v8/issues/detail?id=3598.
            ///
            /// This function allows embedders to register a custom exception handler for
            /// exceptions in V8-generated code.
            pub fn V8_SetUnhandledExceptionCallback(callback: UnhandledExceptionCallback);
        }

        /// Allows the host application to provide a callback that will be called when
        /// v8 has encountered a fatal failure to allocate memory and is about to
        /// terminate.
        pub fn V8_SetFatalMemoryErrorCallback(callback: OOMErrorCallback);

        /// Get statistics about the shared memory usage.
        pub fn V8_GetSharedMemoryStatistics(statistics: *mut SharedMemoryStatistics);
    }

    /// Container class for static utility functions.
    pub struct V8;

    impl V8 {
        /// Hand startup data to V8, in case the embedder has chosen to build
        /// V8 with external startup data.
        ///
        /// # Note
        ///
        /// *   By default the startup data is linked into the V8 library, in which
        /// *   case this function is not meaningful.
        /// *   If this needs to be called, it needs to be called before V8
        /// *   tries to make use of its built-ins.
        /// *   To avoid unnecessary copies of data, V8 will point directly into the
        /// *   given data blob, so pretty please keep it around until V8 exit.
        /// *   Compression of the startup blob might be useful, but needs to be
        /// *   handled entirely on the embedders' side.
        /// *   The call will abort if the data is invalid.
        pub fn set_snapshot_data_blob(startup_blob: &mut StartupData) {
            unsafe {
                V8_SetSnapshotDataBlob(startup_blob);
            }
        }

        /// Set the callback to invoke in case of Dcheck failures.
        pub fn set_dcheck_error_handler(that: DcheckErrorCallback) {
            unsafe {
                V8_SetDcheckErrorHandler(that);
            }
        }

        /// Set the callback to invoke in the case of CHECK failures or fatal
        /// errors. This is distinct from Isolate::SetFatalErrorHandler, which
        /// is invoked in response to API usage failures.
        pub fn set_fatal_error_handler(that: V8FatalErrorCallback) {
            unsafe {
                V8_SetFatalErrorHandler(that);
            }
        }

        /// Sets V8 flags from a string.
        pub fn set_flags_from_string(str: &str) {
            let c_str = CString::new(str).unwrap();
            unsafe {
                V8_SetFlagsFromString(c_str.as_ptr());
            }
        }

        /// Sets V8 flags from a string with a specified length.
        pub fn set_flags_from_string_with_length(str: &str, length: usize) {
            let c_str = CString::new(str).unwrap();
            unsafe {
                V8_SetFlagsFromString_length(c_str.as_ptr(), length as c_size_t);
            }
        }

        /// Sets V8 flags from the command line.
        pub fn set_flags_from_command_line(argc: &mut i32, argv: &mut [*mut c_char], remove_flags: bool) {
            unsafe {
                V8_SetFlagsFromCommandLine(argc, argv.as_mut_ptr(), remove_flags);
            }
        }

        /// Get the version string.
        pub fn get_version() -> String {
            unsafe {
                let version = V8_GetVersion();
                CStr::from_ptr(version).to_string_lossy().into_owned()
            }
        }

        /// Initializes V8. This function needs to be called before the first Isolate
        /// is created. It always returns true.
        pub fn initialize() -> bool {
            let k_v8_target_os_is_android = cfg!(target_os = "android");
            let k_v8_enable_checks = cfg!(debug_assertions);

            let k_build_configuration =
                (if cfg!(feature = "pointer-compression") { 1 << 0 } else { 0 }) |
                (if cfg!(feature = "31bit-smis") { 1 << 1 } else { 0 }) |
                (if cfg!(feature = "sandbox") { 1 << 2 } else { 0 }) |
                (if k_v8_target_os_is_android { 1 << 3 } else { 0 }) |
                (if k_v8_enable_checks { 1 << 4 } else { 0 });

            unsafe { V8_Initialize(k_build_configuration) }
        }

        /// Allows the host application to provide a callback which can be used
        /// as a source of entropy for random number generators.
        pub fn set_entropy_source(source: EntropySource) {
            unsafe {
                V8_SetEntropySource(source);
            }
        }

        /// Allows the host application to provide a callback that allows v8 to
        /// cooperate with a profiler that rewrites return addresses on stack.
        pub fn set_return_address_location_resolver(return_address_resolver: ReturnAddressLocationResolver) {
            unsafe {
                V8_SetReturnAddressLocationResolver(return_address_resolver);
            }
        }

        /// Releases any resources used by v8 and stops any utility threads
        /// that may be running.  Note that disposing v8 is permanent, it
        /// cannot be reinitialized.
        ///
        /// It should generally not be necessary to dispose v8 before exiting
        /// a process, this should happen automatically.  It is only necessary
        /// to use if the process needs the resources taken up by v8.
        pub fn dispose() -> bool {
            unsafe {
                V8_Dispose()
            }
        }

        /// Initialize the ICU library bundled with V8. The embedder should only
        /// invoke this method when using the bundled ICU. Returns true on success.
        ///
        /// If V8 was compiled with the ICU data in an external file, the location
        /// of the data file has to be provided.
        pub fn initialize_icu(icu_data_file: Option<&str>) -> bool {
            unsafe {
                match icu_data_file {
                    Some(file) => {
                        let c_str = CString::new(file).unwrap();
                        V8_InitializeICU(c_str.as_ptr())
                    }
                    None => V8_InitializeICU(std::ptr::null()),
                }
            }
        }

        /// Initialize the ICU library bundled with V8. The embedder should only
        /// invoke this method when using the bundled ICU. If V8 was compiled with
        /// the ICU data in an external file and when the default location of that
        /// file should be used, a path to the executable must be provided.
        /// Returns true on success.
        ///
        /// The default is a file called icudtl.dat side-by-side with the executable.
        ///
        /// Optionally, the location of the data file can be provided to override the
        /// default.
        pub fn initialize_icu_default_location(exec_path: &str, icu_data_file: Option<&str>) -> bool {
            let exec_path_c_str = CString::new(exec_path).unwrap();
            unsafe {
                match icu_data_file {
                    Some(file) => {
                        let c_str = CString::new(file).unwrap();
                        V8_InitializeICUDefaultLocation(exec_path_c_str.as_ptr(), c_str.as_ptr())
                    }
                    None => V8_InitializeICUDefaultLocation(exec_path_c_str.as_ptr(), std::ptr::null()),
                }
            }
        }

        /// Initialize the external startup data. The embedder only needs to
        /// invoke this method when external startup data was enabled in a build.
        ///
        /// If V8 was compiled with the startup data in an external file, then
        /// V8 needs to be given those external files during startup. There are
        /// three ways to do this:
        /// - InitializeExternalStartupData(const char*)
        ///   This will look in the given directory for the file "snapshot_blob.bin".
        /// - InitializeExternalStartupDataFromFile(const char*)
        ///   As above, but will directly use the given file name.
        /// - Call SetSnapshotDataBlob.
        ///   This will read the blobs from the given data structure and will
        ///   not perform any file IO.
        pub fn initialize_external_startup_data(directory_path: &str) {
            let c_str = CString::new(directory_path).unwrap();
            unsafe {
                V8_InitializeExternalStartupData(c_str.as_ptr());
            }
        }

        /// Initialize the external startup data from a specified snapshot blob file.
        pub fn initialize_external_startup_data_from_file(snapshot_blob: &str) {
            let c_str = CString::new(snapshot_blob).unwrap();
            unsafe {
                V8_InitializeExternalStartupDataFromFile(c_str.as_ptr());
            }
        }

        /// Sets the v8::Platform to use. This should be invoked before V8 is
        /// initialized.
        pub fn initialize_platform(platform: *mut c_void) {
            unsafe {
                V8_InitializePlatform(platform);
            }
        }

        /// Clears all references to the v8::Platform. This should be invoked after
        /// V8 was disposed.
        pub fn dispose_platform() {
            unsafe {
                V8_DisposePlatform();
            }
        }

        #[cfg(feature = "V8_ENABLE_SANDBOX")]
        {
            /// Returns true if the sandbox is configured securely.
            ///
            /// If V8 cannot create a regular sandbox during initialization, for example
            /// because not enough virtual address space can be reserved, it will instead
            /// create a fallback sandbox that still allows it to function normally but
            /// does not have the same security properties as a regular sandbox. This API
            /// can be used to determine if such a fallback sandbox is being used, in
            /// which case it will return false.
            pub fn is_sandbox_configured_securely() -> bool {
                unsafe { V8_IsSandboxConfiguredSecurely() }
            }

            /// Provides access to the virtual address subspace backing the sandbox.
            ///
            /// This can be used to allocate pages inside the sandbox, for example to
            /// obtain virtual memory for ArrayBuffer backing stores, which must be
            /// located inside the sandbox.
            ///
            /// It should be assumed that an attacker can corrupt data inside the sandbox,
            /// and so in particular the contents of pages allocagted in this virtual
            /// address space, arbitrarily and concurrently. Due to this, it is
            /// recommended to to only place pure data buffers in them.
            pub fn get_sandbox_address_space() -> *mut c_void {
                unsafe { V8_GetSandboxAddressSpace() }
            }

            /// Returns the size of the sandbox in bytes.
            ///
            /// This represents the size of the address space that V8 can directly address
            /// and in which it allocates its objects.
            pub fn get_sandbox_size_in_bytes() -> usize {
                unsafe { V8_GetSandboxSizeInBytes() as usize }
            }

            /// Returns the size of the address space reservation backing the sandbox.
            ///
            /// This may be larger than the sandbox (i.e. |GetSandboxSizeInBytes()|) due
            /// to surrounding guard regions, or may be smaller than the sandbox in case a
            /// fallback sandbox is being used, which will use a smaller virtual address
            /// space reservation. In the latter case this will also be different from
            /// |GetSandboxAddressSpace()->size()| as that will cover a larger part of the
            /// address space than what has actually been reserved.
            pub fn get_sandbox_reservation_size_in_bytes() -> usize {
                unsafe { V8_GetSandboxReservationSizeInBytes() as usize }
            }
        }

        /// Activate trap-based bounds checking for WebAssembly.
        ///
        /// # Arguments
        ///
        /// * `use_v8_signal_handler` - Whether V8 should install its own signal
        ///   handler or rely on the embedder's.
        pub fn enable_web_assembly_trap_handler(use_v8_signal_handler: bool) -> bool {
            unsafe {
                V8_EnableWebAssemblyTrapHandler(use_v8_signal_handler)
            }
        }

        #[cfg(target_os = "windows")]
        {
            /// On Win64, by default V8 does not emit unwinding data for jitted code,
            /// which means the OS cannot walk the stack frames and the system Structured
            /// Exception Handling (SEH) cannot unwind through V8-generated code:
            /// https://code.google.com/p/v8/issues/detail?id=3598.
            ///
            /// This function allows embedders to register a custom exception handler for
            /// exceptions in V8-generated code.
            pub fn set_unhandled_exception_callback(callback: UnhandledExceptionCallback) {
                unsafe {
                    V8_SetUnhandledExceptionCallback(callback);
                }
            }
        }

        /// Allows the host application to provide a callback that will be called when
        /// v8 has encountered a fatal failure to allocate memory and is about to
        /// terminate.
        pub fn set_fatal_memory_error_callback(callback: OOMErrorCallback) {
            unsafe {
                V8_SetFatalMemoryErrorCallback(callback);
            }
        }

        /// Get statistics about the shared memory usage.
        pub fn get_shared_memory_statistics(statistics: &mut SharedMemoryStatistics) {
            unsafe {
                V8_GetSharedMemoryStatistics(statistics);
            }
        }
    }
}