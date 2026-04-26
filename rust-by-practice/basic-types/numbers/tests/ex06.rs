// 6. 🌟🌟 整数 - 修改 `assert!` 让代码工作

#[test]
fn ex06() {
    let v = 1_024 + 0xff + 0o77 + 0b1111_1111;
    assert!(v == 1597);
}