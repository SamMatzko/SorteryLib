# SorteryLib
A fast, cross-platform file-sorting library based on the Sortery command-line file sorter. Along with fast sorting, SorteryLib comes with an easy-to-use `File` type, which is compatible with many other types, such as `String`, `str`, `std::path::Path`, `std::path::PathBuf`, and of course the `File` type itself.

# Basic Usage

For more in-depth usage information, see the [SorteryLib Docs](https://docs.rs/sorterylib/latest/sorterylib/).
Here is a basic usage example:

```rust
use sorterylib::prelude::*; // Import all the stuff needed for basic operation

fn main() {
    // The fields for initializing the Sorter struct
    let source = File::new("/path/to/source/dir/");
    let target = File::new("/path/to/target/dir/");
    let date_format = String::from("%Y");
    let date_type = String::from("m");
    let preserve_name = true;
    let exclude_type = vec![String::from("txt")];
    let only_type = Vec::new();

    // Create the Sorter instance
    let sorter = Sorter {
        source: source.copy(), // The directory from which to get all the files to sort
        target: target.copy(), // The directory to sort all the files into
        date_format: date_format, // The date format to rename the files using.
        date_type: date_type, // The date type to sort the files by
        preserve_name: preserve_name, // Whether to include the old file name in the new name
        exclude_type: exclude_type, // File type(s) to exclude
        only_type: only_type // File type(s) to exclusively sort. Overrides `exclude_type`
    };

    // Run the sorting algorithm
    sorter.sort(false);
}
```
