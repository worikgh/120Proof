pub trait FileFilter {
    fn process_text(&mut self, input: &str) -> Vec<String>;
}
