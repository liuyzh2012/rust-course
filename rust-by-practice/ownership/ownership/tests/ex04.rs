// 4. 🌟🌟 修复错误，不要删除任何代码行

#[test]
fn ex04() {
    let s = String::from("hello, world");

    print_str(s.clone());

    print_str_ref(&s);

    println!("{}", s);
}

fn print_str(s: String)  {
    println!("{}", s)
}

fn print_str_ref(s: &String)  {
    println!("{}", s)
}
