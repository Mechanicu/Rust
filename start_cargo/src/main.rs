use std::io;
use rand::Rng;

fn main() {
    println!("Hello, world!");

    let low_range: i32 = 0;
    let const_addr = &low_range;

    println!("const addr:{}", const_addr);
    let mut high_range: i32 = 100;
    let stack_addr = &high_range;
    println!("stack addr:{}", stack_addr);
    let rand_num = rand::thread_rng().gen_range(low_range..high_range);

    let mut buffer: String = String::new();
    println!("stack addr:{}", &buffer);
    let err_msg: String = String::from("input error");

    println!("Enter a number:");
    io::stdin().read_line(&mut buffer).expect(&err_msg);

    println!("your input:{}, rand number:{}", buffer, rand_num);
}
