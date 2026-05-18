// 5. 🌟🌟🌟
// 提示: 也许你需要使用 `from_utf8` 方法

#[test]
fn ex05() {
    // 填空
    let mut s = String::new();
    s.push_str("hello");

    let v = vec![104, 101, 108, 108, 111];

    // 将字节数组转换成 String
    let s1 = String::from_utf8(v).unwrap();
    
    
    assert_eq!(s, s1);

    println!("Success!")
}
