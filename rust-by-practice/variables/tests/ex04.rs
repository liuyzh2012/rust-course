// 4. 🌟🌟 修复错误

fn define_x() {
    let x = "hello";
    println!("{}, world", x);
}

fn define_x_string() -> String {
    let x = "hello".to_string();
    x
}

fn define_x_str() -> &'static str {
    let x = "hello";
    x
}

#[test]
fn ex04() {
    define_x();

    let x = define_x_string();
    println!("{}, world", x);

    let x = define_x_str();
    println!("{}, world", x);
}