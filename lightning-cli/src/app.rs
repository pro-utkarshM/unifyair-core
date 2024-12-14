use clap::{Arg, Command};

use crate::nf_type::{DATAWARP_STR, INFINISYNC_STR, OMNIPATH_STR};

fn get_nf_subcommand(nf_name: String) -> Command {
	let nf_about = format!("Runs {nf_name} network function");
	Command::new(nf_name).about(nf_about).arg(
		Arg::new("config")
			.help("Configuration file to use")
			.required(true)
			.value_name("CONFIG_FILE")
			.long("config")
			.short('c'),
	)
}

pub fn get_clap_app(
	name: &'static str,
	about: &'static str,
	author: &'static str,
	version: &'static str,
) -> Command {
	Command::new(name)
		.version(version)
		.author(author)
		.about(about)
		.subcommand_required(true)
		.arg_required_else_help(true)
		.allow_external_subcommands(true)
		.subcommand(get_nf_subcommand(DATAWARP_STR.to_string()))
		.subcommand(get_nf_subcommand(INFINISYNC_STR.to_string()))
		.subcommand(get_nf_subcommand(OMNIPATH_STR.to_string()))
}
