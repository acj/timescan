use anyhow::Result;
use chrono::NaiveDateTime;
use regex::Regex;
use std::io::{prelude::*, BufReader};

/// Finds timestamps in strings based on a configurable format
pub struct TimestampFinder {
    datetime_format: String,
    regex: Regex,
}

impl TimestampFinder {
    /// Creates a new TimestampFinder, assuming timestamps are in the
    /// [Common Log Format (CLF)](https://httpd.apache.org/docs/1.3/logs.html#common), which looks
    /// like this: "02/Jan/2006:15:04:05.000". Timezone offset is ignored.
    pub fn new() -> Result<Self> {
        Self::new_with_format("%d/%b/%Y:%H:%M:%S%.f")
    }

    /// Creates a new TimestampFinder, given a format.
    ///
    /// The format must use format specifiers that are recognized by
    /// [strftime](https://docs.rs/chrono/0.4.13/chrono/format/strftime/index.html).
    ///
    /// ### Currently supported specifiers
    ///
    /// | Specifier | Meaning |
    /// | --------- | ------- |
    /// | %Y        | The full proleptic Gregorian year, zero-padded to 4 digits. |
    /// | %C        | The proleptic Gregorian year divided by 100, zero-padded to 2 digits. |
    /// | %y        | The proleptic Gregorian year modulo 100, zero-padded to 2 digits. |
    /// | %m        | Month number (01--12), zero-padded to 2 digits. |
    /// | %b        | Abbreviated month name. Always 3 letters. |
    /// | %B        | Full month name. Also accepts corresponding abbreviation in parsing. |
    /// | %h        | Same as %b. |
    /// | %d        | Day number (01--31), zero-padded to 2 digits. |
    /// | %H        | Hour number (00--23), zero-padded to 2 digits. |
    /// | %M        | Minute number (00--59), zero-padded to 2 digits. |
    /// | %S        | Second number (00--60), zero-padded to 2 digits. |
    /// | %.f       | Similar to .%f but left-aligned. These all consume the leading dot. |
    /// | %s        | UNIX timestamp. Seconds since 1970-01-01 00:00 UTC. |
    pub fn new_with_format(datetime_format: &str) -> Result<Self, anyhow::Error> {
        let datetime_regex = Self::strftime_to_regex(datetime_format);
        let regex = Regex::new(&datetime_regex)?;

        Ok(TimestampFinder {
            datetime_format: datetime_format.to_string(),
            regex,
        })
    }

    /// Finds a timestamp in a string, returning it as a unix timestamp
    pub fn find_timestamp(&self, s: &str) -> Option<i64> {
        let regex_match = self.regex.captures(s)?.get(0)?;
        let datetime =
            NaiveDateTime::parse_from_str(regex_match.as_str(), &self.datetime_format).ok()?;
        Some(datetime.timestamp())
    }

    /// Scans a reader for times matching the given format, returning them as a vector of unix timestamps
    pub fn scan<R>(&self, reader: R) -> Result<Vec<i64>>
    where
        R: Read,
    {
        let timestamps = BufReader::new(reader)
            .lines()
            .map_while(Result::ok)
            .filter_map(|line| self.find_timestamp(&line))
            .collect();
        Ok(timestamps)
    }

    fn strftime_to_regex(time_format: &str) -> String {
        time_format
            .replace("%Y", r"\d{1,4}")
            .replace("%C", r"\d{1,2}")
            .replace("%y", r"\d{1,2")
            .replace("%m", r"\d{1,2}")
            .replace("%b", r"[A-Za-z]{3}")
            .replace("%B", r"[A-Za-z]{3,4,5,6,7,8,9}")
            .replace("%h", r"[A-Za-z]{3}")
            .replace("%d", r"\d{1,2}")
            .replace("%H", r"\d{1,2}")
            .replace("%M", r"\d{1,2}")
            .replace("%S", r"\d{1,2}")
            .replace("%.f", r"\d{1,}")
            .replace("%s", r"\d{1,10}")
        // TODO: Add support for remaining characters. https://docs.rs/chrono/0.4.13/chrono/format/strftime/index.html
    }
}

#[test]
fn timestamp_finder_strftime_to_regex() {
    let convert_compile_match = |format: &str, match_str: &str| {
        let format_regex = TimestampFinder::strftime_to_regex(format);
        let regex = Regex::new(&format_regex).unwrap();
        assert!(regex.is_match(match_str));
    };

    convert_compile_match("%d/%b/%Y:%H:%M:%S%.f", "06/Jan/2006:13:04:05.000");
}

#[test]
fn timestamp_finder_epochseconds() {
    let format = "%s";
    let date_finder = TimestampFinder::new_with_format(format).unwrap();

    // Full 10 digit recent epoch
    let log = "1621568291 ip-10-1-26-81 haproxy[20128]: 54.242.135...";
    let timestamp = date_finder.find_timestamp(log).unwrap();
    assert_eq!(timestamp, 1621568291);

    // Shorter timestamp (15th Jan 1970)
    let log = "1234567 ip-10-1-26-81 haproxy[20128]: 54.242.135...";
    let timestamp = date_finder.find_timestamp(log).unwrap();
    assert_eq!(timestamp, 1234567);
}
