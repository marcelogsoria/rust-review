fn main() {
    let mut v1: Vec<i32> = vec![1, 2, 3];

    let v2: &mut Vec<i32> = &mut v1;
    v2.push(4);

    /*{
        let v2: &mut Vec<i32> = &mut v1;
        v2.push(4);
        println!("v2: {:?}", v2);
    }*/

    println!("v1: {:?}", v1);
    println!("v2: {:?}", v2);
}
