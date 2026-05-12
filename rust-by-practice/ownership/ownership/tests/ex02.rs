// 2. 🌟🌟 所有权转移：函数传参
// 不要修改 main 中的代码

#[test]
fn ex02() {
    let s1 = String::from("hello, world");
    let s2 = take_ownership(s1);

    println!("{}", s2);
}

// 只能修改下面的代码!
fn take_ownership(s: String) -> String {
    println!("{}", s);
    s
}
