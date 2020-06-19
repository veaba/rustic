/**
* @desc parser config struct
* @reference Nginx document
*/
// Main Module
struct MainModule {
    // TODO auto 1 2 3 4 5 6
    worker_processes: Option<u32>,
    event: Option<EventModule>,
    user: Option<String>,
    error_log: Option<String>,
    pid: Option<String>,
    worker_rlimit_nofile: Option<u32>,
    http: Option<HttpModule>,
}


// Event Module
struct EventModule {
    worker_connections: Option<u32>,
}

// Http Module
struct HttpModule {
    include: Option<String>,
    default_type: Option<String>,
    log_format: Option<String>,
    server_name_hash_bucket_size: Option<u32>,
    client_header_buffer_size: Option<String>,
    sendfile: Option<String>,

    // on | off
    gzip: Option<String>,
    gzip_comp_level: Option<u32>,
    gzip_buffers: Option<u32>,

    // text/plain  text/css application/javascript application/x-javascript text/xml application/xml application/xml+rss text/javascript;
    gzip_types: Option<String>,

    //TODO {1.1}
    gzip_http_version: Option<String>,
    keepalive_timeout: Option<String>,
    server: Option<ServerModule>,
    location: Option<Vec<String>>,
}

// Server Module
struct ServerModule {
    listen: Option<u32>,
    location: Option<Vec<LocationModule>>,
}

// Location Module
struct LocationModule {
    root: Option<String>,
    index: Option<String>,
}

// Upstream Module


/** Rust syntax config demo
* @TODO need `conf` file translate => `json` struct file
{
    user: "www www",
    worker_processes: "2",
    error_log: "/usr/local/rustic/logs/error.log cert", //log path and level
    pid: "/usr/local/rustic/logs/rustic.pid', // pid path
    worker_rlimit_nofile: 65535, //指定此进程可以打开的最大文件描述符的值。
    events:{
        use: "epoll", //what is mean?
        worker_connection:65535, // Specifies value ?
    },
    http:{
        gzip: "on", // default on, on|off

        server:{
            listen: 80,
            server_name: "localhost", // domain
            index: "index.html index.htm index.php",
            root: "/srv/www/", // site dir
            location:[
                {
                    "/":"what?",
                    "~.*\.(js|css)?$":"what ??",
                    "api":"to api proxy"
                }
            ],
            access_log:"off", // default on, value: on| off
        }
    }
}




*/


