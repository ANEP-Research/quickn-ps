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

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a_t, mut b_t) = (a, b);
    while b_t != 0 {
        a_t %= b_t;
        a_t ^= b_t;
        b_t ^= a_t;
        a_t ^= b_t;
    }
    a_t
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let q: usize = scan.token();
    for _case in 0..q {
        let s = s2vec(scan.token());
        let t = s2vec(scan.token());
        let new_len = s.len()*t.len() / (gcd(s.len() as i32, t.len() as i32) as usize);
        let mut new_s = String::new();
        let mut new_t = String::new();
        for i in 0..(new_len / s.len()) {
            for c in s.clone() {
                new_s.push(c);
            }
        }
        for i in 0..(new_len / t.len()) {
            for c in t.clone() {
                new_t.push(c);
            }
        }
        if new_s != new_t {
            writeln!(sout, "-1").ok();
        } else {
            writeln!(sout, "{}", new_s).ok();
        }
    }
}
