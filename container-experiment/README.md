# Exploring running Rust apps in containers

## Build and run the Docker image

To build:

```shell
$ docker build -t container-experiment:latest .
```

To run:

```shell
$ docker run -it --rm container-experiment:latest
Num logical cpus: 4
Num physical cpus: 4
```

To constrain the number of logical CPUs available to the container, use [cpuset-cpus](https://docs.docker.com/engine/reference/run/#cpuset-constraint):

```shell
$ docker run -it --rm --cpuset-cpus="0-2" container-experiment:latest
Num logical cpus: 3
Num physical cpus: 4
```

## References

* [rust - Docker Hub](https://hub.docker.com/_/rust/)
* [num_cpus - Rust](https://docs.rs/num_cpus/latest/num_cpus/)
