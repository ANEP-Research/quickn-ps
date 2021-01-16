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

const MOD: i64 = 998244353;

fn pow_mod(a: i64, x: i64) -> i64 {
    let (mut r, mut a_t, mut x_t) = (1, a, x);
    while x_t != 0 {
        if (x_t & 1) == 1 {
            r *= a_t;
            r %= MOD;
        }
        a_t *= a_t;
        a_t %= MOD;
        x_t >>= 1;
    }
    r
}

fn mod_inv(a: i64) -> i64 {
    pow_mod(a, MOD-2)
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: i64 = scan.token();
    let mut dp: Vec<i64> = vec![0;(n+10) as usize];
    let mut inv2 = mod_inv(2);
    dp[0] = 1;
    dp[1] = inv2;
    dp[2] = (dp[1]*inv2)%MOD;
    for i in 3..=(n as usize) {
        dp[i] = (((dp[i-1]*inv2)%MOD) + ((dp[i-2]*pow_mod(inv2, 2))%MOD))%MOD;
    }
    writeln!(sout, "{}", dp[n as usize]).ok();
}
