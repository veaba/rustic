# Benchmark


## Use autocannon test 

> autocannon -c 100 -d 5 -p 100 http://127.0.0.1

## rust-nickel
- rust
- cargo

cd rust-nickel

```cmd
cargo run 
```

somethind error:
> Error: Failed to start response: 您的主机中的软件中止了一个已建立的连接。 (os error 10053)


## rust-nickel
- rust
- cargo

cd rust-hyper

```cmd
cargo run 
```

## node-fastify-fast
- node 
- npm

cd node-fastify-fast

```cmd
npm install 
node app.js
```

## node-fastify-normal
- node
- npm 

cd node-fastify-normal

```cmd
npm install 
node app.js
```

## node-express
- node
- npm

cd node-express

```cmd
npm install 
node app.js
```

## go-iris
- go
- go mod

cd go-iris

```cmd
go mod init main
go run app.go
```

## python aiohttp
- pip
- python3

cd python-aiohttp

```cmd
python app.py
```

## node-express
- node
- npm

cd node-express 

```cmd
npm install
node app.js
```