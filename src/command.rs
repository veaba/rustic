use std::{env, fs};
use std::path::Path;

/**
 * stop rustic
 *
*/
pub fn stop() {
    println!("Stop rustic")
}

/**
 * quit rustic
 *
*/
pub fn quit() {
    println!("Quit rustic")
}

/**
 * start rustic
 *
*/

pub fn start() {
    println!("Start rustic")
}

/**
 * restart rustic
 *
*/

pub fn restart() {
    println!("Restart rustic")
}

/**
 * show help command options
 * @run cargo run help
 *
*/
pub fn help() {
    println!("Help Options")
}

/**
 * show help command options,alias help
 * @run cargo run ?
 *
*/
pub fn question_mask() {
    println!("Question mask")
}

pub fn show_help() {
    print!("
    Rustic version: rustic/0.0.1

    Options:
    ?                   : help alias
    help                : show help options
    test                : test=config.toml,test config file is ok
    key                 : setting key file path
    cert                : setting cert file path
    config              : eg: config=config.toml,confif path
    version | v | ver   : 0.0.1
    author              ：veaba

    ------------------------------------
    Commands:
    start               : start rustic server
    restart             : restart rustic server
    stop                : stop rusic and exit
    exit                : stop alias
    "
    )
}

/**
 * show version number
 * @run cargo run version
 * or: cargo run v
 * or: cargo run ver
 *
*/
pub fn op_version() {
    print!("rustic version: {}", get_version())
}

fn get_version() -> &'static str {
    return "v0.0.1";
}

/**
 * @desc spell，拼写检查
 *
*/
pub fn spell_check(arg: String) {
    // TODO 正则移除后缀不是rustic.exe，提取前缀是rustic
    println!("\n{}",
             arg + ": Not a rustic command. try run: ‘rustic help’ see help!"
    );
}

/**
* @desc create instance
*/
pub fn create_app(config: String) {
    println!("Create app  config==>{}", config);

    let is_hasHtml_dir = fs::read_dir("html").is_ok();

    if is_hasHtml_dir {
        let is_read_index_page=fs::read("html/index.html").is_ok();
        // let is_read_500_page=fs::read("html/500.html").is_ok();
        // let is_read_400_pag=fs::read("html/400.html").is_ok();
        if !is_read_index_page{
            fs::write("html/index.html","<!doctype>\
            <html> \
            <head>\
            <title> rustic is ok! Home page!</title> \
            </head>\
            <body>\
              <h1>Hello world!</h1>\
            </body>\
            </html>\
            ")
        }
    } else {
        // TODO if no has ,will be create html folder
        //
    }
    // TODO if has html folder
}




