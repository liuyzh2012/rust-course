// 7. 使用以下方法来修复编译器输出的 warning:
//    🌟 一种方法
//    🌟🌟 两种方法
// 注意: 你可以使用两种方法解决，但是它们没有一种是移除 `let x = 1` 所在的代码行
// compiler warning: unused variable: `x`

#[test]
#[allow(unused_variables)]
fn ex07() {
    let _x = 1;
}