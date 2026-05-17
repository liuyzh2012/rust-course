// 6. 🌟🌟 你只能将 `String` 跟 `&str` 类型进行拼接，并且 `String` 的所有权在此过程中会被 move

#[test]
fn ex06() {
    // 修复所有错误，不要删除任何一行代码
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1.clone() + &s2; // add: .clone() and &
    assert_eq!(s3, "hello,world!");
    println!("{}", s1);
}
