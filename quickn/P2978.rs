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
    let st: String = scan.token();
    let arr: Vec<char> = st.chars().collect();
    let mut arr2: Vec<i32> = vec![0];
    let (mut a, mut s): (usize, i32) = (0, 0);
    let mut i = 0;
    while i < arr.len() {
        if arr[i] == '=' {
            break;
        }
        arr2.push(((arr[i] as u8) - ('0' as u8)) as i32);
        a += 1;
        i += 1;
    }
    i += 1;
    while i < arr.len() {
        s *= 10;
        s += ((arr[i] as u8) - ('0' as u8)) as i32;
        i += 1;
    }
    let mut dp: Vec<Vec<i32>> = vec![vec![std::i32::MAX;(s as usize)+1];a+1];
    let mut pi: Vec<Vec<(usize, i32)>> = vec![vec![(0, 0);(s as usize)+1];a+1];
    dp[0][0] = 0;
    for i in 1..=a {
        for c in 0..=s {
            let (mut min_val, mut min_idx, mut min_c) = (std::i32::MAX, 0, 0);
            let lower = if i >= 4 { i - 4 } else { 0 };
            let mut j = i-1;
            let mut c2 = 0;
            let mut exp = 1;
            while j >= lower {
                c2 += arr2[j+1]*exp;
                if c < c2 { break; }
                let w = if j != 0 && c2 != 0 { 1 } else { 0 };
                if dp[j][(c-c2) as usize] != std::i32::MAX && min_val > dp[j][(c-c2) as usize] + w {
                    min_val = dp[j][(c-c2) as usize] + w;
                    min_c = c - c2;
                    min_idx = j;
                }
                if j == 0 { break; }
                exp *= 10;
                j -= 1;
            }
            dp[i][c as usize] = min_val;
            pi[i][c as usize] = (min_idx, min_c);
        }
    }
    let mut stack: Vec<usize> = vec![];
    let (mut j_t, mut c_t) = pi[a][s as usize];
    while j_t != 0 {
        stack.push(j_t);
        let tmp = pi[j_t][c_t as usize];
        j_t = tmp.0;
        c_t = tmp.1;
    }
    let mut num = 0;
    for i in 0..a {
        write!(sout, "{}", arr[i]).ok();
        num *= 10;
        num += arr2[i+1];
        if let Some(idx) = stack.last() {
            if idx-1 == i {
                stack.pop();
                if num != 0 {
                    write!(sout, "+").ok();
                    num = 0;
                }
            }
        }
    }
    for i in a..arr.len() {
        write!(sout, "{}", arr[i]).ok();
    }
    //writeln!(sout, "{}", dp[a][s as usize]).ok();
}
