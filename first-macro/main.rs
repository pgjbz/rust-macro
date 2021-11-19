macro_rules! four {
    () => { 1 + 3 };   
}

fn main() {
    let four = four!();
    println!("{}", four);
}
