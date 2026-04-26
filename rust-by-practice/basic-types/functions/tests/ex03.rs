// 3. 🌟🌟🌟 函数 - 用两种方法实现发散函数
// 提示: 不要修改函数签名

#[test]
fn ex03() {
    never_return();
}

/*
fn never_return() -> ! {
    // 实现这个函数，不要修改函数签名!
    panic!("I return nothing!")
}
*/

use std::thread;
use std::time;

fn never_return() -> ! {
    // implement this function, don't modify fn signatures
    loop {
        println!("I return nothing!");
        // sleeping for 1 second to avoid exhausting the cpu resource
        thread::sleep(time::Duration::from_secs(1))
    }
}