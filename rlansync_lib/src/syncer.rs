use std::collections::HashMap;
use crate::{scanner::{Scanner, EntryInfo}, database::Database};

use super::FileInfo;

pub struct Syncer {
    pub entries_info: HashMap<String, FileInfo>,
    pub scanner: Scanner,
    pub root: String,
    database: Database,
    source: String,
}

impl Syncer {
    pub fn new(path: &str) -> Syncer {
        let database = Database::new(Some(path));
        let mut s = Syncer {
            entries_info: HashMap::new(),
            scanner: Scanner::new(),
            root: path.to_string(),
            database: database,
            source: "".to_string()
        };
        s.run();
        return s;
    }

    pub fn run(&mut self) {
        self.source = self.database.source.to_owned();
        let files = self.scanner.scan(&self.root);
        println!("scan {} files", files.len());

        for (_, value) in files.into_iter() {
            self.add(&value);
        }
    }

/*
    1. Add from scanner
        create if not exist in db
        if already exist, check whether hash has changed
    2. Add from watcher (already in )
    3. Add from puller (different source)
*/
    pub fn add(&mut self, entry: &EntryInfo) {
        if let Some(e) = self.database.get(entry.path.to_owned()) {
            //TODO check digest
            if e.digest == entry.digest {

            } else {

            }
        } else {
            let file = FileInfo {
                path: entry.path.to_owned(),
                source: self.source.to_string(),
                digest: entry.digest.to_owned(),
                tag: "".to_string(),
                modify: entry.modified,
                operation: "".to_string(),
            };
            self.database.add(file)
        }
    }

/*
    1. Add from watcher
 */
    pub fn update(&mut self) {
    }

/*
    1. Add from watcher
 */
    pub fn remove(&mut self) {

    }

/*
    1. Update from App
 */
    pub fn update_tag(&mut self, path: String, tag: String) {

    }

/*
    1. Return file list to App
 */
    pub fn get_file_list(&mut self) -> String {
        let mut entries: Vec<FileInfo> = Vec::new();
        let infos = &self.entries_info;
        for (_, value) in infos.into_iter() {
            entries.push(value.clone());
        }
        let json = serde_json::to_string(&entries).unwrap();
        return json;
    }
}