// 3. 🌟 修复错误

#[test]
fn ex03() {
    // 修复错误
    let mut s = String::from("hello, ");

    borrow_object(&s)
}

fn borrow_object(_s: &String) {}
