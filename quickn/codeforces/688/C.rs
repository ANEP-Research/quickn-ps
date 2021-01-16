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
        let n: usize = scan.token();
        let mut arr: Vec<Vec<usize>> = vec![vec![0;n];n];
        for i in 0..n {
            let a_i: String = scan.token();
            let arr_i: Vec<char> = a_i.chars().collect();
            for j in 0..n {
                arr[i][j] = ((arr_i[j] as u8) - ('0' as u8)) as usize;
            }
        }
        let mut up: Vec<[i32;10]> = vec![[-1;10];n];
        let mut down: Vec<[i32;10]> = vec![[-1;10];n];
        let mut left: Vec<[i32;10]> = vec![[-1;10];n];
        let mut right: Vec<[i32;10]> = vec![[-1;10];n];
        for i in 0..n {
            for a in 0..10 {
                let mut first = false;
                for j in 0..n {
                    if arr[i][j] == a {
                        if !first {
                            left[i][a] = j as i32;
                        }
                        first = true;
                        right[i][a] = j as i32;
                    }
                }
            }
        }
        for j in 0..n {
            for a in 0..10usize {
                let mut first = false;
                for i in 0..n {
                    if arr[i][j] == a {
                        if !first {
                            up[j][a] = i as i32;
                        }
                        first = true;
                        down[j][a] = i as i32;
                    }
                }
            }
        }
        let mut res: Vec<i32> = vec![0;10];
        use std::cmp::max;
        for a in 0..10usize {
            for i in 0..n {
                let mut d = right[i][a] - left[i][a];
                let t = i as i32;
                let u = (n-1) as i32;
                res[a] = max(res[a], d*(u - t).abs());
                res[a] = max(res[a], d*(t).abs());
                let d2 = max(if right[i][a] != -1 { max(right[i][a], u - right[i][a]) } else { 0 }, if left[i][a] != -1 { max(left[i][a], u - left[i][a]) } else { 0 });
                for j in 0..n {
                    if up[j][a] != -1 {
                        res[a] = max(res[a], d2*(up[j][a] - t).abs());
                    }
                    if down[j][a] != -1 {
                        res[a] = max(res[a], d2*(down[j][a] - t).abs());
                    }
                }
            }
        }
        for a in 0..10usize {
            for j in 0..n {
                let mut d = down[j][a] - up[j][a];
                let t = j as i32;
                let u = (n-1) as i32;
                res[a] = max(res[a], d*(u - t).abs());
                res[a] = max(res[a], d*(t).abs());
                let d2 = max(if down[j][a] != -1 { max(down[j][a], u - down[j][a]) } else { 0 }, if up[j][a] != -1 { max(up[j][a], u - up[j][a]) } else { 0 });
                for i in 0..n {
                    if left[i][a] != -1 {
                        res[a] = max(res[a], d2*(left[i][a] - t).abs());
                    }
                    if right[i][a] != -1 {
                        res[a] = max(res[a], d2*(right[i][a] - t).abs());
                    }
                }
            }
        }
        for a in 0..10usize {
            write!(sout, "{} ", res[a]).ok();
        }
        write!(sout, "\n").ok();
    }
}
