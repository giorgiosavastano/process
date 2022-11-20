use std::path::PathBuf;

/// An item.
///
#[derive(Debug)]
pub struct Item {
    pub name: String,
    pub input_item_paths: Vec<PathBuf>,
    pub output_item_paths: Vec<PathBuf>,
    pub tmp_item_paths: Option<Vec<PathBuf>>,
}