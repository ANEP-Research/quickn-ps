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
use std::collections::{BTreeMap, BTreeSet};

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let (n, k): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<i32> = vec![0;n];
    for i in 0..n {
        arr[i] = scan.token();
    }
    let mut hash: BTreeMap<i32, usize> = BTreeMap::new();
    let mut set: BTreeSet<i32> = BTreeSet::new();
    let (mut i, mut j) = (0, 0);
    let mut res: usize = 1;
    while j < n {
        set.insert(arr[j]);
        if let Some(cnt) = hash.get_mut(&arr[j]) {
            *cnt += 1;
            res = max(res, *cnt);
        } else {
            hash.insert(arr[j], 1);
        }
        if set.len() > k+1 {
            while i < j {
                if let Some(cnt) = hash.get_mut(&arr[i]) {
                    *cnt -= 1;
                    if *cnt == 0 {
                        set.remove(&arr[i]);
                        hash.remove(&arr[i]);
                        i += 1;
                        break;
                    }
                }
                i += 1;
            }
        }
        j += 1;
    }
    writeln!(sout, "{}", res).ok();
}
