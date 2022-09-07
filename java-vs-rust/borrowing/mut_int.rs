fn main() {
    let mut x = 5;
    add_five(&mut x);
    println!("x is now {}", x);
}

fn add_five(x: &mut i32) {
    *x += 5
}