# processing-chain

`processing-chain` provides a convenient way to seamlessly set up processing
chains for large amounts of data.

All the user needs to do is to provide the input/output paths, and the function that processes a single file.
`processing-chain` will take care of spawning the process across all files via parallelization.
The user can also provide some extra processing configuration information (e.g., overwrite).

## Highlights

- Set-up generic data processing chains

## Write your `_process_item` function

In rust:
```rust
fn _process_item(item: &Item) -> Result<bool> {
    // define how to process a single item
    println!(
        "Processing {} {:?} -> {:?}",
        item.name, item.input_item_path, item.output_item_path
    );
    // ...

    Ok(true)
}
```
If your function is written in Python and you don't feel like converting it to Rust (yet), you could use the [inline-python](https://crates.io/crates/inline-python) crate.
```rust
use inline_python::python;

fn _process_item(item: &Item) -> Result<bool> {
    // define how to process a single item
    python! {
        print("Processing {} {} -> {}".format('item.name, item.input_item_path, item.output_item_path))
	};
    // ...

    Ok(true)
}
```
The full example can be found [`here`](https://github.com/nsat/processing-chain/blob/main/examples/processing.rs).