# Named Pipe to TCP Proxy

Created so that programs which use Named Pipes on Windows can be proxyed to TCP Sockets

For example:
- I use Docker without Docker Desktop by installing it on WSL2 containers
- I start TCP server for Docker daemon
- I do not wish to modify the Windows DOCKER_HOST as I would have to change it at a lot of places.  
  So I do nothing there
- I start this binary and request it to proxy from the Docker Host Default Named Pipe to the WSL2 TCP Sockets

## Multiple Connection support

- Supports multiple connections at the same time.

## Contact me

You can contact me [via LinkedIn](https://in.linkedin.com/in/pratik-chowdhury-889bb2183)
