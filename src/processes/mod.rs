use std::path::PathBuf;

use crate::items::Item;

pub mod simple_process;

/// A process.
///
#[derive(Debug, Default)]
pub struct Process {
    pub name: String,
    pub inputs_dir_path: PathBuf,
    pub inputs_extenion: String,
    pub outputs_dir_path: PathBuf,
    pub tmp_dir_path: Option<PathBuf>,
    pub overwrite: bool,
    pub items: Vec<Item>,
}