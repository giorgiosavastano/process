use anyhow::{Ok, Result};

use processing_chain::{run_process_json, items::Item};


fn _process_item(item: &Item) -> Result<bool> {
    // define how to process a single item
    println!(
        "Processing {} {:?} -> {:?}",
        item.name, item.input_item_paths, item.output_item_paths
    );
    // ...

    Ok(true)
}


fn main() -> Result<()> {

    let _proc = run_process_json(
        String::from("JSON process"),
        String::from("examples/items.json"),
        _process_item)?;

    Ok(())

}

