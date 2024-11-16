use serde::de::DeserializeOwned; // Trait bound for deserialization
use serde_yaml;

pub struct YamlParser;

// Provide a concrete implementation of from_yaml for any type that implements
// DeserializeOwned
impl YamlParser {
	pub fn from_yaml<T: DeserializeOwned>(yaml_str: &str) -> T {
		serde_yaml::from_str(yaml_str).expect("Failed to parse config file")
	}
}
