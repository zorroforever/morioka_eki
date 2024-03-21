My simple RPG client:
* Written in RUST.
* A terminal program.
* Action base on send commands.
```
login
exit
```

* This program connects to the server using the TCP/IP protocol..
* Configure
```shell
server endpoint base url:
BASE_URL="http://127.0.0.1:8000"
server endpoint fetch_token api url:
API_FETCH_TOKEN="/fetch_token/"
server endpoint api url:
API_UNION="/v1/api/"
encryption key：  
ENCRYPTION_KEY="8ea8593bb2e44ccda1ccbb1fa07db5b6"
```