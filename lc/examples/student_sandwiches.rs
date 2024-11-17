
fn count_students(students: Vec<i32>, sandwiches: Vec<i32>) -> i32 {
    let mut s0 = 0;
    let mut s1 = 0;
    for s in students.iter() {
        if s == &0 {
            s0 += 1;
        } else {
            s1 += 1;
        }
    }
    for s in sandwiches.iter() {
        if s == &0 {
            s0 -= 1;
            if s0 < 0 {
                return s1;
            }
        } else {
            s1 -= 1;
            if s1 < 0 {
                return s0;
            }
        }
    }
    return 0;
}

fn main() {
    let stud = [0, 1, 1, 1].to_vec();
    let sand = [1, 1, 0, 0].to_vec();
    println!("{}", count_students(stud, sand));
}
