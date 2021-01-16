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

const MOD: i64 = 1_000_000_007;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, x, pos): (usize, usize, usize) = (scan.token(), scan.token(), scan.token());
    let (mut l, mut r) = (0, n);
    let (mut c_l, mut c_r) = (0, 0);
    while l < r {
        let mid = (l + r) >> 1;
        //dbg!(l, r, mid);
        if mid <= pos {
            l = mid + 1;
            if mid != pos {
                c_l += 1;
            }
        } else {
            r = mid;
            //if l < r {
                c_r += 1;
            //}
        }
    }
    let mut comb: Vec<Vec<i64>> = vec![vec![0;n+1];n+1];
    comb[0][0] = 1;
    for i in 1..=n {
        comb[i][0] = 1;
        comb[i][n] = 1;
        for j in 1..n {
            comb[i][j] = (comb[i-1][j] + comb[i-1][j-1])%MOD;
        }
    }
    let mut fact: Vec<i64> = vec![0;n+1];
    fact[0] = 1;
    for i in 1..=n {
        fact[i] = (fact[i-1] * (i as i64)) % MOD;
    }
    //dbg!(c_l, c_r);
    //dbg!(comb[n-x][c_r], comb[x-1][c_l]);
    writeln!(sout, "{}", (((if c_r == 0 { 1 } else { (comb[n-x][c_r]*fact[c_r])%MOD }*if c_l == 0 { 1 } else { (comb[x-1][c_l]*fact[c_l])%MOD })%MOD)*fact[n-(c_r+c_l+1)])%MOD).ok();
}
