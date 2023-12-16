# GRPC Server Generator

[![Build Template](https://github.com/codeitlikemiley/server_template/actions/workflows/build.yml/badge.svg)](https://github.com/codeitlikemiley/server_template/actions/workflows/build.yml)

## Requirements

Note: Install the following before you can use the template generator.

- [rust, rustup, cargo](https://www.rust-lang.org/tools/install)
- [cargo-workspace](https://crates.io/crates/cargo-workspaces)
- [cargo-generate](https://cargo-generate.github.io/cargo-generate/installation.html)
- [protoc](https://grpc.io/docs/protoc-installation/)


## Project Structure

```sh
workspace (root)
├── Cargo.toml
│
├── server <-- Generate Template this from **server_template**
│   └── .env.example
│   └── rust-toolchain.toml
│   └── Cargo.toml
│   └── README.md
│
├── frontend
│
└── services
```

## Usage

### Create a workspace

```sh
cd <workspace_root>
cargo workspaces init .
```
or you can simply create a `Cargo.toml` with this content

```toml
[workspace]
resolver = "2"
members = [
    server, # <- add this
   # add here services here after you generated it
]
```

### Generate a new GRPC server template.

```sh
cargo generate --git codeitlikemiley/server_template --name server
cd server
```

### Build , Testing and Documentation

```sh
cargo build
cargo test
cargo doc --open
```

### Run  Server

```sh
cargo run -p server
```

### Adding services

To add more services please check this [Services Template Repository](https://github.com/codeitlikemiley/services_template)


## [License](LICENSE)

## Pull Requests

If you need to make changes to the template, please submit a pull request.
