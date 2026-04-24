// 2. 🌟🌟 可以使用 `mut` 将变量标记为可变
// 完形填空，让代码编译

#[test]
fn ex02() {
    let mut x = 1;
    x += 2;

    println!("x = {}", x);
}