use anyhow::{Ok, Result};
use processing_chain::{run_process, Item, Process};
use std::env;
use std::path::PathBuf;

fn _process_item(item: &Item) -> Result<bool> {
    // define how to process a single item
    println!(
        "Processing {} {:?} -> {:?}",
        item.name, item.input_item_path, item.output_item_path
    );
    // ...

    Ok(true)
}

fn main() -> Result<()> {
    let proc = Process {
        name: String::from("Test"),
        inputs_dir_path: env::current_dir()?,
        inputs_extenion: String::from("toml"),
        outputs_dir_path: PathBuf::from("Test"),
        ..Process::default()
    };
    let _proc = run_process(proc, _process_item)?;
    Ok(())
}
