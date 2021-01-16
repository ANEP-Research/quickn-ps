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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Marker {
    is_start: bool,
    gene_type: usize,
}

impl Marker { 
    fn new(is_start: bool, gene_type: usize) -> Self {
        Self {
            is_start,
            gene_type,
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
    let mut arr: Vec<Marker> = vec![];
    let mut max_i = 0;
    for i in 0..n {
        let s: String = scan.token();
        let s_chars: Vec<char> = s.chars().collect();
        let num_s = s.get(1..).unwrap();
        let num = num_s.parse::<usize>().unwrap()-1;
        let is_start = if s_chars[0] == 's' { true } else { false };
        arr.push(Marker::new(is_start, num));
        use std::cmp::max;
        max_i = max(max_i, num);
    }
    let mut start_stack: Vec<Vec<usize>> = vec![vec![];max_i+1];
    let mut enabled: Vec<bool> = vec![false;max_i+1];
    let mut cnt1: Vec<usize> = vec![0;max_i+1];
    let mut cnt2: Vec<usize> = vec![0;max_i+1];
    for i in 0..n {
        let (is_start, j) = (arr[i].is_start, arr[i].gene_type);
        if is_start {
            start_stack[j].push(i);
            cnt1[j] += 1;
        } else {
            start_stack[j].pop();
            cnt2[j] += 1;
        }
        enabled[j] = true;
    }
    let mut res: usize = 0;
    let (mut max_val, mut max_idx): (usize, usize) = (0, 0);
    for i in 0..=max_i {
        if cnt1[i] != cnt2[i] {
            enabled[i] = false;
        }
        if enabled[i] {
            if start_stack[i].is_empty() {
                res += 1;
            }
        }
    }
    for i in 0..n {
        let (is_start, j) = (arr[i].is_start, arr[i].gene_type);
        let prev = start_stack[j].len();
        if max_val < res {
            max_idx = i;
            max_val = res;
        }
        if enabled[j] {
            if is_start {
                start_stack[j].push(i);
                if prev == 0 {
                    res -= 1;
                }
            } else {
                start_stack[j].pop();
                if prev != start_stack[j].len() && start_stack[j].len() == 0 {
                    res += 1;
                }
            }
        }
    }
    writeln!(sout, "{} {}", max_idx+1, max_val).ok();
}
