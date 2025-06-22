// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::io::{self, Read, Write};
use std::net::{self, TcpListener, TcpStream, SocketAddr};
use std::os::unix::io::{AsRawFd, RawFd}; // For Unix-specific code
use std::time::Duration;

#[cfg(unix)]
use libc::{EINTR};

#[cfg(windows)]
use winapi::um::winsock2::{INVALID_SOCKET, closesocket, WSAGetLastError, WSAEINTR, SOCKET};
#[cfg(windows)]
use winapi::shared::ws2def::AF_INET;
#[cfg(windows)]
use winapi::shared::inaddr::INADDR_ANY;
#[cfg(windows)]
use winapi::shared::netinet::in_addr;
#[cfg(windows)]
use std::mem;
#[cfg(windows)]
use winapi::um::winsock2::{socket, SOCK_STREAM, IPPROTO_TCP, bind, listen, accept, setsockopt, SOL_SOCKET, SO_REUSEADDR};
#[cfg(windows)]
use winapi::shared::minwindef::DWORD;
#[cfg(windows)]
use winapi::um::winsock2::ioctlsocket;
#[cfg(windows)]
use winapi::um::winsock2::FIONBIO;
#[cfg(windows)]
use std::ptr;
#[cfg(windows)]
use winapi::shared::windef::HWND;
#[cfg(windows)]
use winapi::um::synchapi::{CreateEventW, WaitForSingleObject, SetEvent};
#[cfg(windows)]
use winapi::shared::ntdef::HANDLE;
#[cfg(windows)]
use winapi::um::handleapi::CloseHandle;

pub mod gdb_server {

    use super::*;

    #[cfg(windows)]
    type SocketHandle = SOCKET;

    #[cfg(unix)]
    type SocketHandle = i32;

    #[cfg(windows)]
    const INVALID_SOCKET: SocketHandle = INVALID_SOCKET as SocketHandle;

    #[cfg(unix)]
    const INVALID_SOCKET: SocketHandle = -1;

    #[cfg(windows)]
    fn socket_get_last_error() -> i32 {
        unsafe { WSAGetLastError() }
    }

    #[cfg(unix)]
    fn socket_get_last_error() -> i32 {
        unsafe { libc::errno }
    }

    #[cfg(windows)]
    const K_ERR_INTERRUPT: i32 = WSAEINTR as i32;

    #[cfg(unix)]
    const K_ERR_INTERRUPT: i32 = EINTR;

    pub struct SocketBinding {
        socket_handle_: SocketHandle,
    }

    impl SocketBinding {
        /// Wrap existing socket handle.
        pub fn new(socket_handle: SocketHandle) -> Self {
            SocketBinding { socket_handle_: socket_handle }
        }

        /// Bind to the specified TCP port.
        pub fn bind(tcp_port: u16) -> Result<Self, io::Error> {
            #[cfg(windows)]
            unsafe {
                let socket_handle = socket(AF_INET as i32, SOCK_STREAM as i32, IPPROTO_TCP as i32);
                if socket_handle == INVALID_SOCKET {
                    return Err(io::Error::last_os_error());
                }

                let mut addr: winapi::shared::ws2def::SOCKADDR_IN = mem::zeroed();
                addr.sin_family = AF_INET as u16;
                addr.sin_addr = in_addr { s_addr: (INADDR_ANY as u32).to_be() };
                addr.sin_port = tcp_port.to_be();

                if bind(socket_handle, &mut addr as *mut _ as *mut _, mem::size_of::<winapi::shared::ws2def::SOCKADDR_IN>() as i32) == -1 {
                   closesocket(socket_handle);
                   return Err(io::Error::last_os_error());
                }

                if listen(socket_handle, 1) == -1 {
                    closesocket(socket_handle);
                    return Err(io::Error::last_os_error());
                }

                let mut reuseaddr: i32 = 1;
                if setsockopt(socket_handle, SOL_SOCKET, SO_REUSEADDR, &mut reuseaddr as *mut _ as *const _, mem::size_of::<i32>() as i32) == -1 {
                    closesocket(socket_handle);
                    return Err(io::Error::last_os_error());
                }

                Ok(SocketBinding { socket_handle_: socket_handle })
            }

            #[cfg(unix)]
            {
                let listener = TcpListener::bind(format!("127.0.0.1:{}", tcp_port))?;
                let socket_handle = listener.as_raw_fd();
                Ok(SocketBinding { socket_handle_: socket_handle })
            }
        }

        pub fn is_valid(&self) -> bool {
            self.socket_handle_ != INVALID_SOCKET
        }

        /// Create a transport object from this socket binding
        pub fn create_transport(&self) -> Result<SocketTransport, io::Error> {
            Ok(SocketTransport::new(self.socket_handle_))
        }

        /// Get port the socket is bound to.
        pub fn get_bound_port(&self) -> Result<u16, io::Error> {
            #[cfg(windows)]
            unsafe {
                use winapi::shared::ws2def::SOCKADDR;
                use winapi::shared::ws2def::SOCKADDR_IN;

                let mut addr: SOCKADDR_IN = mem::zeroed();
                let mut addrlen = mem::size_of::<SOCKADDR_IN>() as i32;

                if libc::getsockname(self.socket_handle_ as i32, &mut addr as *mut _ as *mut SOCKADDR, &mut addrlen) < 0 {
                    return Err(io::Error::last_os_error());
                }

                Ok(u16::from_be(addr.sin_port))
            }

            #[cfg(unix)]
            {
                use std::os::unix::io::FromRawFd;
                // Safe because we are the only owner of the socket.
                let listener = unsafe { TcpListener::from_raw_fd(self.socket_handle_) };
                let addr = listener.local_addr()?;
                Ok(addr.port())
            }
        }
    }

    pub trait TransportBase {
        /// Waits for an incoming connection on the bound port.
        fn accept_connection(&mut self) -> Result<(), io::Error>;

        /// Read {len} bytes from this transport, possibly blocking until enough data
        /// is available.
        /// {dst} must point to a buffer large enough to contain {len} bytes.
        /// Returns true on success.
        /// Returns false if the connection is closed; in that case the {dst} may have
        /// been partially overwritten.
        fn read(&mut self, dst: &mut [u8]) -> Result<(), io::Error>;

        /// Write {len} bytes to this transport.
        /// Return true on success, false if the connection is closed.
        fn write(&mut self, src: &[u8]) -> Result<(), io::Error>;

        /// Return true if there is data to read.
        fn is_data_available(&self) -> Result<bool, io::Error>;

        /// If we are connected to a debugger, gracefully closes the connection.
        /// This should be called when a debugging session gets closed.
        fn disconnect(&mut self) -> Result<(), io::Error>;

        /// Shuts down this transport, gracefully closing the existing connection and
        /// also closing the listening socket. This should be called when the GDB stub
        /// shuts down, when the program terminates.
        fn close(&mut self) -> Result<(), io::Error>;

        /// Blocks waiting for one of these two events to occur:
        /// - A network event (a new packet arrives, or the connection is dropped),
        /// - A thread event is signaled (the execution stopped because of a trap or
        /// breakpoint).
        fn wait_for_debug_stub_event(&mut self) -> Result<(), io::Error>;

        /// Signal that this transport should leave an alertable wait state because
        /// the execution of the debuggee was stopped because of a trap or breakpoint.
        fn signal_thread_event(&mut self) -> Result<bool, io::Error>;
    }

    pub struct Transport {
        buf_: Vec<u8>,
        pos_: i32,
        size_: i32,
        handle_bind_: SocketHandle,
        handle_accept_: SocketHandle,
    }

    impl Transport {
        pub const K_BUF_SIZE: usize = 4096;

        pub fn new(s: SocketHandle) -> Self {
            Transport {
                buf_: vec![0; Self::K_BUF_SIZE],
                pos_: 0,
                size_: 0,
                handle_bind_: s,
                handle_accept_: INVALID_SOCKET,
            }
        }

        /// Copy buffered data to *dst up to len bytes and update dst and len.
        fn copy_from_buffer(&mut self, dst: &mut [u8]) -> usize {
            let len = dst.len().min((self.size_ - self.pos_) as usize);
            dst[..len].copy_from_slice(&self.buf_[(self.pos_ as usize)..(self.pos_ as usize + len)]);
            self.pos_ += len as i32;
            len
        }
    }

    #[cfg(windows)]
    pub struct SocketTransport {
        transport: Transport,
        socket_event_: HANDLE,
        faulted_thread_event_: HANDLE,
        stream: Option<TcpStream>,
    }

    #[cfg(windows)]
    impl SocketTransport {
        pub fn new(s: SocketHandle) -> Self {
            unsafe {
                let socket_event_ = CreateEventW(ptr::null_mut(), 0, 0, ptr::null());
                let faulted_thread_event_ = CreateEventW(ptr::null_mut(), 0, 0, ptr::null());
                SocketTransport {
                    transport: Transport::new(s),
                    socket_event_: socket_event_,
                    faulted_thread_event_: faulted_thread_event_,
                    stream: None,
                }
            }
        }

        fn read_some_data(&mut self) -> Result<bool, io::Error> {
            if self.transport.handle_accept_ == INVALID_SOCKET {
                return Ok(false);
            }

            let mut bytes_read = 0;
            if let Some(stream) = &mut self.stream {
                match stream.read(&mut self.transport.buf_) {
                    Ok(size) => {
                        bytes_read = size;
                        self.transport.pos_ = 0;
                        self.transport.size_ = bytes_read as i32;
                        Ok(bytes_read > 0)
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            return Ok(false);
                        }
                        Err(e)
                    }
                }
            } else {
                Ok(false)
            }
        }
    }

    #[cfg(unix)]
    pub struct SocketTransport {
        transport: Transport,
        faulted_thread_fd_read_: i32,
        faulted_thread_fd_write_: i32,
        stream: Option<TcpStream>,
    }

    #[cfg(unix)]
    impl SocketTransport {
        pub fn new(s: SocketHandle) -> Result<Self, io::Error> {
            let mut fds: [i32; 2] = [0; 2];
            let result = unsafe { libc::pipe(fds.as_mut_ptr()) };
            if result != 0 {
                return Err(io::Error::last_os_error());
            }

            Ok(SocketTransport {
                transport: Transport::new(s),
                faulted_thread_fd_read_: fds[0],
                faulted_thread_fd_write_: fds[1],
                stream: None,
            })
        }

        fn read_some_data(&mut self) -> Result<bool, io::Error> {
            if self.transport.handle_accept_ == INVALID_SOCKET {
                return Ok(false);
            }

            let mut bytes_read = 0;
            if let Some(stream) = &mut self.stream {
                match stream.read(&mut self.transport.buf_) {
                    Ok(size) => {
                        bytes_read = size;
                        self.transport.pos_ = 0;
                        self.transport.size_ = bytes_read as i32;
                        Ok(bytes_read > 0)
                    }
                    Err(e) => {
                        if e.kind() == io::ErrorKind::WouldBlock {
                            return Ok(false);
                        }
                        Err(e)
                    }
                }
            } else {
                Ok(false)
            }
        }
    }

    #[cfg(windows)]
    impl TransportBase for SocketTransport {
        fn accept_connection(&mut self) -> Result<(), io::Error> {
            unsafe {
                let client_socket = accept(self.transport.handle_bind_, ptr::null_mut(), ptr::null_mut());
                if client_socket == INVALID_SOCKET {
                    return Err(io::Error::last_os_error());
                }

                let non_blocking: u32 = 1;
                if ioctlsocket(client_socket, FIONBIO as i32, &non_blocking as *const _ as *mut DWORD) == -1 {
                    closesocket(client_socket);
                    return Err(io::Error::last_os_error());
                }

                 self.transport.handle_accept_ = client_socket;

                let stream = TcpStream::from_raw_socket(client_socket as u64);
                self.stream = Some(stream);
                Ok(())
            }
        }

        fn read(&mut self, dst: &mut [u8]) -> Result<(), io::Error> {
            let mut total_read = 0;
            while total_read < dst.len() {
                if self.transport.pos_ >= self.transport.size_ {
                    if !self.read_some_data()? {
                        break;
                    }
                }

                let bytes_to_read = dst.len() - total_read;
                let bytes_read = self.transport.copy_from_buffer(&mut dst[total_read..total_read + bytes_to_read]);
                total_read += bytes_read;

                if bytes_read == 0 && self.transport.size_ == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection closed"));
                }
            }

            if total_read == dst.len() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough data"))
            }
        }

        fn write(&mut self, src: &[u8]) -> Result<(), io::Error> {
           if let Some(stream) = &mut self.stream {
                stream.write_all(src)?;
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected"))
            }
        }

        fn is_data_available(&self) -> Result<bool, io::Error> {
            //TODO: Implement using select
            Ok(self.transport.pos_ < self.transport.size_)
        }

        fn disconnect(&mut self) -> Result<(), io::Error> {
            unsafe {
                 if self.transport.handle_accept_ != INVALID_SOCKET {
                    closesocket(self.transport.handle_accept_);
                    self.transport.handle_accept_ = INVALID_SOCKET;
                 }
                self.stream = None;
            }
            Ok(())
        }

        fn close(&mut self) -> Result<(), io::Error> {
            unsafe {
                self.disconnect()?;
                if self.transport.handle_bind_ != INVALID_SOCKET {
                    closesocket(self.transport.handle_bind_);
                }
                CloseHandle(self.socket_event_);
                CloseHandle(self.faulted_thread_event_);
            }
            Ok(())
        }

        fn wait_for_debug_stub_event(&mut self) -> Result<(), io::Error> {
            unsafe {
                let handles = [self.socket_event_, self.faulted_thread_event_];
                let result = WaitForSingleObject(self.socket_event_, winapi::um::winbase::INFINITE);
                match result {
                    winapi::shared::winerror::WAIT_OBJECT_0 => {
                        println!("Socket event signaled");
                        Ok(())
                    },
                    winapi::shared::winerror::WAIT_TIMEOUT => {
                        println!("Wait timed out");
                        Err(io::Error::new(io::ErrorKind::TimedOut, "Wait timed out"))
                    }
                    winapi::shared::winerror::WAIT_FAILED => {
                        println!("Wait failed");
                        Err(io::Error::last_os_error())
                    },
                    _ => {
                        println!("Unknown wait result");
                        Err(io::Error::new(io::ErrorKind::Other, "Unknown wait result"))
                    }
                }
            }
        }

        fn signal_thread_event(&mut self) -> Result<bool, io::Error> {
            unsafe {
                if SetEvent(self.faulted_thread_event_) == 0 {
                   return Err(io::Error::last_os_error());
                }
                Ok(true)
            }
        }
    }

    #[cfg(unix)]
    impl TransportBase for SocketTransport {
        fn accept_connection(&mut self) -> Result<(), io::Error> {
            use std::os::unix::io::FromRawFd;
            // Safe because we are the only owner of the socket.
            let listener = unsafe { TcpListener::from_raw_fd(self.transport.handle_bind_) };
            listener.set_nonblocking(true)?; // Set non-blocking mode
            match listener.accept() {
                Ok((stream, _)) => {
                    stream.set_nonblocking(true)?;

                    self.transport.handle_accept_ = stream.as_raw_fd();
                    self.stream = Some(stream);
                    Ok(())
                }
                Err(e) if e.kind() == io::ErrorKind::WouldBlock => {
                    // No connection available yet, return Ok to try again later
                    Ok(())
                }
                Err(e) => {
                    Err(e) // Return other errors
                }
            }
        }

        fn read(&mut self, dst: &mut [u8]) -> Result<(), io::Error> {
            let mut total_read = 0;
            while total_read < dst.len() {
                if self.transport.pos_ >= self.transport.size_ {
                    if !self.read_some_data()? {
                        break;
                    }
                }

                let bytes_to_read = dst.len() - total_read;
                let bytes_read = self.transport.copy_from_buffer(&mut dst[total_read..total_read + bytes_to_read]);
                total_read += bytes_read;

                if bytes_read == 0 && self.transport.size_ == 0 {
                    return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Connection closed"));
                }
            }

            if total_read == dst.len() {
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough data"))
            }
        }

        fn write(&mut self, src: &[u8]) -> Result<(), io::Error> {
            if let Some(stream) = &mut self.stream {
                stream.write_all(src)?;
                Ok(())
            } else {
                Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected"))
            }
        }

        fn is_data_available(&self) -> Result<bool, io::Error> {
            //TODO: Implement using select
            Ok(self.transport.pos_ < self.transport.size_)
        }

        fn disconnect(&mut self) -> Result<(), io::Error> {
            #[cfg(unix)]
            unsafe {
                if self.transport.handle_accept_ != INVALID_SOCKET {
                    libc::close(self.transport.handle_accept_);
                    self.transport.handle_accept_ = INVALID_SOCKET;
                }
                self.stream = None;
            }
            Ok(())
        }

        fn close(&mut self) -> Result<(), io::Error> {
            self.disconnect()?;

            #[cfg(unix)]
            unsafe {
                if self.transport.handle_bind_ != INVALID_SOCKET {
                    libc::close(self.transport.handle_bind_);
                }
                libc::close(self.faulted_thread_fd_read_);
                libc::close(self.faulted_thread_fd_write_);
            }
            Ok(())
        }

        fn wait_for_debug_stub_event(&mut self) -> Result<(), io::Error> {
            use std::os::unix::io::RawFd;
            use libc::{fd_set, FD_ZERO, FD_SET, select, timeval};
            use std::mem::zeroed;

            let fd_read = self.faulted_thread_fd_read_ as RawFd;
            let fd_socket = match &self.stream {
                Some(stream) => stream.as_raw_fd(),
                None => return Err(io::Error::new(io::ErrorKind::NotConnected, "Not connected")),
            };

            unsafe {
                let mut read_fds: fd_set = zeroed();
                FD_ZERO(&mut read_fds);
                FD_SET(fd_read, &mut read_fds);
                FD_SET(fd_socket, &mut read_fds);

                let max_fd = std::cmp::max(fd_read, fd_socket) + 1;

                let mut timeout: timeval = zeroed();
                timeout.tv_sec = 5; // Timeout after 5 seconds
                timeout.tv_usec = 0;

                let result = select(max_fd, &mut read_fds, std::ptr::null_mut(), std::ptr::null_mut(), &mut timeout);

                match result {
                    -1 => {
                        let err = io::Error::last_os_error();
                        if err.kind() == io::ErrorKind::Interrupted {
                            // Interrupted by a signal, retry
                            return Ok(());
                        }
                        Err(err)
                    }
                    0 => {
                        // Timeout
                        Err(io::Error::new(io::ErrorKind::TimedOut, "Timeout waiting for events"))
                    }
                    _ => {
                        if libc::FD_ISSET(fd_read, &read_fds) {
                            // Thread event signaled
                            println!("Thread event signaled");
                            Ok(())
                        } else if libc::FD_ISSET(fd_socket, &read_fds) {
                            // Socket event signaled
                            println!("Socket event signaled");
                            Ok(())
                        } else {
                            Err(io::Error::new(io::ErrorKind::Other, "Unknown event"))
                        }
                    }
                }
            }
        }

        fn signal_thread_event(&mut self) -> Result<bool, io::Error> {
            let buf: [u8; 1] = [1];
            let bytes_written = unsafe { libc::write(self.faulted_thread_fd_write_, buf.as_ptr() as *const _, 1) };
            if bytes_written == 1 {
                Ok(true)
            } else {
                Err(io::Error::last_os_error())
            }
        }
    }

    impl Drop for SocketTransport {
        fn drop(&mut self) {
            let _ = self.close(); //ignore error
        }
    }
}