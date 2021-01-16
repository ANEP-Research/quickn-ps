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
 
struct Interval {
    start: usize,
    end: usize,
}
 
fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _i in 0..t {
        let (n, mut k): (usize, usize) = (scan.token(), scan.token());
        let s: String = scan.token();
        let mut arr: Vec<char> = s.chars().collect();
        let mut int: Vec<Interval> = vec![];
        let (mut prev, mut s_idx) = ('L', n);
        for i in 0..n {
            if arr[i] == 'W' {
                if prev != arr[i] {
                    if s_idx != n {
                        int.push(Interval { start: s_idx, end: (i-1) });
                    }
                    s_idx = n;
                }
            } else {
                if s_idx == n {
                    s_idx = i;
                }
                if prev != arr[i] {
                    s_idx = i;
                }
            }
            prev = arr[i];
        }
        if s_idx != n {
            int.push(Interval { start: s_idx, end: (n-1) });
        }
        let mut t = int.len();
        if t > 0 {
            if int[0].start == 0 {
                int.remove(0);
            }
            t = int.len();
            if t > 0 {
                if int[t-1].end == n-1 { 
                    int.remove(t-1);
                }
            }
        }
        int.sort_by(|i1, i2| (i1.end - i1.start).cmp(&(i2.end - i2.start)));
        for i in int {
            for idx in i.start..=i.end {
                if k == 0 {
                    break;
                }
                arr[idx] = 'W';
                k -= 1;
            }
            if k == 0 {
                break;
            }
        }
        let (mut start, mut end) = (1, n);
        while start <= n && arr[start-1] == 'L' {
            start += 1;
        }
        while end > 0 && arr[end-1] == 'L' {
            end -= 1;
        }
        start -= 1;
        end += 1;
        while start > 0 {
            if k == 0 { break; }
            arr[start-1] = 'W';
            k -= 1;
            start -= 1;
        }
        while end <= n {
            if k == 0 { break; }
            arr[end-1] = 'W';
            k -= 1;
            end += 1;
        }
        prev = 'L';
        let mut res = 0;
        for i in 0..n {
            if arr[i] == 'W' {
                if arr[i] != prev {
                    res += 1;
                } else {
                    res += 2;
                }
            }
            prev = arr[i];
        }
        writeln!(sout, "{}", res).ok();
    }
}