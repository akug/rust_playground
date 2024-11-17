use std::collections::HashMap;

fn valid_paren(s: String)->bool{
    let mut hm = HashMap::from([
        ('(', 0),
        (')', 0),
        ('*', 0),
    ]);
    for c in s.chars() {
        if c == '('{
            hm.entry(c).and_modify(|counter| *counter += 1);
        } else if c == '*' {
            hm.entry(c).and_modify(|counter| *counter += 1);
        }
         else if c == ')' {
            let c_open = hm.get(&'(').unwrap();
            let c_star = hm.get(&'*').unwrap();
            if c_open > &0 {
                hm.entry('(').and_modify(|counter| *counter -= 1);
            } else if  c_star > &0 {
                hm.entry('*').and_modify(|counter| *counter -= 1);
            } else {
                return false;
            }
        }
    }
    if hm.get(&'(').unwrap() > &0 {
        return false;
    }
    // println!("{:?}", hm);
    return true;
}

fn main() {
    let mut s = "(())";
    println!("{}", valid_paren(s.to_string()));
    s = "(*))";
    println!("{}", valid_paren(s.to_string()));
    s = "(";
    println!("{}", valid_paren(s.to_string()));
    s = "******(**";
    println!("{}", valid_paren(s.to_string()));
}