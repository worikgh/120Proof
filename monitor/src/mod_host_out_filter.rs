use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;
/// Maintain the knowledge about the file
const READY_RULE_NAME: &str = "ready_rule";

pub struct ModHostOutFilter {
    filter_rules: FilterRules,
}
impl FileFilter for ModHostOutFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        let input = input.to_string();
        let lines: Vec<&str> = input.split('\n').collect();
        let mut output: Vec<String> = vec![];
        for line in lines.iter() {
            if let Some(_caps) = self.filter_rules.evaluate(READY_RULE_NAME, line) {
                output.push("Ready".to_string());
            }
        }
        output
    }
}
impl ModHostOutFilter {
    pub fn new() -> Self {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(READY_RULE_NAME, r"^mod-host: mod-host ready!$");
        ModHostOutFilter { filter_rules }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_texta() {
        let mut default_filter = ModHostOutFilter::new();
        let test1 = default_filter.process_text("");
        assert!(test1.is_empty());
    }
    #[test]
    fn test_process_textb() {
        let mut default_filter = ModHostOutFilter::new();
        let test1 = default_filter.process_text("mod-host: mod-host ready!");
        assert!(!test1.is_empty());
    }
}
