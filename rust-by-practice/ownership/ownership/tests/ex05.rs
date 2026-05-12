// 5. 🌟🌟 不要使用 clone，使用 copy 的方式替代

#[test]
fn ex05() {
    // let x = (1, 2, (), "hello".to_string());
    // let y = x.clone();
    let x = (1, 2, (), "hello");
    let y = x;
    println!("{:?}, {:?}", x, y);
}
