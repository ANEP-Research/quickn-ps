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

const MOD: i32 = 1_000_000_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, b): (usize, usize) = (scan.token(), scan.token());
    let mut dp: Vec<Vec<[[i32;2];2]>> = vec![vec![[[0;2];2];b];2];
    for i in 1..=n {
        for j in 0..b {
            dp[i%2][j] = [[0;2];2];
            if i == 1 {
                if j == 0 {
                    dp[i%2][j][1][0] = 1;
                } else if j == b-1 {
                    dp[i%2][j][0][1] = 1;
                } else {
                    dp[i%2][j][0][0] = 1;
                }
            } else {
                if j < b-1 {
                    if j == 0 {
                        dp[i%2][j][1][0] += dp[(i-1)%2][j+1][1][0];
                        dp[i%2][j][1][1] += dp[(i-1)%2][j+1][0][1];
                        dp[i%2][j][1][0] += dp[(i-1)%2][j+1][0][0];
                        dp[i%2][j][1][1] += dp[(i-1)%2][j+1][1][1];
                    } else {
                        dp[i%2][j][0][0] += dp[(i-1)%2][j+1][0][0];
                        dp[i%2][j][1][0] += dp[(i-1)%2][j+1][1][0];
                        dp[i%2][j][0][1] += dp[(i-1)%2][j+1][0][1];
                        dp[i%2][j][1][1] += dp[(i-1)%2][j+1][1][1];
                    }
                }
                if j > 0 {
                    if j == b-1 {
                        dp[i%2][j][1][1] += dp[(i-1)%2][j-1][1][0];
                        dp[i%2][j][0][1] += dp[(i-1)%2][j-1][0][0];
                        dp[i%2][j][0][1] += dp[(i-1)%2][j-1][0][1];
                        dp[i%2][j][1][1] += dp[(i-1)%2][j-1][1][1];
                    } else {
                        dp[i%2][j][0][0] += dp[(i-1)%2][j-1][0][0];
                        dp[i%2][j][1][0] += dp[(i-1)%2][j-1][1][0];
                        dp[i%2][j][0][1] += dp[(i-1)%2][j-1][0][1];
                        dp[i%2][j][1][1] += dp[(i-1)%2][j-1][1][1];
                    }
                }
            }
            dp[i%2][j][0][0] %= MOD;
            dp[i%2][j][1][0] %= MOD;
            dp[i%2][j][0][1] %= MOD;
            dp[i%2][j][1][1] %= MOD;
        }
    }
    let mut res = 0;
    for i in 1..b {
        res += dp[n%2][i][1][1];
        res %= MOD;
    }
    writeln!(sout, "{}", res).ok();
}
