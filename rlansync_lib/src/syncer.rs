use std::collections::HashMap;
use crate::{scanner::{Scanner}, database::Database, swift_callback};
use sha256::digest_file;
use std::path::Path;
use super::FileInfo;

pub struct Syncer {
    pub file_infos: HashMap<String, FileInfo>,
    pub scanner: Scanner,
    pub root: String,
    database: Database,
    source: String,
}

impl Syncer {
    pub fn new(path: &str) -> Syncer {
        let database = Database::new(Some(path));
        let mut s = Syncer {
            file_infos: HashMap::new(),
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

        for (_, file) in files.into_iter() {
            let input = Path::new(&file.path);
            let digest_string = digest_file(input).unwrap();

            let file_info = FileInfo {
                path: (&file.path).to_owned().replace(&self.root, ""),
                source: self.source.to_string(),
                digest: digest_string,
                tag: "".to_string(),
                modify: (&file.modified).to_owned(),
                operation: "".to_string(),
            };
            self.add_from_scanner(&file_info);
            self.file_infos.entry((&file.path).to_owned()).or_insert(file_info);
        }
    }

/*
    1. Add from scanner
        create if not exist in db
        if already exist, check whether hash has changed
*/
    fn add_from_scanner(&mut self, file_info: &FileInfo) {
        if let Some(e) = self.database.get(&file_info.path) {
            if e.digest == file_info.digest {
                //do nothing
            } else {
                //modified
                
            }
        } else {
            self.database.add(file_info)
        }
    }

//TODO: 2. Add from puller (different source)

/*
    1. Add from watcher
 */
    pub fn add(&mut self, path: String) {
        println!("add from watcher {:?}", path);
        if let Some(file) = self.scanner.scan_one(&path) {
            let input = Path::new(&path);
            let digest_string = digest_file(input).unwrap();
    
            let file_info = FileInfo {
                path: (&file.path).to_owned().replace(&self.root, ""),
                source: self.source.to_string(),
                digest: digest_string,
                tag: "".to_string(),
                modify: (&file.modified).to_owned(),
                operation: "".to_string(),
            };
            self.database.add(&file_info);
            self.file_infos.entry((&file.path).to_owned()).or_insert(file_info);
    
            let json = self.get_file_list();
            swift_callback(&json)
        }
    }

/*
    1. Update from watcher
    //TODO:
 */
    pub fn update(&mut self, path: String) {
        println!("update from watcher {:?}", path);
        let json = self.get_file_list();
        swift_callback(&json)
    }

/*
    1. Remove from watcher
 */
    pub fn remove(&mut self, path: String) {
        self.database.remove(&path);
        println!("remove from watcher {:?}", path);
        let json = self.get_file_list();
        swift_callback(&json)
    }

/*
    1. Update from App
 */
    pub fn update_tag(&mut self, path: &str, tag: &str) {
        println!("update tag {} {}", path, tag);
    }

/*
    1. Return file list to App
 */
    pub fn get_file_list(&mut self) -> String {
        let mut entries: Vec<FileInfo> = Vec::new();
        let infos = &self.file_infos;
        for (_, value) in infos.into_iter() {
            entries.push(value.clone());
        }
        let json = serde_json::to_string(&entries).unwrap();
        return json;
    }
}