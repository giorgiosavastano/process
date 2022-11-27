use std::path::PathBuf;

use crate::items::Item;

pub mod simple_process;
pub mod json_process;

/// The Process structure that assume single input/output paths for each Item.
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


/// The Process structure that parse Items from a JSON file.
///
#[derive(Debug, Default)]
pub struct JsonProcess {
    pub name: String,
    pub json_items: String,
    pub tmp_dir_path: Option<PathBuf>,
    pub overwrite: bool,
    pub items: Vec<Item>,
}