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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, mut p, k): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
        let s: String = scan.token();
        let arr: Vec<char> = s.chars().collect();
        let (x, y): (i64, i64) = (scan.token(), scan.token());
        let mut v: Vec<i64> = vec![];
        p -= 1;
        for st in 0..k {
            let mut i = st+p;
            let mut cnt = 0;
            while i < n {
                if arr[i] == '0' {
                    cnt += 1;
                }
                i += k;
            }
            v.push(cnt);
        }
        let mut cost = std::i64::MAX;
        let mut prefix = 0;
        use std::cmp::min;
        for st in 0..(n-p) {
            cost = min(cost, prefix + x*v[st%k]);
            prefix += y;
            if arr[st+p] == '0' {
                v[st%k] -= 1;
            }
        }
        writeln!(sout, "{}", cost).ok();
    }
}
