use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;
// use std::env;
// use time::OffsetDateTime;

/// Rule names
const SOCKET_RULE_NAME: &str = "socket_rule";
const RESULT_TYPE_FAIL_RULE_NAME: &str = "result_type_rule";

/// Maintain the knowledge about the file
#[derive(Debug)]
pub struct YoshimiErrFilter {
    pid: usize,
    filter_rules: FilterRules,
}
impl FileFilter for YoshimiErrFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        // Break into lines
        let lines: Vec<&str> = input.split('\n').collect();

        let mut result: Vec<String> = vec![];
        for line in lines.iter() {
            let mut line_result: String = String::new();
            if let Some(caps) = self.filter_rules.evaluate(SOCKET_RULE_NAME, line) {
                //             r"Cannot read socket fd = (\d+) err = (.+)",
                let socket_num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                let error: &str = caps.get(1).unwrap().as_str();
                line_result = format!("Failed read socket: {}  {}", socket_num, error);
            } else if let Some(caps) = self.filter_rules.evaluate(RESULT_TYPE_FAIL_RULE_NAME, line)
            {
                //             r"Could not read result type = (\d+)",
                let type_num = caps.get(1).unwrap().as_str().parse::<usize>().unwrap();
                line_result = format!("Failed read result type: {}", type_num,);
            }
            if !line_result.is_empty() {
                result.push(format!("{}:{}", self.pid, line_result));
            }
        }
        result
    }
}

impl YoshimiErrFilter {
    pub fn new(pid: usize) -> YoshimiErrFilter {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(
            SOCKET_RULE_NAME,
            r"Cannot read socket fd = (\d+) err = (.+)",
        );
        filter_rules.add_rule(
            RESULT_TYPE_FAIL_RULE_NAME,
            r"Could not read result type = (\d+)",
        );
        //
        YoshimiErrFilter { filter_rules, pid }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_text() {
        let mut yoshimi_filter = YoshimiErrFilter::new(999);

        // Check if the rules are there
        assert!(yoshimi_filter
            .filter_rules
            .rules
            .keys()
            .any(|x| x == SOCKET_RULE_NAME));
        assert!(yoshimi_filter
            .filter_rules
            .rules
            .keys()
            .any(|x| x == RESULT_TYPE_FAIL_RULE_NAME));

        let result = yoshimi_filter.process_text("Cannot read socket fd = 3 err = Interrupted system call\nCould not read result type = 7");
        assert!(result.len() == 2);
    }
}
