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

fn calculate_pi(p: Vec<char>) -> Vec<u32> {
    let mut pi: Vec<u32> = vec![0;p.len()+1];
    let mut cur = 0;
    for i in 1..p.len() {
        while cur > 0 && p[cur as usize] != p[i] {
            cur = pi[cur as usize];
        }
        if p[cur as usize] == p[i] {
            cur += 1;
        }
        pi[i+1] = cur;
    }
    pi
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let s_p: String = scan.token();
    let s: Vec<char> = s_p.chars().collect();
    let n: usize = scan.token();
    let mut p: Vec<Vec<char>> = vec![vec![];n];
    for i in 0..n {
        let p_i: String = scan.token();
        p[i] = p_i.chars().collect();
    }
    let mut pi: Vec<Vec<bool>> = vec![vec![false;s.len()];n];
    for i in 0..n {
        let pi2 = calculate_pi(p[i].clone());
        //dbg!(pi2.clone());
        let mut cur = 0;
        for j in 0..s.len() {
            while cur > 0 && p[i][cur as usize] != s[j] {
                cur = pi2[cur as usize];
            }
            if p[i][cur as usize] == s[j] {
                cur += 1;
            }
            //dbg!(cur, s[j]);
            if cur as usize == p[i].len() {
                pi[i][j] = true;
                cur = pi2[cur as usize];
            }
        }
    }
    use std::cmp::max;
    let mut dp: Vec<u32> = vec![0;s.len()+1];
    let mut res = 0;
    for l in 1..=s.len() {
        dp[l] = dp[l-1];
        for i in 0..n {
            if pi[i][l-1] {
                dp[l] = max(dp[l-p[i].len()] + (p[i].len() as u32), dp[l]);
            }
        }
        res = max(dp[l], res);
    }
    writeln!(sout, "{}", res).ok();
}
