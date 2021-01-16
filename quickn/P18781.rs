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

const MOD: i64 = 1_000_000_007;

fn pow_mod(a: i64, x: usize) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t > 0 {
        if x_t % 2 == 1 {
            r *= a_t;
            r %= MOD;
        }
        a_t *= a_t;
        a_t %= MOD;
        x_t /= 2;
    }
    r
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut segs: Vec<(i64, i64)> = vec![];
    let mut axis: Vec<i32> = vec![0;2*n + 2];
    for i in 0..n {
        let (x, y): (i64, i64) = (scan.token(), scan.token());
        segs.push((x, y));
        axis[x as usize] += 1;
        axis[(y as usize) + 1] -= 1;
    }
    for i in 1..=(2*n) {
        axis[i] += axis[i-1];
    }
    let mut res = 0;
    for i in 0..n {
        res += pow_mod(2, n - (axis[segs[i].0 as usize] as usize));
        res %= MOD;
    }
    writeln!(sout, "{}", res).ok();
}
