use anyhow::{Result};

use crate::items::Item;

/// Processing trait.
///
pub trait ProcessingCore {
    fn set_items(&mut self) -> Result<()>;
    fn check_all_inputs_exist(&self) -> Result<bool>;
    fn check_tmp_dir_exist(&self) -> Result<bool>;
    fn create_tmp_directory(&self) -> Result<()>;
    fn process_items<F>(&mut self, f: F) -> Result<&Vec<Item>>
    where
        F: Fn(&mut Item) -> Result<bool> + Send + Sync;
    fn move_files(&self) -> Result<bool>;
}