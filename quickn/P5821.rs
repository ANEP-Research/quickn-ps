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
    let (n, len, b): (usize, i64, i64) = (scan.token(), scan.token(), scan.token());
    let mut prefix: Vec<i64> = vec![0;n+1];
    let mut arr: Vec<i64> = vec![0;n+1];
    for i in 1..=n {
        let t: i64 = scan.token();
        arr[i] = t;
        prefix[i] = prefix[i-1] + t;
    }
    if n == 1 {
        writeln!(sout, "{}", 1).ok();
    } else {
        let mut res = 0;
        use std::cmp::max;
        for i in 1..=n {
            let (mut l, mut r) = (i, n);
            while l < r {
                let mid = (l + r) / 2;
                let t = (mid + i)/2;
                let mut func = (prefix[mid] - prefix[t-1]) - (((mid - t + 1) as i64)*arr[t]);
                func += (((t - i) as i64)*arr[t]) - (prefix[t-1] - prefix[i-1]);
                if func <= b {
                    res = max(res, mid - i + 1);
                    l = mid + 1;
                } else {
                    r = mid;
                }
            }
            let mid = l;
            let t = (mid + i)/2;
            let mut func = (prefix[mid] - prefix[t-1]) - (((mid - t + 1) as i64)*arr[t]);
            func += (((t - i) as i64)*arr[t]) - (prefix[t-1] - prefix[i-1]);
            if func <= b {
                res = max(res, l - i + 1);
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
