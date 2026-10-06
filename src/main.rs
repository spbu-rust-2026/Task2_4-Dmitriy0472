use std::io::{self, Read};

fn main() {
    let mut text = String::new();
    io::stdin()
        .read_to_string(&mut text)
        .expect("failed to read input");

    let text = text.trim_end_matches(|character| character == '\n' || character == '\r');
    println!("{}", longest_palindromic_substring(text));
}

fn longest_palindromic_substring(text: &str) -> &str {
    let bytes = text.as_bytes();
    let n = bytes.len();
    if n == 0 {
        return "";
    }

    let mut t: Vec<u8> = Vec::with_capacity(2 * n + 1);
    t.push(b'#');
    for &b in bytes {
        t.push(b);
        t.push(b'#');
    }
    let m = t.len();

    let mut p = vec![0usize; m];
    let (mut center, mut right) = (0usize, 0usize);
    let (mut best_len, mut best_center) = (0usize, 0usize);

    for i in 0..m {
        if i < right {
            let mirror = 2 * center - i;
            p[i] = p[mirror].min(right - i);
        }

        while i + p[i] + 1 < m && i >= p[i] + 1 && t[i + p[i] + 1] == t[i - p[i] - 1] {
            p[i] += 1;
        }

        if i + p[i] > right {
            center = i;
            right = i + p[i];
        }

        if p[i] > best_len {
            best_len = p[i];
            best_center = i;
        }
    }

    let start = (best_center - best_len) / 2;
    &text[start..start + best_len]
}

#[cfg(test)]
#[path = "../tests/unit/palindrome.rs"]
mod palindrome_tests;
