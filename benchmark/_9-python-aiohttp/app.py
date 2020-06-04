from aiohttp import web

app = web.Application()


async def index(request):
    return web.Response(text="Hello World!", content_type="text/html")

app.router.add_get('/', index)

def server_app(application):
    web.run_app(application,port=8087)


if __name__ == '__main__':
    server_app(app)