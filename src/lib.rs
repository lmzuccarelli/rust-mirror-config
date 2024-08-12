use serde_derive::{Deserialize, Serialize};
use std::fs;

mod error;

use crate::error::handler::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ImageSetConfig {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "apiVersion")]
    pub api_version: String,

    #[serde(rename = "mirror")]
    pub mirror: Mirror,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mirror {
    #[serde(rename = "platform")]
    pub platform: Option<Platform>,

    #[serde(rename = "release")]
    pub release: Option<String>,

    #[serde(rename = "operators")]
    pub operators: Option<Vec<Operator>>,

    #[serde(rename = "additionalImages")]
    pub additional_images: Option<Vec<Image>>,

    #[serde(rename = "helm")]
    pub helm: Option<Helm>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Image {
    #[serde(rename = "name")]
    pub name: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Helm {}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Operator {
    #[serde(rename = "catalog")]
    pub catalog: String,

    #[serde(rename = "packages")]
    pub packages: Option<Vec<Package>>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Package {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "channels")]
    pub channels: Option<Vec<IncludeChannel>>,

    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,

    #[serde(rename = "maxVersion")]
    pub max_version: Option<String>,

    #[serde(rename = "minBundle")]
    pub min_bundle: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct IncludeChannel {
    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "minVersion")]
    pub min_version: Option<String>,

    #[serde(rename = "maxVersion")]
    pub max_version: Option<String>,

    #[serde(rename = "minBundle")]
    pub min_bundle: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Platform {
    #[serde(rename = "channels")]
    channels: Vec<Channel>,

    #[serde(rename = "graph")]
    graph: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Channel {
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "type")]
    channel_type: String,
}

// read the 'image set config' file
impl ImageSetConfig {
    pub fn load_config(config_file: String) -> Result<String, Box<dyn std::error::Error>> {
        // Create a path to the desired file
        let data = fs::read_to_string(config_file.clone());
        if data.is_ok() {
            Ok(data.unwrap())
        } else {
            let err = Box::new(MirrorError::new(&format!(
                "reading config {:?}",
                data.err().unwrap().to_string().to_lowercase()
            )));
            Err(err)
        }
    }

    // parse the 'image set config' file
    pub fn parse_yaml_config(data: String) -> Result<ImageSetConfig, serde_yaml::Error> {
        // Parse the string of data into serde_json::ImageSetConfig.
        let res = serde_yaml::from_str::<ImageSetConfig>(&data);
        res
    }
}

#[cfg(test)]
mod tests {
    // this brings everything from parent's scope into this scope
    use super::*;

    #[test]
    fn test_load_config_pass() {
        let res = ImageSetConfig::load_config(String::from("./imagesetconfig.yaml"));
        assert!(res.is_ok());
    }

    #[test]
    #[should_panic]
    fn test_load_config_fail() {
        let res = ImageSetConfig::load_config(String::from("./nada.yaml"));
        assert!(res.is_err());
    }

    // finally test that the parser is working correctly
    #[test]
    fn test_isc_parser() {
        let data = ImageSetConfig::load_config(String::from("./imagesetconfig.yaml"));
        let res = ImageSetConfig::parse_yaml_config(data.unwrap().to_string());
        assert!(res.is_ok());
    }
}
