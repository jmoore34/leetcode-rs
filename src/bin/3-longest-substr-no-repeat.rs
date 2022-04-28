use std::ops::Range;

pub fn length_of_longest_substring(s: String) -> i32 {
    let s: Vec<char> = s.chars().collect();

    let mut right: usize = 0;

    let mut max_len = 0;

    let mut used_chars = [false; 128];

    (0..s.len()).for_each(|left| {
        while right < s.len() && !used_chars[s[right] as usize] {
            used_chars[s[right] as usize] = true;
            right += 1;
        }
        used_chars[s[left] as usize] = false;
        max_len = (right - left).max(max_len);
    });

    max_len as i32
}

fn main() {
    dbg!(length_of_longest_substring("dvdf".into()));
}
