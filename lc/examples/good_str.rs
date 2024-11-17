fn main() {
    let s: &'static str = "leEetcode";
    for c in s.chars(){
        println!("{}", c);
    }
}