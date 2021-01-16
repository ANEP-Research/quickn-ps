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
        let mut arr: Vec<usize> = vec![0;n];
        let mut cnt: Vec<usize> = vec![0;n+1];
        for i in 0..n {
            arr[i] = scan.token();
            //cnt[arr[i]] += 1;
        }
        let mut prev = 0;
        let mut arr2: Vec<usize> = vec![];
        for i in 0..n {
            if prev != arr[i] && prev != 0 {
                arr2.push(prev);
            }
            prev = arr[i];
        }
        arr2.push(prev);
        for i in 0..arr2.len() {
            cnt[arr2[i]] += 1;
        }
        use std::cmp::min;
        let mut res = std::usize::MAX;
        if arr2[0] == arr2[arr2.len()-1] {
            res = min(res, cnt[arr2[0]]-1);
        } else {
            res = min(res, cnt[arr2[0]]);
            res = min(res, cnt[arr2[arr2.len()-1]]);
        }
        for i in 0..arr2.len() {
            res = min(res, cnt[arr2[i]]+1);
        }
        writeln!(sout, "{}", res).ok();
    }
}
