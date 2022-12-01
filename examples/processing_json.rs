use anyhow::{Ok, Result};
use log::info;

use processing_chain::{run_process, items::Item, processes::json_process::JsonProcess};


fn _process_item(item: &mut Item) -> Result<bool> {

    // define how to process a single item
    info!(
        "Processing {} {:?} -> {:?}",
        item.name, item.input_item_paths, item.output_item_paths
    );

    let mut results: f64 = 0.0;
    let mut count: f64 = 0.0;

    for in_path in &item.input_item_paths {

        let mut rdr = csv::Reader::from_path(in_path)?;
    
        for result in rdr.records() {
            let record = result?;
            let dol: f64 = record[3].parse()?;
            results += dol;
            count += 1.0;
        }
    }

    println!("{}: Mean dollar spent {:?}", item.name, results / count);

    item.results.push(results / count);

    Ok(true)
}


fn main() -> Result<()> {

    let proc = JsonProcess {
        name: String::from("JSON process"),
        json_items: String::from("examples/items.json"),
        ..JsonProcess::default()
    };
    let items = run_process(proc, _process_item)?;
    println!("{:?}", items);
    println!("{:?}", items.iter().next().unwrap().results);

    Ok(())

}

