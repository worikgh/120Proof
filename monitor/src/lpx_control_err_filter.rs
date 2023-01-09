use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;
use std::env;
/// Rule names
const MARK_RULE_NAME: &str = "mark_rule";
const INFO_RULE_NAME: &str = "info_rule";
const STRIP_LINE_RULE_NAME: &str = "strip_line_info_rule";
pub struct LPXControlErrFilter {
    filter_rules: FilterRules,
}
impl FileFilter for LPXControlErrFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        // Break into lines
        let lines: Vec<&str> = input.split('\n').collect();

        let mut result: Vec<String> = vec![];
        for line in lines.iter() {
            let mut line_result: String = line.to_string();
            if let Some(_) = self.filter_rules.evaluate(MARK_RULE_NAME, *line) {
                // Abandon this line
                line_result = line.to_string();
            };
            if let Some(_) = self.filter_rules.evaluate(INFO_RULE_NAME, *line) {
                // Abandon this line
                continue;
            };
            if line_result.is_empty() {
                if let Some(caps) = self.filter_rules.evaluate(STRIP_LINE_RULE_NAME, line) {
                    // "r^(.+) at <Home directory>"
                    let binding = caps.get(1).unwrap().as_str();
                    line_result = binding.to_string();
                }
            }
            if !line_result.is_empty() {
                result.push(line_result)
            }
        }
        result
    }
}
impl LPXControlErrFilter {
    pub fn new() -> Self {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(MARK_RULE_NAME, r"^MARK");
        filter_rules.add_rule(INFO_RULE_NAME, r"^Info");
        let strip_line_rule = format!(r"^(.+)\s+at\s+{}", env::var("Home120Proof").unwrap());
        filter_rules.add_rule(STRIP_LINE_RULE_NAME, strip_line_rule.as_str());
        LPXControlErrFilter { filter_rules }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_texta() {
        let mut default_filter = LPXControlErrFilter::new();
        let test1 = default_filter.process_text("MARK");
        assert!(test1.is_empty());
        let test1 = default_filter.process_text("Info:");
        assert!(test1.is_empty());
        //
        let test1 = default_filter
            .process_text("Owner: root  at /home/patch/120Proof/Perl/One20Proof.pm line 106.");
        let result = test1.first().unwrap().as_str();
        let test_string = "Owner: root ";
        assert!(
            result == test_string,
            "result: {} test: {}",
            result,
            test_string,
        );
        //.to_string());
    }
}
