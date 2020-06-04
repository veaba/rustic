const fastify = require('fastify')()

fastify.route({
  method: 'GET',
  url: '/',
  schema: {
    // request needs to have a querystring with a `name` parameter
    querystring: {
      name: { type: 'string' }
    },
    // the response needs to be an object with an `hello` property of type 'string'
    response: {
      200: {
        type: 'object',
        properties: {
          hello: { type: 'string' }
        }
      }
    }
  },
  // this function is executed for every request before the handler is executed
  preHandler: async (request, reply) => {
    // E.g. check authentication
  },
  handler: async (request, reply) => {
    return { hello: 'world' }
  }
})

const start = async () => {
  try {
    await fastify.listen(8083)
    // fastify.log.info(`server listening on ${fastify.server.address().port}`)
  } catch (err) {
    // fastify.log.error(err)
    process.exit(1)
  }
}
start()