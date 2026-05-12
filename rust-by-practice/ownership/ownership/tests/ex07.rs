// 7. 🌟🌟🌟 可变性与 Box
// 完成该行代码，不要修改其它行！

#[test]
fn ex07() {
    let x = Box::new(5);

    let mut y = Box::new(3);      // 完成该行代码，不要修改其它行！

    *y = 4;

    assert_eq!(*x, 5);
}
