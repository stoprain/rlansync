
#[cfg(test)]
mod database_tests;

use serde::{Serialize, Deserialize};
use uuid::Uuid;
use std::fs;
use std::path::Path;
use rusqlite::{params, Connection};
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize, Debug)]
pub struct FileInfo {
    pub path: String,
    pub source: String,
    pub digest: String,
    pub tag: String,
    pub modify: u64,
    pub operation: String,
}

pub struct Database {
    pub source: String,
    pub conn: Connection,
}

impl Database {
    pub fn new(parent_pathbuf: &str) -> Database {
        let root = parent_pathbuf.to_string();
        let dbdir = root.to_owned() + "/.rlansync/";
        fs::create_dir_all(dbdir.to_owned()).unwrap();
        let initpath = dbdir.to_owned() + "/init";
        let dbpath;
        let source: String;
        let conn: Connection;
        if Path::new(&(initpath)).exists() {
            let initdata = fs::read_to_string(initpath).unwrap();
            dbpath = dbdir.to_owned() + &initdata + ".db";
            println!("exist db path {}", dbpath);
            source = initdata;
        } else {
            let id = Uuid::new_v4().to_string();
            dbpath = dbdir.to_owned() + &id + ".db";
            fs::write(initpath, id.to_owned()).unwrap();
            println!("new db path {}", dbpath);
            source = id;
        }

        conn = Connection::open(dbpath).unwrap();
        conn.execute(
            "CREATE TABLE IF NOT EXISTS entries (
                path TEXT PRIMARY KEY,
                source TEXT,
                digest TEXT,
                tag TEXT,
                modify INTEGER,
                operation TEXT
                )",
            [],
        ).unwrap();

        Database { 
            source: source,
            conn: conn,
            }
    }

    /*
        Create if not exist
        Write if digest changed
        Remove
    */

    pub fn update(&mut self, path: String, digest: String) {
        // let data = format!(r"{"source"");
        // let v: FileInfo = serde_json::from_str(&data).unwrap();
        // let file_info = FileInfo {
        //     source: path.to_lowercase(),
        //     tag: "".to_string(),
        // };
        // let _j = serde_json::to_string(&file_info);
        // println!("{} > {:?}", path, j)
        // println!("update path {}, digest {}", path, digest);
        let mut stmt = self.conn.prepare("SELECT path, source, digest, tag, modify, operation FROM entries WHERE path=:path;").unwrap();
        let t = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let entry_iter = stmt.query_map(&[(":path", &path)], |row| {
            Ok(FileInfo {
                path: row.get(0).unwrap(),
                source: row.get(1).unwrap(),
                digest: row.get(2).unwrap(),
                tag: row.get(3).unwrap(),
                modify: row.get(4).unwrap(),
                operation: row.get(5).unwrap(),
            })
        }).unwrap();
        for entry in entry_iter {
            if entry.unwrap().digest == digest {
                println!("Found {:?}", path);
            } else {
                println!("Write {:?}", path);
                self.conn.execute(
                    "UPDATE entries SET digest = ?1",
                    params![digest],
                ).unwrap();
            }
            return
        }
        println!("CREATE {} {}", path, digest);
        let entry = FileInfo {
            path: path,
            source: "".to_string(),
            digest: digest,
            tag: "".to_string(),
            modify: t,
            operation: "".to_string(),
        };
        self.conn.execute(
            "INSERT INTO entries (path, digest) VALUES (?1, ?2)",
            (&entry.path, &entry.digest),
        ).unwrap();
    
    }

}