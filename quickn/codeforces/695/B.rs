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

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let n: usize = scan.token();
        let mut arr: Vec<i32> = vec![0;n];
        for i in 0..n {
            arr[i] = scan.token();
        }
        if n == 1 {
            writeln!(sout, "0").ok();
        } else {
        let mut prefix: Vec<i32> = vec![0;n];
        for i in 1..(n-1) {
            prefix[i] = prefix[i-1];
            if arr[i-1] < arr[i] && arr[i] > arr[i+1] {
                prefix[i] += 1;
            }
            if arr[i-1] > arr[i] && arr[i] < arr[i+1] {
                prefix[i] += 1;
            }
        }
        if n >= 2 {
            prefix[n-1] = prefix[n-2];
        }
        let mut res = std::i32::MAX;
        for i in 0..n {
            let u = arr[i];
            let mut b = vec![];
            if i > 0 {
                b.push(arr[i-1]);
            }
            b.push(arr[i]);
            if i < n-1 {
                b.push(arr[i+1]);
            }
            for a in b.clone() {
                arr[i] = a;
                let mut j = max(1, (i as i32)-5) as usize;
                let mut t = 0;
                while j <= min(n-2, i+5) {
                    if arr[j-1] < arr[j] && arr[j] > arr[j+1] {
                        t += 1;
                    }
                    if arr[j-1] > arr[j] && arr[j] < arr[j+1] {
                        t += 1;
                    }
                    j += 1;
                }
                res = min(res, prefix[(max(1, (i as i32)-5) as usize)-1] + t + prefix[n-1] - prefix[min(n-2, i+5) as usize]);
            }
            arr[i] = u;
        }
        writeln!(sout, "{}", res).ok();
    }
    }
}
