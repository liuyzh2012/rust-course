// 8. 🌟 错误: 从不可变对象借用可变

#[test]
fn ex08() {
    // 通过修改下面一行代码来修复错误
    let mut s = String::from("hello, ");

    borrow_object(&mut s)
}

fn borrow_object(s: &mut String) {}
