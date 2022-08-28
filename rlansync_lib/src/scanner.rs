use std::{path::PathBuf};
use std::fs::ReadDir;
use std::fs;
use std::collections::HashMap;
use std::time::{UNIX_EPOCH};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct File {
    pub path: String,
    pub modified: u64,
}

impl std::fmt::Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value path: {}, value modified: {})", self.path, self.modified)
    }
}

const IGNORE_FILES: [&'static str; 2] = [
    ".DS_Store",
    ".rlansync"
];

pub struct Scanner {
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
        }
    }
    pub fn scan(&mut self, root: &str) -> HashMap<String, File> {

        let mut files: HashMap<String, File> = HashMap::new();

        let mut iter = FileIteratror::from(root);
        while let Some((pathbuf, is_folder)) = iter.next() {
            let _ = pathbuf.file_name().to_owned();
            let string = pathbuf.to_owned().into_os_string().into_string().unwrap();

            if is_folder {
            } else {
                let metadata = fs::metadata(string.to_owned());
                let secs = metadata.unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let file = File {
                    path: string.to_owned(),
                    modified: secs
                };
                files.entry(string.to_owned()).or_insert(file);
            }
        }

        return files;
    }

    pub fn scan_one(&mut self, path: &str) -> Option<File> {

        let split = path.split("/");
        for s in split {
            if IGNORE_FILES.contains(&s) {
                println!("ignore {}", path);
                return None
            }
        }

        let metadata = fs::metadata(path.to_owned());
        let secs = metadata.unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let file = File {
            path: path.to_owned(),
            modified: secs
        };
        return Some(file);
    }


}

struct FileIteratror {
    dirs: Vec<PathBuf>,
    files: Option<ReadDir>,
}

impl From<&str> for FileIteratror {
    fn from(path: &str) -> Self {
        FileIteratror {
            dirs: vec![PathBuf::from(path)],
            files: None,
        }
    }
}

impl Iterator for FileIteratror {
    type Item = (PathBuf, bool);
    fn next(&mut self) -> Option<(PathBuf, bool)> {
        loop {
            while let Some(read_dir) = &mut self.files {
                match read_dir.next() {
                    Some(Ok(entry)) => {
                        let path = entry.path();
                        if let Ok(md) = entry.metadata() {
                            let str = path.file_name().unwrap().to_str().unwrap();
                            if IGNORE_FILES.contains(&str) {
                                continue;
                            }

                            if !md.is_file() && !md.is_dir() {
                                continue;
                            }
                            if md.is_dir() {
                                self.dirs.push(path.clone());
                                continue;
                            }
                        }
                        return Some((path, false));
                    }
                    None => {
                        self.files = None;
                        break;
                    }
                    _ => { }
                }
            }
            while let Some(dir) = self.dirs.pop() {
                let read_dir = fs::read_dir(&dir);
                if let Ok(files) = read_dir {
                    self.files = Some(files);
                    return Some((dir, true));
                }
            }
            break;
        }
        return None;
    }
}