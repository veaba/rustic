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
    )
}
