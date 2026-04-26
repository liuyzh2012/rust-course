// 2. 🌟 修改一行让代码正常打印

fn print_char(c: char) {
    println!("{}", c);
}

#[test]
fn ex02() {
    let c1 = '中'; // let c1 = "中";
    print_char(c1);
}