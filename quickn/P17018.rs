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

use std::collections::{VecDeque, BinaryHeap};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, k): (usize, usize) = (scan.token(), scan.token());
    let s = s2vec(scan.token());
    let mut dp: Vec<i32> = vec![std::i32::MAX;n];
    let mut vals: Vec<VecDeque<(i32, usize)>> = vec![VecDeque::new();n];
    let mut heap: BinaryHeap<i32> = BinaryHeap::new();
    let mut cnt_h: Vec<i32> = vec![0;n];
    let mut cnt_g: Vec<i32> = vec![0;n];
    for i in 0..n {
        if i != 0 {
            cnt_h[i] = cnt_h[i-1];
            cnt_g[i] = cnt_g[i-1];
        }
        if s[i] == 'H' {
            cnt_h[i] += 1;
        } else {
            cnt_g[i] += 1;
        }
    }
    for i in 0..n {
        if i > 0 {
            let mut changed = false;
            while let Some(&_val) = heap.peek() {
                let val = -_val;
                if changed {
                    let mut new_vals: Vec<(i32, usize)> = vals[val as usize].clone().into_iter().collect();
                    for j in 0..new_vals.len() {
                        let (_, idx) = new_vals[j];
                        let maj_j = (cnt_g[i] - cnt_g[idx]) - (cnt_h[i] - cnt_h[idx]);
                        new_vals[j].0 = maj_j;
                    }
                    new_vals.sort();
                    vals[val as usize] = new_vals.into_iter().collect();
                }
                while let Some(&(_, idx)) = vals[val as usize].front() {
                    if i - idx > k {
                        vals[val as usize].pop_front();
                    } else {
                        break;
                    }
                }
                if vals[val as usize].len() != 0 {
                    if let Some(&(_, idx)) = vals[val as usize].front() {
                        let maj = (cnt_g[i] - cnt_g[idx]) - (cnt_h[i] - cnt_h[idx]);
                        if maj >= 0 {
                            dp[i] = val + 1;
                        } else {
                            dp[i] = val;
                        }
                        break;
                    }
                } else {
                    heap.pop();
                    changed = true;
                }
            }
        }
        if i < k {
            let maj = cnt_g[i] - cnt_h[i];
            if maj < 0 {
                dp[i] = min(dp[i], 0);
            } else {
                dp[i] = min(dp[i], 1);
            }
        }
        while let Some(&(_, idx)) = vals[dp[i] as usize].back() {
            let maj = (cnt_g[i] - cnt_g[idx]) - (cnt_h[i] - cnt_h[idx]);
            if maj > 0 {
                vals[dp[i] as usize].pop_back();
            } else {
                break;
            }
        }
        vals[dp[i] as usize].push_back((0, i));
        heap.push(-dp[i]);
    }
    writeln!(sout, "{}", dp[n-1]).ok();
}
