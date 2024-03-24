pub fn iterate(x: usize) {
    let range = vec![0; x];
    // println!("{}",x)
    // println!("{:?}",range)
    for i in range.iter() {
        println!("{}", i)
    }
}
