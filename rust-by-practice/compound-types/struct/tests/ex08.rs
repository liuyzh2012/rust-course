// 8. 🌟🌟 部分move：当解构一个变量时，可以同时使用 move 和引用模式绑定的方式。

#[test]
fn ex08() {
    // 修复错误
    #[derive(Debug)]
    struct File {
        name: String,
        data: String,
    }
    fn main() {
        let f = File {
            name: String::from("readme.md"),
            data: "Rust By Practice".to_string()
        };

        let _name = f.name;

        // 只能修改这一行
        println!("{}", f.data); // println!("{}, {}, {:?}",f.name, f.data, f);
    }
}
