//! SorteryLib is a fast, cross-platform file-sorting library based on the Sortery command-line file sorter.
//! 
//! Basic usage example:
//! 
//! ```
//! use sorterylib::prelude::*; // Import all the stuff needed for basic operation
//! 
//! fn main() {
//!     // The fields for initializing the Sorter struct
//!     let source = File::new("/path/to/source/dir/");
//!     let target = File::new("/path/to/target/dir/");
//!     let date_format = String::from("%Y");
//!     let date_type = String::from("m");
//!     let preserve_name = true;
//!     let exclude_type = vec![String::from("txt")];
//!     let only_type = Vec::new();
//! 
//!     // Create the Sorter instance
//!     let sorter = Sorter {
//!         source: source.copy(), // The directory from which to get all the files to sort
//!         target: target.copy(), // The directory to sort all the files into
//!         date_format: date_format, // The date format to rename the files using.
//!         date_type: date_type, // The date type to sort the files by
//!         preserve_name: preserve_name, // Whether to include the old file name in the new name
//!         exclude_type: exclude_type, // File type(s) to exclude
//!         only_type: only_type // File type(s) to exclusively sort. Overrides `exclude_type`
//!     };
//! 
//!     // Run the sorting algorithm (uncomment line below)
//!     // sorter.sort(false);
//! }
//! ```
//! 
//! You can find more detailed descriptions of the fields on the [`Sorter`] page.

pub mod structs;

use chrono::{DateTime, TimeZone, Utc, Local};
use filetime::FileTime;
use std::{fs, path::Path};
use structs::*;
use walkdir::WalkDir;

/// Includes all the stuff needed for basic operations, in one neat module.
#[allow(unused_imports)]
pub mod prelude {
    pub use crate::Sorter;
    pub use crate::structs::{File, Join};
}

/// Tests. Each test is named after the function or struct it tests, prefixed with `test_`.
#[cfg(test)]
mod tests {
    use crate::Sorter;
    use std::{env, fs, path::Path};
    use super::structs::*;

    #[test]
    fn test_sorter() {

        // The paths to use for testing
        let current_dir = env::current_dir().expect("Failed to get current dir.");
        let json_path = current_dir.join(Path::new("template.json"));
        let source = File::from(&current_dir).join(Path::new("testing"));
        let target = source.join(Path::new("target"));

        // Get the string from the json file
        let json_string = fs::read_to_string(json_path).expect("Failed to read json file.");
        
        // Create a Sorter instance for testing
        let sorter1 = Sorter {
            source: source.copy(),
            target: target.copy(),
            date_format: String::from("%Y-%m-%d %Hh%Mm%Ss"),
            date_type: String::from("m"),
            preserve_name: false,
            exclude_type: vec![String::from("png")],
            only_type: vec![String::from("json"), String::from("py")]
        };

        // Create a Sorter instance from the json string for testing
        let sorter2 = Sorter::from_json(json_string, source.copy(), target.copy());

        // Compare the two Sorter instances to make sure that the JSON was parsed
        // correctly, and that the values match up.
        assert_eq!(sorter1, sorter2);

        // Test the sorting algorithm
        sorter1.sort(true);
    }
}

/// The sorter struct that sorts the files. There are two ways to create an instance
/// of [`Sorter`]: passing the individual fields, and using [`Sorter::from_json`].
#[derive(Debug, PartialEq)]
pub struct Sorter {
    /// An instance of [`File`] specifying the directory from which to get
    /// all the files to sort
    pub source: File,
    /// An instance of [`File`] specifying the directory to sort all the
    /// files into.
    pub target: File,
    /// A [`String`] representing the date format. Uses the standard `strftime` format.
    /// See [`chrono::format::strftime`] for formatting information.
    pub date_format: String,
    /// A [`String`] representing the date type to sort by. Must be one of `String::from("a")`
    /// (accessed) `String::from("c")` (created), or `String::from("m")` (modified).
    /// Note that sorting by creation date is not available on all filesystems.
    pub date_type: String,
    /// If [`true`], then the sorter adds the old file name onto the end of the new
    /// one. For example, `test.txt` would be renamed to something like `2021-04-22 test.txt`.
    /// If [`false`], then ignores the old file name (would rename `test.txt` to `2021-04-22.txt`).
    pub preserve_name: bool,
    /// A [`Vec<String>`] containing all the file extensions to be ignored during
    /// sorting. For example, if `vec![String::from("jpg")]` is passed, than all files
    /// ending in `.jpg` won't be sorted.
    pub exclude_type: Vec<String>,
    /// A [`Vec<String>`] containing all the file extensions to be exclusively sorted.
    /// For example, if `vec![String::from("png")] is passed, than *only* files ending
    /// in `.png` will be sorted. All other files will be ignored. This option overrides
    /// `exclude_type`.
    pub only_type: Vec<String>
}
impl Sorter {

    // Class functions

    /// Return a new [`Sorter`] instance, created using the configuration data in
    /// a JSON string. The source and target directories must be passed to [`Sorter::from_json`]
    /// as well as the json string. For example:
    /// 
    /// ```ignore
    /// use sorterylib::prelude::*;
    /// use std::{fs, path::Path};
    /// 
    /// fn main() {
    /// 
    ///     // The path to the JSON file
    ///     let json_path = Path::new("test.json");
    /// 
    ///     // Load the JSON string from the JSON file
    ///     let json_string = fs::read_to_string(json_path).expect("Failed to read file");
    /// 
    ///     // Create a Sorter instance based on the json file
    ///     let sorter = Sorter::from_json(
    ///         json_string,
    ///         File::from("/path/to/source/dir/"), // The source directory
    ///         File::from("/path/to/target/dir/") // The target directory
    ///     );
    /// }
    /// ```
    /// 
    /// In future versions, a JSON file will be able to be passed instead of a JSON [`String`].
    pub fn from_json(json_string: String, source: File, target: File) -> Sorter {

        // Get the data from the JSON string
        let data = ConfigData::from_json(&json_string);

        Sorter {
            source: source,
            target: target,
            date_format: data.date_format,
            date_type: data.date_type,
            preserve_name: data.preserve_name,
            exclude_type: data.exclude_type,
            only_type: data.only_type
        }
    }

    // Methods

    /// Return a [`DateTime`] instance representing the creation, modification,
    /// or access time of `path` according to `date_type`.
    /// 
    /// `date_type` must be one of `"c"` (created), `"a"` (accessed), or `"m"` (modified).
    /// Note that creation time is not available on all filesystems.
    fn get_datetime(&self, path: &File, date_type: &str) -> DateTime<Local> {
        let secs: i64;
        if date_type == "m" {
            secs = self.get_epoch_secs_modified(path);
        } else if date_type == "a" {
            secs = self.get_epoch_secs_access(path);
        } else {
            secs = self.get_epoch_secs_creation(path);
        }
        let ctime = Utc.timestamp(secs, 0);
        let mytime = Local.from_utc_datetime(&ctime.naive_utc());

        mytime
    }

    /// Return the access date and time of `path` as the number of seconds since the epoch.
    /// Now works cross-platform.
    fn get_epoch_secs_access(&self, path: &File) -> i64 {
        let metadata = path.pathbuf.metadata().unwrap();
        let secs: i64 = FileTime::from_last_access_time(&metadata).seconds() as i64;

        secs
    }
    
    /// Return the creation date and time of `path` as the number of seconds since the epoch.
    /// Now works cross-platform.
    fn get_epoch_secs_creation(&self, path: &File) -> i64 {
        let metadata = path.pathbuf.metadata().unwrap();
        let secs: i64 = FileTime::from_creation_time(&metadata).expect("Failed to get ctime.").seconds() as i64;

        secs
    }

    /// Return the modification date and time of `path` as the number of seconds since the epoch.
    /// Now works cross-platform.
    fn get_epoch_secs_modified(&self, path: &File) -> i64 {
        let metadata = path.pathbuf.metadata().unwrap();
        let secs: i64 = FileTime::from_last_modification_time(&metadata).seconds() as i64;
        println!("secs: {} timestamp: {}", secs, 1641033122);
        println!("{}", secs < 1641033122);

        secs
    }

    /// Get the new directory stacks for all the files, according to the sorting algorithm.
    fn get_new_date_path(
        &self,
        target: &File,
        old_file: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: bool) -> File {
        
        // Get the time of old_file and set the names of the directories
        let ctime = self.get_datetime(old_file, &date_type);
        let dir = target.join(Path::new(&ctime.format("%Y/%m/").to_string()));

        // Preserve the original file name, if we're supposed to.
        let mut name_to_preserve = String::from("");
        if preserve_name {
            name_to_preserve = format!(
                " {}",
                old_file.file_stem()
            );
        }

        // Create the new file name
        let new_file = dir.join(Path::new(&format!(
            "{}{}.{}",
            &ctime.format(date_format).to_string(),
            name_to_preserve,
            old_file.extension()
        )));

        new_file
    }

    /// Return a [`File`] representing the renamed version of `path`.
    /// 
    /// This function is called only if `path` already exists, but can't/shouldn't
    /// be replaced. The naming logic: if `/path/to/file` already exists, return
    /// `/path/to/file_2`. If `/path/to/file_2` already exists, return `/path/to/file_3`, etc.
    fn get_sequential_name(&self, path: &File, vec: &Vec<File>) -> File {

        let mut num = 2;

        loop {

            // Create the new path name
            let mut new_pathbuf = path.to_path_buf();
            new_pathbuf.set_file_name(&format!(
                "{}_{}.{}",
                path.pathbuf.file_stem().unwrap().to_str().unwrap(),
                num,
                path.pathbuf.extension().unwrap().to_str().unwrap()
            ));
            let new_file = File::from(&new_pathbuf);

            // Check if it exists, and if so, continue the loop
            if !vec.contains(&new_file) {
                return new_file;
            }
            num += 1;
        }
    }

    /// Get the full sorting results for all the files according to the sorting algorithm.
    fn get_sorting_results(
        &self,
        source: &File,
        target: &File,
        date_format: &str,
        date_type: &str,
        preserve_name: &bool,
        exclude_type: (&str, bool),
        only_type: (&str, bool)) -> (usize, Vec<File>, Vec<File>) {

        // The vector to return: a tuple of (old_filename, new_filename)
        let mut vec_old: Vec<File> = Vec::new();
        let mut vec_new: Vec<File> = Vec::new();

        // Count the number of items we are going to sort
        let mut items_to_sort = 0;
        for entry in WalkDir::new(source.to_string()) {

            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {
                if self.is_sortable(&File::from(entry.path()), &exclude_type, &only_type) {
                    items_to_sort += 1;
               }
            }
        }
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The File instance we are sorting
                let path = File::from(entry.path());

                // Make sure that we sort according to the exclude-type and
                // only-type arguments
                if self.is_sortable(&File::from(entry.path()), &exclude_type, &only_type) {

                    let mut new_file = self.get_new_date_path(&target, &path, date_format, date_type, *preserve_name);

                    // Get the sequential file name if new_file already exists
                    if vec_new.contains(&new_file) {
                        new_file = self.get_sequential_name(&new_file, &vec_new);
                    }

                    // Push the new and old file names to their respective vectors
                    vec_old.push(path.copy());
                    vec_new.push(new_file);
                }
            }
        }
        (items_to_sort, vec_old, vec_new)
    }

    /// Return [`true`] if:
    /// 1) `path`'s type is in `only_type.0` and `only_type.1` is [`true`]
    /// 2) `path`'s type is not in `exclude_type.0`, and `only_type.1` is [`false`]
    /// 
    /// "Type" refers to the file extension, as in `"jpg"`, `"png"`, etc. `exclude_type`
    /// and `only_type` correspond with `exclude_type` and `only_type` in [`get_sorting_results`],
    /// respectively.
    fn is_sortable(&self, path: &File, exclude_type: &(&str, bool), only_type: &(&str, bool)) -> bool {

        if self.is_type(path, only_type.0) && only_type.1 {
            return true;
        } else if !self.is_type(path, exclude_type.0) && !only_type.1 {
            return true;
        } else {
            return false;
        }
    }

    /// Return [`true`] if `path`'s type is one of the types in `types`.
    /// "Type" refers to the file extension, as in `"jpg"`, `"png"`, etc.
    fn is_type(&self, path: &File, types: &str) -> bool {
        let mut to_return: bool = false;
        for t in types.split("-") {
            if path.extension() == t {
                to_return = true;
            }
        }
        to_return
    }

    /// The method that runs the sorting algorithm. Returns the sorting results as
    /// a tuple of ([`usize`], [`Vec<String>`], [`Vec<String>`]), where `results.0`
    /// is the number of items sorted, `results.1` contains all the old file names,
    /// and `results.2` contains all the new file names. The two vectors correspond
    /// index-wise, so `results.1[0]` is renamed to `results.2[0]`, etc.
    /// 
    /// If `dry_run` is [`true`], return the results as usual, but without acutally
    /// sorting the files. Can be used to verify that the sorting algorithm is working
    /// as intended. For example:
    /// 
    /// ```ignore
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    /// 
    ///     // The sorter instance
    ///     let sorter = Sorter { ... };
    /// 
    ///     // Dry run, without actually sorting the files
    ///     sorter.sort(true);
    /// 
    ///     // Acutally sort the files
    ///     sorter.sort(false);
    /// }
    /// ```
    pub fn sort(&self, dry_run: bool) -> (usize, Vec<File>, Vec<File>) {

        // Convert the exclude_type and only_type values to the tuples that
        // self.get_sorting_results() takes
        let exclude_type: (&str, bool) = (
            &self.exclude_type.join("-"),
            self.exclude_type.len() > 0
        );
        let only_type: (&str, bool) = (
            &self.only_type.join("-"),
            self.only_type.len() > 0
        );

        // Get the sorting results
        let results = self.get_sorting_results(
            &self.source,
            &self.target,
            self.date_format.as_str(),
            self.date_type.as_str(),
            &self.preserve_name,
            exclude_type,
            only_type
        );

        // Sort the files, or dry run if specified
        if !dry_run {

            // Make another tuple, so the vectors aren't consumed
            let r: (usize, &Vec<File>, &Vec<File>) = (results.0, &results.1, &results.2);
            for i in 0..r.0 {
                fs::rename(
                    r.1[i].to_path_buf(),
                    r.2[i].to_path_buf()
                ).expect("Failed to rename file.");
            }
        }
        (results.0, results.1, results.2)
    }
}