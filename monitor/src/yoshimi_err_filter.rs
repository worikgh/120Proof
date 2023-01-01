use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;
use std::env;
use time::OffsetDateTime;
const SECONDS_XRUN_REPORT: i64 = 60;

/// Rule names
const SOCKET_RULE_NAME: &str = "socket_rule";

/// Maintain the knowledge about the file
#[derive(Debug)]
pub struct YoshimiErrFilter {
    pub sample_rate: Option<usize>,
    pub xruns: usize, // Count how many xruns
    pub xrun_time: Option<OffsetDateTime>,
    pub instrument: Vec<String>,
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
                let sample_rate_text = caps.get(1).map_or("0", |m| m.as_str());
                let new_sample_rate = sample_rate_text.parse().unwrap();
                if self.sample_rate.is_some() {
                    line_result = format!(
                        "Overwriting sample rate.  Was: {} changed to: {}",
                        self.sample_rate.unwrap(),
                        new_sample_rate
                    );
                } else {
                    line_result = String::new();
                }

                self.sample_rate = Some(new_sample_rate);
            }
            if !line_result.is_empty() {
                result.push(line_result);
            }
        }
        result
    }
}

impl YoshimiErrFilter {
    pub fn new() -> YoshimiErrFilter {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(
            SOCKET_RULE_NAME,
            r"Cannot read socket fd = (\d+) err = Interrupted system call",
        );

        YoshimiErrFilter {
            sample_rate: None,
            xruns: 0,
            xrun_time: None,
            instrument: vec![],
            filter_rules,
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_process_text() {
        let mut yoshimi_filter = YoshimiErrFilter::new();
        assert!(yoshimi_filter
            .filter_rules
            .rules
            .keys()
            .any(|x| x == SOCKET_RULE_NAME));
        let result = yoshimi_filter.process_text("Samplerate: 48000");
        assert!(result.is_empty());
        assert!(yoshimi_filter.sample_rate == Some(48000));
        let result = yoshimi_filter.process_text("Samplerate: 48001");
        assert!(!result.is_empty());
        assert!(yoshimi_filter.sample_rate == Some(48001));
        let result = yoshimi_filter.process_text(
            r"Instrument file /home/patch/120Proof/Instruments/xiz/Hammond Organ.xiz loaded",
        );
        assert!(result.is_empty());
        assert!(!yoshimi_filter.instrument.is_empty());

        let result = yoshimi_filter.process_text("Yay! We're up and running :-)");
        assert!(!result.is_empty());
    }
}
