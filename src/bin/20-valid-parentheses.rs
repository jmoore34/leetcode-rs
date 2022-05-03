pub fn is_valid(s: String) -> bool {
    let mut stack = vec![];

    for c in s.chars() {
        match c {
            '(' | '{' | '[' => stack.push(c),
            ']' => if stack.pop().unwrap_or('$') != '[' { return false }
            '}' => if stack.pop().unwrap_or('$') != '{' { return false }
            ')' => if stack.pop().unwrap_or('$') != '(' { return false }
            _ => {}
        }
    }

    stack.is_empty()
}

fn main() {
    dbg!(is_valid("print(hello ${worlds[0]})".into()));
}