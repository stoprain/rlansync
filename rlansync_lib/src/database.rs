use protobuf::well_known_types::type_::Field;
use serde::{Serialize, Deserialize};
use serde_json::Result;
use uuid::Uuid;
use std::fs;
use std::path::Path;
use sled::Db;

#[derive(Serialize, Deserialize, Debug)]
struct FileInfo {
    source: String,
    tag: String,
}

pub struct Database {
    pub source: String,
    pub tree: Db,
}

impl Database {
    pub fn new(parent_pathbuf: &str) -> Database {
        let root = parent_pathbuf.to_string();
        let dbdir = root.to_owned() + "/.rlansync/";
        fs::create_dir_all(dbdir.to_owned()).unwrap();
        let initpath = dbdir.to_owned() + "/init";
        let dbpath;
        let source: String;
        if Path::new(&(initpath)).exists() {
            let initdata = fs::read_to_string(initpath).unwrap();
            dbpath = dbdir.to_owned() + &initdata;
            println!("exist db path {}", dbpath);
            source = initdata;
        } else {
            let id = Uuid::new_v4().to_string();
            dbpath = dbdir.to_owned() + &id;
            fs::write(initpath, id.to_owned()).unwrap();
            println!("new db path {}", dbpath);
            source = id;
        }

        let tree = sled::open(dbpath).unwrap();
        println!("{:?}", tree);
        Database { 
            source: source,
            tree: tree,
         }
    }

    pub fn update(&mut self, path: String) {
        // let data = format!(r"{"source"");
        // let v: FileInfo = serde_json::from_str(&data).unwrap();
        let fileInfo = FileInfo {
            source: path.to_lowercase(),
            tag: "".to_string(),
        };
        let j = serde_json::to_string(&fileInfo);
        println!("{} > {:?}", path, j)
    }

}