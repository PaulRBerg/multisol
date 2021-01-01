//! This is a crate that recursively collects all the contracts that the user provided
//! contracts depends on.
//!
//! It is assumed that the user runs Multisol from the root of their Solidity project,
//! where there is a "node_modules" folder.

use std::collections::HashSet;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;

use multisol_structs::Contract;

// The lazy init ensures that the regular expression is compiled exactly once.
lazy_static! {
    static ref IMPORT_REGEX: Regex = Regex::new("import (?:\\{[^{}]*\\} from )?['\"](.*?.sol)['\"];").unwrap();
}

/// Entry point to the crate.
pub fn run(contract_path: PathBuf) -> Result<Vec<Contract>> {
    let contract = Contract::from_cli(&contract_path)?;
    let mut contracts: Vec<Contract> = vec![];
    let mut visited: HashSet<PathBuf> = HashSet::new();
    collect_contracts(contract, &mut contracts, &mut visited)?;
    Ok(contracts)
}

/// Starts from the contract provided by the user and finds all the imported contracts recursively.
/// The search is depth-first.
fn collect_contracts(
    mut contract: Contract,
    contracts: &mut Vec<Contract>,
    visited: &mut HashSet<PathBuf>,
) -> Result<()> {
    // It is possible for multiple contracts to import the same contract.
    if visited.contains(contract.full_path()) {
        return Ok(());
    }
    visited.insert(PathBuf::from(contract.full_path()));

    let source_code = read_source_code(contract.full_path())?;
    let mut multisol_source_code = source_code.clone();

    // Iterate over all imports in the current contract. There is only one item in each capture group,
    // the actual path to the contract.
    for capture_group in IMPORT_REGEX.captures_iter(&source_code) {
        // Skip the first element in the capture group array because that's the full regex match.
        let import_str = &capture_group[1];
        let import_path = PathBuf::from(String::from(import_str));
        let imported_contract = Contract::from_import_path(&contract, &import_path)?;

        // If the import path is not of the form "./Foo.sol", modify the source code so that all imports
        // use paths relative to the same directory.
        if contract.directory() != imported_contract.directory() {
            // Safe to unwrap because all Solidity files are UTF-8 compliant.
            let file_name = imported_contract.file_name().to_str().unwrap();
            let file_name_with_dot = ["./", file_name].join("");
            multisol_source_code = multisol_source_code.replace(import_str, &file_name_with_dot);
        }

        collect_contracts(imported_contract, contracts, visited)?;
    }

    contract.set_source_code(multisol_source_code);
    contracts.push(contract);

    Ok(())
}

/// Attempts to read the source from either the "node_modules" directory or from the relative location.
fn read_source_code(full_path: &PathBuf) -> Result<String> {
    let mut contract_file = File::open(full_path).with_context(|| format!("Could not open file: {:?}", full_path))?;
    let mut source_code = String::new();
    contract_file
        .read_to_string(&mut source_code)
        .with_context(|| format!("Contract contains invalid UTF-8: {:?}", full_path))?;
    Ok(source_code)
}
