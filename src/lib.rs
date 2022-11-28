#![crate_name = "processing_chain"]

//! The `processing-chain` crate provides a convenient way to seamlessly set up processing
//! chains for large amounts of data.
//!

use anyhow::{Ok, Result};
use log::{info, error};
use process_trait::ProcessingCore;
use items::Item;

pub mod process_trait;
pub mod items;
pub mod processes;

pub fn run_process<P, F>(mut proc: P, f: F) -> Result<Vec<Item>>
where
    P: ProcessingCore,
    F: Fn(&mut Item) -> Result<bool> + Send + Sync,
{
    proc.set_items()?;

    if !proc.check_all_inputs_exist()? {
        error!("Not all input files exist!");
    }

    if proc.check_tmp_dir_exist()? {
        proc.create_tmp_directory()?;
    }

    let items = proc.process_items(f)?;
    info!("All Items processed succesfully!");

    Ok(items.to_vec())
}

#[cfg(test)]
mod tests {
    use super::*;
    use log::info;
    use processes::json_process::JsonProcess;

    fn _process_item(item: &mut Item) -> Result<bool> {
        // define how to process a single item
        info!(
            "Processing {} {:?} -> {:?}",
            item.name, item.input_item_paths, item.output_item_paths
        );
        Ok(true)
    }

    #[test]
    fn run_process_json() {

        let proc = JsonProcess {
            name: String::from("JSON process"),
            json_items: String::from("examples/items.json"),
            ..JsonProcess::default()
        };
        let items = run_process(proc, _process_item).unwrap();
        assert_eq!(items.len(), 3);

    }

}
