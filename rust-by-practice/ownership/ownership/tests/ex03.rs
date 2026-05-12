// 3. 🌟🌟 所有权转移：函数返回值
// 只能修改下面的代码!

#[test]
fn ex03() {
    let s = give_ownership();
    println!("{}", s);
}

fn give_ownership() -> String {
    let s = String::from("hello, world");
    // convert String to Vec
    // 将 String 转换成 Vec 类型
    // let _s = s.into_bytes();
    s
}
