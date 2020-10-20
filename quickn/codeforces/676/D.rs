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

use std::cmp::min;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (mut x, mut y): (i64, i64) = (scan.token(), scan.token());
        let mut c: [i64;6] = [0;6];
        for i in 0..6 {
            c[i] = scan.token();
        }
        for i in 0..6 {
            c[i] = min(c[i], c[(i + 1)%6] + c[(i+5)%6]);
        }
        let mut res = 0;
        if x >= 0 {
            if y <= 0 {
                res = c[5]*x.abs() + c[4]*y.abs();
            } else if x > y {
                res = c[0]*y.abs() + (x-y)*c[5];
            } else {
                res = c[0]*x.abs() + (y-x)*c[1];
            }
        } else {
            if y >= 0 {
                res = c[2]*x.abs() + c[1]*y.abs();
            } else if x > y {
                res = c[3]*x.abs() + c[4]*(x-y);
            } else {
                res = c[3]*y.abs() + (y-x)*c[2];
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
