use std::collections::HashMap;
use super::FileInfo;

pub struct Syncer {
    pub entries_info: HashMap<String, FileInfo>
}

impl Syncer {
    pub fn new() -> Syncer {
        Syncer {
            entries_info: HashMap::new()
        }
    }

/*
    1. Add from scanner
        create if not exist in db
        if already exist, check whether hash has changed
    2. Add from watcher (already in )
    3. Add from puller (different source)
*/
    pub fn add(&mut self) {
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
    pub fn update_tag(&mut self) {

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