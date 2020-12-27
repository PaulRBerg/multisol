use std::env;
use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::{Context, Result};

#[derive(Debug)]
pub struct Contract {
    /// Absolute path to the directory in which the contract is located on disk
    directory: PathBuf,
    /// Whether the contract is imported from "node_modules" or not
    external: bool,
    /// Name of the contract file
    file_name: OsString,
    /// Absolute path to the contract on disk
    full_path: PathBuf,
    /// Source code of the Solidity contract
    source_code: String,
}

impl Contract {
    pub fn directory(&self) -> &PathBuf {
        &self.directory
    }

    pub fn external(&self) -> bool {
        self.external
    }

    pub fn file_name(&self) -> &OsString {
        &self.file_name
    }

    pub fn full_path(&self) -> &PathBuf {
        &self.full_path
    }

    pub fn source_code(&self) -> &str {
        &self.source_code
    }
}

impl Contract {
    pub fn set_source_code(&mut self, source_code: String) {
        self.source_code = source_code;
    }
}

impl Contract {
    pub fn new(
        directory: PathBuf,
        external: bool,
        file_name: OsString,
        full_path: PathBuf,
        source_code: String,
    ) -> Contract {
        Contract {
            directory,
            external,
            file_name,
            full_path,
            source_code,
        }
    }

    /// Generates the struct instance starting from the user provided path to the contract.
    pub fn from_cli(contract_path: &PathBuf) -> Result<Contract> {
        let full_path = contract_path
            .canonicalize()
            .with_context(|| "Couldn't get an absolute path to the contract path")?;

        // Get the curent working directory by "popping" the file name out of the path.
        let mut directory = PathBuf::from(&full_path);
        directory.pop();

        // Gets the file name out of the path provided by the user. It is safe to unwrap because the
        // contract path is sanity checked on program entry.
        let file_name = contract_path.file_name().unwrap().to_os_string();

        let contract = Contract {
            directory,
            file_name,
            external: false,
            full_path,
            source_code: String::from(""),
        };
        Ok(contract)
    }

    /// Generates the struct instance starting from import path captured in another contract.
    pub fn from_import_path(ancestor_contract: &Contract, import_path: &PathBuf) -> Result<Contract> {
        let external: bool;
        let full_path: PathBuf;

        // Safe to unwrap because the regex can't match a path ending in "..".
        let file_name = import_path.file_name().unwrap().to_os_string();

        // If the path starts with "." or "..", the import is local. Otherwise, the paths are constructed as if
        // the contract was defined in "node_modules" in the current working directory.
        if import_path.starts_with(".") || import_path.starts_with("..") {
            // The contract could be external even if the import isn't relative. That's because one of the ancestors
            // may be external.
            external = ancestor_contract.external;

            // Join the ancestor contract's directory with the current import path to the full path.
            full_path = ancestor_contract
                .directory()
                .join(import_path)
                .canonicalize()
                .with_context(|| format!("Couldn't get an absolute path to the contract: {:?}", file_name))?;
        } else {
            // The contract must be external if the import isn't relative. Lest the Solidity program wouldn't compile.
            external = true;

            // Construct the path assuming that "node_modules" is in the current working directory.
            let cwd = env::current_dir().unwrap();
            let cwd_str = cwd.as_os_str().to_str().unwrap();
            let import_path_str = import_path.to_str().unwrap();
            full_path = [cwd_str, "node_modules", import_path_str].iter().collect();
        }

        // Get the curent working directory by popping the file name out of the path.
        let mut directory: PathBuf;
        directory = PathBuf::from(&full_path);
        directory.pop();

        let contract = Contract {
            directory,
            external,
            file_name,
            full_path,
            source_code: String::from(""),
        };
        Ok(contract)
    }
}
