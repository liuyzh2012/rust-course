// 5. 🌟🌟 让代码工作，但不要修改 `implicitly_ret_unit` !

fn implicitly_ret_unit() {
    println!("I will return a ()")
}

fn explicitly_ret_unit() -> () {
    println!("I will return a ()")
}

#[test]
fn ex05() {
    let v0: () = ();

    let v = (2, 3);
    assert_eq!(v0, implicitly_ret_unit());

    println!("Success!")
}