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

fn get(
    i: usize,
    j: usize,
    arr: &Vec<Vec<bool>>,
    c1: &mut Vec<(usize, usize)>,
    c2: &mut Vec<(usize, usize)>,
) {
    for k1 in 0..=1 {
        for k2 in 0..=1 {
            if arr[i + k1][j + k2] {
                c2.push((i + k1, j + k2));
            } else {
                c1.push((i + k1, j + k2));
            }
        }
    }
}

fn one(i: usize, j: usize, mut arr: &mut Vec<Vec<bool>>, mut com: &mut Vec<Vec<(usize, usize)>>) {
    let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) = (vec![], vec![]);
    get(i, j, &arr, &mut c1, &mut c2);
    let mut cnt = 0;
    let mut sub_res: Vec<(usize, usize)> = vec![];
    for (x1, y1) in c1 {
        if cnt == 2 {
            break;
        }
        sub_res.push((x1, y1));
        arr[x1][y1] = true;
        cnt += 1;
    }
    sub_res.push(c2[0]);
    arr[c2[0].0][c2[0].1] = false;
    com.push(sub_res);
    two(i, j, &mut arr, &mut com);
}

fn two(i: usize, j: usize, mut arr: &mut Vec<Vec<bool>>, mut com: &mut Vec<Vec<(usize, usize)>>) {
    let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) = (vec![], vec![]);
    get(i, j, &arr, &mut c1, &mut c2);
    let mut cnt = 0;
    let mut sub_res: Vec<(usize, usize)> = vec![];
    for (x1, y1) in c1 {
        if cnt == 2 {
            break;
        }
        sub_res.push((x1, y1));
        arr[x1][y1] = true;
        cnt += 1;
    }
    sub_res.push(c2[0]);
    arr[c2[0].0][c2[0].1] = false;
    com.push(sub_res);
    three(i, j, &mut arr, &mut com);
}

fn three(i: usize, j: usize, mut arr: &mut Vec<Vec<bool>>, mut com: &mut Vec<Vec<(usize, usize)>>) {
    let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) = (vec![], vec![]);
    get(i, j, &arr, &mut c1, &mut c2);
    let mut sub_res: Vec<(usize, usize)> = vec![];
    for (x1, y1) in c2 {
        sub_res.push((x1, y1));
        arr[x1][y1] = false;
    }
    com.push(sub_res);
}

fn four(i: usize, j: usize, mut arr: &mut Vec<Vec<bool>>, mut com: &mut Vec<Vec<(usize, usize)>>) {
    let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) = (vec![], vec![]);
    get(i, j, &arr, &mut c1, &mut c2);
    let mut cnt = 0;
    let mut sub_res: Vec<(usize, usize)> = vec![];
    for (x1, y1) in c2 {
        if cnt == 3 {
            break;
        }
        sub_res.push((x1, y1));
        arr[x1][y1] = false;
        cnt += 1;
    }
    com.push(sub_res);
    one(i, j, &mut arr, &mut com);
}

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let t: usize = scan.token();
    for _case in 0..t {
        let (n, m): (usize, usize) = (scan.token(), scan.token());
        let mut arr2: Vec<Vec<bool>> = vec![vec![false; m]; n];
        for i in 0..n {
            let s_i: String = scan.token();
            let arr_i: Vec<char> = s_i.chars().collect();
            for j in 0..m {
                if arr_i[j] == '1' {
                    arr2[i][j] = true;
                }
            }
        }
        let mut res: Vec<Vec<(usize, usize)>> = vec![];
        let mut len: usize = std::usize::MAX;
        {
            let mut arr = arr2.clone();
            let mut com: Vec<Vec<(usize, usize)>> = vec![];
            if n % 2 == 1 && m % 2 == 1 {
                let (mut i, mut j) = (1, 1);
                while i < (n - 1) {
                    while j < (m - 1) {
                        let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) =
                            (vec![], vec![]);
                        get(i, j, &arr, &mut c1, &mut c2);
                        if c2.len() != 0 {
                            if c2.len() == 1 {
                                one(i, j, &mut arr, &mut com);
                            } else if c2.len() == 2 {
                                two(i, j, &mut arr, &mut com);
                            } else if c2.len() == 3 {
                                three(i, j, &mut arr, &mut com);
                            } else if c2.len() == 4 {
                                four(i, j, &mut arr, &mut com);
                            }
                        }
                        j += 3;
                    }
                    i += 3;
                }
                i = n - 2;
                j = m - 2;
                let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) = (vec![], vec![]);
                get(i, j, &arr, &mut c1, &mut c2);
                if c2.len() != 0 {
                    if c2.len() == 1 {
                        one(i, j, &mut arr, &mut com);
                    } else if c2.len() == 2 {
                        two(i, j, &mut arr, &mut com);
                    } else if c2.len() == 3 {
                        three(i, j, &mut arr, &mut com);
                    } else if c2.len() == 4 {
                        four(i, j, &mut arr, &mut com);
                    }
                }
            }
            for i in 0..(n - 1) {
                for j in 0..(m - 1) {
                    if (i % 2 == 0 && j % 2 == 0)
                        || (n % 2 == 1 && i == n - 2)
                        || (m % 2 == 1 && j == m - 2)
                    {
                        let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) =
                            (vec![], vec![]);
                        get(i, j, &arr, &mut c1, &mut c2);
                        if c2.len() != 0 {
                            if c2.len() == 1 {
                                one(i, j, &mut arr, &mut com);
                            } else if c2.len() == 2 {
                                two(i, j, &mut arr, &mut com);
                            } else if c2.len() == 3 {
                                three(i, j, &mut arr, &mut com);
                            } else if c2.len() == 4 {
                                four(i, j, &mut arr, &mut com);
                            }
                        }
                    }
                }
            }
            if com.len() < len {
                res = com.clone();
                len = com.len();
            }
        }
        {
            let mut arr = arr2.clone();
            let mut com: Vec<Vec<(usize, usize)>> = vec![];
            for i in 0..(n - 1) {
                for j in 0..(m - 1) {
                    if (i % 2 == 0 && j % 2 == 0)
                        || (n % 2 == 1 && i == n - 2)
                        || (m % 2 == 1 && j == m - 2)
                    {
                        let (mut c1, mut c2): (Vec<(usize, usize)>, Vec<(usize, usize)>) =
                            (vec![], vec![]);
                        get(i, j, &arr, &mut c1, &mut c2);
                        if c2.len() != 0 {
                            if c2.len() == 1 {
                                one(i, j, &mut arr, &mut com);
                            } else if c2.len() == 2 {
                                two(i, j, &mut arr, &mut com);
                            } else if c2.len() == 3 {
                                three(i, j, &mut arr, &mut com);
                            } else if c2.len() == 4 {
                                four(i, j, &mut arr, &mut com);
                            }
                        }
                    }
                }
            }
            if com.len() < len {
                res = com.clone();
                len = com.len();
            }
        }
        writeln!(sout, "{}", res.len()).ok();
        for i in 0..res.len() {
            for j in 0..3 {
                write!(sout, "{} {} ", res[i][j].0 + 1, res[i][j].1 + 1).ok();
            }
            write!(sout, "\n").ok();
        }
    }
}
