#[macro_use]
extern crate serde_derive;
extern crate toml;

use std::env;

pub mod command;
pub mod utils;

fn main() {
    let args = env::args();
    // normal mode
    if args.len() == 1 {
        // TODO default create_app
        command::create_app("hello".to_string());
    } else {
        // TODO 打印参数, 这里的arg 指的是struct吗？
        for arg in env::args() {
            // TODO config=config.toml ==> config
            // TODO port=80 ==> port
            let arg_array = utils::arg_array(&arg);
            if arg_array.len() == 2 {
                let arg_cmd = arg_array[0];
                let arg_value = String::from(arg_array[1]);
                match arg_cmd {
                    // TODO 指令叠加部分
                    "config" => {
                        utils::args_config(&arg_value);
                    }
                    //
                    // "port" | "listen" => {
                    //     utils::args_listen(arg_array.parse().unwrap());
                    // }
                    // "proxy" => {
                    //     utils::args_proxy(arg_array.parse().unwrap());
                    // }
                    _ => {}
                }
            }


            // 单个指令
            match arg.as_str() {
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
                    command::spell_check(arg.to_string());
                    // break 如何组织错误的命令继续执行
                }
            }
        }
    }
}
