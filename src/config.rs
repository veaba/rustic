use std::fs::File;
use std::io::Read;

#[derive(Deserialize)]
#[derive(Debug)]
struct BaseModule {
    config_path: Option<String>,
    access_log: Option<bool>,
    error_log: Option<bool>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct SecurityModule {
    key: Option<String>,
    cert: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct HttpModule {
    server: Option<Vec<ServerModule>>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct ServerModule {
    port: Option<u16>,
    listen: Option<u16>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct ProxyModule {
    api: Option<String>,
}

#[derive(Deserialize)]
#[derive(Debug)]
struct LogsModule {
    access: Option<String>,
    error: Option<String>,
}


#[derive(Deserialize)]
#[derive(Debug)]
struct ConfigModule {
    base: Option<BaseModule>,
    security: Option<SecurityModule>,
    http: Option<HttpModule>,
    proxy: Option<ProxyModule>,
    logs: Option<LogsModule>,
}


/**
 * @desc args config.toml path
 *
 */
pub fn args_config(arg_value: &String) {
    let (default_config, merge_config) = get_merge_config(&arg_value);
    // println!("merge config{:#?} \n{:#?}", default_config, merge_config);
    // let config_path = default_config.base.unwrap().config_path | merge_config.base.unwrap().config_path;
    println!("a=>?{:#?}", default_config.base.unwrap().config_path);
    println!("b=>?{:#?}", merge_config.base.unwrap().config_path);
    // println!("config_path==>{:#?}", config_path);
}


/**
* @desc default config
*/
fn get_default_config() -> ConfigModule {
    let config = ConfigModule {
        base: Option::from(BaseModule {
            config_path: Option::from("config.toml".to_string()),
            access_log: Option::from(true),
            error_log: Option::from(true),
        }),
        security: Option::from(SecurityModule {
            key: Option::from("".to_string()),
            cert: Option::from("".to_string()),
        }),
        http: Option::from(HttpModule {
            server: Option::from(vec![ServerModule { port: None, listen: None }])
        }),
        proxy: Option::from(ProxyModule {
            api: Option::from("".to_string())
        }),
        logs: Option::from(LogsModule {
            access: Option::from("logs/access.log".to_string()),
            error: Option::from("logs/error.log".to_string()),
        }),
    };

    config
}

/**
* @desc get profile of configuration
*/
fn get_outside_config(arg_value: &String) -> ConfigModule {
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
    let config: ConfigModule = toml::from_str(&str_val).unwrap();
    config
}


/**
* @desc merge config
* @TODO How to merge multiple structures
*/
fn get_merge_config(arg_value: &String) -> (ConfigModule, ConfigModule) {
    let default_config = get_default_config();
    let outside_config = get_outside_config(arg_value);
    return (default_config, outside_config);
}
