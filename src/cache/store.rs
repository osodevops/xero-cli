use crate::error::{Result, XeroCliError};
use rusqlite::Connection;
use std::path::PathBuf;
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct CacheStore {
    conn: Mutex<Connection>,
}

#[derive(Debug)]
pub struct CacheEntry {
    pub value: Vec<u8>,
    pub etag: Option<String>,
    pub last_modified: Option<String>,
}

#[derive(Debug)]
pub struct CacheStats {
    pub total_entries: u64,
    pub total_size_bytes: u64,
    pub resource_counts: Vec<(String, u64)>,
}

impl CacheStore {
    pub fn new(cache_dir: &PathBuf) -> Result<Self> {
        std::fs::create_dir_all(cache_dir)
            .map_err(|e| XeroCliError::config(format!("Failed to create cache directory: {e}")))?;

        let db_path = cache_dir.join("cache.db");
        let conn = Connection::open(&db_path)
            .map_err(|e| XeroCliError::config(format!("Failed to open cache database: {e}")))?;

        conn.execute_batch(
            "CREATE TABLE IF NOT EXISTS cache_entries (
                key TEXT PRIMARY KEY,
                value BLOB NOT NULL,
                resource_type TEXT NOT NULL,
                etag TEXT,
                last_modified TEXT,
                created_at INTEGER NOT NULL,
                ttl_seconds INTEGER NOT NULL
            );
            CREATE INDEX IF NOT EXISTS idx_resource_type ON cache_entries(resource_type);",
        )
        .map_err(|e| XeroCliError::config(format!("Failed to initialize cache schema: {e}")))?;

        Ok(Self {
            conn: Mutex::new(conn),
        })
    }

    pub fn get(&self, key: &str) -> Result<Option<CacheEntry>> {
        let conn = self.conn.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        let mut stmt = conn
            .prepare(
                "SELECT value, etag, last_modified, created_at, ttl_seconds
                 FROM cache_entries WHERE key = ?1",
            )
            .map_err(|e| XeroCliError::config(format!("Cache query failed: {e}")))?;

        let result = stmt
            .query_row(rusqlite::params![key], |row| {
                let created_at: i64 = row.get(3)?;
                let ttl: i64 = row.get(4)?;
                Ok((
                    row.get::<_, Vec<u8>>(0)?,
                    row.get::<_, Option<String>>(1)?,
                    row.get::<_, Option<String>>(2)?,
                    created_at,
                    ttl,
                ))
            })
            .ok();

        match result {
            Some((value, etag, last_modified, created_at, ttl)) => {
                if now - created_at > ttl {
                    // Expired — clean up
                    drop(stmt);
                    conn.execute(
                        "DELETE FROM cache_entries WHERE key = ?1",
                        rusqlite::params![key],
                    )
                    .ok();
                    Ok(None)
                } else {
                    Ok(Some(CacheEntry {
                        value,
                        etag,
                        last_modified,
                    }))
                }
            }
            None => Ok(None),
        }
    }

    pub fn put(
        &self,
        key: &str,
        value: &[u8],
        resource_type: &str,
        ttl_seconds: u64,
        etag: Option<&str>,
        last_modified: Option<&str>,
    ) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs() as i64;

        conn.execute(
            "INSERT OR REPLACE INTO cache_entries
             (key, value, resource_type, etag, last_modified, created_at, ttl_seconds)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7)",
            rusqlite::params![
                key,
                value,
                resource_type,
                etag,
                last_modified,
                now,
                ttl_seconds as i64
            ],
        )
        .map_err(|e| XeroCliError::config(format!("Cache write failed: {e}")))?;

        Ok(())
    }

    pub fn invalidate(&self, key: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM cache_entries WHERE key = ?1",
            rusqlite::params![key],
        )
        .map_err(|e| XeroCliError::config(format!("Cache invalidation failed: {e}")))?;
        Ok(())
    }

    pub fn invalidate_by_resource(&self, resource_type: &str) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute(
            "DELETE FROM cache_entries WHERE resource_type = ?1",
            rusqlite::params![resource_type],
        )
        .map_err(|e| XeroCliError::config(format!("Cache invalidation failed: {e}")))?;
        Ok(())
    }

    pub fn clear(&self) -> Result<()> {
        let conn = self.conn.lock().unwrap();
        conn.execute("DELETE FROM cache_entries", [])
            .map_err(|e| XeroCliError::config(format!("Cache clear failed: {e}")))?;
        Ok(())
    }

    pub fn stats(&self) -> Result<CacheStats> {
        let conn = self.conn.lock().unwrap();

        let total_entries: u64 = conn
            .query_row("SELECT COUNT(*) FROM cache_entries", [], |row| row.get(0))
            .unwrap_or(0);

        let total_size_bytes: u64 = conn
            .query_row(
                "SELECT COALESCE(SUM(LENGTH(value)), 0) FROM cache_entries",
                [],
                |row| row.get(0),
            )
            .unwrap_or(0);

        let mut stmt = conn
            .prepare(
                "SELECT resource_type, COUNT(*) FROM cache_entries GROUP BY resource_type ORDER BY resource_type",
            )
            .map_err(|e| XeroCliError::config(format!("Cache stats query failed: {e}")))?;

        let resource_counts: Vec<(String, u64)> = stmt
            .query_map([], |row| {
                Ok((row.get::<_, String>(0)?, row.get::<_, u64>(1)?))
            })
            .map_err(|e| XeroCliError::config(format!("Cache stats query failed: {e}")))?
            .filter_map(|r| r.ok())
            .collect();

        Ok(CacheStats {
            total_entries,
            total_size_bytes,
            resource_counts,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn temp_store() -> (CacheStore, tempfile::TempDir) {
        let dir = tempfile::tempdir().unwrap();
        let store = CacheStore::new(&dir.path().to_path_buf()).unwrap();
        (store, dir)
    }

    #[test]
    fn put_and_get() {
        let (store, _dir) = temp_store();
        store
            .put("Invoices", b"test data", "Invoices", 300, None, None)
            .unwrap();
        let entry = store.get("Invoices").unwrap().unwrap();
        assert_eq!(entry.value, b"test data");
    }

    #[test]
    fn get_nonexistent() {
        let (store, _dir) = temp_store();
        assert!(store.get("missing").unwrap().is_none());
    }

    #[test]
    fn invalidate_key() {
        let (store, _dir) = temp_store();
        store
            .put("key1", b"data", "Resource", 300, None, None)
            .unwrap();
        store.invalidate("key1").unwrap();
        assert!(store.get("key1").unwrap().is_none());
    }

    #[test]
    fn invalidate_by_resource() {
        let (store, _dir) = temp_store();
        store
            .put("Invoices?page=1", b"data1", "Invoices", 300, None, None)
            .unwrap();
        store
            .put("Invoices?page=2", b"data2", "Invoices", 300, None, None)
            .unwrap();
        store
            .put("Contacts", b"data3", "Contacts", 300, None, None)
            .unwrap();
        store.invalidate_by_resource("Invoices").unwrap();
        assert!(store.get("Invoices?page=1").unwrap().is_none());
        assert!(store.get("Invoices?page=2").unwrap().is_none());
        assert!(store.get("Contacts").unwrap().is_some());
    }

    #[test]
    fn clear_all() {
        let (store, _dir) = temp_store();
        store.put("k1", b"d1", "R1", 300, None, None).unwrap();
        store.put("k2", b"d2", "R2", 300, None, None).unwrap();
        store.clear().unwrap();
        let stats = store.stats().unwrap();
        assert_eq!(stats.total_entries, 0);
    }

    #[test]
    fn stats_counts() {
        let (store, _dir) = temp_store();
        store.put("k1", b"d1", "Invoices", 300, None, None).unwrap();
        store.put("k2", b"d2", "Invoices", 300, None, None).unwrap();
        store.put("k3", b"d3", "Contacts", 300, None, None).unwrap();
        let stats = store.stats().unwrap();
        assert_eq!(stats.total_entries, 3);
        assert_eq!(stats.resource_counts.len(), 2);
    }

    #[test]
    fn etag_and_last_modified() {
        let (store, _dir) = temp_store();
        store
            .put(
                "key",
                b"data",
                "R",
                300,
                Some("\"abc123\""),
                Some("Mon, 01 Jan 2024 00:00:00 GMT"),
            )
            .unwrap();
        let entry = store.get("key").unwrap().unwrap();
        assert_eq!(entry.etag.as_deref(), Some("\"abc123\""));
        assert_eq!(
            entry.last_modified.as_deref(),
            Some("Mon, 01 Jan 2024 00:00:00 GMT")
        );
    }

    #[test]
    fn expired_entry_returns_none() {
        let (store, _dir) = temp_store();
        // Insert with 1 TTL and sleep to ensure expiry
        store.put("key", b"data", "R", 1, None, None).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(2));
        assert!(store.get("key").unwrap().is_none());
    }
}
