// 3. 🌟 数组中的所有元素可以一起初始化为同一个值

#[test]
fn ex03() {
    // 填空
    let list: [i32; 100] = [1; 100];

    assert!(list[0] == 1);
    assert!(list.len() == 100);
}
