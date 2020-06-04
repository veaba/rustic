import {Application, Router} from "https://deno.land/x/oak";

const app = new Application();
const router = new Router()

router.get('/', (ctx) => {
    ctx.response.body = "Hello world"
})

const routes = router.routes()

app.use(routes)

await app.listen("127.0.0.1:8000");
