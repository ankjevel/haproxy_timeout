# Proxy-test

Start with: `docker-compose up --build`

[Visit the haproxy stats route](http://localhost:5001/haproxy?stats)


To change timeout, send a POST-request to one of the api's to the route /timeout

```sh
curl -XPOST \
  localhost:3000/timeout \
  -d '{"timeout":3000}' \
  -H 'content-type: application/json'
```
