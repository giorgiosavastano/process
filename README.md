# processing-chain

`processing-chain` provides a convenient way to seamlessly set up processing
chains for large amounts of data.

Please read the [`API documentation on docs.rs`](https://docs.rs/processing_chain/latest/processing_chain/)
or take a look at the [`examples`](https://github.com/giorgiosavastano/process/tree/main/examples).

`processing-chain` is based on the concept of Item which is an abstraction that is used to spawn all the processes in parallel. All the user needs to do is define:

- The Items to be processed
- The function that processes a single Item

`processing-chain` will take care of spawning the process across all Items via parallelization.
The user can also provide some extra processing configuration information (e.g., overwrite).

## Highlights

- Set-up generic data processing chains

## Define the `Items`

Using a JSON file
```json
[
    {
        "name": "item_1",
        "input_item_paths": ["test_1.npy", "test_2.npy", "test_2.npy"],
        "output_item_paths": ["output_1.nc"]
    },
    {
        "name": "item_2",
        "input_item_paths": ["test_1.npy", "test_2.npy"],
        "output_item_paths": ["output_2.nc"]
    },
    {
        "name": "item_3",
        "input_item_paths": ["test_6.npy", "test_7.npy", "test_8.npy"],
        "output_item_paths": ["output_3.nc"]
    }
]
```

## Write the `_process_item` function

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
Some examples can be found [`here`](https://github.com/giorgiosavastano/process/blob/main/examples).
