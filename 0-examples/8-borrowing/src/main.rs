fn main() {
    let v1 = vec![1, 2, 3];
    // let mut v1 = vec![1, 2, 3];

    let v2 = &v1;
    // let v2: &mut Vec<i32> = &mut v1;

    v2.push(4);
}
