# processing-chain

`processing-chain` provides a convenient way to seamlessly set up processing
chains for large amounts of data. `processing-chain` introduces the concept of Item. An Item is an abstraction that is used to spawn all the processes in parallel. At the moment, an Item is defined by a single input path and a single output path. We are currently working on extending this concept to allow parallelization by ouput files [`link`](https://github.com/giorgiosavastano/process/issues/1).

All the user needs to do is to provide the input/output paths, and the function that processes a single file.
`processing-chain` will take care of spawning the process across all Items via parallelization.
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
The full example can be found [`here`](https://github.com/giorgiosavastano/process/blob/main/examples/processing.rs).
