//! Utility methods, mostly for getting sensible path values

use std::path::PathBuf;
use std::old_path::Path as OldPath;
use rustc::util::fs::realpath as rustc_realpath;

pub fn split_tags(tags: String) -> Vec<String> {
    tags.split(',').map(|t| t.to_string()).collect()
}

/// Expand a path relative to the home directory
pub fn expand_home(path: PathBuf) -> PathBuf {
    ::std::env::home_dir().unwrap().join(&path)
}

/// Get the absolute path to a given path
///
/// Yes, this depends on private rustc methods...
pub fn realpath(path: PathBuf) -> PathBuf {
    let old_path = OldPath::new(&path.to_str().unwrap());
    let real_path = rustc_realpath(&old_path).unwrap();
    PathBuf::new(&real_path.as_str().unwrap())
}

/// Call `realpath`, taking and returning a string instead of path
pub fn realpath_string(path: String) -> String {
    realpath(PathBuf::new(&path))
        .into_os_string()
        .to_string_lossy()
        .to_string()
}
