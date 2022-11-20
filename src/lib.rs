#![crate_name = "processing_chain"]

//! The `processing-chain` crate provides a convenient way to seamlessly set up processing
//! chains for large amounts of data.
//!

use anyhow::{Ok, Result};
use process_trait::ProcessingCore;
use items::Item;

pub mod process_trait;
pub mod items;
pub mod processes;

pub fn run_process<P, F>(mut proc: P, f: F) -> Result<P>
where
    P: ProcessingCore,
    F: Fn(&Item) -> Result<bool> + Send + Sync,
{
    proc.set_items()?;

    if proc.check_all_inputs_exist()? {
        println!("All good!");
    }

    if proc.check_tmp_dir_exist()? {
        proc.create_tmp_directory()?;
    }

    if proc.process_items(f)? {
        println!("All file processed!");
    }

    if proc.check_tmp_dir_exist()? {
        proc.move_files()?;
    }

    Ok(proc)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use log::info;
    use std::path::PathBuf;
    use processes::Process;

    fn _process_item(item: &Item) -> Result<bool> {
        // define how to process a single item
        info!(
            "Processing {} {:?} -> {:?}",
            item.name, item.input_item_paths, item.output_item_paths
        );
        Ok(true)
    }

    #[test]
    fn process_default_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            ..Process::default()
        };
        assert_eq!(proc.overwrite, false);
        assert_eq!(proc.tmp_dir_path, None);
        assert_eq!(proc.inputs_extenion, "toml");
        assert_eq!(proc.outputs_dir_path.to_str().unwrap(), "");
    }

    #[test]
    fn run_process_items_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            outputs_dir_path: PathBuf::from("Test"),
            ..Process::default()
        };

        let proc = run_process(proc, _process_item).unwrap();
        let first_item = proc.items.first().unwrap();
        assert_eq!(first_item.name, "file_0");
        assert_eq!(
            first_item.input_item_paths.first().unwrap().file_name().unwrap(),
            "Cargo.toml"
        );
        assert_eq!(first_item.input_item_paths.first().unwrap().extension().unwrap(), "toml");
        assert_eq!(first_item.output_item_paths.first().unwrap().extension().unwrap(), "toml");
    }

    #[test]
    fn run_process_empty_items_test() {
        let proc = Process {
            name: String::from("Test"),
            inputs_dir_path: env::current_dir().unwrap(),
            inputs_extenion: String::from("toml"),
            outputs_dir_path: env::current_dir().unwrap(),
            ..Process::default()
        };

        let proc = run_process(proc, _process_item).unwrap();
        assert!(proc.items.is_empty());
    }
}
