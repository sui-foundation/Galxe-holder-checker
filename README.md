# Galxe-holder-checker
A proxy service for Galxe to check on a coin holder

### How to use it
- The server can be setup with multiple options (`-c ${coin type}=${package id}`) to check a holder has `Coin` more than some number.
- The query is `/${coin type}/${expect value}?address=${user address}`.
- The response is `1` or `0` for true or false, and `-1` if the upstream rpc server is down.

### Quick Example with USDC
The following is an example to query to know a user has 10 USDC or not.

#### Run a service check on USDC
`cargo run -- -c usdc=0xdba34672e30cb065b1f93e3ab55318768fd6fef66c15942c9f7cb846e2f900e7`


#### An account has USDC coins, which are more than 10.

`curl -v 'http://127.0.0.1:3000/usdc/10?address=0x443cf42b0da43c230bff7a64e69ce25d24d65f49e7c9db6adecc0bd176dba79a'`

```console
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
> GET /usdc/10?address=0x443cf42b0da43c230bff7a64e69ce25d24d65f49e7c9db6adecc0bd176dba79a HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.7.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 1
< date: Wed, 09 Apr 2025 08:28:58 GMT
<
* Connection #0 to host 127.0.0.1 left intact
1
```

#### An account has USDC coins, which are less than 10.
`curl -v 'http://127.0.0.1:3000/usdc/10?address=0xc0378ad702e323a054580d9c72dff0ce08f0e6ced5a42e8c5db8f5a2b216402e'`

```console
*   Trying 127.0.0.1:3000...
* Connected to 127.0.0.1 (127.0.0.1) port 3000
> GET /usdc/10?address=0xc0378ad702e323a054580d9c72dff0ce08f0e6ced5a42e8c5db8f5a2b216402e HTTP/1.1
> Host: 127.0.0.1:3000
> User-Agent: curl/8.7.1
> Accept: */*
>
* Request completely sent off
< HTTP/1.1 200 OK
< content-length: 1
< date: Wed, 09 Apr 2025 08:30:38 GMT
<
* Connection #0 to host 127.0.0.1 left intact
0
```

