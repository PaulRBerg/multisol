//! This is a crate that writes the collected contracts to a directory named "multisol-contract_name".

use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};
use multisol_structs::Contract;

/// Entry point to the crate.
pub fn run(contracts: Vec<Contract>) -> Result<()> {
    // The last contract is the one provided by the user since that is the first contract that
    // gets inserted in the vector.
    let user_provided_contract = contracts.last().with_context(|| format!("No contracts found"))?;

    // Delete the output directory if it exists already.
    let output_dir = get_output_dir(&user_provided_contract)?;
    if output_dir.is_dir() {
        fs::remove_dir_all(&output_dir)?;
    }
    fs::create_dir(&output_dir)?;

    for contract in contracts.iter().rev() {
        let output_file_path = output_dir.join(contract.file_name());
        fs::write(output_file_path, contract.source_code())?;
    }

    Ok(())
}

/// Appends the contract name to "multisol-" to generate the name of the output directory.
fn get_output_dir(user_provided_contract: &Contract) -> Result<PathBuf> {
    let contract_file_stem = user_provided_contract.full_path().file_stem().with_context(|| {
        format!(
            "Could not get file stem out of contract path: {:?}",
            user_provided_contract.full_path()
        )
    })?;

    let contract_name = contract_file_stem
        .to_str()
        .with_context(|| format!("Contract name contains invalid UTF-8"))?;
    let contract_name_lowercase = contract_name.to_lowercase();
    let output_dir_str = ["multisol", &contract_name_lowercase].join("-");

    Ok(PathBuf::from(&output_dir_str))
}

#[cfg(test)]
mod tests {
    use super::{get_output_dir, Contract};
    use std::ffi::OsString;
    use std::path::PathBuf;

    #[test]
    fn gets_correct_output_dir() {
        let directory = PathBuf::from("/tmp/contracts");
        let full_path = PathBuf::from("/tmp/contracts/Foo.sol");
        let contract = Contract::new(directory, false, OsString::from("Foo.sol"), full_path, String::from(""));
        let output_dir = get_output_dir(&contract).unwrap();
        assert_eq!(output_dir, PathBuf::from("multisol-foo"));
    }

    /// Technically this edge case should never occur, because the contract path is sanity checked on program entry.
    /// But let's be comprehensive.
    #[test]
    fn cant_get_file_stem_out_of_contract_path() {
        let directory = PathBuf::from("/tmp/contracts");
        let full_path = PathBuf::from("/tmp/contracts/Foo.sol/..");
        let contract = Contract::new(directory, false, OsString::from("Foo.sol"), full_path, String::from(""));
        let err = get_output_dir(&contract).unwrap_err();
        assert_eq!(
            err.to_string(),
            format!(
                "Could not get file stem out of contract path: {:?}",
                contract.full_path()
            )
        );
    }
}
