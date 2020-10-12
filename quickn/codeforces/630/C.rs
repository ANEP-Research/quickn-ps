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

use std::cmp::{min, max};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _i in 0..t {
        let (n, k): (usize, usize) = (scan.token(), scan.token());
        let s: String = scan.token();
        let arr: Vec<char> = s.chars().collect();
        let mut cnt: Vec<[usize;ALPHABETS]> = vec![[0;ALPHABETS];k];
        let strings = n/k;
        let mut i = 0;
        let mut cost = std::usize::MAX;
        for i in 0..n {
            cnt[(i % k)][((arr[i] as u8) - ('a' as u8)) as usize] += 1;
        }
        let pal = arr.get(i..(i+k)).unwrap();
        let mut left_pal: Vec<char> = vec!['a';k];
        let mut right_pal: Vec<char> = vec!['a';k];
        let mut sub_cost1 = 0;
        let mut sub_cost2 = 0;
        for j in 0..(k>>1) {
            let (mut max_idx, mut max_val) = (0, 0);
            for m in 0..ALPHABETS {
                if max_val < (cnt[j][m] + cnt[k - 1 - j][m]) {
                    max_val = (cnt[j][m] + cnt[k - 1 - j][m]);
                    max_idx = m as u8;
                }
            }
            left_pal[j] = (max_idx + ('a' as u8)) as char;
            left_pal[k - 1 - j] = left_pal[j];
        }
        for j in 0..(k>>1) {
            let (mut max_idx, mut max_val) = (0, 0);
            for m in 0..ALPHABETS {
                if max_val < (cnt[j][m] + cnt[k - 1 - j][m]) {
                    max_val = (cnt[j][m] + cnt[k - 1 - j][m]);
                    max_idx = m as u8;
                }
            }
            right_pal[k - 1 - j] = (max_idx + ('a' as u8)) as char;
            right_pal[j] = right_pal[k - 1 - j];
        }
        if k % 2 == 1 {
            let j = k/2;
            let (mut max_idx, mut max_val) = (0, 0);
            for m in 0..ALPHABETS {
                if max_val < cnt[j][m] {
                    max_val = cnt[j][m];
                    max_idx = m as u8;
                }
            }
            left_pal[j] = (max_idx + ('a' as u8)) as char;
            left_pal[k - 1 - j] = left_pal[j];
            right_pal[k - 1 - j] = (max_idx + ('a' as u8)) as char;
            right_pal[j] = right_pal[k - 1 - j];
        }
        for j in 0..k {
            sub_cost1 += strings - cnt[j][((left_pal[j] as u8) - ('a' as u8)) as usize];
            sub_cost2 += strings - cnt[j][((right_pal[j] as u8) - ('a' as u8)) as usize];
        }
        //dbg!(left_pal.clone(), right_pal.clone());
        cost = min(cost, min(sub_cost1, sub_cost2));
        i += k;
        writeln!(sout, "{}", cost).ok();
    }
}
