fn process_array(v1: &Vec<i32>, v2: &Vec<i32>) -> (i32) {
    // Do stuff with `v1` and `v2`.

    // Hand back ownership, and the result of our function.
    42
}

fn main() {
    let v1 = vec![1, 2, 3];
    let v2 = vec![3, 5, 6];

    let answer = process_array(&v1, &v2);

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
    println!("answer: {}", answer);
}


