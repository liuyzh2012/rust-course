// 8. 🌟🌟 我们可以使用 `String::from` 或 `to_string` 将 `&str` 转换成 `String` 类型

#[test]
fn ex08() {
    // 使用两种方法来解决错误，不要新增代码行
    let s = "hello, world".to_string();
    let s1: &str = &s;
    let s = "hello, world";
    let s1: &str = &s;
}
