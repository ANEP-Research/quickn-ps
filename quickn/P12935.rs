/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
    /* https://github.com/EbTech/rust-algorithms */

    /// Same API as Scanner but nearly twice as fast, using horribly unsafe dark arts
    /// **REQUIRES** Rust 1.34 or higher
    pub struct UnsafeScanner<R> {
        reader: R,
        buf_str: Vec<u8>,
        buf_iter: str::SplitAsciiWhitespace<'static>,
    }

    impl<R: io::BufRead> UnsafeScanner<R> {
        pub fn new(reader: R) -> Self {
            Self {
                reader,
                buf_str: Vec::new(),
                buf_iter: "".split_ascii_whitespace(),
            }
        }

        /// This function should be marked unsafe, but noone has time for that in a
        /// programming contest. Use at your own risk!
        pub fn token<T: str::FromStr>(&mut self) -> T {
            loop {
                if let Some(token) = self.buf_iter.next() {
                    return token.parse().ok().expect("Failed parse");
                }
                self.buf_str.clear();
                self.reader
                    .read_until(b'\n', &mut self.buf_str)
                    .expect("Failed read");
                self.buf_iter = unsafe {
                    let slice = str::from_utf8_unchecked(&self.buf_str);
                    std::mem::transmute(slice.split_ascii_whitespace())
                }
            }
        }
    }
}

use std::cmp::{min, max};

fn s2vec(s: String) -> Vec<char> {
    s.chars().collect()
}

const LIMIT: usize = 100;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let s: usize = scan.token();
    if s >= LIMIT {
        let k: usize = (s/LIMIT)+1;
        let mut cnt = 2;
        writeln!(sout, "{}", 2 + LIMIT - 1 + 1 + 1 + (s%LIMIT) + (k-2)).ok();
        writeln!(sout, "0 1").ok();
        for i in 0..(LIMIT-1) {
            writeln!(sout, "1 {}", cnt).ok();
            cnt += 1;
        }
        writeln!(sout, "0 {}", cnt).ok();
        cnt += 1;
        writeln!(sout, "{} {}", cnt-1, cnt).ok();
        cnt += 1;
        for i in 0..(s%LIMIT) {
            writeln!(sout, "{} {}", cnt-1, cnt).ok();
            cnt += 1;
        }
        for i in 0..(k-2) {
            writeln!(sout, "{} {}", 0, i+cnt).ok();
        }
    } else {
        writeln!(sout, "{}", s+3).ok();
        for i in 0..(s+2) {
            writeln!(sout, "{} {}", i, i+1).ok();
        }
    }
}
