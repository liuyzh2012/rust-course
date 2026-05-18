// 1. 🌟🌟 这里, `[i32]` 和 `str` 都是切片类型，但是直接使用它们会造成编译错误，如下代码所示。为了解决，你需要使用切片的引用： `&[i32]`，`&str`。

#[test]
fn ex01() {
    // 修复代码中的错误，不要新增代码行!
    let arr = [1, 2, 3];
    let s1: &[i32] = &arr[0..2];

    let s2: &str = "hello, world" as &str;
    let s2: &str = "hello, world";
}
