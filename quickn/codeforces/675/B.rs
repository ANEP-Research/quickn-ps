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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let mut arr: Vec<Vec<i64>> = vec![vec![0;m];n];
        let mut arr2: Vec<Vec<i64>> = vec![vec![0;m];n];
        for i in 0..n {
            for j in 0..m {
                arr[i][j] = scan.token();
            }
        }
        let mut res = 0;
        for i in 0..n {
            for j in 0..m {
                let mut sub_task = std::i64::MAX;
                if (arr[i][j] - arr[n-i-1][j]).abs() + (arr[i][j] - arr[i][m-j-1]).abs() < sub_task {
                    arr2[i][j] = arr[i][j];
                    arr2[n-i-1][j] = arr[i][j];
                    arr2[i][m-j-1] = arr[i][j];
                    sub_task = (arr[i][j] - arr[n-i-1][j]).abs() + (arr[i][j] - arr[i][m-j-1]).abs();
                }
                if (arr[i][j] - arr[n-i-1][j]).abs() + (arr[n-i-1][j] - arr[i][m-j-1]).abs() < sub_task {
                    arr2[i][j] = arr[n-i-1][j];
                    arr2[n-i-1][j] = arr[n-i-1][j];
                    arr2[i][m-j-1] = arr[n-i-1][j];
                    sub_task = (arr[i][j] - arr[n-i-1][j]).abs() + (arr[n-i-1][j] - arr[i][m-j-1]).abs();
                }
                if (arr[i][j] - arr[i][m-j-1]).abs() + (arr[n-i-1][j] - arr[i][m-j-1]).abs() < sub_task {
                    arr2[i][j] = arr[i][m-j-1];
                    arr2[n-i-1][j] = arr[i][m-j-1];
                    arr2[i][m-j-1] = arr[i][m-j-1];
                    sub_task = (arr[i][j] - arr[i][m-j-1]).abs() + (arr[n-i-1][j] - arr[i][m-j-1]).abs();
                }
                arr[i][j] = arr2[i][j];
                arr[n-i-1][j] = arr2[n-i-1][j];
                arr[i][m-j-1] = arr2[i][m-j-1];
                res += sub_task;
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
