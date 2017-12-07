extern crate minecraft_logtail;

use std::env;
use std::path::Path;

use minecraft_logtail::LogTail;

fn main() {
    let log_path = env::args().nth(1).expect("missing log file argument");
    for line in LogTail::from(Path::new(&log_path)) {
        println!("{}", line.expect("failed to get log line"));
    }
}
