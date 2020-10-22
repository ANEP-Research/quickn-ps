/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
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

const MAX: i32 = 256;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, k): (usize, i32) = (scan.token(), scan.token());
    let mut cnt: Vec<Vec<Vec<i32>>> = vec![vec![vec![0;MAX as usize];MAX as usize];MAX as usize];
    for i in 0..n {
        let (a, b, c): (usize, usize, usize) = (scan.token(),scan.token(),scan.token());
        cnt[a][b][c] += 1;
    }
    let mut s: Vec<Vec<Vec<i32>>> = vec![vec![vec![0;MAX as usize];MAX as usize];MAX as usize];
    for a in 0..MAX {
        for b in 0..MAX {
            for c in 0..MAX {
                let f = |at: i32, bt: i32, ct: i32| -> i32 {
                    if at < 0 || bt < 0 || ct < 0 {
                        0
                    } else {
                        s[at as usize][bt as usize][ct as usize]
                    }
                };
                s[a as usize][b as usize][c as usize] = 
                cnt[a as usize][b as usize][c as usize]
                + f(a-1, b, c)
                + f(a, b-1, c)
                + f(a, b, c-1)
                - (f(a-1, b-1, c) + f(a-1, b, c-1) + f(a, b-1, c-1))
                + f(a-1, b-1, c-1);
            }
        }
    }
    let g = |a: i32, b: i32, c: i32, m: i32| {
        if a < 0 || b < 0 || c < 0 {
            0
        } else {
            let f = |at: i32, bt: i32, ct: i32| -> i32 {
                if at < 0 || bt < 0 || ct < 0 {
                    0
                } else {
                    s[at as usize][bt as usize][ct as usize]
                }
            };
            let (a2, b2, c2) = (a + m, b + m, c + m);
            f(a2, b2, c2)
            - f(a-1, b2, c2)
            - f(a2, b-1, c2)
            - f(a2, b2, c-1)
            + f(a-1, b-1, c2)
            + f(a-1, b2, c-1)
            + f(a2, b-1, c-1)
            - f(a-1, b-1, c-1)
        }
    };
    use std::cmp::min;
    let (mut h, mut ra, mut rb, mut rc) = (MAX, 0, 0, 0);
    for a in 0..MAX {
        for b in 0..MAX {
            for c in 0..MAX {
                let t = min(MAX - a, min(MAX - b, MAX - c));
                let (mut l, mut r) = (0, t - 1);
                while l < r {
                    let m = (l + r) >> 1;
                    if g(a, b, c, m) >= k {
                        r = m;
                    } else {
                        l = m + 1;
                    }
                }
                if g(a, b, c, l) >= k && h > l {
                    h = l;
                    ra = a;
                    rb = b;
                    rc = c;
                }
            }
        }
    }
    writeln!(sout, "{}", h).ok();
    let mut k_t = k;
    for a in ra..=(ra + h) {
        for b in rb..=(rb + h) {
            for c in rc..=(rc + h) {
                while k_t > 0 && cnt[a as usize][b as usize][c as usize] > 0 {
                    writeln!(sout, "{} {} {}", a, b, c).ok();
                    cnt[a as usize][b as usize][c as usize] -= 1;
                    k_t -= 1;
                }
            }
        }
    }
}

