// 1. 🌟🌟 基本操作

#[test]
fn ex01() {
    // 填空并修复错误
    // 1. 不要使用 `to_string()`
    // 2. 不要添加/删除任何代码行
    let mut s: String = String::from("hello, ");
    s.push_str("world"); // del: .to_string()
    s.push('!');

    move_ownership(s.clone()); // add: .clone()

    assert_eq!(s, "hello, world!");

    println!("Success!")
}

fn move_ownership(s: String) {
    println!("ownership of \"{}\" is moved here!", s)
}
