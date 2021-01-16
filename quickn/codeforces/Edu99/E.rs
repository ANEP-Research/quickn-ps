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

const MAX: i64 = 2_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let mut p: [(i64, i64);4] = [(0, 0);4];
        let (mut x0, mut y0) = (0, 0);
        for i in 0..4 {
            p[i] = (scan.token(), scan.token());
            x0 += p[i].0;
            y0 += p[i].1;
        }
        x0 /= 4;
        y0 /= 4;
        let (mut l, mut r) = (0, MAX);
        use std::cmp::min;
        let mut f = |x: i64| -> i64 {
            let (l1, l2) = (x/2, (x+1)/2);
            let mut res = std::i64::MAX;
            let a = [(x0-l1, y0-l1), (x0-l1, y0+l2), (x0+l2, y0+l2), (x0+l2, y0-l1)];
            for i1 in 0..4 {
                for i2 in 0..4 {
                    for i3 in 0..4 {
                        for i4 in 0..4 {
                            let mut sub_res = 0;
                            let mut b = [i1, i2, i3, i4];
                            b.sort();
                            if b == [0, 1, 2, 3] {
                                let c = [i1, i2, i3, i4];
                                for i in 0..4 {
                                    sub_res += (p[c[i]].0-a[i].0).abs() + (p[c[i]].1-a[i].1).abs();
                                }
                            }
                            res = min(res, sub_res);
                        }
                    }
                }
            }
            res
        };
        while l < r {
            let mid = (l+r)/2;
            
        }
    }
}
