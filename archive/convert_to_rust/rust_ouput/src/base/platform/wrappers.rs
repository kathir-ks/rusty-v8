// Converted from V8 C++ source files:
// Header: wrappers.h
// Implementation: N/A
// 
// This file combines both header and implementation into idiomatic Rust code.

pub mod base {
    use std::fs::File;
    use std::io;

    pub fn fopen(filename: &str, mode: &str) -> Result<File, io::Error> {
        #[cfg(target_os = "starboard")]
        {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "fopen not supported on Starboard",
            ))
        }
        #[cfg(not(target_os = "starboard"))]
        {
            File::options().read(mode.contains('r')).write(mode.contains('w')).append(mode.contains('a')).create(true).open(filename)
        }
    }

    pub fn fclose(stream: File) -> Result<(), io::Error> {
        #[cfg(target_os = "starboard")]
        {
            Err(io::Error::new(
                io::ErrorKind::Other,
                "fclose not supported on Starboard",
            ))
        }
        #[cfg(not(target_os = "starboard"))]
        {
            drop(stream);
            Ok(())
        }
    }
}
