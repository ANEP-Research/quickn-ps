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
    let mut b: Vec<i32> = vec![0; n];
    for i in 0..n {
        b[i] = scan.token();
    }
    if n % 3 == 0 {
        let mut a1: Vec<i32> = vec![std::i32::MAX; n];
        let mut a2: Vec<i32> = vec![std::i32::MAX; n];
        let mut a3: Vec<i32> = vec![std::i32::MAX; n];
        let mut i = 0;
        let mut m = std::i32::MAX;
        use std::cmp::min;
        {
            a1[(n + i - 1) % n] = 0;
            loop {
                let j = (i + 1) % n;
                let d = b[j] - b[i];
                a1[(j + 1) % n] = a1[(n + j - 2) % n] + d;
                i += 3;
                i %= n;
                if i == 0 {
                    break;
                }
            }
            for i in 0..n {
                m = min(m, a1[i]);
            }
            for i in 0..n {
                if a1[i] != std::i32::MAX {
                    a1[i] += 1 - m;
                }
            }
        }
        i = 1;
        m = std::i32::MAX;
        {
            a2[(n + i - 1) % n] = 0;
            loop {
                let j = (i + 1) % n;
                let d = b[j] - b[i];
                a2[(j + 1) % n] = a2[(n + j - 2) % n] + d;
                i += 3;
                i %= n;
                if i == 1 {
                    break;
                }
            }
            for i in 0..n {
                m = min(m, a2[i]);
            }
            for i in 0..n {
                if a2[i] != std::i32::MAX {
                    a2[i] += 1 - m;
                }
            }
        }
        i = 2;
        m = std::i32::MAX;
        {
            a3[(n + i - 1) % n] = 0;
            loop {
                let j = (i + 1) % n;
                let d = b[j] - b[i];
                a3[(j + 1) % n] = a3[(n + j - 2) % n] + d;
                i += 3;
                i %= n;
                if i == 2 {
                    break;
                }
            }
            for i in 0..n {
                m = min(m, a3[i]);
            }
            for i in 0..n {
                if a3[i] != std::i32::MAX {
                    a3[i] += 1 - m;
                }
            }
        }
        let mut a: Vec<i32> = vec![0; n];
        for i in 0..n {
            a[i] = min(a1[i], min(a2[i], a3[i]));
        }
        let k = a[0] + a[1] + a[2];
        let d = b[1] - k;
        for i in 0..n {
            if i % 3 == 0 {
                a[i] += d;
            }
        }
        for i in 0..n {
            let s = a[(n + i - 1) % n] + a[i] + a[(i + 1) % n];
            assert_eq!(s, b[i]);
        }
        for i in 0..n {
            writeln!(sout, "{}", a[i]).ok();
        }
    } else {
        let mut a: Vec<i32> = vec![0; n];
        let mut i = 0;
        let mut m = std::i32::MAX;
        use std::cmp::min;
        {
            loop {
                let j = (i + 1) % n;
                let d = b[j] - b[i];
                a[(j + 1) % n] = a[(n + j - 2) % n] + d;
                i += 3;
                i %= n;
                if i == 0 {
                    break;
                }
            }
            for i in 0..n {
                m = min(m, a[i]);
            }
            for i in 0..n {
                a[i] += 1 - m;
            }
        }
        let k = a[0] + a[1] + a[2];
        let d = b[1] - k;
        for i in 0..n {
            a[i] += d/3;
        }
        for i in 0..n {
            let s = a[(n + i - 1) % n] + a[i] + a[(i + 1) % n];
            assert_eq!(s, b[i]);
        }
        for i in 0..n {
            writeln!(sout, "{}", a[i]).ok();
        }
    }
}
