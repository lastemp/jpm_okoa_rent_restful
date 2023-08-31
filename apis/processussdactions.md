# processussdactions API

All examples show cURL and [HTTPie](https://httpie.io/cli) snippets.

## Initiate processussdactions

```sh
curl -d 'sessionId=ikskjsysgayauua&phoneNumber=254700000000&text=' -H 'Content-Type: application/x-www-form-urlencoded' http://localhost:8080/processussdactions

http POST :8080/processussdactions sessionId=ikskjsysgayauua&phoneNumber=254700000000&text=
```

The response should be a 200 OK with the following text/plain:

```text/plain
CON Welcome to Okoa Rent\Mortgage Service, a Real Estate Industry Revolution.
1. Self Register
```

#### NB

Once you register your callback URL, any requests we receive belonging to your service code will trigger a HTTP POST to the registered callback URL.
This request will be sent with content type of application/x-www-form-urlencoded.
