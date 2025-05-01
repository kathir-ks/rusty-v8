// Copyright 2020 the V8 project authors. All rights reserved.
// Use of this source code is governed by a BSD-style license that can be
// found in the LICENSE file.

use std::io::{Read, Write};
use std::str;
use std::string::String;

mod transport;
use transport::TransportBase;

mod packet;
use packet::Packet;

macro_rules! trace_gdb_remote {
    ($($arg:tt)*) => {
        //println!($($arg)*); // Uncomment to enable tracing
    };
}

pub struct Session<'a> {
    io_: Box<dyn TransportBase + 'a>,
    connected_: bool,
    ack_enabled_: bool,
}

impl<'a> Session<'a> {
    pub fn new(transport: Box<dyn TransportBase + 'a>) -> Session<'a> {
        Session {
            io_: transport,
            connected_: true,
            ack_enabled_: true,
        }
    }

    pub fn wait_for_debug_stub_event(&mut self) {
        self.io_.wait_for_debug_stub_event();
    }

    pub fn signal_thread_event(&mut self) -> bool {
        self.io_.signal_thread_event()
    }

    pub fn is_data_available(&self) -> bool {
        self.io_.is_data_available()
    }

    pub fn is_connected(&self) -> bool {
        self.connected_
    }

    pub fn disconnect(&mut self) {
        self.io_.disconnect();
        self.connected_ = false;
    }

    pub fn get_char(&mut self, ch: &mut char) -> bool {
        let mut buf = [0u8; 1];
        if !self.io_.read(&mut buf[..]) {
            self.disconnect();
            return false;
        }

        *ch = buf[0] as char;
        true
    }

    pub fn send_packet(&mut self, pkt: &mut Packet, expect_ack: bool) -> bool {
        loop {
            let data = pkt.get_packet_data();

            trace_gdb_remote!(
                "TX {}\n",
                if data.len() < 160 {
                    data.clone()
                } else {
                    let mut s = data[0..160].to_string();
                    s.push_str("...");
                    s
                }
            );
            if !self.io_.write(data.as_bytes()) {
                return false;
            }

            if !expect_ack || !self.ack_enabled_ {
                break;
            }

            let mut ch = ' ';
            if !self.get_char(&mut ch) {
                return false;
            }

            if ch == '+' {
                break;
            }
        }

        true
    }

    pub fn get_payload(&mut self, pkt: &mut Packet, checksum: &mut u8) -> bool {
        pkt.clear();
        *checksum = 0;

        let mut ch = ' ';
        while self.get_char(&mut ch) {
            if ch == '#' {
                return true;
            } else if ch == '$' {
                trace_gdb_remote!("RX Missing $, retry.\n");
                *checksum = 0;
                pkt.clear();
            } else {
                *checksum = checksum.wrapping_add(ch as u8);
                pkt.add_raw_char(ch);
            }
        }
        false
    }

    pub fn get_packet(&mut self, pkt: &mut Packet) -> bool {
        loop {
            let mut ch = ' ';
            loop {
                if !self.get_char(&mut ch) {
                    return false;
                }
                if ch == '$' {
                    break;
                }
            }

            let mut running_checksum = 0u8;
            if !self.get_payload(pkt, &mut running_checksum) {
                return false;
            }

            let mut trailing_checksum = 0u8;
            let mut chars = [0u8; 2];
            if !self.get_char(&mut chars[0] as *mut u8 as *mut char)
                || !self.get_char(&mut chars[1] as *mut u8 as *mut char)
                || !hex_to_u8(&chars, &mut trailing_checksum)
            {
                return false;
            }

            trace_gdb_remote!(
                "RX ${}#{}{}\n",
                pkt.get_payload(),
                chars[0] as char,
                chars[1] as char
            );

            pkt.parse_sequence();

            if !self.ack_enabled_ {
                return true;
            }

            if trailing_checksum == running_checksum {
                let mut out = [0u8; 3];
                out[0] = '+' as u8;

                if let Some(seq) = pkt.get_sequence() {
                    u8_to_hex(seq as u8, &mut out[1..3]);
                    return self.io_.write(&out[..]);
                } else {
                    return self.io_.write(&out[..1]);
                }
            } else {
                trace_gdb_remote!("RX Bad XSUM, retry\n");
                self.io_.write(b"-");
            }
        }
    }
}

fn hex_to_u8(chars: &[u8; 2], result: &mut u8) -> bool {
    let mut value: u8 = 0;

    for &c in chars.iter() {
        let nibble = match c as char {
            '0'..='9' => c - '0' as u8,
            'a'..='f' => c - 'a' as u8 + 10,
            'A'..='F' => c - 'A' as u8 + 10,
            _ => return false,
        };

        value = value.wrapping_shl(4).wrapping_add(nibble);
    }

    *result = value;
    true
}

fn u8_to_hex(value: u8, result: &mut [u8]) {
    let high_nibble = (value >> 4) & 0x0F;
    let low_nibble = value & 0x0F;

    result[0] = match high_nibble {
        0..=9 => b'0' + high_nibble,
        _ => b'a' + (high_nibble - 10),
    };

    result[1] = match low_nibble {
        0..=9 => b'0' + low_nibble,
        _ => b'a' + (low_nibble - 10),
    };
}