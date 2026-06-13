// 10. 🌟🌟 loop 是一个表达式，因此我们可以配合 `break` 来返回一个值
// 填空

#[test]
fn ex10() {
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    assert_eq!(result, 20);
}
