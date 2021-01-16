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

use std::cmp::min;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let n: usize = scan.token();
    let mut arr: Vec<i32> = vec![0; n + 1];
    for i in 1..=n {
        arr[i] = scan.token();
    }
    let mut dp: Vec<[i32; 3]> = vec![[std::i32::MAX; 3]; n + 1];
    let mut b: Vec<i32> = vec![std::i32::MAX; n + 1];
    if arr[1] == -1 {
        dp[1][0] = 0;
    } else if arr[1] == 0 {
        dp[1][1] = 0;
    } else {
        dp[1][2] = 0;
        b[1] = 1;
    }
    for i in 2..=n {
        {
            let w = if arr[i] >= b[i - 1] {
                0
            } else if arr[i] >= 0 {
                1
            } else {
                2
            };
            let c1 = if dp[i - 1][2] != std::i32::MAX {
                dp[i - 1][2] + w
            } else {
                std::i32::MAX
            };
            let c2 = if arr[i] == 1 {
                dp[i - 1][1]
            } else {
                std::i32::MAX
            };
            let c3 = if arr[i] == 1 {
                dp[i - 1][0]
            } else {
                std::i32::MAX
            };
            let mut arr = vec![(c1, arr[i] + w * b[i - 1]), (c2, arr[i]), (c3, arr[i])];
            arr.sort();
            dp[i][2] = arr[0].0;
            if arr[0].0 == std::i32::MAX {
                b[i] = std::i32::MAX;
            } else {
                b[i] = arr[0].1;
            }
        }
        {
            if arr[i] == 1 {
                if dp[i - 1][0] != std::i32::MAX {
                    dp[i][1] = dp[i - 1][0] + 1;
                }
            } else if arr[i] == 0 {
                dp[i][1] = min(dp[i - 1][0], dp[i - 1][1]);
            }
        }
        {
            if dp[i - 1][0] != std::i32::MAX {
                dp[i][0] = dp[i - 1][0] + 1 + arr[i];
            }
        }
    }
    //dbg!(dp.clone());
    let d = min(min(dp[n][0], dp[n][1]), dp[n][2]);
    if d == std::i32::MAX {
        writeln!(sout, "BRAK").ok();
    } else {
        writeln!(sout, "{}", d).ok();
    }
}
