/// Tests for integration.
use sorterylib::prelude::*;
use std::env;

#[test]
fn test_sorter_dry_run() {
    
    // The path to the JSON template; used for testing
    let current_dir = File::from(env::current_dir().expect("Failed to get current dir."));
    
    // The parameters for testing
    let source = current_dir.join(File::from("testing"));
    let target = source.join(File::from("target"));
    let date_format = String::from("%Y");
    let date_type = String::from("m");
    let preserve_name = true;
    let exclude_type = vec![String::from("txt")];
    let only_type = Vec::new();

    // Create a [`Sorter`] instance from the json, and test it
    let sorter = Sorter {
        source: source.copy(),
        target: target.copy(),
        date_format: date_format,
        date_type: date_type,
        preserve_name: preserve_name,
        exclude_type: exclude_type,
        only_type: only_type
    };

    // Test the sorting algorithm
    let results = sorter.sort(true);
    let (old, new) = (results.1, results.2);

    for i in 0..results.0 {
        println!("{}, {}", old[i], new[i]);
    }
    assert_eq!((old[0].copy(), new[0].copy()), (source.join(File::new("test.jpg")), source.join(File::new("target/2021/02/2021 test.jpg"))));
    assert_eq!((old[1].copy(), new[1].copy()), (source.join(File::new("test")), source.join(File::new("target/2021/02/2021 test."))));
    assert_eq!((old[2].copy(), new[2].copy()), (source.join(File::new("files/test")), source.join(File::new("target/2021/02/2021 test_2."))));
    assert_eq!((old[3].copy(), new[3].copy()), (source.join(File::new("test.png")), source.join(File::new("target/2021/02/2021 test.png"))));
    assert_eq!(results.0, 4);
    assert_eq!(old.len(), 4);
    assert_eq!(new.len(), 4);
}