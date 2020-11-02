/*
    date   : 2020 / 5 / 5
    author : quickn (quickn.ga)
    email  : quickwshell@gmail.com
*/

use std::io::{self, BufWriter, Write};

mod scanner {
    use std::{io, str};
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

const MAX: i32 = 100_000;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    let mut is_prime: Vec<bool> = vec![true;(MAX+1) as usize];
    let mut primes: Vec<i32> = vec![];
    for i in 2..=MAX {
        if is_prime[i as usize] {
            primes.push(i);
        }
        for j in 0..primes.len() {
            let t = (i as i64)*(primes[j] as i64);
            if t > MAX as i64 { break; }
            is_prime[t as usize] = false;
        }
    }
    let mut len: Vec<i32> = vec![0;101];
    use std::cmp::min;
    for i in 0..primes.len() {
        for j in 1..=min(99, primes[i]) {
            if !is_prime[(primes[i] - j) as usize] {
                len[j as usize] = primes[i as usize]-j;
            }
        }
    }
    for _case in 0..t {
        let n: usize = scan.token();
        for i in 0..n {
            for j in 0..n {
                if i == j {
                    write!(sout, "{} ", len[n-1]).ok();
                } else {
                    write!(sout, "1 ").ok();
                }
            }
            writeln!(sout, "").ok();
        }
    }
}
