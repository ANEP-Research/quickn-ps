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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, m): (usize, usize) = (scan.token(), scan.token());
    let mut a: Vec<i32> = vec![0;n+1];
    for i in 1..=n {
        a[i] = scan.token();
    }
    let mut prefix_sum: Vec<i32> = vec![0;n+1];
    for i in 1..=n {
        prefix_sum[i] = prefix_sum[i-1] + a[i];
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![std::i32::MAX;m+1];n+1];
    let mut pi: Vec<Vec<usize>> = vec![vec![0;m+1];n+1];
    use std::cmp::{max, min};
    dp[0][0] = 0;
    for i in 1..=n {
        for j in 1..=min(m, i) {
            for k in (j-1)..i {
                let t = prefix_sum[i] - prefix_sum[k];
                let u = max(dp[k][j-1], t);
                if dp[i][j] > u {
                    pi[i][j] = k;
                    dp[i][j] = u;
                }
            }
        }
    }
    writeln!(sout, "{}", dp[n][m]).ok();
    let mut len: Vec<usize> = vec![];
    let (mut n_t, mut m_t) = (n, m);
    while n_t != 0 {
        len.push(n_t - pi[n_t][m_t]);
        n_t = pi[n_t][m_t];
        m_t -= 1;
    }
    len.reverse();
    for l in len {
        write!(sout, "{} ", l).ok();
    }
}
