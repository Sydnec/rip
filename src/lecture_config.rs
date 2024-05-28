use std::error::Error;
use std::fs;
use serde::Deserialize;
use serde_yaml;
use glob::glob;

#[derive(Debug, Deserialize)]
pub struct Interface {
    pub interface: InterfaceDetails,
}

#[derive(Debug, Deserialize)]
pub struct InterfaceDetails {
    pub device: String,
    pub ip: String,
    pub mask: u8,
}

pub fn read_config_files(pattern: &str) -> Result<Vec<Interface>, Box<dyn Error>> {
    let mut interfaces = Vec::new();

    for entry in glob(pattern)? {
        match entry {
            Ok(path) => {
                let config_content = fs::read_to_string(&path)?;
                
                if let Ok(mut config) = serde_yaml::from_str::<Vec<Interface>>(&config_content) {
                    interfaces.append(&mut config);
                } else if let Ok(interface) = serde_yaml::from_str::<Interface>(&config_content) {
                    interfaces.push(interface);
                } else {
                    eprintln!("Erreur de désérialisation du fichier : {}. Contenu:\n{}", path.display(), config_content);
                }
            }
            Err(e) => eprintln!("Erreur lors de la lecture du fichier : {}", e),
        }
    }

    Ok(interfaces)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_config_files() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("config.test.yaml");

        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "- interface:
    device: eth0
    ip: 192.168.1.254
    mask: 24
- interface:
    device: eth1
    ip: 10.1.1.1
    mask: 30"
        )
        .unwrap();

        let pattern = format!("{}/*.yaml", dir.path().to_str().unwrap());
        let interfaces = read_config_files(&pattern).unwrap();

        assert_eq!(interfaces.len(), 2);
        assert_eq!(interfaces[0].interface.device, "eth0");
        assert_eq!(interfaces[0].interface.ip, "192.168.1.254");
        assert_eq!(interfaces[0].interface.mask, 24);
        assert_eq!(interfaces[1].interface.device, "eth1");
        assert_eq!(interfaces[1].interface.ip, "10.1.1.1");
        assert_eq!(interfaces[1].interface.mask, 30);
    }

    #[test]
    fn test_read_single_config_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("single_config.test.yaml");

        let mut file = File::create(&file_path).unwrap();
        writeln!(
            file,
            "interface:
    device: eth0
    ip: 192.168.1.254
    mask: 24"
        )
        .unwrap();

        let pattern = format!("{}/*.yaml", dir.path().to_str().unwrap());
        let interfaces = read_config_files(&pattern).unwrap();

        assert_eq!(interfaces.len(), 1);
        assert_eq!(interfaces[0].interface.device, "eth0");
        assert_eq!(interfaces[0].interface.ip, "192.168.1.254");
        assert_eq!(interfaces[0].interface.mask, 24);
    }
}
