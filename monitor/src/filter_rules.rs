/// The rules to apply to lines of text.
/// Each rule is a `Regex` regular expression
/// Each rule takes a `String` and returns a `Option<regex::Captures>`
/// Each rule is indexed with String
use regex::Regex;
use std::collections::HashMap;
#[derive(Debug)]
pub struct FilterRules {
    pub rules: HashMap<String, Regex>,
}

impl FilterRules {
    pub fn new() -> Self {
        FilterRules {
            rules: HashMap::new(),
        }
    }
    pub fn add_rule(&mut self, name: &str, rule: &str) {
        let rule = Regex::new(rule).unwrap();
        self.rules.insert(name.to_string(), rule);
    }
    pub fn evaluate<'a>(&'a self, rule_name: &str, input: &'a str) -> Option<regex::Captures> {
        let rule = self.rules.get(rule_name).unwrap();
        rule.captures(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_rule() {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule("first_rule", r".+");
        assert!(filter_rules.rules.len() == 1);
    }

    #[test]
    fn test_evaluate() {
        let mut filter_rules = FilterRules::new();
        const RULE_1_NAME: &str = "rule_1";
        const RULE_2_NAME: &str = "rule_2";
        const RULE_3_NAME: &str = "rule_3";
        filter_rules.add_rule(RULE_1_NAME, r".*");
        filter_rules.add_rule(RULE_2_NAME, r"number (\d+)");
        filter_rules.add_rule(RULE_3_NAME, r"number (\d+) word ([a-z]+)");
        assert!(filter_rules.rules.keys().any(|x| x == RULE_1_NAME));
        assert!(filter_rules.rules.keys().any(|x| x == RULE_2_NAME));
        assert!(filter_rules.rules.keys().any(|x| x == RULE_3_NAME));

        let caps = filter_rules.evaluate(RULE_1_NAME, "a");
        assert!(caps.is_some());
        assert!(filter_rules
            .evaluate(RULE_1_NAME, "")
            .unwrap()
            .get(0)
            .is_some());

        let caps = filter_rules.evaluate(RULE_2_NAME, "a");
        assert!(caps.is_none());
        let caps = filter_rules.evaluate(RULE_2_NAME, "number qq");
        assert!(caps.is_none());
        let caps = filter_rules.evaluate(RULE_2_NAME, "number 202");
        assert!(caps.is_some());
        let caps = caps.unwrap();
        assert!(caps.get(1).unwrap().as_str().parse::<usize>().unwrap() == 202);
        let caps = filter_rules.evaluate(RULE_3_NAME, "number 202 word abcdef");
        assert!(caps.is_some());
        let caps = caps.unwrap();
        assert!(caps.get(1).unwrap().as_str().parse::<usize>().unwrap() == 202);
        assert!(caps.get(2).unwrap().as_str() == "abcdef");
    }
}
