# blr

<p align="left">
  <a href="https://crates.io/crates/blr">                                     <img alt="crates.io"  src="https://img.shields.io/crates/v/blr.svg"></a>
  <a href="https://docs.rs/blr">                                              <img alt="docs.rs"    src="https://docs.rs/blr/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/blr/actions/workflows/rust.yml">   <img alt="Rust"       src="https://github.com/AndrejOrsula/blr/actions/workflows/rust.yml/badge.svg"></a>
  <a href="https://github.com/AndrejOrsula/blr/actions/workflows/docker.yml"> <img alt="Docker"     src="https://github.com/AndrejOrsula/blr/actions/workflows/docker.yml/badge.svg"></a>
  <a href="https://deps.rs/repo/github/AndrejOrsula/blr">                     <img alt="deps.rs"    src="https://deps.rs/repo/github/AndrejOrsula/blr/status.svg"></a>
  <a href="https://codecov.io/gh/AndrejOrsula/blr">                           <img alt="codecov.io" src="https://codecov.io/gh/AndrejOrsula/blr/branch/main/graph/badge.svg"></a>
</p>

Rust interface for the Python API of [Blender](https://www.blender.org). The interface is facilitated via [PyO3](https://pyo3.rs).

## Instructions

### <a href="#-rust"><img src="https://rustacean.net/assets/rustacean-flat-noshadow.svg" width="16" height="16"></a> Rust

Add `blr` as a Rust dependency to your [`Cargo.toml`](https://doc.rust-lang.org/cargo/reference/manifest.html) manifest.

```toml
[dependencies]
blr = { version = "0.1" }
```

<details>
<summary><h3><a href="#-docker"><img src="https://www.svgrepo.com/show/448221/docker.svg" width="16" height="16"></a> Docker</h3></summary>

> To install [Docker](https://docs.docker.com/get-docker) on your system, you can run [`.docker/host/install_docker.bash`](.docker/host/install_docker.bash) to configure Docker with NVIDIA GPU support.
>
> ```bash
> .docker/host/install_docker.bash
> ```

#### Build Image

To build a new Docker image from [`Dockerfile`](Dockerfile), you can run [`.docker/build.bash`](.docker/build.bash) as shown below.

```bash
.docker/build.bash ${TAG:-latest} ${BUILD_ARGS}
```

#### Run Container

To run the Docker container, you can use [`.docker/run.bash`](.docker/run.bash) as shown below.

```bash
.docker/run.bash ${TAG:-latest} ${CMD}
```

#### Run Dev Container

To run the Docker container in a development mode (source code mounted as a volume), you can use [`.docker/dev.bash`](.docker/dev.bash) as shown below.

```bash
.docker/dev.bash ${TAG:-latest} ${CMD}
```

As an alternative, users familiar with [Dev Containers](https://code.visualstudio.com/docs/devcontainers/containers) can modify the included [`.devcontainer/devcontainer.json`](.devcontainer/devcontainer.json) to their needs. For convenience, [`.devcontainer/open.bash`](.devcontainer/open.bash) script is available to open this repository as a Dev Container in VS Code.

```bash
.devcontainer/open.bash
```

#### Join Container

To join a running Docker container from another terminal, you can use [`.docker/join.bash`](.docker/join.bash) as shown below.

```bash
.docker/join.bash ${CMD:-bash}
```

</details>

## Status

This project is in early development, and as such, many features are missing and the API is not yet stable. The initial implementation targets Blender `3.6`, but it will be updated to `4.x` in the future.

## License

This project is dual-licensed to be compatible with the Rust project, under either the [MIT](LICENSE-MIT) or [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
