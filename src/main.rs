use std::env;
// use crate::utils::is_normal;


pub mod command;
pub mod utils;

fn main() {
    let args = env::args();
    // normal mode
    if args.len() == 1 {
        // TODO default create_app
        command::create_app("hello".to_string());
    } else {
        // 打印参数
        for arg in env::args() {
            println!("==>{}", arg);
            match arg.as_str() {
                // TODO 指令叠加部分
                "config" => {
                    utils::args_config(arg)
                }
                "port" | "listen" => {
                    utils::args_listen(arg)
                }
                "proxy" => {
                    utils::args_proxy(arg)
                }
                // TODO 以下都需要打断
                "?" | "help" | "--help" | "-h" => {
                    command::show_help();
                    break;
                }
                "ver" | "version" | "v" => {
                    command::op_version();
                    break;
                }
                "author" => {
                    command::op_author();
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
                    command::spell_check(arg.to_string())
                }
            }
        }
    }
}
