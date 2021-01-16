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
    let n: usize = scan.token();
    let mut arr: Vec<i32> = vec![0;n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    let mut q: Vec<usize> = vec![];
    let mut cnt = 0;
    let is_sorted = |arr: Vec<i32>| -> bool {
        let mut res = true;
        for i in 1..arr.len() {
            if arr[i-1] > arr[i] {
                res = false;
                break;
            }
        }
        res
    };
    let mut visited: Vec<bool> = vec![false;n];
    let it = if n <= 50000 { n } else { 1000 };
    for _i in 0..it {
        for i in 1..n {
            if cnt == n-1 {
                break;
            }
            if arr[i-1] > arr[i] && !visited[i-1] {
                q.push(i);
                arr[i-1] ^= arr[i];
                arr[i] ^= arr[i-1];
                arr[i-1] ^= arr[i];
                cnt += 1;
                visited[i-1] = true;
            }
        }
    }
    if cnt == n-1 && is_sorted(arr.clone()) {
        for i in q {
            writeln!(sout, "{}", i).ok();
        }
    } else {
        writeln!(sout, "-1").ok();
    }
}
