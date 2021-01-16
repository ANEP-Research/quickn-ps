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
    let price = s2vec(scan.token());
    let mut res: i64 = 0;
    let mut sum1: Vec<i64> = vec![0;price.len()+1];
    let mut sum2: Vec<i64> = vec![0;price.len()+1];
    for i in 0..=price.len() {
        if i > 0 {
        sum1[i] = sum1[i-1];
        sum2[i] = sum2[i-1];
        }
        sum1[i] += pow_mod(10, i);
        sum1[i] %= MOD;
        sum2[i] += (pow_mod(10, i) * (i as i64)) % MOD;
        sum2[i] %= MOD;
    }
    let mut prev = 0;
    for i in 0..price.len() {
        let a = (price[i] as u8) - ('0' as u8);
        let cur0 = ((a as i64)*pow_mod(10, price.len() - i - 1))%MOD;
        res += prev*cur0;
        res %= MOD;
        let r = price.len()-i-1;
        if r > 0 {
            res += ((a as i64)*(sum2[r-1] + sum1[r-1]))%MOD;
            res %= MOD;
        }
        prev += (i+1) as i64;
        prev %= MOD;
    }
    writeln!(sout, "{}", res).ok();
}
