use std::fmt;
use cxx::SharedPtr;
use rocksdb::rocksdb::*;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("rust-titan.h");
        include!("rocksdb/db.h");

        #[namespace = "rocksdb"]
        type DBOptions;

        unsafe fn new_db_options() -> SharedPtr<DBOptions>;
    }
}

#[derive(Copy, Clone, Debug)]
enum TitanBlobRunMode {
    /// Titan process read/write as normal
    Normal = 0,

    /// Titan stop writing value into blob log during flush
    /// and compaction. Existing values in blob log is still
    /// readable and garbage collected.
    ReadOnly = 1,

    /// On flush and compaction, Titan will convert blob
    /// index into real value, by reading from blob log,
    /// and store the value in SST file.
    Fallback = 2,
}

struct TitanDBOptions {
    /// The directory to store data specific to TitanDB alongside with
    /// the base DB.
    ///
    /// Default: {dbname}/titandb
    dirname: String,

    /// Disable background GC
    ///
    /// Default: false
    disable_background_gc: bool,

    /// Max background GC thread
    ///
    /// Default: 1
    max_background_gc: i32,

    /// How often to schedule delete obsolete blob files periods.
    /// If set zero, obsolete blob files won't be deleted.
    ///
    /// Default: 10
    purge_obsolete_files_period_sec: u32,

    /// If non-zero, dump titan internal stats to info log every
    /// titan_stats_dump_period_sec.
    ///
    /// Default: 600 (10 min)
    titan_stats_dump_period_sec: u32,

    /// In C++ we would inherit from this class
    db_options: SharedPtr<ffi::DBOptions>,
}

impl fmt::Display for TitanDBOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {} {} {}",
            self.dirname,
            self.disable_background_gc,
            self.max_background_gc,
            self.purge_obsolete_files_period_sec,
            self.titan_stats_dump_period_sec,
        )
    }
}

impl TitanDBOptions {
    pub fn new() -> Self {
        let db_options_ptr = unsafe { ffi::new_db_options() };

        Self {
            dirname: "".to_string(),
            disable_background_gc: false,
            max_background_gc: 1,
            purge_obsolete_files_period_sec: 10,
            titan_stats_dump_period_sec: 600,
            db_options: db_options_ptr,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn create_db_options() {
        let titan_options = TitanDBOptions::new();

        println!("{}", titan_options);
    }
}
