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

use std::collections::HashMap;
use std::cmp::{max, min};

const MAX: i32 = 150;
const MAX_N: i32 = 10000000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let mut is_prime: Vec<bool> = vec![true;(MAX_N+1) as usize];
    let mut primes: Vec<i32> = vec![];
    let mut ppi: Vec<i32> = vec![0;(MAX_N+1) as usize]; 
    let mut hi = 0;
    for p in 2..=MAX_N {
        ppi[p as usize] = ppi[(p-1) as usize];
        if is_prime[p as usize] {
            primes.push(p);
            ppi[p as usize] += 1;
        }
        for i in 0..primes.len() {
            let t = (primes[i] as i64)*(p as i64);
            if t > (MAX_N as i64) { break; }
            is_prime[t as usize] = false;
        }
        if p >= 150 && ppi[p as usize] - ppi[(p - MAX) as usize] == 0 {
            hi = p - MAX;
        }
    }
    let q: usize = scan.token();
    for i in 0..q {
        let (k, l, m): (i32, i32, i32) = (scan.token(), scan.token(), scan.token());
        let count = |a: i32, b: i32 | {
            if a > b {
                0
            } else if b <= m {
                b-a+1
            } else if a <= m {
                m - a + 1 + ppi[b as usize] - ppi[m as usize]
            } else {
                ppi[b as usize] - ppi[(a-1) as usize]
            }
        };
        let (mut left, mut right) = (1, hi);
        let mut first_func = count(1, k);
        let mut res = 0;
        while left <= right {
            let mid = (left + right) >> 1;
            let func = count(mid, mid + k - 1);
            let func2 = count(mid-1, mid + k - 2);
            if func == l {
                res = mid;
                break;
            } else if min(func2, first_func) <= l && l<= max(func2, first_func) {
                right = mid - 1;
            } else {
                left = mid + 1;
                first_func = count(left, left + k - 1);
            }
        }
        if res != 0 {
            writeln!(sout, "{}", res).ok();
        } else {
            writeln!(sout, "-1").ok();
        }
    }
}
