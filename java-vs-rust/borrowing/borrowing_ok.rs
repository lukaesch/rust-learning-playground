fn main () {
    let v = vec![1, 2, 3];
    print_vector(&v); 
    println!("Printing from main {}", v[0]); //This works
}

fn print_vector(v: &Vec<i32>) {
    println!("Inside print_vector {:?}", v);
}