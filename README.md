### Running MUMPS using YottaDB
I'm running the MUMPS programs using YottaDB on my Raspberry Pi 4 (`aarch64`). I want to use Docker to avoid having a system install, but YottaDB only publishes `amd64` images so I need to [build the Dockerfile](https://docs.yottadb.com/AdminOpsGuide/dockercontainer.html#build-steps) first:

```
$ git clone https://gitlab.com/YottaDB/DB/YDB.git
$ cd YDB
$ docker build -t yottadb/yottadb:latest .
```

Then for each puzzle I work in a subdirectory and mount it as the `/data` mount for the YDB container:
```
$ cd 9
$ docker run -it --volume `pwd`:/data --rm  yottadb/yottadb:latest
```

Finally, to recompile and execute the routine:
```
YDB>zlink "BASIN" d ^BASIN
```