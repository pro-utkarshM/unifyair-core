use {
    clap::{crate_authors, crate_description, crate_name, crate_version},
    lightning_cli::{app::get_clap_app, nf_type::NFConfig, nf_type::NFType},
    tokio::runtime::Builder,
};

fn handle_network_function(nf: NFType, matches: &clap::ArgMatches) -> NFConfig {
    let config_path = matches.get_one::<String>("config").unwrap();
    nf.get_config(config_path)
}

async fn run(nf_config: NFConfig) {
    match nf_config {
        NFConfig::DataWarpConfig(ref datawarp_config) => {
            datawarp::run(datawarp_config).await;
        }
        NFConfig::InfiniSyncConfig(ref infinisync_config) => {
            infinisync::run(infinisync_config).await;
        }
    }
}

fn main() {
    let command = get_clap_app(
        crate_name!(),
        crate_description!(),
        crate_authors!(),
        crate_version!(),
    );
    let matches = command.get_matches();
    let (nf_type, matches) = matches.subcommand().unwrap();
    let nf_config = handle_network_function(NFType::from_str(nf_type), matches);

    Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            run(nf_config).await;
        });
}
