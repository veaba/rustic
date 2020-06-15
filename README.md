# Rustic

受Nginx、与BFE 启发，基于测试web服务支持并发的比较结果，想开发一款基于rust 编程语言的HTTP 服务器代理器，用来测试rust 在web方向的极限

比较常见的编程语言和web服务器，rust 的hyper表现的太过卓越！go 已有了BFE。

rust 可以build 为一个可执行文件，这就很方便了。如果嵌入一个内置的rust web server 理论上就可以实现一个类 nginx 的web服务

开发期间，不会考虑内存等性能问题。

自我怀疑，使用`nginx` 作为比较是不是走弯路了？？

## 疑问

- 如何实现类似nginx worker 呢？ 支持更多的worker，并发拉满？
- 集群模式下，如何与其他机子绑定？
- `panic!` ?

## 方案设计

要求

|feature| 描述 |
|-------| --- |
|`hot-load`|热加载|
|`Cache`|缓存|
|`keep-alive`||
|`proxy`||
|`gzip`||
|`log`||
|`protocol`||
|`plugins`||
|`load banlans`|[百度BFE](https://www.bfe-networks.net/zh_cn/)|
|||

### 协议支持
- http
- https
- spdy
- http/2
- websocket
- tls
- gRpc
- http/3

### 配置文件

- 热重载配置文件
- 配置文件，config.yaml

```yaml
root: "绝对路径"
load-balance: ""
http: http
http2: http2
rewrite: /world/anywhere/
proxy:
  /aa/: /bb/
  /cc/: /dd/
log:
  path: /usr/xx.log
```

## 对比测试结果

* 测试的方法不当可能影响会结果，以下为个人的测试结果，仅供参考，测试代码用例见 `/benchmark`：
* 基于`autocannon` 测试
 

### 排名总览

- 期待未来补充 java、php

|rank|language|web 服务| Req/Sec 每秒平均请求(avg) | 5s总请求数据(total in 5s)|
|----|----|-------|---------------------|------------|
|1|go      | iris                | 68784| 344k requests in 5.63   | 
|2|Rust    | hyper               | 67644| 338k requests in 5.4s  | 
|3|Rust    | nickel              | 44892| 224k requests in 5.17s  | 
|4|node(js)| fastify(fast mode)  | 42441| 212k requests in 5.1s  | 
|5|node(js)| fastify(normal)     | 40995| 205k requests in 5.11s  | 
|6|node(js)| express             | 22103| 125k requests in 5.08s | 
|7|deno(ts)| oak                 | 20481| 102k requests in 5.09s | 
|8|nginx(c)|                     | 11098| 55k requests in 5.05s  |
|9|python  | aiohttp             | 5015 | 25k requests in 5.07s  | 


## API

rustic config=xxx.toml 文件
- 默认同目录下的 config.toml 或者yaml、json文件

- `config` config.toml
- `port` 80
- `test` 测试配置文件
- [x] `?`
- [x] `help`
- `key`
- `cert`
- [x] `version`
- [x] `v`
- [x] `ver`
- [x] `author`  
- start 
- restart
- stop
- exit

### default mode

```shell
cargo run
# or : rustic
```

- 此模式下等同于：rustic config=config.toml 
  - 起 80 端口
  - 执行文件所在目录的查找config.toml
  - 如果没有config.toml 则调用内置config
```toml

[root]
root="./html" # 执行文件的当前目录下./html，生成默认的`index.html`、`400.html`、`500.html`

[http]
port="80"
```

### config 自定义配置文件

> rustic config=config.toml

调用用户自定义的`config.toml` 文件

### port 自定义端口

- 默认：80

> rustic port=80

