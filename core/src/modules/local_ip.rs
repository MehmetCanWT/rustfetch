use crate::{Info, Module};
use std::net::{Ipv4Addr, Ipv6Addr, UdpSocket};

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

    // 1b. IPv6 routing check for IPv6-only environments
    if let Ok(socket) = UdpSocket::bind("[::]:0") {
        for target in ["[2001:4860:4860::8888]:53", "[2606:4700:4700::1111]:53"] {
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
        let mut fallback_v4 = None;
        let mut fallback_v6 = None;

        while !cur.is_null() {
            let entry = &*cur;
            if !entry.ifa_addr.is_null() {
                let flags = entry.ifa_flags;
                let is_up = (flags & libc::IFF_UP as u32) != 0;
                let is_loopback = (flags & libc::IFF_LOOPBACK as u32) != 0;

                if is_up && !is_loopback {
                    let family = (*entry.ifa_addr).sa_family as i32;
                    if family == libc::AF_INET && fallback_v4.is_none() {
                        let sockaddr_in = entry.ifa_addr as *const libc::sockaddr_in;
                        let ip_bytes = (*sockaddr_in).sin_addr.s_addr.to_ne_bytes();
                        let ip = Ipv4Addr::new(ip_bytes[0], ip_bytes[1], ip_bytes[2], ip_bytes[3]);
                        if !ip.is_loopback() && !ip.is_unspecified() && !ip.is_link_local() {
                            fallback_v4 = Some(ip.to_string());
                        }
                    } else if family == libc::AF_INET6 && fallback_v6.is_none() {
                        let sockaddr_in6 = entry.ifa_addr as *const libc::sockaddr_in6;
                        let ip = Ipv6Addr::from((*sockaddr_in6).sin6_addr.s6_addr);
                        if !ip.is_loopback()
                            && !ip.is_unspecified()
                            && (ip.segments()[0] & 0xffc0) != 0xfe80
                        {
                            fallback_v6 = Some(ip.to_string());
                        }
                    }
                }
            }
            cur = entry.ifa_next;
        }

        libc::freeifaddrs(ifaddrs);
        fallback_v4.or(fallback_v6)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_local_ip_module_name() {
        assert_eq!(LocalIpModule.name(), "local_ip");
    }

    #[test]
    fn test_detect_local_ip_returns_or_safely_skips() {
        if let Some(ip) = detect_local_ip() {
            assert!(!ip.is_empty());
            assert!(!ip.starts_with("127."));
            assert_ne!(ip, "::1");
        }
    }
}
