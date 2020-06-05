// use regex::Regex;

/**
 * @desc args config 的路径
 *
 */
pub fn args_config(arg_config: String) {
    println!("{}", arg_config + "rustic config=config.toml")
}


/**
 * @desc args listen 端口
 */

pub fn args_listen(arg_port: String) {
    println!("{}", arg_port + "rustic config=config.toml")
}

/**
 * @desc args proxy 反向代理
 *
 */
pub fn args_proxy(arg_proxy: String) {
    println!("{}", arg_proxy + "rustic config=config.toml")
}

// /**
//  * 判断是绝对路径还是相对路径
//  *
// */

// fn is_absolution_path() {}
//
// fn is_relative_path() {}
//
// fn is_windows() {}
//
// fn is_linux() {}

// /**
//  * @desc args 选项命令错误,check it's legal
//  *
// */
// fn args_options_error() {}


// /**
// * @desc 返回两个结果，
// * {
// *   argName,
// *   path
// * }
// * @todo 正则: https://www.cnblogs.com/zwvista/p/12783743.html
// */
// pub fn arg_result(arg: String) -> tuple {
//     return ("a", "2");
// }

// /**
// * @desc 如果以 rustic 执行，后面不带任何参数，则认为是以默认模式下运行
// * @return {bool}
// */

// pub fn is_normal(arg_name: String) -> bool {
//     let reg_first = Regex::new(r"^.+(.+?)\\").unwrap();
//     let filename_suffix = reg_first.replace_all(arg_name.as_str(), "");
//     let reg_second = Regex::new(r"\..*$").unwrap();
//     let filename = reg_second.replace_all(&filename_suffix, "");
//     println!("==?{}", filename);
//     return filename == "rustic";
// }
