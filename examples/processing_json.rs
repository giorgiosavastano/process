use std::fs::File;
use std::io::BufReader;
use std::error::Error;

use processing_chain::{items::Item};
// use serde_json::from_reader;


fn main() -> Result<(), Box<dyn Error>> {

    // Open the file in read-only mode with buffer.
    let file = File::open("examples/items.json")?;
    let reader = BufReader::new(file);

    let items: Vec<Item> = serde_json::from_reader(reader)
    .expect("error while reading or parsing");

    for item in items {
        println!(
            "Processing {} {:?} -> {:?}",
            item.name, item.input_item_paths, item.output_item_paths
        );
    }
    Ok(())

}

