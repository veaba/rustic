package main

import (
	"github.com/kataras/iris/v12"
)

func main() {
	app := iris.New()
	app.Handle("GET", "/", func (ctx iris.Context)  {
		ctx.WriteString("Hello World!")
	})
	app.Run(iris.Addr(":8086"))
}

