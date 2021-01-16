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
    let mut arr: Vec<usize> = vec![0;n];
    let (mut mval, mut midx) = (0, 0);
    let (mut mval2, mut midx2) = (std::usize::MAX, 0);
    for i in 0..n {
        arr[i] = scan.token();
        if arr[i] > mval && arr[i] <= n {
            mval = arr[i];
            midx = i;
        }
    }
    let mut t: Vec<(usize, usize)> = Vec::new();
    if mval == 0 {
        for i in 0..n {
            t.push((arr[i], i+1));
        }
    } else {
    let mut i = (n + midx - 1)%n;
    let mut len = mval-1;
    while len > 0 {
        if arr[i] != len {
            t.push((arr[i], len));
        }
        len -= 1;
        i = (n + i - 1)%n;
    }
    let mut len = n;
    while len > mval {
        if arr[i] != len {
            t.push((arr[i], len));
        }
        len -= 1;
        i = (n + i - 1)%n;
    }
    }
    t.sort_by(|(a, _), (b, _)| a.cmp(&b));
    let mut res: Vec<usize> = Vec::new();
    let mut time = n;
    for u in t {
        res.push(u.1);
        time += 1;
        while time < u.0 {
            res.push(time);
            time += 1;
        }
    }
    write!(sout, "{} ", res.len()).ok();
    for a in res {
        write!(sout, "{} ", a).ok();
    }
}
