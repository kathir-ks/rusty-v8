// Converted from V8 C++ source files:
// Header: icu_util.h
// Implementation: icu_util.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod icu_util {
    #[cfg(all(feature = "v8_intl_support"))]
    use std::{
        ffi::CString,
        fs::File,
        io::{Read, Seek, SeekFrom},
        os::raw::c_void,
        ptr,
        sync::Once,
    };

    #[cfg(all(feature = "v8_intl_support"))]
    extern "C" {
        fn udata_setCommonData(data: *mut c_void, err: *mut i32);
        fn udata_setFileAccess(access: i32, err: *mut i32);
    }

    #[cfg(all(feature = "v8_intl_support"))]
    const UDATA_ONLY_PACKAGES: i32 = 1; // Example value, replace if incorrect

    #[cfg(all(feature = "v8_intl_support"))]
    const U_ZERO_ERROR: i32 = 0;

    #[cfg(all(feature = "v8_intl_support", target_endian = "little"))]
    const ICU_DATA_FILE_DEFAULT: &str = "icudtl.dat";

    #[cfg(all(feature = "v8_intl_support", target_endian = "big"))]
    const ICU_DATA_FILE_DEFAULT: &str = "icudtb.dat";

    #[cfg(all(feature = "v8_intl_support", not(any(target_endian = "little", target_endian = "big"))))]
    compile_error!("Unknown byte ordering");

    static mut ICU_DATA: Option<Vec<u8>> = None;
    static ICU_INIT: Once = Once::new();

    fn load_icu_data(icu_data_file: &str) -> Result<(), String> {
        let mut file = File::open(icu_data_file).map_err(|e| e.to_string())?;
        let size = file
            .seek(SeekFrom::End(0))
            .map_err(|e| e.to_string())? as usize;
        file.seek(SeekFrom::Start(0)).map_err(|e| e.to_string())?;

        let mut buffer = Vec::with_capacity(size);
        buffer.resize(size, 0);
        file.read_exact(&mut buffer).map_err(|e| e.to_string())?;

        let mut err: i32 = U_ZERO_ERROR;
        unsafe {
            udata_setCommonData(buffer.as_mut_ptr() as *mut c_void, &mut err);
            udata_setFileAccess(UDATA_ONLY_PACKAGES, &mut err);

            if err != U_ZERO_ERROR {
                return Err(format!("ICU initialization error: {}", err));
            }
            ICU_DATA = Some(buffer);
        }

        Ok(())
    }

    pub fn initialize_icu_default_location(
        exec_path: &str,
        icu_data_file: Option<&str>,
    ) -> Result<(), String> {
        #[cfg(not(feature = "v8_intl_support"))]
        {
            Ok(())
        }

        #[cfg(feature = "v8_intl_support")]
        {
            if let Some(file) = icu_data_file {
                initialize_icu(file)
            } else {
                let default_path = format!("{}/{}", exec_path, ICU_DATA_FILE_DEFAULT);
                initialize_icu(&default_path)
            }
        }
    }

    pub fn initialize_icu(icu_data_file: &str) -> Result<(), String> {
        #[cfg(not(feature = "v8_intl_support"))]
        {
            Ok(())
        }

        #[cfg(feature = "v8_intl_support")]
        {
            unsafe {
                ICU_INIT.call_once(|| {
                    if let Err(e) = load_icu_data(icu_data_file) {
                        panic!("Failed to initialize ICU: {}", e);
                    }
                });

                if ICU_DATA.is_some() {
                    Ok(())
                } else {
                    Err("ICU initialization failed".to_string())
                }
            }
        }
    }
}
