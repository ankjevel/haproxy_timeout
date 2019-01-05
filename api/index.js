const {
  env: {
    PORT = 3000,
    TIMEOUT = 1000
  }
} = process

const express = require('express')
const bodyParser = require('body-parser')
const app = express()

app.set('timeout', TIMEOUT)

app.use(bodyParser.json())

const wait = () =>
  new Promise(resolve =>
    setTimeout(resolve, app.get('timeout')))

app.get('/', (_req, res) => {
  console.log(`/ on port: ${PORT}`)

  res.json({
    data: `${app.get('timeout')}ms on port: ${PORT}`
  })
})

app.get('/health', async (_req, res) => {
  console.log(`/health on port: ${PORT}`)

  const start = Date.now()

  await wait()

  const time = Date.now() - start

  res.json({
    oracle: {
      healthy: time < 30000,
      time,
      port: +PORT
    }
  })
})

app.post('/timeout', (req, res) => {
  let { body: { timeout = TIMEOUT } = {} } = req

  timeout = Number.parseFloat(timeout)

  if (!isNaN(timeout)) {
    app.set('timeout', timeout)
  }

  res.sendStatus(200)
})

app.listen(PORT, () => console.log(`Listening on port ${PORT}`))
