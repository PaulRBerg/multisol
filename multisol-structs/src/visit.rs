use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug)]
pub struct Visit {
    /// Name of the contract file
    file_name: OsString,
    /// Absolute path to the contract on disk
    full_path: PathBuf,
}

impl Visit {
    pub fn file_name(&self) -> &OsString {
        &self.file_name
    }

    pub fn full_path(&self) -> &PathBuf {
        &self.full_path
    }
}

impl Visit {
    pub fn new(file_name: OsString, full_path: PathBuf) -> Visit {
        Visit { file_name, full_path }
    }
}
