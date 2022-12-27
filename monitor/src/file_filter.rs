pub trait FileFilter {
    fn filter(&mut self, input: String) -> String;
}
