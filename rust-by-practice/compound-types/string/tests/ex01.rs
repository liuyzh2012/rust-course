// 1. 🌟 正常情况下我们无法使用 `str` 类型，但是可以使用 `&str` 来替代

#[test]
fn ex01() {
    // 修复错误，不要新增代码行
    let s: &str = "hello, world";
}
