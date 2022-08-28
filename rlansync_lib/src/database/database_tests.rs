#[cfg(test)]
mod database_tests {

    use crate::{database::Database, FileInfo};
    
    #[test]
    fn test_in_memory() {
        let mut db = Database::new(None);
        let a = db.get("test");
        assert!(a.is_none());

        let entry = FileInfo {
            path: "path".to_string(),
            source: "".to_string(),
            digest: "digest".to_string(),
            tag: "".to_string(),
            modify: 123,
            operation: "".to_string(),
        };
        db.add(&entry);

        let b = db.get("path").unwrap();
        assert_eq!(b.path, "path");
        assert_eq!(b.digest, "digest");
        assert_eq!(b.modify, 123);

        let entry = FileInfo {
            path: "path".to_string(),
            source: "source".to_string(),
            digest: "digest".to_string(),
            tag: "tag".to_string(),
            modify: 456,
            operation: "update".to_string(),
        };
        db.update(&entry);
        let c = db.get("path").unwrap();
        assert_eq!(c.source, "source");
        assert_eq!(c.tag, "tag");
        assert_eq!(c.modify, 456);
        assert_eq!(c.operation, "update");

        db.remove("path");
        let d = db.get("test");
        assert!(d.is_none());

    }
}