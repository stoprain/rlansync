use std::path::PathBuf;
use std::fs::ReadDir;
use std::fs;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read, Write};
use ring::digest::{Context, Digest, SHA256};
use data_encoding::HEXUPPER;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct EntryInfo {
    pub path: String,
    pub digest: String,
    pub modified: u64,
}

impl std::fmt::Display for EntryInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value path: {}, value digest: {}, value modified: {})", self.path, self.digest, self.modified)
    }
}

pub struct Scanner {
    pub entries: Vec<String>,
    pub entries_info: HashMap<String, EntryInfo>,
}

impl Scanner {
    pub fn new() -> Scanner {
        Scanner {
            entries: vec![],
            entries_info: HashMap::new()
        }
    }
    pub fn scan(&mut self, pathbuf: &str) {
        let mut iter = FileIteratror::from(pathbuf);
        while let Some((pathbuf, is_folder)) = iter.next() {
            // println!("is folder {} {:?}", is_folder, path);
            let string = pathbuf.into_os_string().into_string().unwrap();
            self.entries.push(string.to_owned());

            if is_folder {
                // self.entries_hash.entry(pathbuf2).or_insert("".to_owned());
            } else {
                // let path = pathbuf.into_os_string().into_string().unwrap();
                let input = File::open(&string).unwrap();
                let reader = BufReader::new(input);
                let digest = sha256_digest(reader).unwrap();
                let digest_string = HEXUPPER.encode(digest.as_ref());
                let metadata = fs::metadata(string.to_owned());
                let secs = metadata.unwrap().modified().unwrap().duration_since(UNIX_EPOCH).unwrap().as_secs();
                let entry = EntryInfo {
                    path: string.to_owned(),
                    digest: digest_string,
                    modified: secs
                };
                self.entries_info.entry(string.to_owned()).or_insert(entry);
            }
        }
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

fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest, Box<dyn Error>> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer).unwrap();
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}