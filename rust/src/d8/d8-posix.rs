use std::ffi::{CString, NulError};
use std::io::{Read, Write};
use std::mem::MaybeUninit;
use std::os::raw::{c_char, c_int, c_void};
use std::ptr;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};
use libc::{
    close, dup2, EINTR, EEXIST, ENOENT, errno, execvp, fcntl, fork, gettimeofday, kill, mkdir,
    pipe, read, rmdir, select, send, setenv, setsockopt, shutdown, socket, stat, strerror, strlen,
    strrchr, timeval, umask, unsetenv, waitpid, AF_INET, CLD_EXITED, CLD_KILLED, connect,
    FD_CLOEXEC, FD_ISSET, FD_SET, FD_ZERO, F_GETFL, F_SETFL, htonl, htons, INADDR_LOOPBACK,
    O_NONBLOCK, PF_INET, P_PID, SA_RESTART, SIGINT, sockaddr, sockaddr_in, socklen_t, SOCK_STREAM,
    SOL_SOCKET, SO_REUSEADDR, S_IFDIR, WEXITSTATUS, WIFSIGNALED, WNOWAIT, WTERMSIG, WEXITED,
    WNOHANG, waitid, siginfo_t, usleep,
};
use std::convert::TryInto;
use std::net::TcpStream;
use std::os::unix::io::AsRawFd;

//use v8::{Isolate, Local, Value, Array, String, Integer, FunctionCallbackInfo, ObjectTemplate, FunctionTemplate}; // Assuming these are re-implemented or provided
// use base; // Assuming this is re-implemented or provided

// Placeholder for V8 types, adapt to your Rust V8 bindings or mock implementations
pub struct Isolate {}
pub struct Local<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct Value {}
pub struct Array {}
pub struct String {}
pub struct Integer {}
pub struct FunctionCallbackInfo<T> {
    _phantom: std::marker::PhantomData<T>,
}
pub struct ObjectTemplate {}
pub struct FunctionTemplate {}

impl<T> Local<T> {
    pub fn empty() -> Self {
        Local {
            _phantom: std::marker::PhantomData,
        }
    }
    pub fn is_undefined(&self) -> bool {
        false // Replace with actual implementation if needed
    }
}

impl String {
    pub fn new_from_utf8(_isolate: &Isolate, buffer: &[u8]) -> Result<Local<String>, std::string::FromUtf8Error> {
        // Replace with actual V8 string creation
        String::new_from_utf8_with_hint(_isolate, buffer, v8::NewStringType::kNormal, buffer.len())
    }
    
    pub fn new_from_utf8_with_hint(_isolate: &Isolate, buffer: &[u8], _hint: v8::NewStringType, _length: usize) -> Result<Local<String>, std::string::FromUtf8Error> {
        match std::str::from_utf8(buffer) {
            Ok(_) => Ok(Local { _phantom: std::marker::PhantomData }), // Dummy implementation
            Err(e) => Err(e),
        }
    }

    pub fn concat(_isolate: &Isolate, a: Local<String>, b: Local<String>) -> Local<String> {
        // Replace with actual V8 string concatenation
        Local { _phantom: std::marker::PhantomData } // Dummy implementation
    }
}

impl Integer {
    pub fn new(_isolate: &Isolate, value: i32) -> Local<Integer> {
        // Replace with actual V8 integer creation
        Local { _phantom: std::marker::PhantomData } // Dummy implementation
    }
}

impl Array {
    pub fn new(_isolate: &Isolate, length: i32) -> Local<Array> {
        // Replace with actual V8 array creation
        Local { _phantom: std::marker::PhantomData } // Dummy implementation
    }
}

pub mod v8 {
    pub enum NewStringType {
        kNormal,
    }
}

pub mod base {
    pub fn bit_cast<T, U>(input: T) -> U {
        unsafe { std::mem::transmute_copy(&input) }
    }
}

macro_rules! throw_error {
    ($isolate:expr, $message:expr) => {
        {
            // Replace with your V8 error throwing mechanism
            eprintln!("Error: {}", $message);
            return;
        }
    };
}

/// If the buffer ends in the middle of a UTF-8 sequence then we return
/// the length of the string up to but not including the incomplete UTF-8
/// sequence.  If the buffer ends with a valid UTF-8 sequence then we
/// return the whole buffer.
fn length_without_incomplete_utf8(buffer: &mut [u8], len: usize) -> usize {
    let mut answer = len;

    // 1-byte encoding.
    const UTF8_SINGLE_BYTE_MASK: i32 = 0x80;
    const UTF8_SINGLE_BYTE_VALUE: i32 = 0x00;
    // 2-byte encoding.
    const UTF8_TWO_BYTE_MASK: i32 = 0xE0;
    const UTF8_TWO_BYTE_VALUE: i32 = 0xC0;
    // 3-byte encoding.
    const UTF8_THREE_BYTE_MASK: i32 = 0xF0;
    const UTF8_THREE_BYTE_VALUE: i32 = 0xE0;
    // 4-byte encoding.
    const UTF8_FOUR_BYTE_MASK: i32 = 0xF8;
    const UTF8_FOUR_BYTE_VALUE: i32 = 0xF0;
    // Subsequent bytes of a multi-byte encoding.
    const MULTI_BYTE_MASK: i32 = 0xC0;
    const MULTI_BYTE_VALUE: i32 = 0x80;

    let mut multi_byte_bytes_seen = 0;
    while answer > 0 {
        let c = buffer[answer - 1] as i32;

        // Ends in valid single-byte sequence?
        if (c & UTF8_SINGLE_BYTE_MASK) == UTF8_SINGLE_BYTE_VALUE {
            return answer;
        }

        // Ends in one or more subsequent bytes of a multi-byte value?
        if (c & MULTI_BYTE_MASK) == MULTI_BYTE_VALUE {
            multi_byte_bytes_seen += 1;
            answer -= 1;
        } else {
            if (c & UTF8_TWO_BYTE_MASK) == UTF8_TWO_BYTE_VALUE {
                if multi_byte_bytes_seen >= 1 {
                    return answer + 2;
                }
                return answer - 1;
            } else if (c & UTF8_THREE_BYTE_MASK) == UTF8_THREE_BYTE_VALUE {
                if multi_byte_bytes_seen >= 2 {
                    return answer + 3;
                }
                return answer - 1;
            } else if (c & UTF8_FOUR_BYTE_MASK) == UTF8_FOUR_BYTE_VALUE {
                if multi_byte_bytes_seen >= 3 {
                    return answer + 4;
                }
                return answer - 1;
            } else {
                return answer; // Malformed UTF-8.
            }
        }
    }
    0
}

/// Suspends the thread until there is data available from the child process.
/// Returns false on timeout, true on data ready.
fn wait_on_fd(fd: c_int, read_timeout: i32, total_timeout: i32, start_time: &timeval) -> bool {
    let mut readfds: libc::fd_set = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut writefds: libc::fd_set = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut exceptfds: libc::fd_set = unsafe { MaybeUninit::zeroed().assume_init() };
    let mut timeout = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };

    let mut gone = 0;
    if total_timeout != -1 {
        let mut time_now = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        unsafe {
            gettimeofday(&mut time_now as *mut timeval, ptr::null_mut());
        }

        let seconds = time_now.tv_sec - start_time.tv_sec;
        gone = (seconds * 1000 + (time_now.tv_usec - start_time.tv_usec) / 1000) as i32;
        if gone >= total_timeout {
            return false;
        }
    }

    unsafe {
        FD_ZERO(&mut readfds);
        FD_ZERO(&mut writefds);
        FD_ZERO(&mut exceptfds);
        FD_SET(fd, &mut readfds);
        FD_SET(fd, &mut exceptfds);
    }

    if read_timeout == -1 || (total_timeout != -1 && total_timeout - gone < read_timeout) {
        let temp_read_timeout = total_timeout - gone;
        timeout.tv_usec = ((temp_read_timeout % 1000) * 1000).into();
        timeout.tv_sec = (temp_read_timeout / 1000).into();
    } else {
        timeout.tv_usec = ((read_timeout % 1000) * 1000).into();
        timeout.tv_sec = (read_timeout / 1000).into();
    }

    let number_of_fds_ready = unsafe {
        select(
            fd + 1,
            &mut readfds,
            &mut writefds,
            &mut exceptfds,
            if read_timeout != -1 {
                &mut timeout
            } else {
                ptr::null_mut()
            },
        )
    };

    number_of_fds_ready == 1
}

/// Checks whether we ran out of time on the timeout.  Returns true if we ran out
/// of time, false if we still have time.
fn time_is_out(start_time: &timeval, total_time: i32) -> bool {
    if total_time == -1 {
        return false;
    }

    let mut time_now = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    unsafe {
        gettimeofday(&mut time_now as *mut timeval, ptr::null_mut());
    }

    // Careful about overflow.
    let seconds = (time_now.tv_sec - start_time.tv_sec) as i32;
    if seconds > 100 {
        if seconds * 1000 > total_time {
            return true;
        }
        return false;
    }

    let useconds = (time_now.tv_usec - start_time.tv_usec) as i32;
    if seconds * 1000000 + useconds > total_time * 1000 {
        return true;
    }

    false
}

/// A utility class that does a non-hanging waitpid on the child process if we
/// bail out of the System() function early.  If you don't ever do a waitpid on
/// a subprocess then it turns into one of those annoying 'zombie processes'.
struct ZombieProtector {
    pid: i32,
}

impl ZombieProtector {
    fn new(pid: i32) -> Self {
        ZombieProtector { pid }
    }

    fn child_is_dead_now(&mut self) {
        self.pid = 0;
    }
}

impl Drop for ZombieProtector {
    fn drop(&mut self) {
        if self.pid != 0 {
            unsafe {
                waitpid(self.pid, ptr::null_mut(), 0);
            }
        }
    }
}

/// A utility class that closes a file descriptor when it goes out of scope.
struct OpenFdCloser {
    fd: c_int,
}

impl OpenFdCloser {
    fn new(fd: c_int) -> Self {
        OpenFdCloser { fd }
    }
}

impl Drop for OpenFdCloser {
    fn drop(&mut self) {
        unsafe {
            close(self.fd);
        }
    }
}

/// A utility class that takes the array of command arguments and puts then in an
/// array of new[]ed UTF-8 C strings.  Deallocates them again when it goes out of
/// scope.
struct ExecArgs {
    exec_args: Vec<CString>,
    owned_ptrs: Vec<*mut c_char>,
}

const K_MAX_ARGS: usize = 1000;

impl ExecArgs {
    fn new() -> Self {
        ExecArgs {
            exec_args: Vec::new(),
            owned_ptrs: Vec::new(),
        }
    }

    fn init(&mut self, isolate: &Isolate, arg0: Local<Value>, command_args: Local<Array>) -> Result<(), String> {
        // Convert arg0 to CString
        // Replace String::Utf8Value with your string conversion logic
        let prog_result = string_to_cstring(isolate, arg0);
        let prog = match prog_result {
            Ok(p) => p,
            Err(e) => return Err(e),
        };

        let prog_str = prog.to_str().map_err(|_| "Failed to convert program to string".to_string())?;

        let c_arg0 = CString::new(prog_str).map_err(|_| "Failed to create CString for program".to_string())?;
        self.exec_args.push(c_arg0);
        self.owned_ptrs.push(self.exec_args.last().unwrap().as_ptr() as *mut c_char);

        // Handle the rest of the command arguments
        for j in 0..command_args_length(isolate, &command_args).map_err(|e| e.to_string())? {
            // Replace command_args.Get with your array access logic
            let arg_result = command_args_get(isolate, &command_args, j);
            let arg = match arg_result {
                Ok(a) => a,
                Err(e) => return Err(e),
            };

            let utf8_arg_result = string_to_cstring(isolate, arg);
            let utf8_arg = match utf8_arg_result {
                Ok(ua) => ua,
                Err(e) => return Err(e),
            };

            let utf8_arg_str = utf8_arg.to_str().map_err(|_| "Failed to convert argument to string".to_string())?;

            let c_arg = CString::new(utf8_arg_str).map_err(|_| "Failed to create CString for argument".to_string())?;
            self.exec_args.push(c_arg);
            self.owned_ptrs.push(self.exec_args.last().unwrap().as_ptr() as *mut c_char);
        }

        Ok(())
    }

    fn arg_array(&self) -> *const *mut c_char {
        self.owned_ptrs.as_ptr() as *const *mut c_char
    }

    fn arg0(&self) -> *const c_char {
        if let Some(first_arg) = self.exec_args.first() {
            first_arg.as_ptr()
        } else {
            ptr::null()
        }
    }
}

fn command_args_length(_isolate: &Isolate, _command_args: &Array) -> Result<i32, String> {
    // Replace with actual V8 Array length retrieval logic
    Ok(0) // Dummy implementation
}

fn command_args_get(_isolate: &Isolate, _command_args: &Array, _index: i32) -> Result<Local<Value>, String> {
    // Replace with actual V8 Array element retrieval logic
    Ok(Local { _phantom: std::marker::PhantomData }) // Dummy implementation
}

fn string_to_cstring(_isolate: &Isolate, _arg: Local<Value>) -> Result<CString, String> {
    // Replace with actual V8 string conversion logic
    CString::new("dummy").map_err(|_| "Failed to create CString".to_string()) // Dummy implementation
}

/// Gets the optional timeouts from the arguments to the system() call.
fn get_timeouts(info: &FunctionCallbackInfo<Value>) -> Result<(i32, i32), String> {
    let mut read_timeout = -1;
    let mut total_timeout = -1;

    // Replace with your V8 argument access logic
    if false /*info.Length() > 3*/ {
        if false /*info[3]->IsNumber()*/ {
            // total_timeout = info[3]->Int32Value(info.GetIsolate()->GetCurrentContext()).FromJust();
        } else {
            return Err("system: Argument 4 must be a number".to_string());
        }
    }
    if false /*info.Length() > 2*/ {
        if false /*info[2]->IsNumber()*/ {
            // read_timeout = info[2]->Int32Value(info.GetIsolate()->GetCurrentContext()).FromJust();
        } else {
            return Err("system: Argument 3 must be a number".to_string());
        }
    }

    Ok((read_timeout, total_timeout))
}

fn v8_strerror(isolate: &Isolate, err: c_int) -> Local<String> {
    let err_str = unsafe {
        let c_str = strerror(err);
        std::ffi::CStr::from_ptr(c_str).to_string_lossy().into_owned()
    };
    // Replace with your V8 string creation logic
    String::new_from_utf8(isolate, err_str.as_bytes()).unwrap() // Dummy implementation
}

const K_READ_FD: usize = 0;
const K_WRITE_FD: usize = 1;

/// This is run in the child process after fork() but before exec().  It normally
/// ends with the child process being replaced with the desired child program.
/// It only returns if an error occurred.
unsafe fn exec_subprocess(exec_error_fds: &mut [c_int; 2], stdout_fds: &mut [c_int; 2], exec_args: &ExecArgs) {
    close(exec_error_fds[K_READ_FD]); // Don't need this in the child.
    close(stdout_fds[K_READ_FD]);     // Don't need this in the child.
    close(1);                           // Close stdout.
    dup2(stdout_fds[K_WRITE_FD], 1);    // Dup pipe fd to stdout.
    close(stdout_fds[K_WRITE_FD]);     // Don't need the original fd now.
    fcntl(exec_error_fds[K_WRITE_FD], FD_CLOEXEC, FD_CLOEXEC);
    let result = execvp(exec_args.arg0(), exec_args.arg_array());
    // Only get here if the exec failed.  Write errno to the parent to tell
    // them it went wrong.  If it went well the pipe is closed.
    if result == -1 {
        let err = errno();
        let mut bytes_written: ssize_t;
        loop {
            bytes_written = write(exec_error_fds[K_WRITE_FD], &err as *const i32 as *const c_void, std::mem::size_of::<i32>() as usize) as ssize_t;
            if bytes_written == -1 && errno() == EINTR {
                continue;
            }
            break;
        }
        libc::_exit(1);
    }
}

type ssize_t = isize;

/// Runs in the parent process.  Checks that the child was able to exec (closing
/// the file desriptor), or reports an error if it failed.
fn child_launched_ok(isolate: &Isolate, exec_error_fds: &mut [c_int; 2]) -> Result<(), Local<String>> {
    let mut err: i32 = 0;
    let mut bytes_read: ssize_t;
    unsafe {
        loop {
            bytes_read = read(exec_error_fds[K_READ_FD], &mut err as *mut i32 as *mut c_void, std::mem::size_of::<i32>() as usize) as ssize_t;
            if bytes_read == -1 && errno() == EINTR {
                continue;
            }
            break;
        }
    }
    if bytes_read != 0 {
        return Err(v8_strerror(isolate, err));
    }
    Ok(())
}

/// Accumulates the output from the child in a string handle.  Returns true if it
/// succeeded or false if an exception was thrown.
fn get_stdout(
    isolate: &Isolate,
    child_fd: c_int,
    start_time: &timeval,
    read_timeout: i32,
    total_timeout: i32,
) -> Result<Local<Value>, Local<String>> {
    let mut accumulator = Local::<String>::empty();

    let mut fullness = 0;
    const K_STDOUT_READ_BUFFER_SIZE: usize = 4096;
    let mut buffer: [u8; K_STDOUT_READ_BUFFER_SIZE] = [0; K_STDOUT_READ_BUFFER_SIZE];

    if unsafe { fcntl(child_fd, F_SETFL, O_NONBLOCK) } != 0 {
        return Err(v8_strerror(isolate, unsafe { errno() }));
    }

    let mut bytes_read: isize;
    loop {
        unsafe {
            bytes_read = read(
                child_fd,
                buffer.as_mut_ptr().add(fullness) as *mut c_void,
                K_STDOUT_READ_BUFFER_SIZE - fullness,
            ) as isize;
        }

        if bytes_read == -1 {
            let current_errno = unsafe { errno() };
            if current_errno == libc::EAGAIN {
                if !wait_on_fd(child_fd, read_timeout, total_timeout, start_time)
                    || time_is_out(start_time, total_timeout)
                {
                    return Err(v8_strerror(isolate, libc::ETIMEDOUT)); // Replace with a meaningful timeout error
                }
                continue;
            } else if current_errno == EINTR {
                continue;
            } else {
                break;
            }
        }

        if bytes_read + fullness as isize > 0 {
            let length = if bytes_read == 0 {
                (bytes_read as usize) + fullness
            } else {
                length_without_incomplete_utf8(&mut buffer, (bytes_read as usize) + fullness)
            };

            let addition = String::new_from_utf8(isolate, &buffer[..length]).map_err(|_| v8_strerror(isolate, unsafe { errno() }))?; // Replace with actual V8 string creation
            accumulator = String::concat(isolate, accumulator, addition);

            fullness = (bytes_read as usize) + fullness - length;
            unsafe {
                ptr::copy(
                    buffer.as_ptr().add(length),
                    buffer.as_mut_ptr(),
                    fullness,
                );
            }
        }

        if bytes_read == 0 {
            break;
        }
    }

    Ok(Local::<Value>::empty()) // Dummy implementation
}

/// Get exit status of child.
fn wait_for_child(
    isolate: &Isolate,
    pid: i32,
    child_waiter: &mut ZombieProtector,
    start_time: &timeval,
    read_timeout: i32,
    total_timeout: i32,
) -> Result<(), Local<String>> {
    #[cfg(all(
        target_os = "linux",
        not(target_env = "android"),
        not(target_os = "macos"),
        not(target_os = "netbsd"),
        not(target_os = "fuchsia")
    ))]
    {
        let mut child_info: siginfo_t = unsafe { std::mem::zeroed() };
        child_info.si_pid = 0;
        let mut useconds: u32 = 1;
        // Wait for child to exit.
        while child_info.si_pid == 0 {
            unsafe {
                waitid(
                    P_PID,
                    pid as u32,
                    &mut child_info,
                    WEXITED | WNOHANG | WNOWAIT,
                );
                usleep(useconds);
                if useconds < 1_000_000 {
                    useconds <<= 1;
                }
                if (read_timeout != -1 && (useconds / 1000) as i32 > read_timeout)
                    || time_is_out(start_time, total_timeout)
                {
                    return Err(v8_strerror(isolate, libc::ETIMEDOUT));
                }
            }
        }
        if child_info.si_code == CLD_KILLED as i32 {
            let message = format!("Child killed by signal {}", child_info.si_status);
            return Err(String::new_from_utf8(isolate, message.as_bytes()).map_err(|_| v8_strerror(isolate, unsafe { errno() }))?);
        }
        if child_info.si_code == CLD_EXITED as i32 && child_info.si_status != 0 {
            let message = format!("Child exited with status {}", child_info.si_status);
            return Err(String::new_from_utf8(isolate, message.as_bytes()).map_err(|_| v8_strerror(isolate, unsafe { errno() }))?);
        }
    }

    #[cfg(not(all(
        target_os = "linux",
        not(target_env = "android"),
        not(target_os = "macos"),
        not(target_os = "netbsd"),
        not(target_os = "fuchsia")
    )))]
    {
        let mut child_status: i32 = 0;
        unsafe {
            waitpid(pid, &mut child_status, 0); // We hang here if the child doesn't exit.
        }
        child_waiter.child_is_dead_now();

        if libc::WIFSIGNALED(child_status) {
            let message = format!("Child killed by signal {}", WTERMSIG(child_status));
            return Err(String::new_from_utf8(isolate, message.as_bytes()).map_err(|_| v8_strerror(isolate, unsafe { errno() }))?);
        }
        if libc::WEXITSTATUS(child_status) != 0 {
            let exit_status = WEXITSTATUS(child_status);
            let message = format!("Child exited with status {}", exit_status);
            return Err(String::new_from_utf8(isolate, message.as_bytes()).map_err(|_| v8_strerror(isolate, unsafe { errno() }))?);
        }
    }

    Ok(())
}

/// Implementation of the system() function (see d8.h for details).
pub mod shell {
    use super::*;

    pub fn system(info: &FunctionCallbackInfo<Value>) {
        println!("Shell::System called"); // Replace with actual V8 validation logic

        let isolate = &Isolate {}; // Replace with your V8 Isolate instance

        let timeouts_result = get_timeouts(info);
        let (read_timeout, total_timeout) = match timeouts_result {
            Ok(timeouts) => timeouts,
            Err(e) => {
                //info.GetIsolate()->ThrowError(e);
                println!("{}", e);
                return;
            }
        };

        let command_args: Local<Array>; // Replace with actual V8 array retrieval logic
        command_args = Array::new(isolate, 0);

        if false /*command_args.Length() > ExecArgs::kMaxArgs*/ {
            //info.GetIsolate()->ThrowError("Too many arguments to system()");
            println!("Too many arguments to system()");
            return;
        }

        if false /*info.Length() < 1*/ {
            //info.GetIsolate()->ThrowError("Too few arguments to system()");
            println!("Too few arguments to system()");
            return;
        }

        let mut start_time = timeval {
            tv_sec: 0,
            tv_usec: 0,
        };
        unsafe {
            gettimeofday(&mut start_time as *mut timeval, ptr::null_mut());
        }

        let mut exec_args = ExecArgs::new();
        let arg0 = Local::<Value>::empty(); // Replace with actual V8 value retrieval logic
        let init_result = exec_args.init(isolate, arg0, command_args);

        if let Err(e) = init_result {
            println!("{}", e);
            return;
        }

        let mut exec_error_fds: [c_int; 2] = [0; 2];
        let mut stdout_fds: [c_int; 2] = [0; 2];

        if unsafe { pipe(exec_error_fds.as_mut_ptr()) } != 0 {
            //info.GetIsolate()->ThrowError("pipe syscall failed.");
            println!("pipe syscall failed.");
            return;
        }
        if unsafe { pipe(stdout_fds.as_mut_ptr()) } != 0 {
            //info.GetIsolate()->ThrowError("pipe syscall failed.");
            println!("pipe syscall failed.");
            return;
        }

        let pid = unsafe { fork() };
        if pid == 0 {
            // Child process.
            unsafe {
                exec_subprocess(&mut exec_error_fds, &mut stdout_fds, &exec_args);
                libc::_exit(1);
            }
        }

        // Parent process.  Ensure that we clean up if we exit this function early.
        let mut child_waiter = ZombieProtector::new(pid);
        unsafe {
            close(exec_error_fds[K_WRITE_FD]);
            close(stdout_fds[K_WRITE_FD]);
        }
        let _error_read_closer = OpenFdCloser::new(exec_error_fds[K_READ_FD]);
        let _stdout_read_closer = OpenFdCloser::new(stdout_fds[K_READ_FD]);

        let child_launched_result = child_launched_ok(isolate, &mut exec_error_fds);
        if let Err(e) = child_launched_result {
            return;
        }

        let accumulator_result = get_stdout(
            isolate,
            stdout_fds[K_READ_FD],
            &start_time,
            read_timeout,
            total_timeout,
        );

        let accumulator = match accumulator_result {
            Ok(a) => a,
            Err(e) => {
                unsafe {
                    kill(pid, SIGINT); // On timeout, kill the subprocess.
                }
                //info.GetReturnValue().Set(accumulator);
                return;
            }
        };

        let wait_for_child_result =
            wait_for_child(isolate, pid, &mut child_waiter, &start_time, read_timeout, total_timeout);

        if let Err(_e) = wait_for_child_result {
            return;
        }

        //info.GetReturnValue().Set(accumulator);
    }

    pub fn change_directory(info: &FunctionCallbackInfo<Value>) {
        //Replace with actual V8 validation logic
        println!("Shell::ChangeDirectory called");

        if false /*info.Length() != 1*/ {
            println!("chdir() takes one argument");
            return;
        }
        // Replace with actual V8 string retrieval logic

        //Replace with actual V8 string retrieval logic
        let directory_result = string_to_cstring(&Isolate {}, Local::<Value>::empty());
        let directory_cstring = match directory_result {
            Ok(d) => d,
            Err(e) => {
                println!("{}", e);
                return;
            }
        };
        let directory = directory_cstring.as_ptr();
        if unsafe { libc::chdir(directory) } != 0 {
            //info.GetIsolate()->ThrowError(v8_strerror(info.GetIsolate(), unsafe { errno() }));
            println!("chdir failed");
            return;
        }
    }

    pub fn set_umask(info: &FunctionCallbackInfo<Value>) {
        //Replace with actual V8 validation logic
        println!("Shell::SetUMask called");
        if false /*info.Length() != 1*/ {
            println!("umask() takes one argument");
            return;
        }
        if false /*info[0]->IsNumber()*/ {
            println!("umask() argument must be numeric");
            return;
        }
        // Replace with actual V8 integer retrieval logic
        let previous = 0;
        //let previous = umask(info[0]->Int32Value(info.GetIsolate()->GetCurrentContext()).FromJust() as libc::mode_t);
        //info.GetReturnValue().Set(previous);
    }

    pub fn make_directory(info: &FunctionCallbackInfo<Value>) {
        println!("Shell::MakeDirectory called");
        let mask: libc::mode_t = 0o777;

        if false /*info.Length() == 2