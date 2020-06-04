/**
 * stop rustic
 *
*/
fn stop() {}

/**
 * quit rustic
 *
*/
fn quit() {}

/**
 * start rustic
 *
*/

fn start() {}

/**
 * restart rustic
 *
*/

fn restart() {}

/**
 * show help command options
 * @run cargo run help
 *
*/
fn help() {}

/**
 * show help command options,alias help
 * @run cargo run ?
 *
*/
fn question_mask() {}

pub fn show_help() {
    print!(
        "
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
    print!("rustic verison: {}",get_version())
}

fn get_version()->(&'static str) {
    return "v0.0.1"
}

/**
 * @desc spell，拼写检查
 *
*/
fn spell_check() {}
