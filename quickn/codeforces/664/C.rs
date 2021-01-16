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

const MAX: i32 = 1 << 9;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut a: Vec<i32> = vec![0;n];
    let mut b: Vec<i32> = vec![0;m];
    for i in 0..n {
        a[i] = scan.token();
    }
    for i in 0..m {
        b[i] = scan.token();
    }
    let mut res = 0;
    for target in 0..MAX {
        let mut sat = 0;
        for i in 0..n {
            let mut sub_res = MAX;
            for j in 0..m {
                if ((a[i]&b[j]) | target) == target {
                    sub_res = min(sub_res, a[i]&b[j]);
                }
            }
            if sub_res < MAX {
                sat |= sub_res;
            } else {
                sat = MAX;
                break;
            }
        }
        if sat == target {
            res = target;
            break;
        }
    }
    writeln!(sout, "{}", res).ok();
}
