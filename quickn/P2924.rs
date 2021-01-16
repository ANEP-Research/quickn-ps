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

fn gcd(a: i32, b: i32) -> i32 {
    let (mut a_t, mut b_t) = (a, b);
    while b_t != 0 {
        a_t %= b_t;
        a_t ^= b_t;
        b_t ^= a_t;
        a_t ^= b_t;
    }
    a_t
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, a, b, c, d): (usize, usize, usize, usize, usize) = (scan.token(), scan.token(), scan.token(), scan.token(), scan.token());
    let mut arr: Vec<i32> = vec![0;n+1];
    let mut perm: Vec<i32> = vec![0;n+1];
    for i in 1..=n {
        arr[i] = scan.token();
        perm[arr[i] as usize] = i as i32;
    }
    let mut cnt = 0;
    let mut size: Vec<i32> = vec![0];
    let mut group: Vec<i32> = vec![0;n+1];
    for i in 1..=n {
        if group[i] == 0 {
            let mut u = i as i32;
            cnt += 1;
            group[i] = cnt;
            size.push(0);
            size[cnt as usize] += 1;
            while perm[u as usize] != i as i32 {
                size[cnt as usize] += 1;
                group[perm[u as usize] as usize] = cnt;
                u = perm[u as usize];
            }
        }
    }
    let mut res = 1;
    for i in (c+1)..=(n-d) {
        res /= gcd(res, size[group[i] as usize]);
        //dbg!(gcd(res, size[group[i] as usize]));
        res *= size[group[i] as usize];
    }
    //dbg!(res);
    writeln!(sout, "{}", (((b as f64)/(res as f64)).ceil() as usize) - ((((a-1) as f64)/(res as f64)).ceil() as usize)).ok();
}
