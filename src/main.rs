mod lecture_config;
mod routing;

use lecture_config::read_config_files;
use routing::RoutingTable;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config_dir = "./config";
    let pattern = format!("{}/*.yaml", config_dir);
    let interfaces = read_config_files(&pattern)?;

    let mut routing_table = RoutingTable::new();
    for interface in interfaces {
        routing_table.add_interface(interface);
    }

    routing_table.display();

    Ok(())
}
