use anyhow::{Ok, Result};

use processing_chain::{run_process, items::Item, processes::json_process::JsonProcess};


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

    let proc = JsonProcess {
        name: String::from("JSON process"),
        json_items: String::from("examples/items.json"),
        ..JsonProcess::default()
    };
    let _proc = run_process(proc, _process_item)?;

    Ok(())

}

