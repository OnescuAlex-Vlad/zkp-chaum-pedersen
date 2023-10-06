# ZKP gRPC client/server for authentication

## Docker

You can run the program with Docker. First build the containers:

```
$ docker-compose build zkpserver
```

Run the container:

```
$ docker-compose run --rm zkpserver
```

In the remote terminal that appears run the server:

```
root@41a0b4cc6b3b:/zkp-server# cargo run --bin server --release
```

Open a new terminal on your machine and connect to the container:

```
$ docker container ls
CONTAINER ID   IMAGE                  COMMAND   CREATED          STATUS          PORTS     NAMES
41a0b4cc6b3b   zkp-zkpserver   "bash"    20 minutes ago   Up 20 minutes             zkp_zkpserver_run_b1f3fa2cd94a

$ docker exec -it 41a0b4cc6b3b /bin/bash
```

Run the client:

```
root@41a0b4cc6b3b:/zkp-server# cargo run --bin client --release
```

