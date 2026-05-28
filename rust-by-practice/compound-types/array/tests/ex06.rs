// 6. 🌟 越界索引会导致代码的 panic.

#[test]
fn ex06() {
    let names = [String::from("Sunfei"), "Sunface".to_string()];
    
    // `get` 返回 `Option<T>` 类型，因此它的使用非常安全
    let name0 = names.get(0).unwrap();

    // 但是下标索引就存在越界的风险了
    let _name1 = &names[1];
}
