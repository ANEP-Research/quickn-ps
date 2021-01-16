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
        let mut arr: Vec<i32> = vec![0;n];
        for i in 0..n {
            arr[i] = scan.token();
        }
        let get_mex = |a: &Vec<i32>| -> i32 {
            let mut visit: Vec<bool> = vec![false;a.len()];
            for i in 0..a.len() {
                if a[i] <= (a.len()-1) as i32 {
                    visit[a[i] as usize] = true;
                }
            }
            let mut x = 0;
            while x < a.len() {
                if !visit[x] {
                    break;
                }
                x += 1;
            }
            x as i32
        };
        let non_dec = |a: &Vec<i32>| -> bool {
            let mut prev = 0;
            let mut res = true;
            for i in 0..a.len() {
                if a[i] < prev {
                    res = false;
                    break;
                }
                prev = a[i];
            }
            res
        };
        let mut tasks: Vec<usize> = vec![];
        if !non_dec(&arr) {
        for i in 0..n {
            if arr[n-1] == n as i32 {
                break;
            }
            let m = get_mex(&arr);
            //dbg!(m);
            if m > (n-1) as i32 {
                arr[n-1] = m;
                tasks.push(n);
            } else {
                arr[m as usize] = m;
                tasks.push((m+1) as usize);
            }
        }
        dbg!(arr.clone());
        for _i in 0..n {
            if non_dec(&arr) {
                break;
            }
            let m = get_mex(&arr);
            if m == 0 {
                arr[m as usize] = m;
                tasks.push((m+1) as usize);
            } else {
                arr[(m-1) as usize] = m;
                tasks.push(m as usize);
            }
        }
        }
        dbg!(arr.clone());
        writeln!(sout, "{}", tasks.len()).ok();
        for idx in tasks {
            write!(sout, "{} ", idx).ok();
        }
        write!(sout, "\n").ok();
    }
}
