# timescan

A Rust crate for finding timestamps in a string or stream of bytes

## Examples

Find a timestamp in a string:

```
use timescan::TimestampFinder;
let date_finder = TimestampFinder::new().unwrap();
let log = "Nov 23 06:26:40 ip-10-1-26-81 haproxy[20128]: 10.1.1.105:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"";
let timestamp = date_finder.find_timestamp(log).unwrap();
```

Find all timestamps by consuming a reader:

```
use timescan::TimestampFinder;
let log = "Nov 23 06:26:40 ip-10-1-1-1 haproxy[20128]: 10.1.1.10:57305 [23/Nov/2019:06:26:40.781] public myapp/i-05fa49c0e7db8c328 0/0/0/78/78 206 913/458 - - ---- 9/9/6/0/0 0/0 {bytes=0-0} {||1|bytes 0-0/499704} \"GET /2518cb13a48bdf53b2f936f44e7042a3cc7baa06 HTTP/1.1\"
Nov 23 06:26:41 ip-10-1-1-1 haproxy[20128]: 10.1.1.11:51819 [23/Nov/2019:06:27:41.780] public myapp/i-059c225b48702964a 0/0/0/80/80 200 802/142190 - - ---- 8/8/5/0/0 0/0 {} {||141752|} \"GET /2043f2eb9e2691edcc0c8084d1ffce8bd70bc6e7 HTTP/1.1\"
Nov 23 06:26:42 ip-10-1-1-1 haproxy[20128]: 10.1.1.12:38870 [23/Nov/2019:06:28:42.773] public myapp/i-048088fd46abe7ed0 0/0/0/77/100 200 823/512174 - - ---- 8/8/5/0/0 0/0 {} {||511736|} \"GET /eb59c0b5dad36f080f3d261c6257ce0e21ef1a01 HTTP/1.1\"
";
let date_finder = TimestampFinder::new().unwrap();
let timestamps = date_finder.scan(log.as_bytes()).unwrap();
```

## How it works

timescan converts a time format string like `%d/%b/%Y:%H:%M:%S%.f` into a regular expression that can efficiently locate timestamps in strings. It then converts those matched substrings into unix timestamps (integers) and returns them to you.
