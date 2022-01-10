fn main() {
    let s = "t1,t2".to_string();
    let arr: Vec<&str> = s.split(',').collect();
    dbg!(arr);

    for item in s.split(',') {
        dbg!(item);
    }
}
