// 5. 🌟🌟 整数 - 解决代码中的错误和 `panic`

#[test]
fn ex05() {
    let v1 = 247_u8 + 8;
    let v2 = i8::checked_add(119, 8).unwrap();
    println!("{},{}", v1, v2);
}