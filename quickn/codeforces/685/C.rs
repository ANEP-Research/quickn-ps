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

const ALPHABETS: usize = 26;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let k: usize = scan.token();
        let a: String = scan.token();
        let b: String = scan.token();
        let arr1: Vec<char> = a.chars().collect();
        let arr2: Vec<char> = b.chars().collect();
        let mut cnt1: Vec<usize> = vec![0; ALPHABETS];
        let mut cnt2: Vec<usize> = vec![0; ALPHABETS];
        for i in 0..n {
            let ai = ((arr1[i] as u8) - ('a' as u8)) as usize;
            let bi = ((arr2[i] as u8) - ('a' as u8)) as usize;
            cnt1[ai] += 1;
            cnt2[bi] += 1;
        }
        let mut pre_res = true;
        let mut sum = 0;
        for i in 0..ALPHABETS {
            if cnt1[i] < cnt2[i] {
                if (sum < cnt2[i] - cnt1[i]) || ((cnt2[i] - cnt1[i]) % k != 0) {
                    pre_res = false;
                    break;
                } else {
                    sum -= cnt2[i] - cnt1[i];
                }
            } else if cnt1[i] > cnt2[i] {
                if (cnt1[i] - cnt2[i]) % k == 0 {
                    sum += cnt1[i] - cnt2[i];
                } else {
                    pre_res = false;
                    break;
                }
            }
        }
        if sum != 0 {
            pre_res &= false;
        }
        if pre_res {
            writeln!(sout, "YES").ok();
        } else {
            writeln!(sout, "NO").ok();
        }
    }
}
