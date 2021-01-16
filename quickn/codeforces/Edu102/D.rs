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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let s = s2vec(scan.token());
        let mut x = 0;
        let mut arr: Vec<i32> = vec![0;n+1];
        let mut arr2: Vec<i32> = vec![0;n+1];
        let mut prefixm: Vec<i32> = vec![0;n+2];
        let mut prefixmm: Vec<i32> = vec![0;n+2];
        let mut suffixm: Vec<i32> = vec![0;n+2];
        let mut suffixmm: Vec<i32> = vec![0;n+2];
        for i in 0..n {
            if s[i] == '-' {
                x -= 1;
            } else {
                x += 1;
            }
            arr[i+1] = x;
            prefixm[i+1] = prefixm[i];
            prefixmm[i+1] = prefixmm[i];
            prefixm[i+1] = min(prefixm[i+1], x);
            prefixmm[i+1] = max(prefixmm[i+1], x);
        }
        x = 0;
        for _i in 0..n {
            let i = n - _i;
            if s[i-1] == '-' {
                x += 1;
            } else {
                x -= 1;
            }
            arr2[i] = x;
            suffixm[i] = suffixm[i+1];
            suffixmm[i] = suffixmm[i+1];
            suffixm[i] = min(suffixm[i], x);
            suffixmm[i] = max(suffixmm[i], x);
        }
        for i in 0..m {
            let (l, r): (usize, usize) = (scan.token(), scan.token());
            writeln!(sout, "{}", max(prefixmm[l-1], suffixmm[r+1] + arr[l-1] - arr2[r]) - min(prefixm[l-1], suffixm[r+1] + arr[l-1] - arr2[r]) + 1).ok();
        }
    }
}
