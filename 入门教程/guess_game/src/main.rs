use std::io; // prelude

fn main() {
    println!("请猜数！");

    println!("猜测一个数");

    // let mut fool = 1;
    // let bar = fool; // immutable

    // fool = 2;

    let mut guess = String::new();

    io::stdin().read_line(&mut guess).expect("无法读取行");

    println!("你猜测的数字是：{}", guess)
}
