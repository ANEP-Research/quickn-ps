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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let mut arr: Vec<Vec<char>> = vec![vec!['\0';m];n];
        let mut prefix: Vec<Vec<usize>> = vec![vec![0;m+1];n];
        for i in 0..n {
            let arr_i: String = scan.token();
            let a_i: Vec<char> = arr_i.chars().collect();
            for j in 0..m {
                arr[i][j] = a_i[j];
                prefix[i][j+1] = prefix[i][j];
                if arr[i][j] == '*' {
                    prefix[i][j+1] += 1;
                }
            }
        }
        let mut res: i64 = 0;
        for i in 0..n {
            for j in 0..m {
                if arr[i][j] == '*' {
                    let mut k = 0;
                    while k + i < n && j >= k && j + k < m {
                        if prefix[k + i][j + k + 1] - prefix[k + i][j - k] == 2*k + 1 {
                            res += 1;
                        } else {
                            break;
                        }
                        k += 1;
                    }
                }
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
