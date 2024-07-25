fn main() {
    mut_test();
    const_test();
    shadowing_test();
}

fn mut_test() {
    let mut x = 5; // 此处若不加mut关键字将会报错，因为变量默认是不可变： cannot assign twice to immutable variable `x`
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of y is: {x}");
}

fn const_test() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("The value of THREE_HOURS_IN_SECONDS is: {THREE_HOURS_IN_SECONDS}");
}

/**
 * 隐藏（shadowing）与将变量标记为 mut 是有区别的。当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，我们可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
 * mut 与隐藏的另一个区别是，当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字
 */
fn shadowing_test() {
    let x = 5;
    let x = x + 1;
    println!("x = {x}" ); // 6
    {
        let x = x*2;
        println!("The value of x in the inner scope is = {x}" ); // 12
    }
    // 当该作用域结束时，内部 shadowing 的作用域也结束了，x 又返回到 6
    println!("x = {x}" ); // 6

    // 再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型 并且复用这个名字
    // 使用mut修饰则不能改变类型
    let space = "    ";
    let space = space.len();
    println!("space = {space}" ); // 4

}