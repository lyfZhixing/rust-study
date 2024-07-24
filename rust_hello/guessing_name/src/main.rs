use std::io; // io 标准库
use rand::Rng; // Rng 随机数生成 第三方库
use std::cmp::Ordering;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..=100); // start..=end 这样的形式，包含了上下端点 1~100
    // println!("the secret number is: {secret_number}" );

    loop {
        println!("Please input your guess.");
        // rust中变量一旦赋值，默认是不可变的, 使用mut关键字声明变量为可变
        let mut guess = String::new();
    
        io::stdin()
            .read_line(&mut guess) // &符号表示引用，使用mut关键字声明引用为可变
            .expect("Failed to read line"); // expect方法用于拦截错误

        // Rust 允许用一个新值来 隐藏 （Shadowing） guess 之前的值
        // 将 expect 调用换成 match 语句，以从遇到错误就崩溃转换为处理错误
        let guess: u32 = match guess.trim().parse(){
            Ok(num) => num,
            Err(_) => {
                println!("Please type a number!");
                continue;
            },
        };
        println!("You guessed: {}", guess);
    
        // match 表达式由 分支（arms） 构成;Rust 获取提供给 match 的值并挨个检查每个分支的模式
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break; // 跳出循环
            },
        }
    }
   
}
