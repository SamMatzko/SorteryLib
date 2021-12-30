mod structs;

use chrono::{DateTime, TimeZone, Utc, Local};
use std::{fs, path::Path, time::UNIX_EPOCH};
use structs::*;
use walkdir::WalkDir;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}

/// The sorter struct that sorts the files, and interfaces with it's caller for
/// progress bar purposes, if desired.
pub struct Sorter<'a> {
    pub source: &'a File,
    pub target: &'a File,
    pub date_format: &'a str,
    pub date_type: &'a str,
    pub preserve_name: &'a bool,
    pub exclude_type: (&'a str, bool),
    pub only_type: (&'a str, bool)
}
impl <'a> Sorter <'a> {

    // Class functions
    pub fn from_json(&self, json: String, source: File, target: File, dry_run: bool) {
        ConfigData::from_json(&json);
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

    /// Return the access date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_access(&self, path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().accessed().expect("Failed to get atime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }
    
    /// Return the creation date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_creation(&self, path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().created().expect("Failed to get ctime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

    /// Return the modification date and time of `path` as the number of seconds since the
    /// UNIX epoch.
    fn get_epoch_secs_modified(&self, path: &File) -> i64 {
        let ctime_system = path.pathbuf.metadata().unwrap().modified().expect("Failed to get mtime");
        let secs: i64 = ctime_system.duration_since(UNIX_EPOCH).unwrap().as_secs() as i64;

        secs
    }

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
            let new_file = File::from_pathbuf(&new_pathbuf);

            // Check if it exists, and if so, continue the loop
            if !vec.contains(&new_file) {
                return new_file;
            }
            num += 1;
        }
    }

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
                if self.is_sortable(&File::from_path(entry.path()), &exclude_type, &only_type) {
                    items_to_sort += 1;
               }
            }
        }
        
        // Sort the everything, excluding the directories
        for entry in WalkDir::new(source.to_string()) {
            
            let entry = entry.unwrap();
            if !entry.metadata().expect("Failed to get dir metadata").is_dir() {

                // The File instance we are sorting
                let path = File::from_path(entry.path());

                // Make sure that we sort according to the exclude-type and
                // only-type arguments
                if self.is_sortable(&File::from_path(entry.path()), &exclude_type, &only_type) {

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

    /// The method that runs the sorting algorithm, and sends information through
    /// to the caller if specified.
    pub fn sort(&self) {
        let results = self.get_sorting_results(
            self.source,
            self.target,
            self.date_format,
            self.date_type,
            self.preserve_name,
            self.exclude_type,
            self.only_type
        );
    }
}