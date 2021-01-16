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

use std::cmp::Ordering;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Boots {
    idx: usize,
    s_i: i32,
    d_i: i32,
}

impl Boots {
    fn new(idx: usize, s_i: i32, d_i: i32) -> Self {
        Self {
            idx,
            s_i,
            d_i,
        }
    }
}

impl PartialOrd for Boots {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.s_i.cmp(&other.s_i).then_with(|| self.idx.cmp(&other.idx)))
    }
}

impl Ord for Boots {
    fn cmp(&self, other: &Self) -> Ordering {
        self.s_i.cmp(&other.s_i).then_with(|| self.idx.cmp(&other.idx))
    }
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, b): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<(i32, usize)> = vec![(0, 0);n];
    let mut next: Vec<usize> = vec![n;n];
    let mut prev: Vec<usize> = vec![n;n];
    for i in 0..n {
        arr[i] = (scan.token(), i);
    }
    for i in 0..(n-1) {
        next[i] = i+1;
    }
    for i in 1..n {
        prev[i] = i-1;
    }
    let mut boots: Vec<Boots> = vec![];
    for i in 0..b {
        let (s_i, d_i): (i32, i32) = (scan.token(), scan.token());
        boots.push(Boots::new(i, s_i, d_i));
    }
    boots.sort();
    boots.reverse();
    let mut stack: Vec<usize> = vec![];
    let mut arr2 = arr.clone();
    arr2.sort();
    for i in 0..n {
        stack.push(arr2[i].1);
    }
    let mut res: Vec<usize> = vec![0;b];
    let mut max_d: i32 = 1;
    use std::cmp::max;
    for i in 0..b {
        while let Some(&idx) = stack.last() {
            if arr[idx].0 > boots[i].s_i {
                if next[idx] != n {
                    prev[next[idx]] = prev[idx];
                }
                if prev[idx] != n {
                    next[prev[idx]] = next[idx];
                }
                max_d = max(max_d, (next[idx] - prev[idx]) as i32);
                stack.pop();
            } else {
                break;
            }
        }
        if max_d <= boots[i].d_i {
            res[boots[i].idx] = 1;
        }
    }
    for i in 0..b {
        writeln!(sout, "{}", res[i]).ok();
    }
}
