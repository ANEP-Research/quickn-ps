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

use std::collections::VecDeque;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let mut k: Vec<usize> = vec![0;n];
        let mut c: Vec<i64> = vec![0;m];
        for i in 0..n {
            k[i] = scan.token();
            k[i] -= 1;
        }
        for i in 0..m {
            c[i] = scan.token();
        }
        k.sort();
        k.reverse();
        let mut deque: VecDeque<(i64, usize)> = VecDeque::new();
        for i in 0..=k[0] {
            deque.push_back((c[i], i));
        }
        let mut res = 0;
        for i in 0..n {
            if let Some(&(val, idx)) = deque.front() {
                if idx <= k[i] && c[k[i]] > val {
                    deque.pop_front();
                    res += val;
                } else {
                    res += c[k[i]];
                }
            } else {
                res += c[k[i]];
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
