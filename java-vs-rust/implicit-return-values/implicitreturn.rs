fn main() {
    println!("This was implicit: {}", calc_implicit());
    println!("This was explicit: {}", calc_explicit());
}

fn calc_implicit() -> i32 {
    21 + 21
}

fn calc_explicit() -> i32 {
    return 21 + 21
}