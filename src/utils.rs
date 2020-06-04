/**
 * @desc args config 的路径
 *
 */
fn args_config() {}


/**
 * @desc args listen 端口
 */

fn args_listen() {}

/**
 * @desc args proxy 反向代理
 *
 */
fn args_proxy() {}

/**
 * @desc args 选项命令错误,check it's legal
 *
*/
fn args_options_error() {}


/**
 * 判断是绝对路径还是相对路径
 *
*/

fn is_absolution_path() {}

fn is_relative_path() {}

fn is_windows() {}

fn is_linux() {}

/**
* @desc 返回两个结果，
* {
*   argName,
*   path
* }
* @todo 正则: https://www.cnblogs.com/zwvista/p/12783743.html
*/
pub fn arg_result(arg: String) -> tuple {
    return ("a", "2");
}

