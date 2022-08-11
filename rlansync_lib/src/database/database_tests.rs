use super::FileInfo;

#[cfg(test)]
mod database_tests {

    use rusqlite::{Connection};
    
    #[test]
    fn test_in_memory() {
        let conn = Connection::open_in_memory().unwrap();
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

        let entry = super::FileInfo {
            path: "a".to_string(),
            source: "b".to_string(),
            digest: "c".to_string(),
            tag: "d".to_string(),
            modify: 123,
            operation: "e".to_string(),
        };
        conn.execute(
            "INSERT INTO entries (path, source, digest, tag, modify, operation) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            (&entry.path, &entry.source, &entry.digest, &entry.tag, &entry.modify, &entry.operation),
        ).unwrap();

        let mut stmt = conn.prepare("SELECT path, source, digest, tag, modify, operation FROM entries WHERE path=:path;").unwrap();
        let entry_iter = stmt.query_map(&[(":path", "a")], |row| {
            Ok(super::FileInfo {
                path: row.get(0).unwrap(),
                source: row.get(1).unwrap(),
                digest: row.get(2).unwrap(),
                tag: row.get(3).unwrap(),
                modify: row.get(4).unwrap(),
                operation: row.get(5).unwrap(),
            })
        }).unwrap();
        let mut found = false;
        for entry in entry_iter {
            if entry.unwrap().digest == "c" {
                found = true
            }
            break
        }
        assert!(found);
    }
}