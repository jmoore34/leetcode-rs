pub fn longest_common_prefix(strs: Vec<String>) -> String {
    
    let strs: Vec<Vec<char>> = strs.iter().map(|str| str.chars().collect()).collect();

    let mut idx = 0;
    'outer: loop {

        if idx >= strs[0].len() {
            break 'outer;
        }

        let char = strs[0][idx];
        
        for str in &strs {
            if idx >= str.len() || str[idx] != char {
                break 'outer;
            }
        }

        idx += 1;
    }

    return strs[0][0..idx].iter().collect()
}


fn main() {
    dbg!(longest_common_prefix(vec!["flower".into(),"flow".into(),"flight".into()]));
}