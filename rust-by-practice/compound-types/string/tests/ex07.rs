// 7. 🌟🌟 我们可以使用两种方法将 `&str` 转换成 `String` 类型

#[test]
fn ex07() {
    // 使用至少两种方法来修复错误
    let s = "hello, world".to_string();
    greetings(s);
    let s = String::from("hello, world");
    greetings(s);
}

fn greetings(s: String) {
    println!("{}",s)
}
