// 6. 🌟🌟🌟 ref 与 & 类似，可以用来获取一个值的引用，但是它们的用法有所不同。

#[test]
fn ex06() {
    let c = '中';

    let r1 = &c;
    // 填写空白处，但是不要修改其它行的代码
    let ref r2 = c;

    assert_eq!(*r1, *r2);

    // 判断两个内存地址的字符串是否相等
    assert_eq!(get_addr(r1), get_addr(r2));
}

fn get_addr(r: &char) -> String {
    format!("{:p}", r)
}
