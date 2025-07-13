// Converted from V8 C++ source files:
// Header: startup-data-util.h
// Implementation: startup-data-util.cc
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod startup_data_util {
    use std::fs::File;
    use std::io::{Read, Seek, SeekFrom};
    use std::path::Path;
    use std::ffi::CStr;
    use std::os::raw::c_char;
    use std::sync::Mutex;
    use std::ptr;

    use crate::init::v8::V8;

    #[derive(Debug)]
    pub enum StartupDataError {
        FileOpenError,
        FileSeekError,
        FileSizeError,
        FileReadError,
        SnapshotSetError,
        RelativePathError,
    }

    type Result<T> = std::result::Result<T, StartupDataError>;

    struct StartupData {
        data: *mut i8,
        raw_size: usize,
    }

    unsafe impl Send for StartupData {}
    unsafe impl Sync for StartupData {}

    impl StartupData {
        fn new() -> Self {
            StartupData {
                data: ptr::null_mut(),
                raw_size: 0,
            }
        }
    }

    static mut G_SNAPSHOT: StartupData = StartupData {
        data: ptr::null_mut(),
        raw_size: 0,
    };

    static G_SNAPSHOT_MUTEX: Mutex<()> = Mutex::new(());

    fn clear_startup_data(data: &mut StartupData) {
        data.data = ptr::null_mut();
        data.raw_size = 0;
    }

    fn delete_startup_data(data: &mut StartupData) {
        if !data.data.is_null() {
            unsafe {
                drop(Box::from_raw(data.data as *mut i8));
            }
            clear_startup_data(data);
        }
    }

    extern "C" {
        fn atexit(func: unsafe extern "C" fn());
    }

    unsafe extern "C" fn free_startup_data() {
        let _guard = G_SNAPSHOT_MUTEX.lock().unwrap();
        delete_startup_data(&mut G_SNAPSHOT);
    }

    fn load(blob_file: &str, startup_data: &mut StartupData, setter_fn: fn(*mut i8, usize) -> Result<()>) -> Result<()> {
        clear_startup_data(startup_data);

        let path = Path::new(blob_file);
        let mut file = File::open(path).map_err(|_| StartupDataError::FileOpenError)?;

        let file_size = file.seek(SeekFrom::End(0))
            .map_err(|_| StartupDataError::FileSeekError)? as usize;
        file.seek(SeekFrom::Start(0)).map_err(|_| StartupDataError::FileSeekError)?;

        startup_data.raw_size = file_size;
        let mut buffer: Vec<i8> = vec![0; startup_data.raw_size];

        let read_size = file.read(unsafe {std::slice::from_raw_parts_mut(buffer.as_mut_ptr() as *mut u8, startup_data.raw_size)})
            .map_err(|_| StartupDataError::FileReadError)? as usize;

        if startup_data.raw_size == read_size {
            startup_data.data = Box::into_raw(buffer.into_boxed_slice()) as *mut i8;
            setter_fn(startup_data.data, startup_data.raw_size)?;
        } else {
            eprintln!("Corrupted startup resource '{}'.", blob_file);
        }

        Ok(())
    }

    fn load_from_file(snapshot_blob: &str) -> Result<()> {
        unsafe {
            let _guard = G_SNAPSHOT_MUTEX.lock().unwrap();
            load(snapshot_blob, &mut G_SNAPSHOT, set_snapshot_data_blob)?;
            atexit(free_startup_data);
        }
        Ok(())
    }

    fn set_snapshot_data_blob(data: *mut i8, raw_size: usize) -> Result<()> {
        unsafe {
            V8::set_snapshot_data_blob(data, raw_size);
        }
        Ok(())
    }

    pub fn initialize_external_startup_data(directory_path: &str) -> Result<()> {
        let snapshot_name = "snapshot_blob.bin";
        let snapshot_path = Path::new(directory_path).join(snapshot_name);
        let snapshot_path_str = snapshot_path.to_str().ok_or(StartupDataError::RelativePathError)?;
        load_from_file(snapshot_path_str)?;
        Ok(())
    }

    pub fn initialize_external_startup_data_from_file(snapshot_blob: &str) -> Result<()> {
        load_from_file(snapshot_blob)?;
        Ok(())
    }

    impl V8 {
        unsafe fn set_snapshot_data_blob(data: *mut i8, raw_size: usize) {
             println!("V8::SetSnapshotDataBlob called with data: {:p}, size: {}", data, raw_size);
        }
    }
}
