use std::env;
fn main() {
    println!("Hello, world!");


    // 打印参数
    for argument in env::args(){
        println!("==>{}",argument)
    }
}
