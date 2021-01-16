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
    let n: usize = scan.token();
    let mut arr: Vec<u8> = vec![0;n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    let mut failed = false;
    let mut res: Vec<(usize, usize)> = vec![];
    let mut row = 1;
    let mut s2: Vec<usize> = vec![];
    let mut s3: Vec<usize> = vec![];
    for i in 0..n {
        let mut assigned = false;
        if arr[i] != 0 && s3.len() > 0 {
            if let Some(r) = s3.pop() {
                res.push((r, i+1));
                assigned = true;
            }
        }
        if arr[i] == 1 {
            if s2.len() > 0 && !assigned {
                if let Some(r) = s2.pop() {
                    res.push((r, i+1));
                }
            } else {
                res.push((row, i+1));
                row += 1;
            }
        } else if arr[i] == 2 {
            res.push((row, i+1));
            s2.push(row);
            row += 1;
        } else if arr[i] == 3 {
            res.push((row, i+1));
            s3.push(row);
            row += 1;
        }
    }
    if s3.len() > 0 || s2.len() > 0 {
        failed = true;
    }
    let mut row: Vec<usize> = vec![0;n];
    let mut col: Vec<usize> = vec![0;n];
    for (a, b) in res.clone() {
        if a > n {
            failed = true;
            break;
        }
        row[a-1] += 1;
        col[b-1] += 1;
    }
    for i in 0..n {
        if row[i] > 2 {
            failed = true;
            break;
        }
    }
    for i in 0..n {
        if col[i] > 2 {
            failed = true;
            break;
        }
    }
    if failed {
        writeln!(sout, "-1").ok();
    } else {
        writeln!(sout, "{}", res.len()).ok();
        for (a, b) in res.clone() {
            writeln!(sout, "{} {}", a, b).ok();
        }
    }
}
