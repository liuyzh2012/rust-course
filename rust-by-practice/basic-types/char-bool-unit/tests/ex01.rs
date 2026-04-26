// 1. 🌟 修改2处 `assert_eq!` 让代码工作

#[test]
fn ex01() {
    use std::mem::size_of_val;
    let c1 = 'a';
    assert_eq!(size_of_val(&c1), 4);

    let c2 = '中';
    assert_eq!(size_of_val(&c2), 4);

    println!("Success!")
}