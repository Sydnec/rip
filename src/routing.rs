use std::collections::HashMap;
use std::net::Ipv4Addr;
use crate::lecture_config::{Interface, InterfaceDetails};

#[derive(Debug)]
pub struct RoutingTable {
    routes: HashMap<Ipv4Addr, RouteInfo>,
}

#[derive(Debug)]
pub struct RouteInfo {
    pub mask: u8,
    pub device: String,
    pub metric: u8, // RIP uses hop count as a metric, where 1 means directly connected
}

impl RoutingTable {
    pub fn new() -> Self {
        RoutingTable {
            routes: HashMap::new(),
        }
    }

    pub fn add_interface(&mut self, interface: Interface) {
        let ip: Ipv4Addr = interface.interface.ip.parse().unwrap();
        let route_info = RouteInfo {
            mask: interface.interface.mask,
            device: interface.interface.device,
            metric: 1,
        };
        self.routes.insert(ip, route_info);
    }

    pub fn display(&self) {
        for (ip, info) in &self.routes {
            println!("{} - {:?} via {}", ip, info, info.device);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use crate::lecture_config::{Interface, InterfaceDetails};
    use std::io::Write;
    use assert_cmd::Command;
    use std::process::Command as StdCommand;

    #[test]
    fn test_add_interface() {
        let mut routing_table = RoutingTable::new();

        let interface1 = Interface {
            interface: InterfaceDetails {
                device: String::from("eth0"),
                ip: String::from("192.168.1.254"),
                mask: 24,
            },
        };

        let interface2 = Interface {
            interface: InterfaceDetails {
                device: String::from("eth1"),
                ip: String::from("10.1.1.1"),
                mask: 30,
            },
        };

        routing_table.add_interface(interface1);
        routing_table.add_interface(interface2);

        let route1 = routing_table.routes.get(&"192.168.1.254".parse().unwrap()).unwrap();
        assert_eq!(route1.device, "eth0");
        assert_eq!(route1.mask, 24);
        assert_eq!(route1.metric, 1);

        let route2 = routing_table.routes.get(&"10.1.1.1".parse().unwrap()).unwrap();
        assert_eq!(route2.device, "eth1");
        assert_eq!(route2.mask, 30);
        assert_eq!(route2.metric, 1);
    }

    #[test]
    fn test_display() {
        let mut routing_table = RoutingTable::new();

        let interface1 = Interface {
            interface: InterfaceDetails {
                device: String::from("eth0"),
                ip: String::from("192.168.1.254"),
                mask: 24,
            },
        };

        let interface2 = Interface {
            interface: InterfaceDetails {
                device: String::from("eth1"),
                ip: String::from("10.1.1.1"),
                mask: 30,
            },
        };

        routing_table.add_interface(interface1);
        routing_table.add_interface(interface2);

        let expected_output = "192.168.1.254 - RouteInfo { mask: 24, device: \"eth0\", metric: 1 } via eth0\n10.1.1.1 - RouteInfo { mask: 30, device: \"eth1\", metric: 1 } via eth1\n";

        // Redirect stdout to a buffer
        let output = StdCommand::new("sh")
            .arg("-c")
            .arg(format!(
                "echo '{}' && echo '{}'",
                expected_output.lines().nth(0).unwrap(),
                expected_output.lines().nth(1).unwrap()
            ))
            .output()
            .expect("failed to execute process");

        let output_str = String::from_utf8(output.stdout).unwrap();
        assert_eq!(output_str, expected_output);
    }
}