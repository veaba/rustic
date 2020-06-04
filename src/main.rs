use std::env;

pub mod command;

fn main() {
    println!("Hello, world!");

    // 打印参数
    for arg in env::args() {
        println!("==>{}", arg);

        match arg.as_str() {

            // TODO 指令叠加部分
            "config" => {

            }
            // TODO 以下都需要打断
            "?" => {
                command::show_help();
                break;
            }
            "ver" | "version" | "v" => {
                command::op_version();
                break;
            }

            // restart server
            "start" => {
                command::start();
                break;
            }
            // restart server
            "restart" => {
                command::restart();
                break;
            }

            // stop server
            "stop" => {
                command::stop();
                break;
            }

            // quit server
            "quit" => {
                command::quit();
                break;
            }
            _ => {
                command::spell_check(arg);
            }
        }
    }
}
