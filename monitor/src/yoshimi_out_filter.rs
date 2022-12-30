use crate::file_filter::FileFilter;
use crate::filter_rules::FilterRules;
use std::env;
use time::OffsetDateTime;
const SECONDS_XRUN_REPORT: i64 = 60;

/// Rule names
const SAMPLE_RATE_RULE_NAME: &str = "sample_rate_rule";
const INSTRUMENT_RULE_NAME: &str = "instrument_rule_name";
const YAY_RUNNING_RULE_NAME: &str = "yay_running_rul_name";

/// Maintain the knowledge about the file
#[derive(Debug)]
pub struct YoshimiOutFilter {
    pub sample_rate: Option<usize>,
    pub xruns: usize, // Count how many xruns
    pub xrun_time: Option<OffsetDateTime>,
    pub instrument: Vec<String>,
    filter_rules: FilterRules,
}
impl FileFilter for YoshimiOutFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        // Value to return
        println!("Processing text: {}", input);
        // Break into lines
        let lines: Vec<&str> = input.split('\n').collect();

        let mut result: Vec<String> = vec![];
        for line in lines.iter() {
            let mut line_result: String = String::new();
            if *line == "xrun reported" {
                self.xruns += 1;
                let now = OffsetDateTime::now_utc();
                let xrun_last_time = self.xrun_time.unwrap_or(now);
                // The time since last report of xrun
                let seconds_last_xrun_report =
                    now.unix_timestamp() - xrun_last_time.unix_timestamp();
                if seconds_last_xrun_report >= SECONDS_XRUN_REPORT {
                    // Report xruns and reset counters
                    line_result = if self.xruns > 0 {
                        self.xruns = 0;
                        self.xrun_time = Some(now);
                        format!(
                            "{} xruns. {} per minute\n",
                            self.xruns,
                            self.xruns as f64 / (seconds_last_xrun_report * 60) as f64
                        )
                    } else {
                        "".to_string()
                    };
                }
            } else if let Some(caps) = self.filter_rules.evaluate(SAMPLE_RATE_RULE_NAME, line) {
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
            } else if self
                .filter_rules
                .evaluate(YAY_RUNNING_RULE_NAME, line)
                .is_some()
            {
                // Yoshimi is set up
                line_result = format!(
                    "Yoshimi up: Sample rate: {} Instruments: {}",
                    if self.sample_rate.is_some() {
                        format!("{}", self.sample_rate.unwrap())
                    } else {
                        "<None>".to_string()
                    },
                    self.instrument.join(", ")
                );
            } else if let Some(caps) = self.filter_rules.evaluate(INSTRUMENT_RULE_NAME, line) {
                self.instrument
                    .push(caps.get(1).unwrap().as_str().to_string());

                line_result = String::new();
            }
            if !line_result.is_empty() {
                result.push(line_result);
            }
        }
        result
    }
}

impl YoshimiOutFilter {
    pub fn new() -> YoshimiOutFilter {
        let mut filter_rules = FilterRules::new();
        filter_rules.add_rule(SAMPLE_RATE_RULE_NAME, r"Samplerate: (\d+)");
        filter_rules.add_rule(YAY_RUNNING_RULE_NAME, r"^Yay! We're up and running :-\)$");
        filter_rules.add_rule(
            INSTRUMENT_RULE_NAME,
            format!(
                r"Instrument file {}/Instruments/xiz/(.+)\.xiz loaded",
                env::var("Home120Proof").unwrap()
            )
            .as_str(),
        );
        YoshimiOutFilter {
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
        let mut yoshimi_filter = YoshimiOutFilter::new();
        assert!(yoshimi_filter
            .filter_rules
            .rules
            .keys()
            .any(|x| x == SAMPLE_RATE_RULE_NAME));
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
