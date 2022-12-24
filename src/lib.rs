#![feature(map_first_last,wasi_ext)]

pub use keydir::HashmapKeydir;
pub use storage::DiskStorage;

pub mod errors;
mod format;
mod keydir;
pub mod storage;

pub type RumDb = DiskStorage<HashmapKeydir>;

/// Database options.
#[derive(Debug)]
pub struct DbOptions {
    /// Maximum log file size in bytes.
    max_log_file_size: usize,
}

impl Default for DbOptions {
    fn default() -> Self {
        Self {
            max_log_file_size: 100 * 1024 * 1024, // 100 MB
        }
    }
}

impl DbOptions {
    pub fn max_log_file_size(mut self, value: usize) -> Self {
        self.max_log_file_size = value;
        self
    }
}

/* 
#[no_mangle]
pub extern "C" fn main() {
    use crate::{storage::Storage};

    let mut db = RumDb::open_default("/tmp/basic.rumdb/").unwrap();
    println!("{:?}", db);

    db.put(b"hello".to_vec(), b"world".to_vec()).unwrap();
    assert_eq!(db.get(b"hello").unwrap(), Some(b"world".to_vec()));

    db.remove(b"hello").unwrap();
    
    println!("{:?}", db.get(b"hello"));

}

*/