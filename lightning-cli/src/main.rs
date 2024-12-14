#![feature(error_generic_member_access)]

use clap::{crate_authors, crate_description, crate_name, crate_version};
use lightning_cli::{app::get_clap_app, nf_type::App};

fn main() -> color_eyre::Result<()> {
	let command = get_clap_app(
		crate_name!(),
		crate_description!(),
		crate_authors!(),
		crate_version!(),
	);
	let matches = command.get_matches();
	let (nf_type, matches) = matches.subcommand().expect("Subcommand Not present");
	let config_path = matches
		.get_one::<String>("config")
		.expect("Config not present");
	App::start_app(nf_type, config_path)?;
	Ok(())
}
