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

use std::collections::HashSet;

const ALPHABETS: usize = 26;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let s: String = scan.token();
    let t: String = scan.token();
    let arr1: Vec<char> = s.chars().collect();
    let arr2: Vec<char> = t.chars().collect();
    let mut hash: HashSet<[u16;ALPHABETS]> = HashSet::new();
    let mut a: [u16;ALPHABETS] = [0;ALPHABETS];
    for i in 0..arr1.len() {
        let h = ((arr1[i] as u8) - ('a' as u8)) as usize;
        a[h] += 1;
        hash.insert(a);
        let mut b = a;
        for j in 0..i {
            let h2 = ((arr1[j] as u8) - ('a' as u8)) as usize;
            b[h2] -= 1;
            hash.insert(b);
        }
    }
    a = [0;ALPHABETS];
    let mut res: u16 = 0;
    use std::cmp::max;
    for i in 0..arr2.len() {
        let h = ((arr2[i] as u8) - ('a' as u8)) as usize;
        a[h] += 1;
        if hash.contains(&a) {
            let mut sub_res = 0;
            for k in 0..ALPHABETS {
                sub_res += a[k];
            }
            res = max(res, sub_res);
        }
        let mut b = a;
        for j in 0..i {
            let h2 = ((arr2[j] as u8) - ('a' as u8)) as usize;
            b[h2] -= 1;
            if hash.contains(&b) {
                let mut sub_res = 0;
                for k in 0..ALPHABETS {
                    sub_res += b[k];
                }
                res = max(res, sub_res);
            }
        }
    }
    writeln!(sout, "{}", res).ok();
}
