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
        let (mut r, mut g, mut b, mut w): (i32, i32, i32, i32) = (scan.token(), scan.token(), scan.token(), scan.token());
        let mut can = false;
        let mut cnt = 0;
        while r >= 0 &&  g >= 0 && b >= 0 && cnt <= 10 {
        let mut odds = 0;
        let mut evens = 0;
        if r % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
        if g % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
        if b % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
        if w % 2 == 0 {
            evens += 1;
        } else {
            odds += 1;
        }
        if evens == 3 && odds == 1 || evens == 4 {
            can = true;
        }
        r -= 1;
        g -= 1;
        b -= 1;
        w += 3;
        cnt += 1;
        }
        if can {
            writeln!(sout, "Yes").ok();
        } else {
            writeln!(sout, "No").ok();
        }
    }
}
