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
    let (r, c): (usize, usize) = (scan.token(), scan.token());
    let mut arr: Vec<Vec<u32>> = vec![vec![0;c];r];
    for i in 0..r {
        for j in 0..c {
            arr[i][j] = scan.token();
        }
    }
    let mut res: String = String::new();
    if r % 2 == 0 && c % 2 == 0 {
        let (mut min_val, mut min_idx) = (std::u32::MAX, (0, 0));
        for i in 0..(r/2) {
            for j in 0..c {
                let k = if j % 2 == 0 {
                    (i*2)+1
                } else {
                    i*2
                };
                if arr[k][j] < min_val {
                    min_val = arr[k][j];
                    min_idx = (k, j);
                }
            }
        }
        for i in 0..(r/2) {
            if (min_idx.0/2) == i {
                let mut t = 0;
                for j in 0..c {
                    if min_idx.1 != j {
                        if t % 2 == 0 {
                            res.push('D');
                        } else {
                            res.push('U');
                        }
                        t += 1;
                    }
                    if j != c-1 {
                        res.push('R');
                    }
                }
            } else if (min_idx.0/2) < i {
                for j in 0..c {
                    if j != c-1 {
                        res.push('L');
                    }
                }
                res.push('D');
                for j in 0..c {
                    if j != c-1 {
                        res.push('R');
                    }
                }
            } else {
                for j in 0..c {
                    if j != c-1 {
                        res.push('R');
                    }
                }
                res.push('D');
                for j in 0..c {
                    if j != c-1 {
                        res.push('L');
                    }
                }
            }
            if i != (r/2)-1 {
                res.push('D');
            }
        }
    } else if r % 2 == 1 {
        for i in 0..r {
            for j in 0..c {
                if j != c-1 {
                    if i % 2 == 0 {
                        res.push('R');
                    } else {
                        res.push('L');
                    }
                }
            }
            if i != r-1 {
                res.push('D');
            }
        } 
    } else {
        for j in 0..c {
            for i in 0..r {
                if i != r-1 {
                    if j % 2 == 0 {
                        res.push('D');
                    } else {
                        res.push('U');
                    }
                }
            }
            if j != c-1 {
                res.push('R');
            }
        }
    }
    writeln!(sout, "{}", res).ok();
}
