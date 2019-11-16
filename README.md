# drap

Docker app written in rust.

This is just a test... don't read the code, I don't know rust.

Before trying to run a bundle you need to run this:
```
docker run -d -v /var/run/docker.sock:/var/run/docker.sock -p 127.0.0.1:1234:1234 bobrik/socat TCP-LISTEN:1234,fork UNIX-CONNECT:/var/run/docker.sock
```
