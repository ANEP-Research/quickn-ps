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

fn calculate_pi(p: Vec<char>) -> Vec<usize> {
    let mut cur = 0;
    let mut res: Vec<usize> = vec![0;p.len()+1];
    for i in 1..p.len() {
        while cur > 0 && p[i] != p[cur] {
            cur = res[cur-1];
        }
        if p[i] == p[cur] {
            cur += 1;
        }
        res[i] = cur;
    }
    res
}

fn match_str(s: Vec<char>, p: Vec<char>) -> Vec<(usize, usize)> {
    let mut cur = 0;
    let pi = calculate_pi(p.clone());
    let mut res: Vec<(usize, usize)> = vec![];
    for i in 0..s.len() {
        while cur > 0 && p[cur] != s[i] {
            cur = pi[cur-1];
        }
        if s[i] == p[cur] {
            cur += 1;
        }
        if cur == p.len() {
            res.push((i + 1 - p.len(), i));
            cur = pi[cur-1];
        }
    }
    res
}

const ALPHABETS: usize = 26;
const NIL: usize = std::usize::MAX;

struct Trie {
    n: usize,
    node: Vec<bool>,
    go: Vec<[usize;ALPHABETS]>,
    res: usize,
}

impl Trie {
    fn new() -> Self {
        Self {
            n: 1,
            node: vec![false],
            go: vec![[NIL;ALPHABETS]],
            res: 0,
        }
    }

    fn insert(&mut self, s: Vec<u8>, endpoint: Vec<usize>) {
        let mut node = 0;
        let mut i = 0;
        let mut ep = endpoint;
        while i < s.len() {
            if self.go[node][s[i] as usize] == NIL {
                self.go[node][s[i] as usize] = self.n;
                self.node.push(false);
                self.go.push([NIL;ALPHABETS]);
                node = self.n;
                self.n += 1;
            } else {
                node = self.go[node][s[i] as usize];
            }
            if let Some(&j) = ep.last() {
                if j == i {
                    if !self.node[node] {
                        self.res += 1;
                    }
                    self.node[node] = true;
                    ep.pop();
                }
            }
            i += 1;
        }
    }
}

use std::collections::HashSet;

fn main() {
    let (stdin, stdout) = (io::stdin(), io::stdout());
    let (mut scan, mut sout) = (
        scanner::UnsafeScanner::new(stdin.lock()),
        BufWriter::new(stdout.lock()),
    );
    let s: String = scan.token();
    let a: String = scan.token();
    let b: String = scan.token();
    let arr: Vec<char> = s.chars().collect();
    let mut res1 = match_str(arr.clone(), a.chars().collect());
    let mut res2 = match_str(arr.clone(), b.chars().collect());
    let mut trie = Trie::new();
    res1.sort_by(|(a1, b1), (a2, b2)| a1.cmp(&a2).then_with(|| b1.cmp(&b2)));
    res2.sort_by(|(a1, b1), (a2, b2)| a1.cmp(&a2).then_with(|| b1.cmp(&b2)));
    //dbg!(res1.clone());
    for (s1, e1) in res1.clone() {
        let mut j = s1;
        let mut s: Vec<u8> = vec![];
        let mut v: Vec<usize> = vec![];
        for (s2, e2) in res2.clone() {
            if s1 <= s2 && e1 <= e2 {
                while j < e2 {
                    s.push((arr[j] as u8) - ('a' as u8));
                    j += 1;
                }
                s.push((arr[j] as u8) - ('a' as u8));
                v.push(s.len()-1);
                j += 1;
            }
        }
        v.reverse();
        trie.insert(s, v);
    }
    writeln!(sout, "{}", trie.res).ok();
}
