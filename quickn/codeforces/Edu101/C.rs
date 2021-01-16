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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, k): (usize, i32) = (scan.token(), scan.token());
        let mut h: Vec<i32> = vec![0;n];
        let mut hf: Vec<i32> = vec![0;n];
        for i in 0..n {
            h[i] = scan.token();
        }
        let mut failed = false;
        let mut prev = 0;
        for i in 0..n {
            let mut delta = 0;
            if i == 0 {
                if h[i] < h[i+1] {
                    if h[i+1] - h[i] > (k-1) {
                        delta += h[i+1] - h[i] - (k-1);
                    }
                }
            } else if i == n-1 {
                if h[i-1] > h[i] {
                    if h[i-1] - h[i] > (k-1) {
                        delta += h[i-1] - h[i] - (k-1);
                    }
                }
            } else {
                if h[i+1] > h[i] {
                    if h[i+1] - h[i] > (k-1) {
                        delta += h[i+1] - h[i] - (k-1);
                    }
                }
                if h[i-1] > h[i] {
                    if h[i-1] - h[i] > (k-1) {
                        delta += h[i-1] - h[i] - (k-1);
                    }
                }
            }
            if delta > 0 {
                if delta > (k-1) {
                    failed = true;
                }
                hf[prev] = delta;
                prev = i;
            }
        }
        let mut first = 0;
        for i in 0..n {
            if hf[i] != 0 {
                first = hf[i];
            } else {
                h[i] += first;
            }
        }
        for i in 0..(n-1) {
            if (h[i] - h[i+1]).abs() > (k-1) {
                failed = true;
                break;
            }
        }
        if failed {
            writeln!(sout, "NO").ok();
        } else {
            writeln!(sout, "YES").ok();
        }
    }
}
