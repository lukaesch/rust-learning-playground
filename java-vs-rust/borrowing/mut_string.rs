fn main() {
    let mut company: String = String::from("SoTrusty");
    modify_company(&mut company);
    println!("String is now: {}", company);
}

fn modify_company(company: &mut String) {
    println!("String is: {}", company);
    company.push_str(" is a great company");
}