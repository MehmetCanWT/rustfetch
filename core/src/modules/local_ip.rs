use crate::{Info, Module};
use std::net::UdpSocket;

pub struct LocalIpModule;

impl Module for LocalIpModule {
    fn name(&self) -> &'static str {
        "local_ip"
    }
    fn detect(&self) -> Option<Info> {
        let socket = UdpSocket::bind("0.0.0.0:0").ok()?;
        // Connect to a public DNS server to force the OS to resolve the local routing IP
        socket.connect("8.8.8.8:53").ok()?;
        let ip = socket.local_addr().ok()?.ip();

        Some(Info {
            label: "Local IP".to_string(),
            value: ip.to_string(),
        })
    }
}
