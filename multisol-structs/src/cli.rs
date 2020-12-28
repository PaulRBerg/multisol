use std::path::PathBuf;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(
    name = "multisol",
    about = "Command-line application for verifying Solidity contracts on Etherscan."
)]
pub struct Cli {
    /// The path to the Solidity contract to look for
    #[structopt(parse(from_os_str))]
    pub contract_path: PathBuf,
    /// Disable the Solidity optimizer, which is enabled by default
    #[structopt(long, parse(try_from_str))]
    pub no_optimization: Option<bool>,
}
