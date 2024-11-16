use std::{fmt, fs::File, io::Read};

use parsing::YamlParser;
const DATAWARP_STR: &'static str = "datawarp";
const INFINISYNC_STR: &'static str = "infinisync";

pub enum NFType {
	DataWarp,
	InfiniSync,
}

#[derive(Debug)]
pub enum NFConfig {
	DataWarpConfig(datawarp::config::DataWarpConfig),
	InfiniSyncConfig(infinisync::config::InfiniSyncConfig),
}

impl fmt::Display for NFType {
	fn fmt(
		&self,
		f: &mut fmt::Formatter,
	) -> fmt::Result {
		match self {
			Self::DataWarp => write!(f, "{}", DATAWARP_STR),
			Self::InfiniSync => write!(f, "{}", INFINISYNC_STR),
		}
	}
}

impl NFType {
	pub fn from_str(s: &str) -> Self {
		match s {
			DATAWARP_STR => NFType::DataWarp,
			INFINISYNC_STR => NFType::InfiniSync,
			&_ => todo!(),
		}
	}

	pub fn to_str(&self) -> &'static str {
		// TODO: Analyze the clone here later on if it is necessary
		match self {
			Self::DataWarp => DATAWARP_STR.clone(),
			Self::InfiniSync => INFINISYNC_STR.clone(),
		}
	}

	pub fn get_config(
		&self,
		config_path: &str,
	) -> NFConfig {
		let mut file = File::open(config_path).expect("Failed to open config file");
		let mut contents = String::new();
		file.read_to_string(&mut contents)
			.expect("Failed to read config file");

		match self {
			Self::DataWarp => NFConfig::DataWarpConfig(YamlParser::from_yaml(&contents)),
			Self::InfiniSync => NFConfig::InfiniSyncConfig(YamlParser::from_yaml(&contents)),
		}
	}
}
