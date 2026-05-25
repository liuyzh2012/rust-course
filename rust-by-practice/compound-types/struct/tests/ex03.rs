// 3. 🌟🌟🌟 元组结构体看起来跟元组很像，但是它拥有一个结构体的名称，该名称可以赋予它一定的意义。由于它并不关心内部数据到底是什么名称，因此此时元组结构体就非常适合。

#[test]
fn ex03() {
    // 填空并修复错误
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);
    fn main() {
        let v = Point(0, 127, 255);
        check_color(v);
    }

    fn check_color(p: Point) { // repl: Color -> Point
        let Point(x, _, _) = p; // add: Point
        assert_eq!(x, 0);
        assert_eq!(p.1, 127);
        assert_eq!(p.2, 255);
     }
}
