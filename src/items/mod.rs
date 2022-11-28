use std::path::PathBuf;

use serde::{Deserialize};

/// An item.
///
#[derive(Deserialize, Debug, Clone)]
pub struct Item {
    pub name: String,
    pub input_item_paths: Vec<PathBuf>,
    pub output_item_paths: Vec<PathBuf>,
    pub tmp_item_paths: Option<Vec<PathBuf>>,
    pub results: Vec<f64>,
}