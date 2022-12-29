use crate::file_filter::FileFilter;
/// Maintain the knowledge about the file
pub struct DefaultFilter {}
impl FileFilter for DefaultFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        let binding = input.to_string();
        let intermediate: Vec<&str> = binding.split('\n').collect();
        intermediate
            .iter()
            .filter(|x| !x.is_empty())
            .map(|x| x.to_string())
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_texta() {
        let mut default_filter = DefaultFilter {};
        let test1 = default_filter.process_text("");
        assert!(test1.is_empty());
    }
    #[test]
    fn test_process_textb() {
        let mut default_filter = DefaultFilter {};
        let test1 = default_filter.process_text("a");
        assert!(test1.len() == 1);
    }
    #[test]
    fn test_process_textc() {
        let mut default_filter = DefaultFilter {};
        let test1 = default_filter.process_text("a\nb");
        assert!(test1.len() == 2);
        assert!(test1[0] == "a");
    }
}
