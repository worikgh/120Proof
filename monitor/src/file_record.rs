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

    //fn process(&mut self
    pub fn summarise(&self, filter: Option<&mut dyn FileFilter>) -> String {
        match filter {
            Some(f) => f.filter(self.cache.clone()),
            None => self.cache.clone(),
        }
    }
}
