use crate::file_filter::FileFilter;
use regex::Regex;
use time::OffsetDateTime;
const SECONDS_XRUN_REPORT: i64 = 60;
/// Maintain the knowledge about the file
pub struct YoshimiOutFilter {
    pub sample_rate: Option<usize>,
    pub xruns: usize, // Count how many xruns
    pub xrun_time: Option<OffsetDateTime>,
    sample_rate_re: Regex,
}
impl FileFilter for YoshimiOutFilter {
    fn process_text(&mut self, input: &str) -> Vec<String> {
        // Value to return

        // Break into lines
        let lines: Vec<&str> = input.split("\n").collect();

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
            } else if let Some(caps) = self.sample_rate_re.captures(line) {
                let sample_rate_text = caps.get(1).map_or("0", |m| m.as_str());
                self.sample_rate = Some(sample_rate_text.parse().unwrap());
                line_result = String::new();
            }
            result.push(line_result);
        }
        result
    }
}

impl YoshimiOutFilter {
    pub fn new() -> YoshimiOutFilter {
        YoshimiOutFilter {
            sample_rate: None,
            xruns: 0,
            xrun_time: None,
            sample_rate_re: Regex::new(r"Samplerate: (\d+)").unwrap(),
        }
    }
}
