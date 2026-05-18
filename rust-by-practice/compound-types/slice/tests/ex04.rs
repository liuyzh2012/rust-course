// 4. 🌟 字符串切片

#[test]
fn ex04() {
    let s = String::from("hello");

    let slice1 = &s[0..2];
    // 填空，不要再使用 0..2
    let slice2 = &s[..2];

    assert_eq!(slice1, slice2);
}
