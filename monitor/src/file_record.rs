use crate::file_filter::FileFilter;
/// Maintain the knowledge about the file
pub struct FileRecord {
    /// The position in the file that has been processed upto.
    pub position: u64,

    /// The contents of the file that has not been read but not
    /// processed
    pub cache: String,
}
impl FileRecord {
    pub fn new() -> FileRecord {
        FileRecord {
            position: 0,
            cache: String::new(),
        }
    }

    /// Summarise the contents of the filter for display.  When it is
    /// possible the PID of the process generating the output is
    /// included.  But it is not allways possible
    pub fn summarise(&self, _pid: Option<usize>, f: &mut dyn FileFilter) -> Vec<String> {
        f.process_text(self.cache.as_str())
    }
}
