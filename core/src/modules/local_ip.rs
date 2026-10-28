use crate::{Info, Module};
use std::net::{Ipv4Addr, UdpSocket};

pub struct LocalIpModule;

impl Module for LocalIpModule {
    fn name(&self) -> &'static str {
        "local_ip"
    }

    fn detect(&self) -> Option<Info> {
        let ip = detect_local_ip()?;
        Some(Info {
            label: "Local IP".to_string(),
            value: ip,
        })
    }
}

pub fn detect_local_ip() -> Option<String> {
    // 1. Fast routing table check via UDP socket (0 network packets sent; pure kernel FIB lookup)
    if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
        for target in [
            "8.8.8.8:53",
            "1.1.1.1:53",
            "10.255.255.255:53",
            "192.168.255.255:53",
        ] {
            if socket.connect(target).is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    let ip = addr.ip();
                    if !ip.is_loopback() && !ip.is_unspecified() {
                        return Some(ip.to_string());
                    }
                }
            }
        }
    }

    // 2. Offline fallback: Inspect local active non-loopback network interfaces
    get_interface_ip_fallback()
}

fn get_interface_ip_fallback() -> Option<String> {
    unsafe {
        let mut ifaddrs: *mut libc::ifaddrs = std::ptr::null_mut();
        if libc::getifaddrs(&mut ifaddrs) != 0 || ifaddrs.is_null() {
            return None;
        }

        let mut cur = ifaddrs;
        let mut fallback_ip = None;

        while !cur.is_null() {
            let entry = &*cur;
            if !entry.ifa_addr.is_null() && (*entry.ifa_addr).sa_family as i32 == libc::AF_INET {
                let flags = entry.ifa_flags;
                let is_up = (flags & libc::IFF_UP as u32) != 0;
                let is_loopback = (flags & libc::IFF_LOOPBACK as u32) != 0;

                if is_up && !is_loopback {
                    let sockaddr_in = entry.ifa_addr as *const libc::sockaddr_in;
                    let ip_bytes = (*sockaddr_in).sin_addr.s_addr.to_ne_bytes();
                    let ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
                    if !ip.is_loopback() && !ip.is_unspecified() && !ip.is_link_local() {
                        fallback_ip = Some(ip.to_string());
                        break;
                    }
                }
            }
            cur = entry.ifa_next;
        }

        libc::freeifaddrs(ifaddrs);
        fallback_ip
    }
}
