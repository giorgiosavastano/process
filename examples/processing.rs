use processing_chain::{Process, ProcessingCore, Item};
use std::path::PathBuf;
use anyhow::{Ok, Result};


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

    let mut proc = Process {
        name: String::from("Test"),
        inputs_dir_path: PathBuf::from("Test"),
        inputs_extenion: String::from("Test"),
        outputs_dir_path: PathBuf::from("Test"),
        tmp_dir_path: PathBuf::from("Test"),
        overwrite: false,
        items: Vec::new(),
    };

    proc.set_items()?;

    if proc.check_all_inputs_exist()? {
        println!("All good!");
    }

    if proc.tmp_dir_path.to_str() != Some("default") {
        proc.create_tmp_directory()?;
    }

    if proc.process_items(_process_item)? {
        println!(" Daje!")
    }
    Ok(())
}