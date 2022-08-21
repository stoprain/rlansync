#[cfg(test)]
mod database_tests;
// use protobuf::well_known_types::type_::Field;
use uuid::Uuid;
use std::fs;
use std::path::Path;
use rusqlite::{params, Connection};
// use std::time::{SystemTime, UNIX_EPOCH};
// use crate::protos::generated_with_pure::example::file_info;

use super::FileInfo;

pub struct Database {
    pub source: String,
    pub conn: Connection,
}

const SQL: &str = "CREATE TABLE IF NOT EXISTS entries ( \
    path TEXT PRIMARY KEY, \
    source TEXT, \
    digest TEXT, \
    tag TEXT, \
    modify INTEGER, \
    operation TEXT \
    )";

impl Database {

    pub fn new(parent_pathbuf: Option<&str>) -> Database {
        let conn: Connection;
        let source: String;
        if let Some(path) = parent_pathbuf {
            let root = path;
            let dbdir = root.to_owned() + "/.rlansync/";
            fs::create_dir_all(dbdir.to_owned()).unwrap();
            let initpath = dbdir.to_owned() + "/init";
            let dbpath;
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
        } else {
            source = "in_memory".to_string();
            conn = Connection::open_in_memory().unwrap();
        }
        conn.execute(
            &SQL,
            [],
        ).unwrap();

        Database { 
            source: source,
            conn: conn,
            }
    }

    pub fn get(&mut self, path: String) -> Option<FileInfo> {
        let mut stmt = self.conn.prepare("SELECT path, source, digest, tag, modify, operation FROM entries WHERE path=:path;").unwrap();
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
            return Some(entry.unwrap());
        }
        return None;
    }

    pub fn add(&mut self, entry: FileInfo) {
        self.conn.execute(
            "INSERT INTO entries (path, source, digest, tag, modify, operation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&entry.path, &entry.source, &entry.digest, &entry.tag, &entry.modify, &entry.operation),
        ).unwrap();
    }

    pub fn update(&mut self, entry: FileInfo) {
        self.conn.execute("UPDATE entries SET source = ?1, digest = ?2, tag = ?3, modify = ?4, operation = ?5 WHERE path = ?6", 
            params![entry.source, entry.digest, entry.tag, entry.modify, entry.operation, entry.path]).unwrap();
    }

    pub fn remove(&mut self, path: String) {
        self.conn.execute("DELETE FROM entries WHERE path = (?1)", params![path]).unwrap();
    }

}