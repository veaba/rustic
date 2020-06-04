# Rustic

受Nginx、与BFE 启发，基于前阵子测试web服务支持并发的比较结果，想开发一款基于rust 编程语言的HTTP 服务器代理器。比较常见时间的编程语言和web服务器，rust 的hyper表现的太过卓越！


## 方案设计
要求
- 七层负载均衡 见百度的bfe： https://www.bfe-networks.net/zh_cn/
- 可拓展插件
- 丰富的协议：http https spdy http/2 websocket tls gRpc http/3
- 一键增容
- log 分析
- gzip 压缩


### 协议支持

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

## 测试结果

* 测试的方法不当可能影响结果，以下为个人的测试结果，仅供参考，测试代码用例见 `/benchmark`：
(基于`autocannon` 测试)



### 排名总览

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
