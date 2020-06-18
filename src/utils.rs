use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
#[derive(Debug)]
struct ConfigStruct {
    bb: Option<String>,
    ee: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct IpConfig {
    name: Option<String>,
    ip: Option<String>,
    port: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct Conf {
    ip_config: Option<Vec<IpConfig>>
}


// struct ConfigAAStruct {
//     // What Vec mean ?
//     aa: Option<Vec<ConfigStruct>>,
//     dd: Option<Vec<ConfigStruct>>,
// }

/**
 * @desc args config.toml path
 *
 */
pub fn args_config(arg_config: String) {
    println!("{}", arg_config + "rustic config=config.toml");
    let file_path = "config.toml";
    let mut file = match File::open(file_path) {
        Ok(f) => f,
        Err(e) => panic!("no such file {} exception:{}", file_path, e)
    };
    let mut str_val = String::new();
    match file.read_to_string(&mut str_val) {
        Ok(s) => s
        ,
        Err(e) => panic!("Error Reading file: {}", e)
    };
    let config: Conf = toml::from_str(&str_val).unwrap();

    for x in config.ip_config.unwrap() {
        println!("{:?}", x);
    }

    // // TODO read config
    // let mut config_toml = match fs::File::open("config.toml") {
    //     Ok(f) => f,
    //     Err(e) => panic!("no such file {} exception:{}", "config.toml", e)
    // };
    // let mut str_val = String::new();
    // match config_toml.read_to_string(&mut str_val) {
    //     Ok(s) => s,
    //     Err(e) => println!("{:?}", e)
    //     // Err(e) => panic!("Error: reading the file :{}", e)
    // }
    // let config: ConfigAAStruct = toml::from_str(&str_val).unwrap();
    // for x in config.aa.unwrap() {
    //     println!("{}", x);
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
