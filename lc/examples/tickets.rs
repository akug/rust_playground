
fn buy_tickets(tickets: Vec<i32>, k: i32) -> i32 {
    let mut time: i32 = 0;
    let b = k as usize;
    for (idx, tick) in tickets.iter().enumerate() {
        time += std::cmp::min(*tick, tickets[b] - ((idx > b) as i32));
    }
    return time;
}

fn main() {
    let stud = [5, 1, 1, 20].to_vec();
    println!("{}", buy_tickets(stud, 0));
}

