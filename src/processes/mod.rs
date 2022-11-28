use std::path::PathBuf;

use crate::items::Item;

pub mod json_process;


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