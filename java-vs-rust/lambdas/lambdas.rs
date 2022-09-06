fn main() {
    let companies = ["SoTrusty", "Apple", "Microsoft"];

    let result = companies
        .iter()
        .map(|company| format!("{} is a great company", company));

    for company in result {
        println!("{}", company);
    }
}
