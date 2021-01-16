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

use std::collections::HashSet;

#[derive(Clone, Debug)]
struct DnC {
    arr: Vec<i64>,
    psum: Vec<i64>,
    res: HashSet<i64>,
}

impl DnC {
    fn new(arr: Vec<i64>) -> Self {
        let mut psum: Vec<i64> = vec![0;arr.len()+1];
        for i in 0..arr.len() {
            psum[i+1] = psum[i];
            psum[i+1] += arr[i];
        }
        Self {
            arr: arr.clone(),
            psum,
            res: HashSet::new(),
        }
    }

    fn run(&mut self, i: usize, j: usize) {
        if i == j {
            self.res.insert(self.arr[i]);
        } else if i < j {
            let mid_val = (self.arr[i]+self.arr[j])/2;
            self.res.insert(self.psum[j+1] - self.psum[i]);
            let (mut l, mut r): (i32, i32) = (i as i32, (j+1) as i32);
            while l < r {
                let m = (l+r)/2;
                if self.arr[m as usize] > mid_val {
                    r = m;
                } else {
                    l = m + 1;
                }
            }
            let mid = (r-1) as usize;
            if mid_val < self.arr[j] || mid_val > self.arr[i] {
                self.run(i, mid);
                self.run(mid+1, j);
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
        let (n, q): (usize, usize) = (scan.token(), scan.token());
        let mut arr: Vec<i64> = vec![0;n];
        for i in 0..n {
            arr[i] = scan.token();
        }
        arr.sort();
        let mut dnc = DnC::new(arr);
        dnc.run(0, n-1);
        for i in 0..q {
            let s_i: i64 = scan.token();
            if !dnc.res.contains(&s_i) {
                writeln!(sout, "No").ok();
            } else {
                writeln!(sout, "Yes").ok();
            }
        }
    }
}
