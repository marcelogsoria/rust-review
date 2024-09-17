fn skip_prefix(line: &str, prefix: &str) -> &str {
    // ^ it doesn't work
    // fn skip_prefix<'a>(line: &'a str, prefix: &'a str) -> &'a str {   //it doesn't work
    // fn skip_prefix<'a, 'b>(line: &'a str, prefix: &'b str) -> &'a str { //it works
    if line.starts_with(prefix) {
        return &line[prefix.len()..];
    } else {
        return line;
    }
}

fn main() {
    let line = "lang:en=Hello World!";
    let lang = "en";

    let v;
    {
        let p = format!("lang:{}=", lang); // -+ `p` comes into scope.
        v = skip_prefix(line, p.as_str()); //  |
    } // -+ `p` goes out of scope.
    println!("{}", v);
}
