// 6. 🌟 你可以使用结构体更新语法基于一个结构体实例来构造另一个

#[test]
fn ex06() {
    // 填空，让代码工作
    struct User {
        active: bool,
        username: String,
        email: String,
        sign_in_count: u64,
    }
    fn main() {
        let u1 = User {
            email: String::from("someone@example.com"),
            username: String::from("sunface"),
            active: true,
            sign_in_count: 1,
        };

        let u2 = set_email(u1);
    }

    fn set_email(u: User) -> User {
        User {
            email: String::from("contact@im.dev"),
            ..u
        }
    }
}
