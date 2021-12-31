//! Commonly-used structs.

use serde::{Deserialize, Serialize};
use std::fmt;
use std::path::{Path, PathBuf};

#[cfg(test)]
/// Tests for the structs. Each test is named after the function and/or struct
/// it tests, prefixed with test.
mod tests {

    use std::{env, fs, path::Path};
    use super::{ConfigData, File, Join};
    
    #[test]
    /// Test the [`ConfigData`] struct
    fn test_configdata() {
        
        // Read the json string from template.json
        let current_dir = env::current_dir().expect("Failed to get current dir.");
        let path = current_dir.join(Path::new("template.json"));
        let json_string = fs::read_to_string(path).expect("Failed to parse json.");

        // Create the ConfigData instance and test it's fields
        let config_data = ConfigData::from_json(&json_string);
        assert_eq!(config_data.date_format, String::from("%Y-%m-%d %Hh%Mm%Ss"));
        assert_eq!(config_data.date_type, String::from("m"));
        assert_eq!(config_data.exclude_type.len(), 1);
        assert_eq!(config_data.exclude_type[0], String::from("png"));
        assert_eq!(config_data.only_type.len(), 2);
        assert_eq!(config_data.only_type[0], String::from("json"));
        assert_eq!(config_data.only_type[1], String::from("py"));
        assert_eq!(config_data.preserve_name, false);
    }

    #[test]
    /// Test the [`File`] struct
    fn test_file() {

        // The variables used for testing
        let path = Path::new("my_file.txt");
        let joined_path = Path::new("my_file.txt/my_file.txt");
        let file = File::from(path);

        // Test the methods
        assert!(!file.exists());
        assert_eq!(file.copy(), File { pathbuf: path.to_path_buf() });
        assert_eq!(File::from(path), File { pathbuf: path.to_path_buf() });
        assert_eq!(File::from(&path.to_path_buf()), File { pathbuf: path.to_path_buf() });
        assert_eq!(file.extension(), String::from("txt"));
        assert_eq!(file.file_name(), String::from("my_file.txt"));
        assert_eq!(file.file_stem(), String::from("my_file"));
        assert_eq!(file.join(path), File { pathbuf: joined_path.to_path_buf() });
        assert_eq!(file.join(String::from("my_file.txt")), File { pathbuf: joined_path.to_path_buf() });
        assert_eq!(File::new("my_file.txt"), File { pathbuf: path.to_path_buf() });
        assert_eq!(file.to_path_buf(), path.to_path_buf());
        assert_eq!(file.to_string(), String::from("my_file.txt"));
    }
}

/// The struct used for getting the config data from a JSON [`String`]. Future versions
/// will be able to load JSON data directly from a JSON file.
#[derive(Debug)]
#[derive(Serialize, Deserialize)]
pub struct ConfigData {
    pub date_format: String,
    pub date_type: String,
    pub exclude_type: Vec<String>,
    pub only_type: Vec<String>,
    pub preserve_name: bool
}
impl ConfigData {

    /// Return an instance of ConfigData from the data in [`String`] `json`.
    /// Example:
    /// 
    /// ```
    /// use sorterylib::structs::ConfigData;
    /// 
    /// fn main() {
    /// 
    ///     // The JSON string
    ///     let json_string = String::from("
    ///     {
    ///         \"date_format\": \"%Y-%m-%d %Hh%Mm%Ss\",
    ///         \"date_type\": \"m\",
    ///         \"exclude_type\": [\"png\"],
    ///         \"only_type\": [\"json\", \"py\"],
    ///         \"preserve_name\": false
    ///     }");
    /// 
    ///     // Load the JSON string
    ///     let config_data = ConfigData::from_json(&json_string);
    /// }
    /// ```
    /// 
    /// **NOTE:** the backslashes are only needed when defining a [`String`] from
    /// a string literal like this. The JSON file will not need them.
    pub fn from_json(json: &String) -> ConfigData {

        let json_data: ConfigData = serde_json::from_str(json.as_str()).expect("Failed to parse json.");

        ConfigData {
            date_format: json_data.date_format,
            date_type: json_data.date_type,
            exclude_type: json_data.exclude_type,
            only_type: json_data.only_type,
            preserve_name: json_data.preserve_name
        }
    }
}

/// Traits used by [`File`]
pub trait Join<T> {
    fn join(&self, path:T) -> File;
}

/// [`File`] is designed to be as easy-to-use as possible. It supports the default
/// formatter `"{}"` (used with [`println!`]), can be created from many different
/// types, and can be joined with many different types. It underlies all file-related
/// operations in `SorteryLib`.
#[derive(Debug)]
#[derive(PartialEq)]
pub struct File {
    pub pathbuf: PathBuf,
}
impl File {

    /// Returns an instance of [`File`] with the same path as ours. Used to resolve
    /// ownership problems.
    pub fn copy(&self) -> File {
        File { pathbuf: PathBuf::from(&self.pathbuf) }
    }

    /// Return [`true`] if our path exists, [`false`] if it does not.
    pub fn exists(&self) -> bool {
        if self.pathbuf.exists() {
            return true;
        } else {
            return false;
        }
    }

    /// Return a [`String`] representing the extension of our path. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    ///     let file = File::from("test.txt");
    ///     assert_eq!(file.extension(), String::from("txt"));
    /// }
    /// ```
    pub fn extension(&self) -> String {
        match self.pathbuf.as_path().extension() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// Return a [`String`] representing the file name of our path. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    ///     let file = File::from("/path/to/test.txt");
    ///     assert_eq!(file.file_name(), String::from("test.txt"));
    /// }
    /// ```
    pub fn file_name(&self) -> String {
        match self.pathbuf.as_path().file_name() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// Return a [`String`] representing the file stem of our path. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    ///     let file = File::from("test.txt");
    ///     assert_eq!(file.file_stem(), String::from("test"));
    /// }
    /// ```
    pub fn file_stem(&self) -> String {
        match self.pathbuf.as_path().file_stem() {
            None => return String::from(""),
            s => return String::from(s.unwrap().to_str().unwrap()),
        }
    }

    /// DEPRECATED: Please use [`File::from`] instead.
    /// Return a new instance of [`File`] from `from`. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    ///     let file = File::new("test.txt");
    /// }
    /// ```
    pub fn new(from: &str) -> File {
        File { pathbuf: PathBuf::from(from) }
    }

    /// Return an instance of [`PathBuf`] representing our path. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// use std::path::PathBuf;
    /// 
    /// fn main() {
    ///     let file = File::from("test.txt");
    ///     assert_eq!(file.to_path_buf(), PathBuf::from("test.txt"));
    /// }
    /// ```
    pub fn to_path_buf(&self) -> PathBuf {
        PathBuf::from(&self.pathbuf)
    }
    
    /// Return a [`String`] representing our path. For example:
    /// 
    /// ```
    /// use sorterylib::prelude::*;
    /// 
    /// fn main() {
    ///     let file = File::from("test.txt");
    ///     assert_eq!(file.to_string(), String::from("test.txt"));
    /// }
    pub fn to_string(&self) -> String {
        self.pathbuf.display().to_string()
    }
}
impl fmt::Display for File {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.pathbuf.display())
    }
}
impl<'f> From<&'f Path> for File {
    /// Return a new instance of [`File`], with `path` as the path.
    fn from(path: &Path) -> File {
        File { pathbuf: path.to_path_buf() }
    }
}
impl From<PathBuf> for File {
    /// Return a new instance of [`File`], with `path` as the path.
    fn from(path: PathBuf) -> File {
        File { pathbuf: path }
    }
}
impl<'f> From<&'f PathBuf> for File {
    /// Return a new instance of [`File`], with `path` as the path.
    fn from(path: &PathBuf) -> File {
        File { pathbuf: path.to_path_buf() }
    }
}
impl<'f> From<&'f str> for File {
    /// Return a new instance of [`File`], with `path` as the path
    fn from(path: &str) -> File {
        File { pathbuf: PathBuf::from(path) }
    }
}
impl From<String> for File {
    /// Return a new instance of [`File`], with `path` as the path
    fn from(path: String) -> File {
        File { pathbuf: PathBuf::from(path) }
    }
}
impl Join<File> for File {
    /// Return an instance of [`File`] representing the joining of our path and `path`.
    fn join(&self, path: File) -> File {
        let join_start = self.to_path_buf();
        let join_end = path.to_path_buf();
        let pathbuf = join_start.join(join_end);
        File { pathbuf: pathbuf }
    }
}
impl<'j> Join<&'j Path> for File {
    /// Return an instance of [`File`] representing the joining of our path and `path`.
    fn join(&self, path: &Path) -> File {
        let join_start = self.to_path_buf();
        let join_end = path.to_path_buf();
        let pathbuf = join_start.join(join_end);
        File { pathbuf: pathbuf }
    }
}
impl<'j> Join<&'j PathBuf> for File {
    /// Return an instance of [`File`] representing the joining of our path and `path`.
    fn join(&self, path: &PathBuf) -> File {
        let join_start = self.to_path_buf();
        let pathbuf = join_start.join(path);
        File { pathbuf: pathbuf }
    }
}
impl Join<String> for File {
    /// Return an instance of [`File`] representing the joining of our path and `path`.
    fn join(&self, path: String) -> File {
        let join_start = self.to_path_buf();
        let join_end = PathBuf::from(path);
        let pathbuf = join_start.join(join_end);
        File { pathbuf: pathbuf }
    }
}