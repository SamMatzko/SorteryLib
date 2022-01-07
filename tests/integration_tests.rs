/// Tests for integration.
use chrono::{DateTime, Utc};
use filetime;
use sorterylib::prelude::*;
use std::{env, time::SystemTime};

fn callback(data: (usize, usize, usize), v: &mut Vec<(usize, usize, usize)>) {
    println!("{:?}", data);
    v.push(data);
}

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

    // Create the old and new path names for each file that's being sorted
    let old_test_jpg = source.join(File::new("test.jpg"));
    let new_test_jpg = source.join(File::new("target/2022/01/2022 test.jpg"));
    let old_test = source.join(File::new("test"));
    let new_test = source.join(File::new("target/2022/01/2022 test."));
    let old_files_test = source.join(File::new("files/test"));
    let new_files_test = source.join(File::new("target/2022/01/2022 test_2."));
    let old_test_png = source.join(File::new("test.png"));
    let new_test_png = source.join(File::new("target/2022/01/2022 test.png"));

    // Get the DateTime to which to set the modification time of all the files for testing
    let system_time: SystemTime = DateTime::parse_from_rfc2822("Sat, 1 Jan 2022 10:32:02 +0000").unwrap().into();
    let date_time = DateTime::<Utc>::from(system_time);
    
    // Print debugging information. Only shows if the test fails.
    println!("{}", date_time.timestamp());
    println!("{:?}", date_time.format("%a, %d %b %Y %H:%M:%S").to_string());

    // Set the modification times of the files we're sorting
    filetime::set_file_mtime(
        old_test_jpg.to_string(),
        filetime::FileTime::from(system_time)
    ).expect("Failed to set modification time of file.");
    filetime::set_file_mtime(
        old_test.to_string(),
        filetime::FileTime::from(system_time)
    ).expect("Failed to set modification time of file.");
    filetime::set_file_mtime(
        old_files_test.to_string(),
        filetime::FileTime::from(system_time)
    ).expect("Failed to set modification time of file.");
    filetime::set_file_mtime(
        old_test_png.to_string(),
        filetime::FileTime::from(system_time)
    ).expect("Failed to set modification time of file.");

    // The vector for testing the callback
    let mut v: Vec<(usize, usize, usize)> = Vec::new();

    // Test the sorting algorithm and it's callback
    let results = sorter.sort_with_callback(true, |data| {
        callback(data, &mut v);
    });

    // Test the callback output by making sure that the vector is what it should be
    assert_eq!(v, vec![(1, 4, 25), (2, 4, 50), (3, 4, 75), (4, 4, 100)]);
    
    let (old, new) = (results.1, results.2);

    for i in 0..results.0 {
        println!("{}, {}", old[i], new[i]);
    }
    assert_eq!((old[0].copy(), new[0].copy()), (old_test_jpg, new_test_jpg));
    assert_eq!((old[1].copy(), new[1].copy()), (old_test, new_test));
    assert_eq!((old[2].copy(), new[2].copy()), (old_files_test, new_files_test));
    assert_eq!((old[3].copy(), new[3].copy()), (old_test_png, new_test_png));
    assert_eq!(results.0, 4);
    assert_eq!(old.len(), 4);
    assert_eq!(new.len(), 4);
}