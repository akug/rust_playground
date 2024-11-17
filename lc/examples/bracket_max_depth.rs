fn main() {
    let s: &'static str = "(wac)(dwa(c9)dc)";
    let mut max_depth: i32 = 0;
    let mut p_open: i32 = 0;
    for c in s.chars() {
        if c == '(' {
            p_open += 1;
            if p_open > max_depth {
                max_depth = p_open;
            }
        } else if c == ')' {
            p_open -= 1;
        }
    }
    println!("{}", max_depth)
}
