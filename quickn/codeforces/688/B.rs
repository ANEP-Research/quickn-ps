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
            let mut arr: Vec<i64> = vec![0;n];
            for i in 0..n {
                arr[i] = scan.token();
            }
            let calc = |_arr: Vec<i64>| -> i64 {
                let mut arr = _arr.clone();
                let mut cost = 0;
                let mut suffix = 0;
                for i in 1..arr.len() {
                    arr[i] += suffix;
                    cost += (arr[0]-arr[i]).abs();
                    suffix += arr[0]-arr[i];
                }
                cost
            };
            let mut t = 1;
            while t < n && arr[t-1] == arr[t] {
                t += 1;
            }
            if t == n {
                t -= 1;
            }
            // r1
            let mut res = std::i64::MAX;
            use std::cmp::min;
            {
                let mut a = arr.clone();
                a[t] = a[t-1];
                res = min(res, calc(a.clone()));
            }
            {
                let mut a = arr.clone();
                a[t-1] = a[t];
                res = min(res, calc(a.clone()));
            }
            if t <= n-2 {
                {
                    let mut a = arr.clone();
                    a[t+1] = a[t];
                    res = min(res, calc(a.clone()));
                }
                {
                    let mut a = arr.clone();
                    a[t] = a[t+1];
                    res = min(res, calc(a.clone()));
                }
            }
            writeln!(sout, "{}", res).ok();
        }
    }