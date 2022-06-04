/*

kv store
* in memory: HashMap
* sled

configuration
file vs hash


*/


// use sqlite;
use uuid::Uuid;

// mod Datastore;
use sqlite::Value;
extern crate machine_uid;

pub struct Datastore {
    connection: sqlite::Connection,
    machineUid: String,
}

impl Datastore {
    pub fn new() -> Datastore {
        Datastore {
            connection: sqlite::open(".rlansync.db").unwrap(),
            machineUid: machine_uid::get().unwrap(),
        }
    }

    pub fn save(&self, path: String, digest: String) {
        self.connection
            .execute(
                "
                CREATE TABLE IF NOT EXISTS entries (uuid TEXT, path TEXT, digest TEXT, owner TEXT);
                ",
            )
            .unwrap();

        let id = Uuid::new_v4();
        let sid = id.hyphenated().to_string();
        let s = format!("INSERT INTO entries VALUES ('{sid}', '{path}', '{digest}');");
        self.connection
        .execute(s,
        )
        .unwrap();
        // let mut cursor = self.connection
        //     .prepare("INSERT INTO entries VALUES (?1, ?2, ?3);")
        //     .unwrap()
        //     .into_cursor();
        // let id = Uuid::new_v4();
        // let sid = id.hyphenated().to_string();
        // println!("uuid {}", sid);
        // cursor.bind(&[Value::String(sid), Value::String(path), Value::String(digest)]).unwrap();
        // cursor.execute().unwrap();
    }
}