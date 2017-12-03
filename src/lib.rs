//! This crate provides the `LogTail` type, which can be constructed from a path, and iterates over a Minecraft world's log file line by line, blocking until new lines are appended.

#![cfg_attr(test, deny(warnings))]
#![warn(trivial_casts)]
#![deny(missing_docs, unused)]
#![forbid(unused_extern_crates, unused_import_braces, unused_qualifications)]

use std::mem;
use std::io::{self, BufReader, SeekFrom};
use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

/// The log tail iterator. See the crate-level documentation for details.
pub struct LogTail<'a> {
    path: &'a Path,
    file: Option<BufReader<File>>,
    pos: u64
}

impl<'a> From<&'a Path> for LogTail<'a> {
    fn from(path: &'a Path) -> LogTail<'a> {
        LogTail {
            path,
            file: None,
            pos: 0
        }
    }
}

impl<'a> Iterator for LogTail<'a> {
    type Item = io::Result<String>;

    fn next(&mut self) -> Option<io::Result<String>> {
        // main calculation is in a function that returns Result, for cleaner code using try!
        let inner = &mut || -> io::Result<Option<String>> {
            if self.file.is_none() {
                let mut f = BufReader::new(File::open(self.path)?);
                self.pos = f.seek(SeekFrom::End(0))?;
                // seek back to last newline
                while self.pos > 0 {
                    self.pos = f.seek(SeekFrom::Current(-1))?;
                    let mut buf = [0];
                    f.read_exact(&mut buf)?;
                    if buf[0] == b'\n' {
                        break;
                    } else {
                        self.pos = f.seek(SeekFrom::Current(-1))?;
                    }
                }
                self.file = Some(f);
            }
            let mut buf = String::default();
            let mut f = mem::replace(&mut self.file, None).expect("failed to initialize Minecraft logtail");
            loop {
                //TODO watch for new file in logs archive
                self.pos += f.read_line(&mut buf)? as u64;
                if buf.chars().last().map_or(false, |c| c == '\n') {
                    buf.pop();
                    break;
                }
            }
            self.file = Some(f);
            Ok(Some(buf))
        };

        match inner() {
            Ok(Some(s)) => Some(Ok(s)),
            Ok(None) => None,
            Err(e) => Some(Err(e))
        }
    }
}
