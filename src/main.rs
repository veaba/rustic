use std::env;

pub mod command;
fn main() {
    println!("Hello, world!");

    // 打印参数
    for arg in env::args() {
        println!("==>{}", arg);

        if arg == "?" {
            command::show_help();
            break;
        }
        
        if arg=="ver"|| arg=="version" || arg=="v"{
            command::op_version()
        }
    
    }
}
