//! Command-line application for verifying Solidity contracts on Etherscan.

use std::path::PathBuf;
use std::process;

use anyhow::{bail, Result};
use multisol_structs::Cli;
use structopt::StructOpt;

/// Entry point to the program.
pub fn main() {
    let args = Cli::from_args();

    if let Err(error) = sanity_check_path(&args.contract_path) {
        eprintln!("{}", error);
        process::exit(1);
    }

    let contracts = match multisol_collector::run(args.contract_path) {
        Ok(contracts) => contracts,
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
    };

    if let Err(error) = multisol_writer::run(contracts) {
        eprintln!("{}", error);
        process::exit(1);
    }
}

/// Performs some sanity checks on the program input.
///
/// # Checks
///
///   1. The contract path exists.
///   2. The contract path is not a directory.
///   3. The contract path has the "sol" extension.
fn sanity_check_path(contract_path: &PathBuf) -> Result<()> {
    if contract_path.exists() == false {
        bail!("Contract does not exist");
    }

    if contract_path.is_dir() {
        bail!("Provided path is a directory, not a contract");
    }

    if contract_path.is_file() == false {
        bail!("Provided path is either a broken symlink or you do not have permission to access it");
    }

    if let Some(extension) = contract_path.extension() {
        if extension != "sol" {
            bail!("Contract file does not have the \"sol\" extension");
        }
    } else {
        bail!("Contract file does not have the \"sol\" extension");
    }

    Ok(())
}
