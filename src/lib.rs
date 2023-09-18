//! This library provides the `TimestampFinder` type for finding timestamps in a string or stream of bytes.
//!
//! ## Examples
//!
//! Find a timestamp in a string:
//!
//! ```
//! use timescan::TimestampFinder;
//! let date_finder = TimestampFinder::new().unwrap();
//! let log = "Nov 23 06:26:40 ip-10-1-26-81 haproxy[20128]: 10.1.1.105:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"";
//! let timestamp = date_finder.find_timestamp(log).unwrap();
//! ```
//!
//! Find all timestamps by consuming a reader:
//!
//! ```
//! use timescan::TimestampFinder;
//! let log = "Nov 23 06:26:40 ip-10-1-1-1 haproxy[20128]: 10.1.1.10:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"
//! Nov 23 06:26:41 ip-10-1-1-1 haproxy[20128]: 10.1.1.11:51819 [23/Nov/2019:06:27:41.780] public myapp/i-059c225b48702964a 0/0/0/80/80 200 802/142190 - - ---- 8/8/5/0/0 0/0 {} {||141752|} \"GET /2043f2eb9e2691edcc0c8084d1ffce8bd70bc6e7 HTTP/1.1\"
//! Nov 23 06:26:42 ip-10-1-1-1 haproxy[20128]: 10.1.1.12:38870 [23/Nov/2019:06:28:42.773] public myapp/i-048088fd46abe7ed0 0/0/0/77/100 200 823/512174 - - ---- 8/8/5/0/0 0/0 {} {||511736|} \"GET /eb59c0b5dad36f080f3d261c6257ce0e21ef1a01 HTTP/1.1\"
//! ";
//! let date_finder = TimestampFinder::new().unwrap();
//! let timestamps = date_finder.scan(log.as_bytes()).unwrap();
//! ```
//!
//! ## How it works
//!
//! timescan converts a time format string like `%d/%b/%Y:%H:%M:%S%.f` into a regular expression
//! that can efficiently locate timestamps in strings. It then converts those matched substrings
//! into unix timestamps (integers) and returns them to you.

mod timestamp_finder;

pub use crate::timestamp_finder::TimestampFinder;

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn timestamp_finder() -> Result<()> {
        let date_finder = TimestampFinder::new()?;
        let log = "Nov 23 06:26:40 ip-10-1-26-81 haproxy[20128]: 54.242.135.245:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"";
        let timestamp = date_finder.find_timestamp(log).unwrap();
        assert_eq!(timestamp, 1574490400);

        let format = "%d/%b/%Y:%H:%M:%S%.f";
        let date_finder = TimestampFinder::new_with_format(format).unwrap();
        let log = "Nov 23 06:26:40 ip-10-1-26-81 haproxy[20128]: 54.242.135.245:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"";
        let timestamp = date_finder.find_timestamp(log).unwrap();
        assert_eq!(timestamp, 1574490400);
        Ok(())
    }

    #[test]
    fn scan_for_timestamps() -> Result<()> {
        let log = "Nov 23 06:26:40 ip-10-1-1-1 haproxy[20128]: 10.1.1.10:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET \
        /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"
Nov 23 06:26:41 ip-10-1-1-1 haproxy[20128]: 10.1.1.11:51819 [23/Nov/2019:06:27:41.780] public myapp/i-059c225b48702964a 0/0/0/80/80 200 802/142190 - - ---- 8/8/5/0/0 0/0 {} {||141752|} \"GET /2043f2eb9e2691edcc0c8084d1ff\
ce8bd70bc6e7 HTTP/1.1\"
Nov 23 06:26:42 ip-10-1-1-1 haproxy[20128]: 10.1.1.12:38870 [23/Nov/2019:06:28:42.773] public myapp/i-048088fd46abe7ed0 0/0/0/77/100 200 823/512174 - - ---- 8/8/5/0/0 0/0 {} {||511736|} \"GET /eb59c0b5dad36f080f3d261c625\
7ce0e21ef1a01 HTTP/1.1\"
Nov 23 06:26:43 ip-10-1-1-1 haproxy[20128]: 10.1.1.13:35528 [23/Nov/2019:06:29:43.775] public myapp/i-05e9315b035d50f62 0/0/0/103/105 200 869/431481 - - ---- 8/8/1/0/0 0/0 {} {|||} \"GET /164672c9d75c76a8fa237c24f9cbfd22\
22554f6d HTTP/1.1\"
Nov 23 06:26:44 ip-10-1-1-1 haproxy[20128]: 10.1.1.14:48553 [23/Nov/2019:06:30:44.808] public myapp/i-0008bfe6b1c98e964 0/0/0/72/73 200 840/265518 - - ---- 7/7/5/0/0 0/0 {} {||265080|} \"GET /e3b526928196d19ab3419d433f3d\
e0ceb71e62b5 HTTP/1.1\"
Nov 23 06:26:45 ip-10-1-1-1 haproxy[20128]: 10.1.1.15:60969 [23/Nov/2019:06:31:45.727] public myapp/i-005a2bfdba4c405a8 0/0/0/146/167 200 852/304622 - - ---- 7/7/5/0/0 0/0 {} {||304184|} \"GET /52f5edb4a46276defe54ead2fa\
e3a19fb8cafdb6 HTTP/1.1\"
Nov 23 06:26:46 ip-10-1-1-1 haproxy[20128]: 10.1.1.14:48539 [23/Nov/2019:06:32:46.730] public myapp/i-03b180605be4fa176 0/0/0/171/171 200 889/124142 - - ---- 6/6/4/0/0 0/0 {} {||123704|} \"GET /ef9e0c85cc1c76d7dc777f5b19\
d7cb85478496e4 HTTP/1.1\"
Nov 23 06:26:47 ip-10-1-1-1 haproxy[20128]: 10.1.1.11:51847 [23/Nov/2019:06:33:47.886] public myapp/i-0aa566420409956d6 0/0/0/28/28 206 867/458 - - ---- 6/6/4/0/0 0/0 {bytes=0-0} {} \"GET /3c7ace8c683adcad375a4d14995734a\
c0db08bb3 HTTP/1.1\"
Nov 23 06:26:48 ip-10-1-1-1 haproxy[20128]: 10.1.1.13:35554 [23/Nov/2019:06:34:48.866] public myapp/i-07f4205f35b4774b6 0/0/0/23/49 200 816/319662 - - ---- 5/5/3/0/0 0/0 {} {||319224|} \"GET /b95db0578977cd32658fa28b386c\
0db67ab23ee7 HTTP/1.1\"
Nov 23 06:26:49 ip-10-1-1-1 haproxy[20128]: 10.1.1.12:38899 [23/Nov/2019:06:35:49.879] public myapp/i-08cb5309afd22e8c0 0/0/0/59/59 200 1000/112110 - - ---- 5/5/3/0/0 0/0 {} {||111672|} \"GET /5314ca870ed0f5e48a71adca185\
e4ff7f1d9d80f HTTP/1.1\"
";
        let date_finder = TimestampFinder::new().unwrap();
        let timestamps = date_finder.scan(log.as_bytes()).unwrap();
        assert_eq!(timestamps.len(), 10);
        assert_eq!(
            timestamps,
            vec![
                1574490400, 1574490461, 1574490522, 1574490583, 1574490644, 1574490705, 1574490766,
                1574490827, 1574490888, 1574490949
            ]
        );
        Ok(())
    }
}
