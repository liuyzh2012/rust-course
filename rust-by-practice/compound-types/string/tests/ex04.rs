// 4. 🌟🌟🌟

#[test]
fn ex04() {
    // 修复所有错误，并且不要新增代码行
    let mut s = String::from("hello"); // add: mut
    s.push(',');
    s.push_str(" world"); // repl: push -> push_str
    s += "!"; // del: .to_string()

    println!("{}", s)
}
