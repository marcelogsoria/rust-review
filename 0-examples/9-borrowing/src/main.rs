fn main() {
    let y: &i32;
    {
        // <--remove this

        let x = 5;
        y = &x;
    } // <--remove this

    println!("{}", y);
}
