fn main() {
    let name = String::from("Alice");

    let customer_name = name;
    //let customer_name: &String = &name;
    println!("customer_name: {}", customer_name);

    println!("name: {}", name);
}
