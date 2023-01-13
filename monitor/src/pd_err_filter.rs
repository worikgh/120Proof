use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;

/// Rule names
const RUNNING_RULE_NAME: &str = "running_rule";
const JACK_RULE_NAME: &str = "jack_rule";

pub struct PdErrFilter {
    pid: usize,
    filter_rules: FilterRules,
}

impl FileFilter for PdErrFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        // Break into lines
        let lines: Vec<&str> = input.split('\n').collect();

        let mut result: Vec<String> = vec![];
        for line in lines.iter() {
            let mut line_result: String = "".to_string();
            if let Some(caps) = self.filter_rules.evaluate(RUNNING_RULE_NAME, *line) {
                line_result = format!("Priority: {}", caps.get(1).unwrap().as_str());
            } else if let Some(caps) = self.filter_rules.evaluate(JACK_RULE_NAME, *line) {
                line_result = format!("Pure Data up. Jack-Pipe: {}", caps.get(1).unwrap().as_str());
            }
            if !line_result.is_empty() {
                result.push(format!("{}:{}", self.pid, line_result));
            }
        }
        result
    }
}
impl PdErrFilter {
    pub fn new(pid: usize) -> Self {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(RUNNING_RULE_NAME, r"running at (.+) priority");
        filter_rules.add_rule(JACK_RULE_NAME, r"JACK: registered as '(\S+)'");
        PdErrFilter { filter_rules, pid }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_texta() {
        let mut pd_filter = PdErrFilter::new(0);

        // Test that text that is not passed through is dropped
        let test1 = pd_filter.process_text("MARK");
        assert!(test1.is_empty(), "Text should have been empty: {:?}", test1);

        // Test the rule for detecting running priority
        let test1 =
            pd_filter.process_text("verbose(4): running at normal (non-real-time) priority.");

        let result = test1.first().unwrap().as_str();
        let test_string = "0:Priority: normal (non-real-time)";
        assert!(
            result == test_string,
            "result: '{}' test: '{}'",
            result,
            test_string,
        );
        //.to_string());
    }
}
