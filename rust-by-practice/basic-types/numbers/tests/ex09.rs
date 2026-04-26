// 9. 🌟🌟 序列Range - 两个目标: 1. 修改 assert! 让它工作 2. 让 println! 输出: 97 - 122

#[test]
fn ex09() {
    let mut sum = 0;
    for i in -3..2 {
        sum += i
    }

    assert!(sum == -5);

    for c in 'a'..='z' {
        println!("{}", c as u8);
    }
}