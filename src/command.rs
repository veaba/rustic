use std::{fs};

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
    ?                   : Help alias
    help                : Show help options
    test                : Test=config.toml,test config file is ok
    key                 : Setting key file path
    cert                : Setting cert file path
    config              : Eg: config=config.toml,confif path
    version | v | ver   : Show rustic version number
    author              : List rustic author

    ------------------------------------
    Commands:
    start               : Start rustic server
    restart             : Restart rustic server
    stop                : Stop rusic and exit
    exit                : Stop alias
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

pub fn op_author() {
    print!("rustic author: {}", get_author())
}

fn get_author() -> &'static str {
    "veaba"
}

fn get_version() -> &'static str {
    "v0.0.1"
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
    println!("create app {}", config);
    create_default_page()
}


/**
* @desc create default page
*/
fn create_default_page() {
    let is_has_html_dir = fs::read_dir("html").is_ok();
    if is_has_html_dir {
        let is_read_index_page = fs::read("html/index.html").is_ok();
        let is_read_500_page = fs::read("html/500.html").is_ok();
        let is_read_400_page = fs::read("html/400.html").is_ok();
        if !is_read_index_page {
            fs::write("html/index.html", "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>Index page by Rustic </title><body>Hello world! Index page</body></head></html>").expect("Write `html/index.html` error");
        }
        if !is_read_500_page {
            fs::write("html/500.html", "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>500 page by Rustic </title><body>The Rustic has something wrong</body></head></html>").expect("Write `html/500.html` error");
        }
        if !is_read_400_page {
            fs::write("html/400.html", "<!DOCTYPE html><html lang=\"en\"><head><meta charset=\"UTF-8\"><title>400 page by Rustic </title><body>Rustic no found the page! sorry :) </body></head></html>").expect("Write `html/400.html` error");
        }
    } else {
        fs::create_dir_all("html").expect("Create dir");
        create_default_page()
    }
}


