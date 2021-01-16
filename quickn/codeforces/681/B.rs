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
        let (a, b): (usize, usize) = (scan.token(), scan.token());
        let s: String = scan.token();
        let arr: Vec<char> = s.chars().collect();
        let mut prev = '1';
        let mut cost: Vec<u64> = vec![];
        let mut start = s.len();
        let mut cnt = 0;
        for i in 0..arr.len() {
            if arr[i] == '0' {
                if prev == '1' {
                    start = i;
                }
            } else {
                cnt += 1;
                if prev == '0' {
                    cost.push(((i-start) as u64)*(b as u64));
                    start = s.len();
                }
            }
            prev = arr[i];
        }
        if cost.len() > 0 && arr[0] == '0' {
            cost.remove(0);
        }
        cost.sort();
        let mut res = 0;
        let mut i = 0;
        while i < cost.len() && res + cost[i] + (((cost.len()-i)*a) as u64) < res + (((cost.len()-i+1)*a) as u64) {
            res += cost[i];
            i += 1;
        }
        //dbg!(res);
        res += if cnt > 0 { (((cost.len()-i+1)*a) as u64) } else { 0 };
        writeln!(sout, "{}", res).ok();
    }
}
