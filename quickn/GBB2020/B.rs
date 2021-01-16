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

fn eval(s: usize) -> usize {
    let mut res = 0;
    for i in 0..4 {
        if ((1 << i) & s) != 0 {
            res += 1;
        }
    }
    res
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let mut arr: Vec<usize> = vec![0;16];
        for i in 0..n {
            let mut s = s2vec(scan.token());
            let mut r = 0;
            s.reverse();
            for i in 0..4 {
                if s[i] == 'J'
                || s[i] == 'T'
                || s[i] == 'S'
                || s[i] == 'E' {
                    r += 1 << i;
                }
            }
            arr[r] += 1;
        }
        let mut res = std::usize::MAX;
        for i in 0..16 {
            for j in 0..16 {
                for k in 0..16 {
                    let mut cnt: [usize;16] = [0;16];
                    cnt[i] += 1;
                    cnt[j] += 1;
                    cnt[k] += 1;
                    if arr[i] >= cnt[i] && arr[j] >= cnt[j] && arr[k] >= cnt[k] {
                        res = min(res, eval(i ^ j) + eval(j ^ k) + eval(k ^ i));
                    }
                }
            }
        }
        writeln!(sout, "{}", res).ok();
    }
}
