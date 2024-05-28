mod lecture_config;

use lecture_config::read_config_files;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let config_dir = "./";
    let pattern = format!("{}/*.yaml", config_dir);
    let interfaces = read_config_files(&pattern)?;

    for interface in interfaces {
        println!("{:?}", interface);
    }

    Ok(())
}
