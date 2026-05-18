// 6. 🌟🌟 如果 String 的当前容量足够，那么添加字符将不会导致新的内存分配

#[test]
fn ex06() {
    // 修改下面的代码以打印如下内容: 
    // 25
    // 25
    // 25
    // 循环中不会发生任何内存分配
    let mut s = String::with_capacity(25); // add: with_capacity 25

    println!("{}", s.capacity());

    for _ in 0..2 {
        s.push_str("hello");
        println!("{}", s.capacity());
    }

    println!("Success!")
}
