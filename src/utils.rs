use std::fs::File;
use std::io::Read;
use toml::Value;

#[derive(Deserialize)]
#[derive(Debug)]
struct ConfigStruct {
    config: Option<String>,
    root: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct RootStruct {
    root: Option<String>,
    server_root: Option<String>,
    client_root: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct HttpStruct {
    port: Option<u32>
}

#[derive(Deserialize)]
#[derive(Debug)]
struct ProxyStruct {
    proxy: Option<String>
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Conf {
    config: Option<ConfigStruct>,
    root: Option<RootStruct>,
    http: Option<HttpStruct>,
    proxy: Option<ProxyStruct>,
}

/**
 * @desc args config.toml path
 *
 */
pub fn args_config(arg_value: &String) {
    println!("运行的命令是 ===>{:?}", arg_value);
    let file_path = arg_value;
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("One !! no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s,
        Err(e) => panic!("Two ! !Error Reading file: {}", e)
    };

    println!("读取Config.toml ==>{}{}", str_val, "\n");

    let v = str_val.parse::<Value>().unwrap();
    println!("vvv==>{}", v["proxy"])


    // let config: Conf = toml::from_str(&str_val).unwrap();
    // println!("打印config struct==?{:#?}", config)
    // for x in config.http.unwrap() {
    //     println!("===>{:?}", x);
    // }
}

/**
 * @desc args listen port
 */

pub fn args_listen(arg_port: String) {
    println!("{}", arg_port + "rustic config=config.toml")
}

/**
 * @desc args reverse proxy
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
// fn is_relative_path() {
// use std::path::Path;
// assert!(Path::new("foo.txt").is_relative());
// }
//
// fn is_windows() {}
//
// fn is_linux() {}

// /**
//  * @desc args 选项命令错误,check it's legal
//  *
// */
// fn args_options_error() {}


/**
* @desc 返回不定长长度的数组
* @return ["config=config.toml","port=80","proxy=/api/"]
* @todo 正则: https://www.cnblogs.com/zwvista/p/12783743.html
*/
// [&'static str; 2] 定长2个长度的字符串数组
// &mut [String] 干嘛的？
// pub fn arg_result(arg: &String) -> Vec<&str> {
//     let vec_array = arg_array(&arg);
//     return vec_array;
// }

/**
* @param arg_string
* @desc check arg is:  xx=oo
* @error
    "xx=oo x"
    "xx=oo=xx"
* @support
    "xx=oo "
    "xx=oo xx"
* @return {bool}
*/
pub fn arg_array(arg_string: &String) -> Vec<&str> {
    let mut vec_array = vec![];
    let str_split = arg_string.split("=");
    for code in str_split {
        vec_array.push(code.trim())
    };
    return vec_array;
}

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

pub fn is_config() {}

pub fn is_listen() {}

pub fn is_proxy() {}
